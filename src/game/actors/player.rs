use bevy::{prelude::{Commands, Transform, BuildChildren}, transform::TransformBundle};

use crate::game::{movement::structs::MoveParams, camera::first_person::{PlayerCameraBundle, PlayerCamera}};

use super::structs::PlayerBundle;

pub fn setup_player(mut cmd: Commands){
    cmd.spawn_bundle(PlayerBundle{
        move_params: MoveParams{
            accel: 0.2,
            max_speed: 3.0,
            boost_mod: 2.3,
            max_fall_speed: 10.0,
            vel_decay_rate: 0.4,
            turn_speed: 3.0,
            flying: false,
        },
        transfrom: TransformBundle::from_transform(Transform::from_xyz(1.0, 1.0, 20.0)),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn_bundle(PlayerCameraBundle{
            player_cam: PlayerCamera {
                height: 1.5,
                x_rot_max_deg: 85.0,
                x_rot_speed: 3.0
            },
            ..Default::default()
        });
    });
}
