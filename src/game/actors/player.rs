use bevy::prelude::Res;
use bevy::{
    prelude::{BuildChildren, Commands, Transform},
    transform::TransformBundle,
};

use crate::game::hex_grid::chunk_loading::ChunkLoader;
use crate::game::{
    camera::first_person::{PlayerCamera, PlayerCameraBundle},
    movement::structs::MoveParams,
};
use crate::io::config::WorldConfig;

use super::structs::PlayerBundle;

pub fn setup_player(mut command: Commands, config: Res<WorldConfig>) {
    command
        .spawn(PlayerBundle {
            move_params: MoveParams {
                accel: 14.4,
                max_speed: 11.1,
                boost_mod: 1.8,
                max_fall_speed: 10.0,
                vel_decay_factor: 0.94, // closer to 1 means slower decay
                turn_speed: 0.15,
                flying: true,
            },
            chunk_loader: ChunkLoader {
                radius_min: config.render_distance_min,
                radius_max: config.render_distance_max,
            },
            transform: TransformBundle::from_transform(Transform::from_xyz(1.0, 1.0, 12.0)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(PlayerCameraBundle {
                player_cam: PlayerCamera {
                    height: 1.5,
                    x_rot_max_deg: 85.0,
                    x_rot_speed: 0.15,
                },
                ..Default::default()
            });
        });
}
