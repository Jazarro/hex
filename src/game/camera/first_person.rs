use bevy::input::Input;
use bevy::input::mouse::MouseMotion;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{
    default, Camera3dBundle, Commands, Component, Query, Res, Time, Transform, EventReader, EulerRot, Bundle, Vec2,
};
#[derive(Default, Component)]
pub struct PlayerCamera {
    pub height:f32,
    pub x_rot_max_deg:f32,
    pub x_rot_speed:f32,
}

#[derive(Bundle, Default)]
pub struct PlayerCameraBundle {
    pub player_cam: PlayerCamera,
    #[bundle]
    pub camera_3d: Camera3dBundle
}


// The camera rotates only on it's local X-axis. Z-axis rotation is deferred to player's body.
// Player's body is the camera's parent.
pub fn rotate_player_camera(mut q: Query<(&mut Transform, &mut PlayerCamera)>,
                            mut mouse: EventReader<MouseMotion>, 
                            time: Res<Time>){
    if let Ok((mut tform, cam)) = q.get_single_mut() {
        let mut mouse_mov = Vec2::ZERO;
        let mut new_vel = Vec3::ZERO;
        for ev in mouse.iter() {
            mouse_mov += ev.delta;
        }
        let mut x_rot:f32 = tform.rotation.to_euler(EulerRot::XYZ).0;
        x_rot += mouse_mov.y * time.delta_seconds() * cam.x_rot_speed;
        let max = cam.x_rot_max_deg.to_radians();
        x_rot = x_rot.clamp(-max, max);
        tform.rotation = Quat::from_euler(EulerRot::XYZ, x_rot, 0.0, 0.0);
    }
}

pub fn position_player_camera(mut q: Query<(&mut Transform, &mut PlayerCamera)>){
    if let Ok((mut tform, cam)) = q.get_single_mut() {
        tform.translation = Vec3::new(0.0, 0.0, cam.height);
    }
}