use bevy::prelude::*;

use super::parry::Parry;

#[derive(Component)]
pub struct Player {
    pub max_run_speed: f32,
    pub run_acceleration: f32,
    pub run_decceleration: f32,
    pub air_acceleration: f32,
    pub air_decceleration: f32,

    pub jump_height: f32,
    pub jump_time_to_apex: f32,
    pub max_jumps: u32,
    pub current_jumps: u32,

    pub jump_cut_gravity_multiplier: f32,
    pub jump_hang_gravity_multiplier: f32,
    pub jump_hang_time_threshold: f32,
    pub jump_hang_acceleration_multiplier: f32,
    pub jump_hang_max_speed_multiplier: f32,

    pub coyote_time: f32,
    pub jump_input_buffer_time: f32,

    pub last_on_ground_time: f32,
    pub last_pressed_jump_time: f32,
    pub is_jumping: bool,
    pub is_jump_cut: bool,
    pub is_jump_falling: bool,

    pub max_fall_speed: f32,
    pub max_fast_fall_speed: f32,
    pub fall_gravity_multiplier: f32,
    pub fast_fall_gravity_multiplier: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            max_run_speed: 8.0,
            run_acceleration: 2.5,
            run_decceleration: 5.0,
            air_acceleration: 0.65,
            air_decceleration: 0.65,
            jump_height: 20.5,
            jump_time_to_apex: 1.75,
            max_jumps: 2,
            current_jumps: 2,
            jump_cut_gravity_multiplier: 2.0,
            jump_hang_gravity_multiplier: 0.5,
            jump_hang_time_threshold: 1.0,
            jump_hang_acceleration_multiplier: 1.1,
            jump_hang_max_speed_multiplier: 1.3,
            coyote_time: 0.1,
            jump_input_buffer_time: 0.1,
            max_fall_speed: 25.0,
            max_fast_fall_speed: 30.0,
            fall_gravity_multiplier: 1.5,
            fast_fall_gravity_multiplier: 2.0,

            last_on_ground_time: 0.0,
            last_pressed_jump_time: 0.0,
            is_jump_cut: false,
            is_jump_falling: false,
            is_jumping: false,
        }
    }
}

impl Player {
    pub fn run_acceleration_amount(&self) -> f32 {
        50.0 * self.run_acceleration / self.max_run_speed
    }

    pub fn run_decceleration_amount(&self) -> f32 {
        50.0 * self.run_decceleration / self.max_run_speed
    }

    pub fn jump_force(&self) -> f32 {
        self.gravity_strength().abs() * self.jump_time_to_apex
    }

    fn gravity_strength(&self) -> f32 {
        -(2.0 * self.jump_height) / (self.jump_time_to_apex.powi(2))
    }

    pub fn gravity_scale(&self) -> f32 {
        self.gravity_strength() * 60.0 / -9.8
    }
}

#[derive(Component)]
pub struct AimDirection(pub Vec2);

impl Default for AimDirection {
    fn default() -> Self {
        Self(Vec2::X)
    }
}

#[derive(Component)]
pub struct Range(pub f32);

impl Default for Range {
    fn default() -> Self {
        Range(200.0)
    }
}

#[derive(Component)]
pub struct Radius {
    pub min: f32,
    pub current: f32,
    pub max: f32,
    pub speed: f32,
}

impl Default for Radius {
    fn default() -> Self {
        Radius {
            min: 30.0,
            current: 30.0,
            max: 80.0,
            speed: 0.2,
        }
    }
}

impl Radius {
    pub fn ease_out_expo(&mut self, speed: f32) {
        let x = self.current / self.max;
        // let p = if x >= 1.0 {
        //     1.0
        // } else {
        //     speed * x.powf(-2.0)
        //     //(1.0 - (x - 1.0).powf(2.0)).sqrt() * speed // ease out circ
        //                                                //1.0 - (2.0_f32).powf(-speed * x) // ease out expo
        // };
        let p = speed * x.powf(-4.0);

        //println!("P; {}, X: {}", p, x);
        self.current = self.max * (x + p);
    }
}

#[derive(Component, Default)]
pub struct HeldObject(pub Option<Entity>);

#[derive(Default, Component)]
pub struct PlayerHealth(pub f32);

#[derive(Component)]
pub struct ExtraJumps {
    pub max: i32,
    pub current: i32,
}

impl Default for ExtraJumps {
    fn default() -> Self {
        let jumps = 1;
        ExtraJumps {
            max: jumps,
            current: jumps,
        }
    }
}

#[derive(Default, Component)]
pub struct PlayerIndex(pub usize);

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    pub aim_direction: AimDirection,
    pub range: Range,
    pub radius: Radius,
    pub parry: Parry,
    pub jumps: ExtraJumps,
    pub held: HeldObject,
    pub health: PlayerHealth,
    pub index: PlayerIndex,
}
