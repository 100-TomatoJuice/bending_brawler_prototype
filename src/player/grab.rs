use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::{Ccd, Damping, GravityScale, ReadMassProperties, RigidBody, Velocity},
    geometry::{ActiveEvents, Collider, CollisionGroups, Friction, Group},
};
use leafwing_input_manager::prelude::ActionState;

use crate::sandbox::{particle_types::get_particle, sandbox::Sandbox};

use super::{Action, AimDirection, HeldObject, Radius, Range};

pub struct GrabPlugin;

impl Plugin for GrabPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GrabState>().add_systems(
            Update,
            (
                (
                    increase_grab_radius,
                    (grab_dirt, release_grab).chain(),
                    release_held,
                )
                    .chain(),
                move_object,
                update_actual_velocity,
                place_back,
                place_back_mini,
                place_back_particle,
            ),
        );
    }
}

#[derive(Component)]
pub struct ParentObject;

#[derive(Component)]
pub struct Held;

#[derive(Component)]
pub struct PutBackIntoSandbox;

#[derive(Component)]
pub struct Rock;

#[derive(Component)]
pub struct Owner(pub Entity);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GrabState {
    #[default]
    Empty,
    Holding,
}

#[derive(Component, Default)]
pub struct ActualVelocity {
    pub linvel: Vec2,
    pub previous_pos: Vec3,
}

fn increase_grab_radius(mut query: Query<(&ActionState<Action>, &mut Radius)>, time: Res<Time>) {
    for (action, mut radius) in query.iter_mut() {
        if action.pressed(&Action::Grab) {
            let speed = time.delta_seconds() * radius.speed;
            radius.ease_out_expo(speed);
        }
    }
}

fn release_grab(mut query: Query<(&ActionState<Action>, &mut Radius)>) {
    for (action, mut radius) in query.iter_mut() {
        if action.just_released(&Action::Grab) {
            radius.current = radius.min;
        }
    }
}

fn release_held(
    mut commands: Commands,
    mut query: Query<(&ActionState<Action>, &mut HeldObject)>,
    mut parent_query: Query<
        (Entity, &mut Velocity, &mut GravityScale),
        (With<ParentObject>, With<Held>),
    >,
    mut state: ResMut<NextState<GrabState>>,
) {
    for (action, mut held) in query.iter_mut() {
        let Some(entity) = held.0 else {
            continue;
        };
        if action.pressed(&Action::Grab) {
            held.0 = None;
            state.set(GrabState::Empty);

            let Ok((entity, mut velocity, mut scale)) = parent_query.get_mut(entity) else {
                continue;
            };
            commands.entity(entity).remove::<Held>();
            scale.0 = 1.0;
            velocity.linvel *= 2.0;
        }
    }
}

fn grab_dirt(
    mut commands: Commands,
    mut sandbox_query: Query<&mut Sandbox>,
    mut player_query: Query<(
        Entity,
        &Transform,
        &ActionState<Action>,
        &AimDirection,
        &Radius,
        &Range,
        &mut HeldObject,
    )>,
    mut state: ResMut<NextState<GrabState>>,
) {
    let mut sandbox = sandbox_query.single_mut();

    for (entity, transform, action, aim, radius, range, mut held) in player_query.iter_mut() {
        if held.0.is_some() {
            continue;
        }

        if action.just_released(&Action::Grab) {
            let min = -(radius.current / 8.0).round() as i32;
            let max = (radius.current / 8.0).round() as i32;

            let (grid_x, grid_y) = (
                ((transform.translation.x + aim.0.x * (range.0 + radius.current)) / 8.0).floor()
                    as i32
                    + sandbox.width() as i32 / 2,
                ((transform.translation.y + aim.0.y * (range.0 + radius.current)) / 8.0).floor()
                    as i32
                    + sandbox.height() as i32 / 2,
            );

            let mut dirt = vec![];

            for offset_x in min..=max {
                for offset_y in min..=max {
                    let x = grid_x + offset_x;
                    let y = grid_y + offset_y;

                    if !sandbox.out_of_bounds_i32(x, y) && sandbox.checked_get_i32(x, y).is_some() {
                        sandbox.set(x as usize, y as usize, None);
                        dirt.push(
                            commands
                                .spawn((
                                    Name::new("Rock"),
                                    SpriteBundle {
                                        sprite: Sprite {
                                            color: Color::srgb_u8(189, 139, 139),
                                            custom_size: Some(Vec2::new(8.0, 8.0)),
                                            ..default()
                                        },
                                        transform: Transform::from_translation(Vec3::new(
                                            offset_x as f32 * 8.0,
                                            offset_y as f32 * 8.0,
                                            0.1,
                                        )),
                                        ..default()
                                    },
                                    Collider::cuboid(3.0, 3.0),
                                    Ccd::enabled(),
                                    Friction::coefficient(0.0),
                                    ActiveEvents::COLLISION_EVENTS,
                                    CollisionGroups::new(
                                        Group::all(),
                                        Group::all().difference(Group::GROUP_1),
                                    ),
                                    Velocity::default(),
                                    ActualVelocity::default(),
                                    Rock,
                                ))
                                .id(),
                        );
                    }
                }
            }

            if !dirt.is_empty() {
                held.0 = Some(
                    commands
                        .spawn((
                            Name::new("Rock Parent"),
                            SpatialBundle::from_transform(Transform::from_translation(
                                transform.translation
                                    + (aim.0 * (range.0 + radius.current)).extend(0.0),
                            )),
                            RigidBody::Dynamic,
                            GravityScale(0.0),
                            ReadMassProperties::default(),
                            Velocity::default(),
                            ActualVelocity::default(),
                            Ccd::enabled(),
                            Damping {
                                linear_damping: 0.0,
                                angular_damping: 100.0,
                            },
                            Owner(entity),
                            ParentObject,
                            Held,
                        ))
                        .push_children(&dirt)
                        .id(),
                );
                state.set(GrabState::Holding);
            }
        }
    }
}

