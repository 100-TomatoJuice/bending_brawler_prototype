use bevy::prelude::*;

use super::parry::Parry;

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
        let p = speed * x.powf(-4.0);

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

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    pub aim_direction: AimDirection,
    pub range: Range,
    pub radius: Radius,
    pub parry: Parry,
    pub jumps: ExtraJumps,
    pub held: HeldObject,
    pub health: PlayerHealth,
}
