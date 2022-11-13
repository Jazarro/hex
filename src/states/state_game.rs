use std::time::Duration;

use bevy::app::CoreStage::Update;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use iyes_loopless::condition::ConditionSet;
use iyes_loopless::prelude::{AppLooplessFixedTimestepExt, AppLooplessStateExt};

use crate::animate_simple::rotate;
use crate::game::actors::player::setup_player;
use crate::game::actors::structs::Player;
use crate::game::camera::first_person::{
    cursor_grab, position_player_camera, rotate_player_camera,
};
use crate::game::hex_grid::axial::{ChunkId, Pos};
use crate::game::hex_grid::chunk_loading::{
    check_chunk_loader, load_unload_chunks, LoadUnloadEvent,
};
use crate::game::hex_grid::chunks::Chunks;
use crate::game::meshes::debug_lines::spawn_debug_lines;
use crate::game::meshes::sun::{animate_sun, process_day_night_input, spawn_sun};
use crate::game::movement::char_control::player_movement_system;
use crate::states::appstate::AppState;

pub struct GameState;

impl Plugin for GameState {
    fn build(&self, app: &mut App) {
        app.insert_resource(Chunks::default());
        app.add_event::<LoadUnloadEvent>();
        app.add_enter_system_set(
            AppState::Game,
            ConditionSet::new()
                .run_in_state(AppState::Game)
                .with_system(cursor_grab)
                .with_system(spawn_sun)
                .with_system(setup_player)
                .with_system(spawn_debug_lines)
                // .with_system(spawn_test_grid)
                .into(),
        )
        // Checking to see if chunks must be loaded is only necessary every once in a while, not every tick:
        .add_fixed_timestep(Duration::from_millis(1000), "check_chunk_loading")
        .add_fixed_timestep_child_stage("check_chunk_loading")
        .add_fixed_timestep_system("check_chunk_loading", 0, check_chunk_loader)
        // The actual chunk loading is only done when a LoadUnloadEvent is sent:
        .add_stage_after(Update, "perform_chunk_loading", SystemStage::parallel())
        .add_system_set_to_stage(
            "perform_chunk_loading",
            ConditionSet::new()
                .run_in_state(AppState::Game)
                .run_on_event::<LoadUnloadEvent>()
                .with_system(load_unload_chunks)
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
                // .with_system(debug_print_coordinates)
                .into(),
        );
    }
}

/// For testing.
fn debug_print_coordinates(query: Query<(&Transform, &Player)>) {
    if let Ok((transform, _)) = query.get_single() {
        let pos = Pos::from_xyz(&transform.translation).as_ipos_round();
        let chunk_pos = ChunkId::from_block_pos(&pos);
        info!("Pos={:?}\tChunk={:?}", pos, chunk_pos);
        info!("Player is at {:?}", transform.translation);
    }
}