pub fn move_object(
    player_query: Query<
        (&Transform, &AimDirection, &Range, &Radius, &HeldObject),
        Without<ParentObject>,
    >,
    mut parent_query: Query<
        (&Transform, &mut Velocity, &ReadMassProperties),
        (With<ParentObject>, With<Held>),
    >,
) {
    for (transform, aim, range, radius, held) in player_query.iter() {
        let Some(parent_entity) = held.0 else {
            continue;
        };

        let Ok((parent_transform, mut velocity, properties)) = parent_query.get_mut(parent_entity)
        else {
            continue;
        };
        let desired_pos = transform.translation.truncate() + (aim.0 * (range.0 + radius.current));
        let force = (desired_pos - parent_transform.translation.truncate()) * 400.0;
        let mass = if properties.mass.is_nan() || properties.mass <= f32::EPSILON {
            64.0
        } else {
            properties.mass.sqrt()
        };

        velocity.linvel = force / mass;
    }
}

fn update_actual_velocity(mut query: Query<(&Transform, &mut ActualVelocity), Changed<Transform>>) {
    for (transform, mut velocity) in query.iter_mut() {
        let new_velocity = transform.translation - velocity.previous_pos;
        velocity.linvel = new_velocity.truncate();
        velocity.previous_pos = transform.translation;
    }
}

fn place_back(
    mut commands: Commands,
    mut sandbox_query: Query<&mut Sandbox>,
    parent_query: Query<(Entity, &Velocity), (With<ParentObject>, Without<Held>)>,
    children_query: Query<&Children>,
    transform_query: Query<&GlobalTransform>,
) {
    let mut sandbox = sandbox_query.single_mut();

    for (entity, velocity) in parent_query.iter() {
        if velocity.linvel.length_squared() > 0.1 {
            continue;
        }

        for child in children_query.iter_descendants(entity) {
            let Ok(position) = transform_query.get(child).map(|x| x.translation()) else {
                continue;
            };
            let (grid_x, grid_y) = (
                (position.x / 8.0).floor() as i32 + sandbox.width() as i32 / 2,
                (position.y / 8.0).floor() as i32 + sandbox.height() as i32 / 2,
            );

            if !sandbox.out_of_bounds_i32(grid_x, grid_y) {
                sandbox.set(
                    grid_x as usize,
                    grid_y as usize,
                    Some(get_particle(
                        crate::sandbox::particle_types::ParticleTypes::Dirt,
                    )),
                );
            }
        }

        commands.entity(entity).despawn_recursive();
    }
}

fn place_back_mini(
    mut commands: Commands,
    mut sandbox_query: Query<&mut Sandbox>,
    parent_query: Query<(Entity, &Velocity, &Parent), (With<Rock>, With<PutBackIntoSandbox>)>,
    held_query: Query<(), With<Held>>,
    transform_query: Query<&GlobalTransform>,
) {
    let mut sandbox = sandbox_query.single_mut();

    for (entity, velocity, parent) in parent_query.iter() {
        if velocity.linvel.length_squared() > 0.1 || held_query.contains(parent.get()) {
            continue;
        }

        let Ok(position) = transform_query.get(entity).map(|x| x.translation()) else {
            continue;
        };
        let (grid_x, grid_y) = (
            (position.x / 8.0).floor() as i32 + sandbox.width() as i32 / 2,
            (position.y / 8.0).floor() as i32 + sandbox.height() as i32 / 2,
        );

        if !sandbox.out_of_bounds_i32(grid_x, grid_y) {
            sandbox.set(
                grid_x as usize,
                grid_y as usize,
                Some(get_particle(
                    crate::sandbox::particle_types::ParticleTypes::Dirt,
                )),
            );
        }

        commands.entity(entity).despawn_recursive();
    }
}

fn place_back_particle(
    mut commands: Commands,
    mut sandbox_query: Query<&mut Sandbox>,
    parent_query: Query<
        (Entity, &Velocity),
        (With<Rock>, With<PutBackIntoSandbox>, Without<Parent>),
    >,
    transform_query: Query<&GlobalTransform>,
) {
    let mut sandbox = sandbox_query.single_mut();

    for (entity, velocity) in parent_query.iter() {
        if velocity.linvel.length_squared() > 0.1 {
            continue;
        }

        let Ok(position) = transform_query.get(entity).map(|x| x.translation()) else {
            continue;
        };
        let (grid_x, grid_y) = (
            (position.x / 8.0).floor() as i32 + sandbox.width() as i32 / 2,
            (position.y / 8.0).floor() as i32 + sandbox.height() as i32 / 2,
        );

        if !sandbox.out_of_bounds_i32(grid_x, grid_y) {
            sandbox.set(
                grid_x as usize,
                grid_y as usize,
                Some(get_particle(
                    crate::sandbox::particle_types::ParticleTypes::Dirt,
                )),
            );
        }

        commands.entity(entity).despawn_recursive();
    }
}
