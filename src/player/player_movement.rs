use bevy::prelude::*;
use bevy_tnua::{
    builtins::{TnuaBuiltinJump, TnuaBuiltinWalk},
    controller::TnuaController,
    TnuaUserControlsSystemSet,
};
use leafwing_input_manager::prelude::ActionState;

use crate::player::*;
pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_controls.in_set(TnuaUserControlsSystemSet));
        //.add_systems(Update, apply_movement.after(jump));
    }
}

fn apply_controls(mut query: Query<(&ActionState<Action>, &mut TnuaController, &mut ExtraJumps)>) {
    for (action, mut controller, mut jumps) in query.iter_mut() {
        let direction = action.clamped_axis_pair(&Action::Move).x;

        controller.basis(TnuaBuiltinWalk {
            desired_velocity: Vec3::new(direction * 800.0, 0.0, 0.0),
            float_height: 25.0,
            free_fall_extra_gravity: 320.0,
            acceleration: 2600.0,
            air_acceleration: 2600.0,
            coyote_time: 500.0,
            max_slope: std::f32::consts::FRAC_PI_3,
            ..default()
        });

        if action.pressed(&Action::Jump) {
            let allow = if jumps.current > 0 { true } else { false };

            controller.action(TnuaBuiltinJump {
                height: 250.0,
                allow_in_air: allow,
                takeoff_extra_gravity: 800.0,
                fall_extra_gravity: 200.0,
                shorten_extra_gravity: 5000.0,
                ..default()
            });
        }

        if action.just_pressed(&Action::Jump) {
            jumps.current -= 1;
        }
        if !controller.is_airborne().unwrap() {
            jumps.current = jumps.max;
        }
    }
}
