use bevy::{
    prelude::{BuildChildren, Commands, Transform},
    transform::TransformBundle,
};

use crate::game::{
    camera::first_person::{PlayerCamera, PlayerCameraBundle},
    movement::structs::MoveParams,
};

use super::structs::PlayerBundle;

pub fn setup_player(mut cmd: Commands) {
    cmd.spawn_bundle(PlayerBundle {
        move_params: MoveParams {
            accel: 14.4,
            max_speed: 11.1,
            boost_mod: 1.8,
            max_fall_speed: 10.0,
            vel_decay_factor: 0.94, // closer to 1 means slower decay
            turn_speed: 0.15,
            flying: true,
        },
        transfrom: TransformBundle::from_transform(Transform::from_xyz(1.0, 1.0, 12.0)),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(PlayerCameraBundle {
            player_cam: PlayerCamera {
                height: 1.5,
                x_rot_max_deg: 85.0,
                x_rot_speed: 0.15,
            },
            ..Default::default()
        });
    });
}
