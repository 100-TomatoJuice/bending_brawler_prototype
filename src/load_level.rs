use bevy::prelude::*;

use crate::sandbox::{
    particle_types::{get_particle, ParticleTypes},
    sandbox::Sandbox,
};

pub struct LoadLevelPlugin;

impl Plugin for LoadLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_level).add_systems(
            Update,
            draw_level.run_if(
                resource_exists::<DirtImageHandle>
                    .and_then(is_dirt_loaded)
                    .and_then(run_once()),
            ),
        );
    }
}

#[derive(Resource)]
struct DirtImageHandle(Handle<Image>);

fn is_dirt_loaded(asset_server: Res<AssetServer>, asset_handle: Res<DirtImageHandle>) -> bool {
    asset_server.is_loaded_with_dependencies(&asset_handle.0)
}

fn load_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(DirtImageHandle(asset_server.load("dirt.png")));
}

fn draw_level(
    mut query: Query<&mut Sandbox>,
    dirt_image_handle: Res<DirtImageHandle>,
    images: Res<Assets<Image>>,
) {
    let mut sandbox = query.single_mut();

    let image = images.get(&dirt_image_handle.0).unwrap();
    let size = image.size().as_ivec2();

    for x in 0..size.x {
        for y in 0..size.y {
            let bytes_per_pixel = 4;
            let index = to_index(x, y, size.x) * bytes_per_pixel;

            let alpha = image.data[index + 3];
            if alpha == 0 {
                continue;
            }

            let r = image.data[index];
            let g = image.data[index + 1];
            let b = image.data[index + 2];

            let mut particle = get_particle(ParticleTypes::Dirt);
            particle.color = (r, g, b, alpha);

            sandbox.set(
                x as usize,
                (y - size.y + 1).unsigned_abs() as usize,
                Some(particle),
            );
        }
    }
}

fn to_index(x: i32, y: i32, width: i32) -> usize {
    ((y * width) + x) as usize
}
