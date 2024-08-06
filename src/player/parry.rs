use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::Velocity,
    geometry::{Collider, ShapeCastOptions},
    pipeline::QueryFilter,
    plugin::RapierContext,
};
use leafwing_input_manager::action_state::ActionState;

use crate::player::grab::Held;

use super::{grab::{Owner, ParentObject, Rock}, Action};

pub struct ParryPlugin;

impl Plugin for ParryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (regen_parry, initate_parry, parry.before(regen_parry), draw_parry));
    }
}

#[derive(Component)]
pub struct Parry {
    max_radius: f32,
    min_radius: f32,
    current_radius: f32,
    duration: f32,
    time_left: Timer,
    regen_rate: f32,
    falloff: f32,
}

impl Default for Parry {
    fn default() -> Self {
        Parry {
            max_radius: 70.0,
            min_radius: 10.0,
            current_radius: 140.0,
            duration: 0.10,
            time_left: Timer::from_seconds(0.0, TimerMode::Once),
            regen_rate: 10.0,
            falloff: 70.0,
        }
    }
}

fn regen_parry(mut query: Query<&mut Parry>, time: Res<Time>) {
    for mut parry in query.iter_mut() {
        parry.current_radius = (parry.current_radius + parry.regen_rate * time.delta_seconds()).min(parry.max_radius);
    }
}

fn initate_parry(mut query: Query<(&ActionState<Action>, &mut Parry)>) {
    for (action, mut parry) in query.iter_mut() {
        if !action.just_pressed(&Action::Parry) || parry.time_left.remaining_secs() > 0.0{
            continue;
        }

        let duration = parry.duration;
        parry
            .time_left
            = Timer::from_seconds(duration, TimerMode::Once);
    }
}

fn parry(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut Parry)>,
    collider_query: Query<&Parent, With<Rock>>,
    mut parent_query: Query<(&Transform, &mut Owner, &mut Velocity), With<ParentObject>>,
    context: Res<RapierContext>,
    time: Res<Time>,
) {
    for (player, transform, mut parry) in query.iter_mut() {
        if parry.time_left.finished() {
            continue;
        }

        context.intersections_with_shape(
            transform.translation.xy(),
            0.0,
            &Collider::ball(parry.current_radius),
            QueryFilter::only_dynamic(),
            |entity| {
                let Ok(parent) = collider_query.get(entity) else {
                    return true;
                };
                let Ok((rock_transform, mut owner, mut velocity)) = parent_query.get_mut(parent.get()) else {
                    return true;
                };
                let unit_direction = (transform.translation - rock_transform.translation)
                    .xy()
                    .normalize_or_zero();
                let unit_velocity = velocity.linvel.normalize_or_zero();
                let parryed_velocity =
                    (unit_velocity - unit_direction).normalize_or_zero() * velocity.linvel.length();
                velocity.linvel = parryed_velocity;

                if let Some(mut entity) = commands.get_entity(parent.get()) {
                    entity.remove::<Held>();
                }
                owner.0 = player;
                println!("Parryed ");
                true
            },
        );

        if parry.time_left.tick(time.delta()).just_finished() {
            parry.current_radius = (parry.current_radius - parry.falloff).max(parry.min_radius);
        }
    }
}

fn draw_parry(query: Query<(&Transform, &Parry)>, mut gizmos: Gizmos) {
    for (transform, parry) in query.iter() {
        let alpha = if parry.time_left.remaining_secs() > 0.0 {
            1.0
        } else {
            0.02
        };

        gizmos.circle_2d(
            transform.translation.xy(),
            parry.current_radius,
            Color::WHITE.with_alpha(alpha),
        );
    }
}