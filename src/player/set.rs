use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::sandbox::{particle_types::get_particle, sandbox::Sandbox};

use super::{
    grab::{Held, ParentObject},
    Action, HeldObject,
};

pub struct SetPlugin;

impl Plugin for SetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, set);
    }
}

fn set(
    mut commands: Commands,
    mut sandbox_query: Query<&mut Sandbox>,
    mut player_query: Query<(&ActionState<Action>, &mut HeldObject)>,
    parent_query: Query<Entity, (With<ParentObject>, With<Held>)>,
    children_query: Query<&Children>,
    transform_query: Query<&GlobalTransform>,
) {
    let mut sandbox = sandbox_query.single_mut();

    for (action, mut held) in player_query.iter_mut() {
        if !action.just_pressed(&Action::Set) || held.0.is_none() {
            continue;
        }
        let Ok(entity) = parent_query.get(held.0.unwrap()) else {
            continue;
        };

        for child in children_query.iter_descendants(entity) {
            let Ok(position) = transform_query.get(child).map(|x| x.translation()) else {
                continue;
            };
            let (grid_x, grid_y) = (
                (position.x / 8.0).floor() as i32 + sandbox.width() as i32 / 2,
                (position.y / 8.0).floor() as i32 + sandbox.height() as i32 / 2,
            );

            if !sandbox.out_of_bounds_i32(grid_x, grid_y) {
                sandbox.set(
                    grid_x as usize,
                    grid_y as usize,
                    Some(get_particle(
                        crate::sandbox::particle_types::ParticleTypes::Dirt,
                    )),
                );
            }
        }

        commands.entity(entity).despawn_recursive();
        held.0 = None;
    }
}
