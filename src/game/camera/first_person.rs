use bevy::input::Input;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{
    default, Camera3dBundle, Commands, Component, KeyCode, Query, Res, Time, Transform,
};

/// set up a simple 3D camera
pub fn setup_camera(mut commands: Commands) {
    // camera
    commands
        .spawn_bundle(Camera3dBundle {
            // transform: Transform::from_xyz(200.0, 200.0, 100.0).looking_at(Vec3::ZERO, Vec3::Z),
            // transform: Transform::from_xyz(20.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Z),
            transform: Transform::from_xyz(1.0, 1.0, 20.0).looking_at(Vec3::ZERO, Vec3::Z),
            ..default()
        })
        .insert(ActiveCamera::default());
}

#[derive(Default, Component)]
pub struct ActiveCamera {}

pub fn move_camera(
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
        if keys.any_pressed([KeyCode::S, KeyCode::Down]) {
            1.
        } else if keys.any_pressed([KeyCode::W, KeyCode::Up]) {
            -1.
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
    );
    let rotation = if keys.pressed(KeyCode::Q) {
        -1.
    } else if keys.pressed(KeyCode::E) {
        1.
    } else {
        0.
    };
    for (mut transform, _) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(rotation * time.delta_seconds()));
        transform.translation.x += direction.x;
        transform.translation.y += direction.y;
        transform.translation.z += direction.z;
    }
}
