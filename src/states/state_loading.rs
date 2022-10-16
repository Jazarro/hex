use bevy::app::{App, Plugin};
use bevy::prelude::Commands;
use iyes_loopless::condition::ConditionSet;
use iyes_loopless::prelude::{AppLooplessStateExt, NextState};

use crate::config::config_debug::DebugConfig;
use crate::states::appstate::AppState;

pub struct LoadingState;

impl Plugin for LoadingState {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            AppState::Loading,
            ConditionSet::new()
                .run_in_state(AppState::Loading)
                .with_system(load_configs)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Loading)
                .with_system(check_load_state)
                .into(),
        );
    }
}

fn load_configs(mut commands: Commands) {
    commands.insert_resource(DebugConfig::load_from_file());
}

pub fn check_load_state(mut commands: Commands) {
    commands.insert_resource(NextState(AppState::Game));
}
