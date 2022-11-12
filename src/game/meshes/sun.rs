use std::time::Duration;

use bevy::prelude::*;
use splines::Spline;

use crate::assets::config::config_debug::DebugConfig;
use crate::assets::config::config_keys::{InputAction, InputHandler};
use crate::assets::config::config_world::WorldConfig;
use crate::game::meshes::hexagon::create_single_block_mesh;

#[derive(Component, Default)]
pub struct Sun;

pub fn spawn_sun(
    mut commands: Commands,
    world_config: Res<WorldConfig>,
    debug_config: Res<DebugConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut std_mats: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 100_000.,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 500.0),
            ..default()
        })
        .insert(Sun::default())
        .with_children(|parent| {
            parent.spawn_bundle(MaterialMeshBundle {
                mesh: meshes.add(create_single_block_mesh()),
                material: std_mats.add(Color::YELLOW.into()),
                transform: Transform::default().with_scale(Vec3::splat(10.)),
                ..default()
            });
        });
    let mut timer = Timer::from_seconds(world_config.day_night_duration_seconds, true);
    timer.set_elapsed(Duration::from_secs_f32(
        world_config.day_night_duration_seconds * 3. / 8.,
    )); // <== At game start, set the time to 9:00 AM.
    commands.insert_resource(DayNight {
        paused: debug_config.pause_time_at_game_start,
        timer,
        illuminance: world_config.illuminance(),
        redness: world_config.redness(),
        greenness: world_config.greenness(),
        blueness: world_config.blueness(),
    });
}

pub struct DayNight {
    pub paused: bool,
    pub timer: Timer,
    pub illuminance: Spline<f32, f32>,
    pub redness: Spline<f32, f32>,
    pub greenness: Spline<f32, f32>,
    pub blueness: Spline<f32, f32>,
}

pub fn process_day_night_input(input: InputHandler, mut day_night: ResMut<DayNight>) {
    if input.is_active(&InputAction::PauseTime) {
        day_night.paused ^= true;
        info!(
            "Toggled day-night cycle {}!",
            if day_night.paused { "off" } else { "on" }
        );
    }
    let speed = input.direction(&InputAction::SpeedDownTime, &InputAction::SpeedUpTime);
    if !speed.is_neutral() {
        let percent = day_night.timer.percent();
        let seconds_old = day_night.timer.duration().as_secs_f32();
        let seconds_new = (seconds_old + speed.signum() * 10.).max(10.);
        day_night
            .timer
            .set_duration(Duration::from_secs_f32(seconds_new));
        day_night
            .timer
            .set_elapsed(Duration::from_secs_f32(seconds_new * percent));
        info!(
            "Duration of day-night cycle changed from {}s to {}s",
            seconds_old, seconds_new
        );
    }
    let set_time = input.direction(&InputAction::SetTimeBack, &InputAction::SetTimeForward);
    if !set_time.is_neutral() {
        let duration = day_night.timer.duration().as_secs_f32();
        let percent =
            ((day_night.timer.percent() * 8.) as i32 + set_time.signum_i()).rem_euclid(8) as f32;
        day_night
            .timer
            .set_elapsed(Duration::from_secs_f32(duration * percent / 8.));
        info!("Set time of day to {} o'clock.", percent * 3.);
    }
}

pub fn animate_sun(
    time: Res<Time>,
    config: Res<WorldConfig>,
    mut day_night: ResMut<DayNight>,
    mut query: Query<(&mut Transform, &mut DirectionalLight, &Sun)>,
) {
    if !day_night.paused {
        day_night.timer.tick(time.delta());
    }
    for (mut transform, mut light, sun) in query.iter_mut() {
        light.illuminance = day_night
            .illuminance
            .clamped_sample(day_night.timer.percent())
            .unwrap();
        light.color = Color::rgb(
            day_night
                .redness
                .clamped_sample(day_night.timer.percent())
                .unwrap(),
            day_night
                .greenness
                .clamped_sample(day_night.timer.percent())
                .unwrap(),
            day_night
                .blueness
                .clamped_sample(day_night.timer.percent())
                .unwrap(),
        );
        let radius = 500.;
        let angle = (day_night.timer.percent() - 0.25) * std::f32::consts::TAU;
        transform.translation = Vec3::new(angle.cos() * radius, 0., angle.sin() * radius);
        let angle = (day_night.timer.percent() - 0.5) * std::f32::consts::TAU;
        transform.rotation = Quat::from_rotation_y(-angle);
    }
}
