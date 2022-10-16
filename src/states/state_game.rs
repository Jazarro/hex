use crate::animate_simple::rotate;
use crate::meshes::debug_lines::{apply_debug_lines};
use crate::meshes::hexagon::spawn_hex;
use bevy::app::{App, Plugin};
use bevy::input::Input;
use bevy::math::{Quat, Vec3};
use bevy::pbr::{PointLight, PointLightBundle};
use bevy::prelude::{
    default, Camera3dBundle, Commands, Component, KeyCode, ParamSet, Query, Res, Time, Transform,
};
use iyes_loopless::condition::ConditionSet;
use iyes_loopless::prelude::AppLooplessStateExt;

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

/// set up a simple 3D camera
fn setup_camera(mut commands: Commands) {
    // camera
    commands
        .spawn_bundle(Camera3dBundle {
            // transform: Transform::from_xyz(200.0, 100.0, 200.0).looking_at(Vec3::ZERO, Vec3::Y),
            // transform: Transform::from_xyz(20.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
            transform: Transform::from_xyz(1.0, 20.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(ActiveCamera::default());
}

#[derive(Default, Component)]
pub struct ActiveCamera {}

fn move_camera(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &ActiveCamera)>,
) {
    let direction = Vec3::new(
        if keys.any_pressed([KeyCode::A, KeyCode::Left]) {
            -1.
        } else if keys.any_pressed([KeyCode::D, KeyCode::Right]) {
            1.
        } else {
            0.
        },
        if keys.any_pressed([KeyCode::LShift, KeyCode::C]) {
            -1.
        } else if keys.any_pressed([KeyCode::Space]) {
            1.
        } else {
            0.
        },
        if keys.any_pressed([KeyCode::S, KeyCode::Down]) {
            1.
        } else if keys.any_pressed([KeyCode::W, KeyCode::Up]) {
            -1.
        } else {
            0.
        },
    );
    let rotation = if keys.pressed(KeyCode::Q) {
        -1.
    } else if keys.pressed(KeyCode::E) {
        1.
    } else {
        0.
    };
    for (mut transform, _) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(rotation * time.delta_seconds()));
        transform.translation.x += direction.x;
        transform.translation.y += direction.y;
        transform.translation.z += direction.z;
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
