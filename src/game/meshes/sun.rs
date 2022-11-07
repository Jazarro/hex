use bevy::prelude::*;
use splines::Spline;

use crate::assets::config::config_world::WorldConfig;
use crate::game::meshes::hexagon::create_mesh;

#[derive(Component, Default)]
pub struct Sun {}

pub fn spawn_sun(
    mut commands: Commands,
    config: Res<WorldConfig>,
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
                mesh: meshes.add(create_mesh()),
                material: std_mats.add(Color::YELLOW.into()),
                transform: Transform::default().with_scale(Vec3::splat(10.)),
                ..default()
            });
        });
    commands.insert_resource(DayNight {
        timer: Timer::from_seconds(config.daynight_duration_seconds, true),
        illuminance: config.illuminance(),
        redness: config.redness(),
        greenness: config.greenness(),
        blueness: config.blueness(),
    });
}

pub struct DayNight {
    pub timer: Timer,
    pub illuminance: Spline<f32, f32>,
    pub redness: Spline<f32, f32>,
    pub greenness: Spline<f32, f32>,
    pub blueness: Spline<f32, f32>,
}

pub fn animate_sun(
    time: Res<Time>,
    config: Res<WorldConfig>,
    mut day_night: ResMut<DayNight>,
    mut query: Query<(&mut Transform, &mut DirectionalLight, &Sun)>,
) {
    day_night.timer.tick(time.delta());
    for (mut transform, mut light, sun) in query.iter_mut() {
        light.illuminance = day_night.illuminance.clamped_sample(day_night.timer.percent()).unwrap();
        light.color = Color::rgb(
            day_night.redness.clamped_sample(day_night.timer.percent()).unwrap(),
            day_night.greenness.clamped_sample(day_night.timer.percent()).unwrap(),
            day_night.blueness.clamped_sample(day_night.timer.percent()).unwrap(),
        );
        let radius = 500.;
        let angle = (day_night.timer.percent() - 0.25) * std::f32::consts::TAU;
        transform.translation = Vec3::new(angle.cos() * radius, 0., angle.sin() * radius);
        let angle = (day_night.timer.percent() - 0.5) * std::f32::consts::TAU;
        transform.rotation = Quat::from_rotation_y(-angle);
    }
}
