use bevy::{
    input::gamepad::{GamepadButtonChangedEvent, GamepadConnectionEvent},
    prelude::*,
};
use bevy_rapier2d::prelude::*;
use bevy_tnua::controller::TnuaControllerBundle;
use bevy_tnua_rapier2d::{TnuaRapier2dIOBundle, TnuaRapier2dSensorShape};
use leafwing_input_manager::prelude::*;

use super::{Action, PlayerBundle, PlayerHealth, PlayerIndex, UsedGamepads};

pub struct PlayerConnectionPlugin;

impl Plugin for PlayerConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_player, despawn_on_disconnect));
    }
}

fn spawn_player(
    mut commands: Commands,
    mut gamepad_button_events: EventReader<GamepadButtonChangedEvent>,
    mut used_gamepads: ResMut<UsedGamepads>,
) {
    for event in gamepad_button_events.read() {
        if used_gamepads.gamepads.contains_key(&event.gamepad) {
            continue;
        }

        println!("Join with gamepad {:?}", event.gamepad);
        let mut map = InputMap::default()
            .with_dual_axis(Action::Move, GamepadStick::LEFT)
            .with_dual_axis(Action::Aim, GamepadStick::RIGHT)
            .with(Action::Jump, GamepadButtonType::LeftTrigger2)
            .with(Action::Grab, GamepadButtonType::RightTrigger2)
            .with(Action::Set, GamepadButtonType::LeftTrigger)
            .with(Action::Parry, GamepadButtonType::RightTrigger);
        map.set_gamepad(event.gamepad);
        let entity = commands
            .spawn((
                Name::new("Player"),
                SpriteBundle {
                    sprite: Sprite {
                        color: bevy::color::palettes::basic::BLUE.into(),
                        custom_size: Some(Vec2::new(30.0, 30.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(0., 0., 0.1)),
                    ..default()
                },
                InputManagerBundle::<Action>::with_map(map),
                Collider::ball(15.0),
                RigidBody::Dynamic,
                ExternalImpulse::default(),
                ColliderMassProperties::Mass(20000.0),
                TnuaControllerBundle::default(),
                TnuaRapier2dSensorShape(Collider::capsule_x(15.0, 0.0)),
                TnuaRapier2dIOBundle::default(),
                CollisionGroups::new(Group::all(), Group::all().difference(Group::GROUP_1)),
                LockedAxes::ROTATION_LOCKED,
                ActiveEvents::COLLISION_EVENTS,
                PlayerBundle {
                    health: PlayerHealth(50000.0),
                    index: PlayerIndex(used_gamepads.gamepads.len()),
                    ..default()
                },
            ))
            .id();

        used_gamepads.gamepads.insert(event.gamepad, entity);
    }
}

fn despawn_on_disconnect(
    mut commands: Commands,
    mut gamepad_connection_events: EventReader<GamepadConnectionEvent>,
    mut used_gamepads: ResMut<UsedGamepads>,
) {
    for event in gamepad_connection_events.read() {
        if !event.disconnected() {
            continue;
        }

        if let Some((gamepad, entity)) = used_gamepads.gamepads.get_key_value(&event.gamepad) {
            commands.entity(*entity).despawn();

            let gamepad = *gamepad;
            used_gamepads.gamepads.remove(&gamepad);
        }
    }
}
