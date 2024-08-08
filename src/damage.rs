use bevy::{prelude::*, utils::hashbrown::HashSet};
use bevy_rapier2d::prelude::*;

use crate::{
    player::{
        components::{HeldObject, PlayerHealth},
        grab::{move_object, ActualVelocity, Held, Owner, ParentObject, PutBackIntoSandbox, Rock},
    },
    sandbox::{collider::Ground, sandbox::Sandbox},
};

const OVERPOWERDIFFERENCE: f32 = 20000.0;
const VELOCITYTHRESHOLD: f32 = 500.0;
const GROUNDVELOCITYTHRESHOLD: f32 = 5.0;
const BREAKGROUNDVELOCITYTHRESHOLD: f32 = 10.0;
const BREAKRADIUS: i32 = 1;
const VELOCITYVSEXTERNALDIFFERENCE: f32 = 200.0;
const DESPAWNFALLENY: f32 = -1000.0;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                damage_player,
                despawn_player,
                damage_rock,
                break_from_ground,
                despawn_fallen_rocks,
            ),
        );
    }
}

fn damage_player(
    mut player_query: Query<(&Transform, &mut ExternalImpulse, &mut PlayerHealth)>,
    collider_query: Query<(&GlobalTransform, &Parent), With<Rock>>,
    rock_query: Query<
        (Entity, &Transform, &Owner, &Velocity, &ReadMassProperties),
        With<ParentObject>,
    >,
    mut events: EventReader<CollisionEvent>,
) {
    for event in events.read() {
        let CollisionEvent::Started(e1, e2, _) = event else {
            continue;
        };

        if let Ok((transform, mut external, mut health)) = player_query.get_mut(*e2) {
            if let Ok((colldier_transform, parent)) = collider_query.get(*e1) {
                let Ok((entity, colldier_transforma, owner, velocity, mass)) =
                    rock_query.get(parent.get())
                else {
                    continue;
                };
                if velocity.linvel.length() < VELOCITYTHRESHOLD || owner.0 == *e2 {
                    continue;
                }

                health.0 -= (velocity.linvel.length() * mass.mass).sqrt();

                let direction = (colldier_transform.translation() - transform.translation)
                    .truncate()
                    .normalize_or_zero();
                external.impulse =
                    -direction * (velocity.linvel.length() * mass.mass).sqrt() * 5000.0;

                println!("V: {}", velocity.linvel.length());
                println!(
                    "Damage {} by {}",
                    *e2,
                    (velocity.linvel.length() * mass.mass).sqrt()
                );
                continue;
            }
        }
        if let Ok((transform, mut external, mut health)) = player_query.get_mut(*e1) {
            if let Ok((colldier_transform, parent)) = collider_query.get(*e2) {
                let Ok((entity, colldier_transforma, owner, velocity, mass)) =
                    rock_query.get(parent.get())
                else {
                    continue;
                };
                if velocity.linvel.length() < VELOCITYTHRESHOLD || owner.0 == *e1 {
                    continue;
                }

                health.0 -= (velocity.linvel.length() * mass.mass).sqrt();

                let direction = (colldier_transform.translation() - transform.translation)
                    .truncate()
                    .normalize_or_zero();
                external.impulse =
                    -direction * (velocity.linvel.length() * mass.mass).sqrt() * 5000.0;

                println!(
                    "Damage {} by {}",
                    *e1,
                    (velocity.linvel.length() * mass.mass).sqrt()
                );
                continue;
            }
        }
    }
}

fn despawn_player(mut commands: Commands, query: Query<(Entity, &PlayerHealth)>) {
    for (entity, health) in query.iter() {
        if health.0 <= 0.0 {
            warn!("Player {} has died", entity);
            commands.entity(entity).despawn();
        }
    }
}

fn despawn_fallen_rocks(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), With<Rock>>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation().y <= DESPAWNFALLENY {
            commands.entity(entity).despawn();
        }
    }
}

