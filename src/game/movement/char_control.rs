use bevy::input::mouse::MouseMotion;
use bevy::prelude::{Query, Res, Time, Transform, With, Vec3, Vec2, EventReader, Camera, Quat, Without, debug};
use crate::game::actors::structs::Player;
use crate::game::camera::first_person::PlayerCamera;
use crate::game::movement::structs::{MoveState, MoveParams};
use crate::{EulerRot, MoveInput};

pub fn player_movement_system(mut q: Query<(&mut MoveState, &mut MoveParams, &mut Transform), With<Player>>,
                            cam_q: Query<&Transform, (With<Camera>, With<PlayerCamera>, Without<Player>)>,
                            input: Res<MoveInput>, 
                            mut mouse: EventReader<MouseMotion>,
                            time: Res<Time>){
    if let Ok((mut move_state, move_params, mut tform)) = q.get_single_mut() {
        if let Ok(cam_tform) = cam_q.get_single(){
            let dt = time.delta_seconds();
            let mut mouse_mov = Vec2::ZERO;
            let mut new_vel = Vec3::ZERO;
            for ev in mouse.iter() {
                mouse_mov += ev.delta;
            }
            // Rotate body using mouse.x delta. Use only the Z axis.
            tform.rotate_local_z(-mouse_mov.x * dt * move_params.turn_speed);
    
            if move_params.flying{
                new_vel = flying_movement(&input, &move_params, &move_state, tform.rotation.clone(), cam_tform.rotation.clone(), dt);
            }
            else {
                new_vel = walking_movement(&input, &move_params, &move_state, tform.rotation.clone(), dt);
            }
    
            move_state.velocity = new_vel;
    
    
            let mut mov = tform.translation;
            // TODO: Insert collision detection and velocity deflection here.
            mov += move_state.velocity * time.delta_seconds();
            tform.translation = mov;
        }

    }
}

fn walking_movement(input: &MoveInput, move_params: &MoveParams, move_state: &MoveState, body_rot: Quat, dt: f32) -> Vec3 {
    // Split planar movement from falling and jumping for greater control.
    let mut planar_vel = move_state.velocity.truncate();
    let mut vert_vel = move_state.velocity.z;
    
    if input.xy_plane.length_squared() < 0.01 {
        // No input, decay planar velocity.
        // TODO: change decay rate depending on grounding
        planar_vel *= move_params.vel_decay_factor;
        if planar_vel.length_squared() < 0.001{
            planar_vel = Vec2::ZERO; // filter out low velocities
        }
    } else {
        // Input detected, adjust velocity.
        let accel_vector = (body_rot * (input.xy_plane.normalize() * move_params.accel).extend(0.0)).truncate();
        // Boost acceleration for vectors countering current velocity.
        let mut accel_mod:f32 = 1.0;
        if planar_vel.length_squared() > 0.0
        {
            let dot_to_vel = accel_vector.normalize().dot(planar_vel.normalize());
            accel_mod = (2.0 - ((dot_to_vel + 1.0) * 0.5)) * 3.8;
        }
        planar_vel += accel_vector * dt * accel_mod;
        planar_vel = planar_vel.clamp_length_max(move_params.max_speed);
    }
    // We don't need to project the vector onto terrain slope, since every surface is flat.
    // TODO: resolve gravity and jumping - vertical movement. Requires collisions and gravity.
    Vec3::new(planar_vel.x, planar_vel.y, vert_vel)
}

fn flying_movement(input: &MoveInput, move_params: &MoveParams, move_state: &MoveState, body_rot: Quat, mut cam_rot: Quat, dt:f32) -> Vec3 {
    let mut vel = move_state.velocity;
    let mut input_dir = input.xy_plane.extend(0.0);
    let mut max_speed = move_params.max_speed;
    if input.fast_held {
        max_speed *= move_params.boost_mod;
    }
    cam_rot *= Quat::from_euler(EulerRot::XYZ, -1.5, 0.0, 0.0);
    input_dir = cam_rot * input_dir;
    if input.up_held {
        input_dir.z += 1.0;
    }
    if input.down_held {
        input_dir.z -= 1.0;
    }
    input_dir = input_dir.normalize_or_zero();
    if input_dir.length_squared() < 0.01 {
        // No input, decay velocity.
        vel *= move_params.vel_decay_factor;
        if vel.length_squared() < 0.001 {
            vel = Vec3::ZERO; // filter out low velocities
        }
    } else {
        // Input detected, adjust velocity.
        let accel_vector = body_rot * (input_dir * move_params.accel);
        // Boost acceleration for vectors countering current velocity.
        let mut accel_mod:f32 = 1.0;
        if input.fast_held {
            accel_mod *= move_params.boost_mod;
        }
        if vel.length_squared() > 0.0
        {
            let dot_to_vel = accel_vector.normalize().dot(vel.normalize());
            accel_mod = (2.0 - ((dot_to_vel + 1.0) * 0.5)) * 3.8;
        }
        vel += accel_vector * dt * accel_mod;
        vel = vel.clamp_length_max(max_speed);
    }
    vel
}

fn remap(source: f32, source_from: f32, source_to:f32, target_from: f32, target_to: f32) -> f32{
    target_from + (source-source_from)*(target_to-target_from)/(source_to/source_from)
}