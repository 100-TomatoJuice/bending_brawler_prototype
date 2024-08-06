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

fn apply_controls(mut query: Query<(&ActionState<Action>, &mut TnuaController)>) {
    for (action, mut controller) in query.iter_mut() {
        let direction = action.clamped_axis_pair(&Action::Move).x;

        controller.basis(TnuaBuiltinWalk {
            desired_velocity: Vec3::new(direction * 800.0, 0.0, 0.0),
            float_height: 25.0,
            free_fall_extra_gravity: 320.0,
            acceleration: 1600.0,
            air_acceleration: 1600.0,
            max_slope: std::f32::consts::FRAC_PI_3,
            ..default()
        });

        if action.pressed(&Action::Jump) {
            controller.action(TnuaBuiltinJump {
                height: 350.0,
                allow_in_air: false,
                takeoff_extra_gravity: 800.0,
                fall_extra_gravity: 200.0,
                shorten_extra_gravity: 5000.0,
                ..default()
            });
        }
    }
}

// fn move_player(
//     mut query: Query<(&ActionState<Action>, &mut MovementData, &Player)>,
//     time: Res<Time>,
// ) {
//     for (action_state, mut data, player) in query.iter_mut() {
//         let mut target_speed =
//             action_state.clamped_axis_pair(&Action::Move).unwrap().x() * player.max_run_speed;

//         let mut acceleration_rate = if player.last_on_ground_time > 0.0 {
//             if target_speed.abs() > 0.01 {
//                 player.run_acceleration_amount()
//             } else {
//                 player.run_decceleration_amount()
//             }
//         } else {
//             if target_speed.abs() > 0.01 {
//                 player.run_acceleration_amount() * player.air_acceleration
//             } else {
//                 player.run_decceleration_amount() * player.air_decceleration
//             }
//         };

//         // Add bonus jump apex acceleration
//         if (player.is_jumping || player.is_jump_falling)
//             && data.0.y.abs() < player.jump_hang_time_threshold
//         {
//             acceleration_rate *= player.jump_hang_acceleration_multiplier;
//             target_speed *= player.jump_hang_max_speed_multiplier;
//         }

//         // Conserve momentum
//         if data.0.x.abs() > target_speed.abs()
//             && data.0.x.signum() == target_speed.signum()
//             && target_speed.abs() > 0.01
//         {
//             acceleration_rate = 0.0;
//         }

//         let speed_difference = target_speed - data.0.x;
//         let movement = speed_difference * acceleration_rate;

//         data.0.x += movement * time.delta_seconds();
//     }
// }

// fn gravity(mut query: Query<(&mut MovementData, &ActionState<Action>, &Player)>, time: Res<Time>) {
//     for (mut data, input, player) in query.iter_mut() {
//         if data.0.y < 0.0 && input.clamped_axis_pair(&Action::Move).unwrap().y() < 0.0 {
//             data.0.y -=
//                 player.gravity_scale() * player.fast_fall_gravity_multiplier * time.delta_seconds();
//             data.0 = Vec2::new(data.0.x, data.0.y.max(-player.max_fast_fall_speed))
//         } else if player.is_jump_cut {
//             data.0.y -=
//                 player.gravity_scale() * player.jump_cut_gravity_multiplier * time.delta_seconds();
//             data.0 = Vec2::new(data.0.x, data.0.y.max(-player.max_fall_speed))
//         } else if (player.is_jumping || player.is_jump_falling)
//             && data.0.y.abs() < player.jump_hang_time_threshold
//         {
//             data.0.y -=
//                 player.gravity_scale() * player.jump_hang_gravity_multiplier * time.delta_seconds();
//         } else if data.0.y < 0.0 {
//             data.0.y -=
//                 player.gravity_scale() * player.fall_gravity_multiplier * time.delta_seconds();
//             data.0 = Vec2::new(data.0.x, data.0.y.max(-player.max_fall_speed))
//         } else {
//             data.0.y -= player.gravity_scale() * time.delta_seconds();
//         }
//     }
// }

// fn jump_checks(mut query: Query<(&ActionState<Action>, &mut MovementData, &mut Player)>) {
//     for (action_state, data, mut player) in query.iter_mut() {
//         if action_state.just_pressed(&Action::Jump) {
//             player.last_pressed_jump_time = player.jump_input_buffer_time;
//         }
//         if action_state.released(&Action::Jump) && player.is_jumping && data.0.y > 0.0 {
//             player.is_jump_cut = true;
//         }
//     }
// }

// fn jump(mut query: Query<(&mut MovementData, &mut Player)>) {
//     for (mut data, mut player) in query.iter_mut() {
//         if player.is_jumping
//             && !player.is_jump_cut
//             && !player.is_jump_falling
//             && player.last_pressed_jump_time > 0.0
//         {
//             player.last_pressed_jump_time = 0.0;
//             player.last_on_ground_time = 0.0;
//             player.current_jumps -= 1;

//             data.0.y = player.jump_force();
//         }
//     }
// }

// fn apply_movement(
//     mut movement_query: Query<(&mut KinematicCharacterController, &MovementData)>,
//     time: Res<Time>,
// ) {
//     for (mut movement, data) in movement_query.iter_mut() {
//         movement.translation = Some(data.0 * 60.0 * time.delta_seconds());
//     }
// }

// fn update_checks(
//     mut query: Query<(&MovementData, &Transform, &mut Player)>,
//     rapier_context: Res<RapierContext>,
//     time: Res<Time>,
// ) {
//     for (data, transform, mut player) in query.iter_mut() {
//         player.last_on_ground_time -= time.delta_seconds();
//         player.last_pressed_jump_time -= time.delta_seconds();

//         // Collision Check
//         if !player.is_jumping && on_ground(transform.translation.truncate(), 30.0, &rapier_context)
//         {
//             player.last_on_ground_time = player.coyote_time;
//             player.current_jumps = player.max_jumps;
//         }

//         // Jump Checks
//         if player.is_jumping && data.0.y < 0.0 {
//             player.is_jumping = false;
//         }
//         if player.last_on_ground_time > 0.0 && !player.is_jumping {
//             player.is_jump_cut = false;

//             if !player.is_jumping {
//                 player.is_jump_falling = false;
//             }
//         }
//         if player.current_jumps > 0 && player.last_pressed_jump_time > 0.0 {
//             player.is_jumping = true;
//             player.is_jump_cut = false;
//             player.is_jump_falling = false;
//         }
//     }
// }

// fn on_ground(position: Vec2, ray_length: f32, rapier_context: &Res<RapierContext>) -> bool {
//     if let Some((_, _)) = rapier_context.cast_shape(
//         position,
//         Rot::NAN,
//         Vec2::NEG_Y,
//         &Collider::ball(25.0),
//         ShapeCastOptions::with_max_time_of_impact(ray_length),
//         QueryFilter::only_fixed().exclude_sensors(),
//     ) {
//         return true;
//     }

//     false
// }

// fn on_ground(position: Vec2, ray_length: f32, rapier_context: &Res<RapierContext>) -> bool {
//     if let Some((_, _)) = rapier_context.cast_shape(
//         position,
//         Rot::NAN,
//         Vec2::NEG_Y,
//         &Collider::ball(25.0),
//         ShapeCastOptions::with_max_time_of_impact(ray_length),
//         QueryFilter::only_fixed().exclude_sensors(),
//     ) {
//         return true;
//     }

//     false
// }
