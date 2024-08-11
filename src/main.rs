use bevy::{
    prelude::*, render::camera::ScalingMode,
    window::PresentMode,
};
use bevy_rapier2d::prelude::*;

#[cfg(feature = "dev")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
#[cfg(feature = "dev")]
use bevy::input::common_conditions::input_toggle_active;
#[cfg(not(debug_assertions))]
use bevy::window::WindowMode;

mod damage;
mod load_level;
mod player;
mod sandbox;
mod vector;
use bevy_tnua::controller::TnuaControllerPlugin;
use bevy_tnua_rapier2d::TnuaRapier2dPlugin;
use damage::DamagePlugin;
use load_level::LoadLevelPlugin;
use player::PlayerPlugin;
use sandbox::SandboxPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Falling Sand".into(),
                    resolution: (1280.0, 720.0).into(),
                    #[cfg(not(debug_assertions))]
                    mode: WindowMode::BorderlessFullscreen,
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .insert_resource(Msaa::Off)
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    //.add_plugins(RapierDebugRenderPlugin::default())
    .add_plugins((
        TnuaControllerPlugin::default(),
        TnuaRapier2dPlugin::default(),
        LoadLevelPlugin,
        DamagePlugin,
        SandboxPlugin,
        PlayerPlugin,
    ))
    .add_systems(Startup, setup);

    #[cfg(feature = "dev")]
    app.add_plugins(
        WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
    );

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: 1920.0,
                height: 1080.0,
            },
            near: -1000.0,
            ..default()
        },
        ..default()
    });
}
