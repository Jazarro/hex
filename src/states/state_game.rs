use bevy::app::{App, Plugin};
use bevy::pbr::{PointLight, PointLightBundle};
use bevy::prelude::*;
use iyes_loopless::condition::ConditionSet;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::animate_simple::rotate;
use crate::game::actors::player::setup_player;
use crate::game::camera::first_person::{rotate_player_camera, position_player_camera, PlayerCamera};
use crate::game::meshes::debug_lines::apply_debug_lines;
use crate::game::meshes::hexagon::spawn_random_chunk;
use crate::game::movement::char_control::player_movement_system;
use crate::states::appstate::AppState;

pub struct GameState;

impl Plugin for GameState {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            AppState::Game,
            ConditionSet::new()
                .run_in_state(AppState::Game)
                .with_system(setup_light)
                .with_system(setup_player)
                .with_system(apply_debug_lines)
                .with_system(spawn_random_chunk)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Game)
                .with_system(move_light)
                .with_system(rotate)
                .with_system(player_movement_system)
                .with_system(rotate_player_camera)
                .with_system(position_player_camera)
                .into(),
        );
    }
}

fn move_light(
    mut queries: ParamSet<(
        Query<(&Transform, &PlayerCamera)>,
        Query<(&mut Transform, &PointLight)>,
    )>,
) {
    if let Ok(camera_pos) = queries
        .p0()
        .get_single()
        .map(|(transform, _)| transform.translation)
    {
        for (mut light_transform, _) in queries.p1().iter_mut() {
            light_transform.translation = camera_pos;
        }
    }
}

fn setup_light(mut commands: Commands) {
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 15_0_000.0,
            shadows_enabled: true,
            range: 1000.,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 100.0),
        ..default()
    });
}