fn damage_rock(
    mut commands: Commands,
    collider_query: Query<&Parent, With<Rock>>,
    rock_query: Query<(&Velocity, &ReadMassProperties), With<ParentObject>>,
    children_query: Query<&Children>,
    transform_query: Query<&GlobalTransform>,
    mut events: EventReader<CollisionEvent>,
) {
    let mut broke_both: HashSet<Entity> = HashSet::new();

    for event in events.read() {
        let CollisionEvent::Started(e1, e2, _) = event else {
            continue;
        };

        let Ok(p1) = collider_query.get(*e1) else {
            continue;
        };
        let Ok(p2) = collider_query.get(*e2) else {
            continue;
        };

        let e1 = p1.get();
        let e2 = p2.get();

        let Ok((v1, m1)) = rock_query.get(e1) else {
            continue;
        };
        let Ok((v2, m2)) = rock_query.get(e2) else {
            continue;
        };

        let momentum1 = v1.linvel.length() * m1.mass;
        let momentum2 = v2.linvel.length() * m2.mass;

        if v1.linvel.length() < VELOCITYTHRESHOLD && v2.linvel.length() < VELOCITYTHRESHOLD {
            continue;
        }

        let difference = momentum1 - momentum2;

        if difference.is_sign_positive() && difference.abs() >= OVERPOWERDIFFERENCE {
            for child in children_query.iter_descendants(e2) {
                let Ok(translation) = transform_query.get(child).map(|x| x.translation()) else {
                    continue;
                };
                commands.spawn((
                    Name::new("Rock"),
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb_u8(89, 39, 39),
                            custom_size: Some(Vec2::new(8.0, 8.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(translation),
                        ..default()
                    },
                    Collider::cuboid(3.0, 3.0),
                    Ccd::enabled(),
                    RigidBody::Dynamic,
                    *v1,
                    CollisionGroups::new(Group::GROUP_1, Group::GROUP_1),
                    PutBackIntoSandbox,
                    ActiveEvents::COLLISION_EVENTS,
                    Rock,
                ));
            }
            commands.entity(e2).despawn_recursive();
            println!("Broke Rock");
            continue;
        }
        if difference.is_sign_negative() && difference.abs() >= OVERPOWERDIFFERENCE {
            for child in children_query.iter_descendants(e1) {
                let Ok(translation) = transform_query.get(child).map(|x| x.translation()) else {
                    continue;
                };
                commands.spawn((
                    Name::new("Rock"),
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb_u8(89, 39, 39),
                            custom_size: Some(Vec2::new(8.0, 8.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(translation),
                        ..default()
                    },
                    Collider::cuboid(3.0, 3.0),
                    Ccd::enabled(),
                    RigidBody::Dynamic,
                    *v1,
                    PutBackIntoSandbox,
                    CollisionGroups::new(Group::GROUP_1, Group::GROUP_1),
                    ActiveEvents::COLLISION_EVENTS,
                    Rock,
                ));
            }
            commands.entity(e1).despawn_recursive();
            println!("Broke Rock");
            continue;
        }

        if !broke_both.contains(&e2) {
            commands.entity(e2).despawn_recursive();
            for child in children_query.iter_descendants(e2) {
                let Ok(translation) = transform_query.get(child).map(|x| x.translation()) else {
                    continue;
                };
                commands.spawn((
                    Name::new("Rock"),
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb_u8(89, 39, 39),
                            custom_size: Some(Vec2::new(8.0, 8.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(translation),
                        ..default()
                    },
                    Collider::cuboid(3.0, 3.0),
                    Ccd::enabled(),
                    RigidBody::Dynamic,
                    *v2,
                    CollisionGroups::new(Group::GROUP_1, Group::GROUP_1),
                    PutBackIntoSandbox,
                    ActiveEvents::COLLISION_EVENTS,
                    Rock,
                ));
            }
            broke_both.insert(e2);
        }

        if !broke_both.contains(&e1) {
            commands.entity(e1).despawn_recursive();
            for child in children_query.iter_descendants(e1) {
                let Ok(translation) = transform_query.get(child).map(|x| x.translation()) else {
                    continue;
                };
                commands.spawn((
                    Name::new("Rock"),
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb_u8(89, 39, 39),
                            custom_size: Some(Vec2::new(8.0, 8.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(translation),
                        ..default()
                    },
                    Collider::cuboid(3.0, 3.0),
                    Ccd::enabled(),
                    RigidBody::Dynamic,
                    *v1,
                    CollisionGroups::new(Group::GROUP_1, Group::GROUP_1),
                    PutBackIntoSandbox,
                    ActiveEvents::COLLISION_EVENTS,
                    Rock,
                ));
            }
            broke_both.insert(e1);
        }

        println!("Broke Both Rocks");
    }
}

fn break_from_ground(
    mut commands: Commands,
    mut sandbox_query: Query<&mut Sandbox>,
    collider_query: Query<(Entity, &GlobalTransform, &Parent, &ActualVelocity), With<Rock>>,
    parent_query: Query<(&ActualVelocity, &Velocity), (With<ParentObject>)>,
    ground_query: Query<(), With<Ground>>,
    mut events: EventReader<CollisionEvent>,
) {
    let mut sandbox = sandbox_query.single_mut();

    for event in events.read() {
        let CollisionEvent::Started(e1, e2, _) = event else {
            continue;
        };

        if let Ok((entity, transform, parent, e)) = collider_query.get(*e1) {
            if !ground_query.contains(*e2) {
                continue;
            }

            let Ok((external, v)) = parent_query.get(parent.get()) else {
                continue;
            };

            if v.linvel.length() < GROUNDVELOCITYTHRESHOLD {
                continue;
            }

            if (v.linvel.length() - external.linvel.length()).abs() <= VELOCITYVSEXTERNALDIFFERENCE
            {
                // println!(
                //     "Difference not great rnought: Thres: {}, ABS: {}, V: {}, E: {}",
                //     VELOCITYVSEXTERNALDIFFERENCE,
                //     (external.linvel.length() - v.linvel.length()).abs(),
                //     v.linvel.length(),
                //     external.linvel.length()
                // );
                continue;
            }

            // println!(
            //     "Difference is great rnought: Thres: {}, ABS: {}, V: {}, E: {}",
            //     VELOCITYVSEXTERNALDIFFERENCE,
            //     (external.linvel.length() - v.linvel.length()).abs(),
            //     v.linvel.length(),
            //     external.linvel.length()
            // );

            commands.spawn((
                Name::new("Rock"),
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb_u8(89, 39, 39),
                        custom_size: Some(Vec2::new(8.0, 8.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(transform.translation()),
                    ..default()
                },
                Collider::cuboid(3.0, 3.0),
                Ccd::enabled(),
                RigidBody::Dynamic,
                *v,
                PutBackIntoSandbox,
                ActiveEvents::COLLISION_EVENTS,
                CollisionGroups::new(Group::GROUP_1, Group::GROUP_1),
                Rock,
            ));

            if v.linvel.length() >= BREAKGROUNDVELOCITYTHRESHOLD {
                let (grid_x, grid_y) = (
                    (transform.translation().x / 8.0).floor() as i32 + sandbox.width() as i32 / 2,
                    (transform.translation().y / 8.0).floor() as i32 + sandbox.height() as i32 / 2,
                );
                for x in -BREAKRADIUS..=BREAKRADIUS {
                    for y in -BREAKRADIUS..=BREAKRADIUS {
                        if !sandbox.out_of_bounds_i32(grid_x + x, grid_y + y) {
                            sandbox.set((grid_x + x) as usize, (grid_y + y) as usize, None);
                        }
                    }
                }
            }

            commands.entity(entity).despawn();
        }
        if let Ok((entity, transform, parent, external)) = collider_query.get(*e2) {
            if !ground_query.contains(*e1) {
                continue;
            }
            let Ok((external, velocity)) = parent_query.get(parent.get()) else {
                continue;
            };
            if (velocity.linvel.length() - external.linvel.length()).abs()
                <= VELOCITYVSEXTERNALDIFFERENCE
            {
                // println!(
                //     "Difference not great rnought: Thres: {}, ABS: {}, V: {}, E: {}",
                //     VELOCITYVSEXTERNALDIFFERENCE,
                //     (external.linvel.length() - velocity.linvel.length()).abs(),
                //     velocity.linvel.length(),
                //     external.linvel.length()
                // );
                continue;
            }

            commands.spawn((
                Name::new("Rock"),
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb_u8(89, 39, 39),
                        custom_size: Some(Vec2::new(8.0, 8.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(transform.translation()),
                    ..default()
                },
                Collider::cuboid(3.0, 3.0),
                Ccd::enabled(),
                RigidBody::Dynamic,
                *velocity,
                CollisionGroups::new(Group::GROUP_1, Group::GROUP_1),
                PutBackIntoSandbox,
                ActiveEvents::COLLISION_EVENTS,
                Rock,
            ));

            if velocity.linvel.length() >= BREAKGROUNDVELOCITYTHRESHOLD {
                let (grid_x, grid_y) = (
                    (transform.translation().x / 8.0).floor() as i32 + sandbox.width() as i32 / 2,
                    (transform.translation().y / 8.0).floor() as i32 + sandbox.height() as i32 / 2,
                );
                for x in -BREAKRADIUS..=BREAKRADIUS {
                    for y in -BREAKRADIUS..=BREAKRADIUS {
                        if !sandbox.out_of_bounds_i32(grid_x + x, grid_y + y) {
                            sandbox.set((grid_x + x) as usize, (grid_y + y) as usize, None);
                        }
                    }
                }
            }

            commands.entity(entity).despawn();
        }
    }
}
