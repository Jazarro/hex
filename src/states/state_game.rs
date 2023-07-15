use std::time::Duration;
use bevy::app::{App, Plugin};
use bevy::prelude::*;

use crate::game::actors::player::setup_player;
use crate::game::actors::structs::Player;
use crate::game::camera::first_person::{
    cursor_grab, position_player_camera, rotate_player_camera,
};
use crate::game::hex_grid::axial::{ChunkId, Pos};
use crate::game::hex_grid::chunk_loading::{check_chunk_loader, LoadUnloadEvent};
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
        app.add_systems(
            (
                cursor_grab,
                spawn_sun,
                setup_player,
                spawn_debug_lines,
                // spawn_test_grid,
            )
                .in_schedule(OnEnter(AppState::Game)),
        );

        app.add_systems(
            (
                process_day_night_input,
                animate_sun,
                player_movement_system,
                rotate_player_camera,
                position_player_camera,
                // debug_print_coordinates,
            )
                .in_set(OnUpdate(AppState::Game)),
        );

        // TODO: Use async tasks
        app.insert_resource(FixedTime::new(Duration::from_millis(100)));
        app.add_system(check_chunk_loader
            .in_set(OnUpdate(AppState::Game))
            .in_schedule(CoreSchedule::FixedUpdate));

        // app
        //     // Checking to see if chunks must be loaded is only necessary every once in a while, not every tick:
        //     .add_fixed_timestep(Duration::from_millis(1000), "check_chunk_loading")
        //     .add_fixed_timestep_child_stage("check_chunk_loading")
        //     .add_fixed_timestep_system("check_chunk_loading", 0, check_chunk_loader)
        //     // The actual chunk loading is only done when a LoadUnloadEvent is sent:
        //     .add_stage_after(Update, "perform_chunk_loading", SystemStage::parallel())
        //     .add_system_set_to_stage(
        //         "perform_chunk_loading",
        //         ConditionSet::new()
        //             .run_in_state(AppState::Game)
        //             .run_on_event::<LoadUnloadEvent>()
        //             .with_system(load_unload_chunks)
        //             .into(),
        //     );
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
