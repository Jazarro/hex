use bevy::app::{App, Plugin};
use bevy::pbr::{PointLight, PointLightBundle};
use bevy::prelude::*;
use iyes_loopless::condition::ConditionSet;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::animate_simple::rotate;
use crate::game::camera::first_person::{move_camera, setup_camera, ActiveCamera};
use crate::game::meshes::debug_lines::apply_debug_lines;
use crate::game::meshes::hexagon::spawn_hex;
use crate::states::appstate::AppState;

pub struct GameState;

impl Plugin for GameState {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            AppState::Game,
            ConditionSet::new()
                .run_in_state(AppState::Game)
                .with_system(setup_camera)
                .with_system(setup_light)
                .with_system(apply_debug_lines)
                .with_system(spawn_hex)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Game)
                .with_system(move_camera)
                .with_system(move_light)
                .with_system(rotate)
                .into(),
        );
    }
}

fn move_light(
    mut queries: ParamSet<(
        Query<(&Transform, &ActiveCamera)>,
        Query<(&mut Transform, &PointLight)>,
    )>,
) {
    if let Ok(pos) = queries
        .p0()
        .get_single()
        .map(|(transform, _)| transform.translation)
    {
        for (mut transform, _) in queries.p1().iter_mut() {
            transform.translation = pos;
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
        transform: Transform::from_xyz(0.0, 200.0, 0.0),
        ..default()
    });
}
