use bevy::{
    input::common_conditions::input_toggle_active, prelude::*, render::camera::ScalingMode,
    window::PresentMode,
};
#[cfg(feature = "dev")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

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
    let mut res = (1280.0, 720.0);

    #[cfg(not(debug_assertions))]
    {
        res = (1920.0, 1080.0);
    }

    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Falling Sand".into(),
                    resolution: res.into(),
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
