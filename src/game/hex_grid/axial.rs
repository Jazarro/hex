use bevy::prelude::*;

/// Height of a standard hex block in meters.
pub const HEIGHT: f32 = 1.;
/// Long radius (distance between center and corner point) of a standard hex block in meters.
pub const RADIUS: f32 = 0.6;
/// The square root of three. Precalculated for performance and ease of use.
const SQRT_THREE: f32 = 1.7320508;
/// Distance away from the origin in a regular 'square' coordinate system for every 1m distance
/// along the q-axis in the flat-topped axial coordinate system.
const XY_PER_Q: Vec2 = Vec2::new(3. / 2., SQRT_THREE / 2.);
/// Distance away from the origin in a regular 'square' coordinate system for every 1m distance
/// along the r-axis in the flat-topped axial coordinate system.
const XY_PER_R: Vec2 = Vec2::new(0., SQRT_THREE);
/// Distance away from the origin in a flat-topped axial coordinate system for every 1m distance
/// along the x-axis in a regular 'square' coordinate system.
const QR_PER_X: Vec2 = Vec2::new(3. / 2., 1. / 3.);
/// Distance away from the origin in a flat-topped axial coordinate system for every 1m distance
/// along the y-axis in a regular 'square' coordinate system.
const QR_PER_Y: Vec2 = Vec2::new(0., SQRT_THREE / 3.);


/// A hexagonal coordinate in the flat-topped axial coordinate system.
/// For more information, see https://www.redblobgames.com/grids/hexagons/
pub struct Pos {
    pub q: f32,
    pub r: f32,
    /// The height. This is unchanged from the regular xyz coordinate system.
    pub z: f32,
}

/// TODO: Use matrix calculation instead?
impl Pos {
    pub fn new(q: f32, r: f32, z: f32) -> Self {
        Self { q, r, z }
    }

    /// The third coordinate 's' in the axial system is always equal to minus the sum of q and r.
    pub fn s(&self) -> f32 {
        -self.q - self.r
    }

    /// Convert axial coordinates to regular, square coordinates.
    pub fn to_xyz(&self) -> Vec3 {
        let xy = (XY_PER_Q * self.q + XY_PER_R * self.r) * RADIUS;
        Vec3::new(xy.x, xy.y, self.z)
    }

    pub fn from_xyz(xyz: &Vec3) -> Self {
        let qr = (QR_PER_X * xyz.x + QR_PER_Y * xyz.y) / RADIUS;
        Pos::new(qr.x, qr.y, xyz.z)
    }

    /// Round to the nearest integral hex.
    pub fn round(&self) -> Self {
        let rounded_q = self.q.round();
        let rounded_r = self.r.round();
        let rounded_s = self.s().round();
        let delta_q = (rounded_q - self.q).abs();
        let delta_r = (rounded_r - self.r).abs();
        let delta_s = (rounded_s - self.s()).abs();

        if delta_q > delta_r && delta_q > delta_s {
            Self::new(-rounded_r - rounded_s, rounded_r, self.z.round())
        } else if delta_r > delta_s {
            Self::new(rounded_q, -rounded_q - rounded_s, self.z.round())
        } else {
            Self::new(rounded_q, rounded_r, self.z.round())
        }
    }
}
