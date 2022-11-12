use bevy::app::{App, Plugin};
use bevy::pbr::{PointLight, PointLightBundle};
use bevy::prelude::*;
use iyes_loopless::condition::ConditionSet;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::animate_simple::rotate;
use crate::game::actors::player::setup_player;
use crate::game::actors::structs::Player;
use crate::game::camera::first_person::{
    cursor_grab, position_player_camera, rotate_player_camera, PlayerCamera,
};
use crate::game::hex_grid::axial::{ChunkId, Pos};
use crate::game::hex_grid::chunks::Chunks;
use crate::game::meshes::debug_lines::spawn_debug_lines;
use crate::game::meshes::hexagon::spawn_chunk_new;
use crate::game::meshes::sun::{animate_sun, process_day_night_input, spawn_sun};
use crate::game::movement::char_control::player_movement_system;
use crate::states::appstate::AppState;

pub struct GameState;

impl Plugin for GameState {
    fn build(&self, app: &mut App) {
        app.insert_resource(Chunks::default());
        app.add_enter_system_set(
            AppState::Game,
            ConditionSet::new()
                .run_in_state(AppState::Game)
                .with_system(cursor_grab)
                .with_system(spawn_sun)
                .with_system(setup_player)
                .with_system(spawn_debug_lines)
                .with_system(spawn_chunk_new)
                // .with_system(spawn_test_grid)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Game)
                .with_system(process_day_night_input)
                .with_system(animate_sun)
                .with_system(rotate)
                .with_system(player_movement_system)
                .with_system(rotate_player_camera)
                .with_system(position_player_camera)
                // .with_system(temp_chunk_test)
                .into(),
        );
    }
}

#[derive(Component)]
pub struct ChunkTest {
    pos: ChunkId,
}

fn temp_chunk_test(query: Query<(&Transform, &Player)>) {
    if let Ok((transform, _)) = query.get_single() {
        let pos = Pos::from_xyz(&transform.translation).as_ipos_round();
        let chunk_pos = ChunkId::from_block_pos(&pos);
        info!("Pos={:?}\tChunk={:?}", pos, chunk_pos);
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
            intensity: 150_000.0,
            shadows_enabled: true,
            range: 1000.,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 100.0),
        ..default()
    });
}
