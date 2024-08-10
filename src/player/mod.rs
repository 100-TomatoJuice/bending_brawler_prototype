use self::{
    aim_direction::PlayerAimPlugin, components::*, grab::GrabPlugin, parry::ParryPlugin,
    player_movement::PlayerMovementPlugin, set::SetPlugin, spawn_player::PlayerConnectionPlugin,
};
use bevy::{prelude::*, utils::HashMap};
use leafwing_input_manager::prelude::*;

mod aim_direction;
pub mod components;
pub mod grab;
mod parry;
mod player_movement;
mod set;
mod spawn_player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .insert_resource(UsedGamepads::default())
            .add_plugins((
                PlayerMovementPlugin,
                PlayerAimPlugin,
                PlayerConnectionPlugin,
                GrabPlugin,
                SetPlugin,
                ParryPlugin,
            ))
            .add_systems(Update, change_color_on_health);
    }
}

#[derive(Reflect, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    Move,
    Aim,
    Jump,
    Grab,
    Set,
    Parry,
}

impl Actionlike for Action {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Action::Move => InputControlKind::DualAxis,
            Action::Aim => InputControlKind::DualAxis,
            Action::Jump => InputControlKind::Button,
            Action::Grab => InputControlKind::Button,
            Action::Set => InputControlKind::Button,
            Action::Parry => InputControlKind::Button,
        }
    }
}

fn change_color_on_health(mut query: Query<(&mut Sprite, &PlayerHealth), Changed<PlayerHealth>>) {
    for (mut sprite, health) in query.iter_mut() {
        sprite.color = bevy::color::palettes::basic::RED
            .mix(&bevy::color::palettes::basic::BLUE, health.0 / 50000.0)
            .into();
    }
}

#[derive(Default, Resource)]
pub struct UsedGamepads {
    gamepads: HashMap<Gamepad, Entity>,
}
