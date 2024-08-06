use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::player::*;

pub struct PlayerAimPlugin;

impl Plugin for PlayerAimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_aim, draw_circle).chain());
    }
}

fn update_aim(mut player_query: Query<(&ActionState<Action>, &mut AimDirection)>) {
    for (input, mut aim) in player_query.iter_mut() {
        // let aim_input = match input.clamped_axis_pair(&Action::Aim) {
        //     Some(direction) => direction.xy(),
        //     None => panic!("Player doesn't have aim input"),
        // };

        aim.0 = input.clamped_axis_pair(&Action::Aim);
    }
}

fn draw_circle(mut gizmos: Gizmos, query: Query<(&Transform, &AimDirection, &Range, &Radius)>) {
    for (transform, aim, range, radius) in query.iter() {
        if aim.0 == Vec2::ZERO {
            continue;
        }

        gizmos.circle_2d(
            transform.translation.xy() + aim.0 * (range.0 + radius.current),
            radius.current,
            Color::BLACK,
        );
    }
}
