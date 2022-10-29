use bevy::prelude::*;

/// Height of a standard hex block.
pub const HEIGHT: f32 = 5.;
/// Long radius, ie the distance between center and corner point.
pub const RADIUS: f32 = 3.;

const SQRT_THREE: f32 = 1.7320508;
const BASIS_Q: Vec2 = Vec2::new(3. / 2., SQRT_THREE / 2.);
const BASIS_R: Vec2 = Vec2::new(0., SQRT_THREE);

pub struct Pos {
    pub q: f32,
    pub r: f32,
    pub height: f32,
}

impl Pos {
    pub fn new(q: f32, r: f32, height: f32) -> Self {
        Self { q, r, height }
    }
    /// The third coordinate 's' in the axial system is always equal to minus the sum of q and r.
    pub fn s(&self) -> f32 {
        -self.q - self.r
    }

    /// TODO: Use matrix calculation instead?
    pub fn to_pixel(&self) -> Vec3 {
        let xz = (BASIS_Q * self.q + BASIS_R * self.r) * RADIUS;
        Vec3::new(xz.x, self.height, xz.y)
    }
}

// impl From<Vec3> for Pos {
//     fn from(world_coords: Vec3) -> Self {
//
//     }
// }
