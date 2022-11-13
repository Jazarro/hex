use std::cmp::Ordering;
use std::ops::{Add, Sub};

use bevy::prelude::*;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use crate::game::hex_grid::axial::IPos;

/// Height of a standard hex block in meters.
pub const HEIGHT: f32 = 1.;
/// Long radius (distance between center and corner point) of a standard hex block in meters.
pub const RADIUS: f32 = 0.6;
/// The square root of three. Precalculated for performance and ease of use.
pub const SQRT_THREE: f32 = 1.7320508;
/// One sixth Tau or one third Pi. Convenience constant because Tau is just nicer.
pub const FRAC_TAU_6: f32 = std::f32::consts::FRAC_PI_3;
/// Matrix for converting coordinates from xyz to qrz.
static XYZ_TO_QRZ: OnceCell<Mat4> = OnceCell::new();
/// Matrix for converting coordinates from qrz to xyz.
static QRZ_TO_XYZ: OnceCell<Mat4> = OnceCell::new();

/// A hexagonal coordinate in the flat-topped axial coordinate system.
/// This is the floating-point version. There is also an integral version.
/// For more information, see https://www.redblobgames.com/grids/hexagons/
#[derive(Deserialize, Serialize, Default, Copy, Clone, Debug, PartialEq)]
pub struct Pos(pub(crate) Vec3);

/// TODO: Use matrix calculation instead?
impl Pos {
    #[must_use]
    pub fn new(q: f32, r: f32, z: f32) -> Self {
        Self(Vec3::new(q, r, z))
    }
    #[must_use]
    pub fn from_xyz(xyz: &Vec3) -> Self {
        Pos(XYZ_TO_QRZ
            .get()
            .expect("Pos::setup() was never called!")
            .transform_vector3(*xyz))
    }
    #[must_use]
    pub fn q(&self) -> f32 {
        self.0.x
    }
    #[must_use]
    pub fn r(&self) -> f32 {
        self.0.y
    }
    /// The third coordinate 's' in the axial system is always equal to minus the sum of q and r.
    #[must_use]
    pub fn s(&self) -> f32 {
        -self.q() - self.r()
    }
    /// The height-coordinate. This is the same as in the xyz-coordinate system.
    #[must_use]
    pub fn z(&self) -> f32 {
        self.0.z
    }
    /// Convert to the integer version of this struct by applying the floor function to each of
    /// its components.
    #[must_use]
    pub fn as_ipos_floor(&self) -> IPos {
        IPos::new(self.q() as i32, self.r() as i32, self.z() as i32)
    }
    /// Convert to the integer version of this struct by applying the ceil function to each of
    /// its components.
    #[must_use]
    pub fn as_ipos_ceil(&self) -> IPos {
        IPos::new(
            self.q().ceil() as i32,
            self.r().ceil() as i32,
            self.z().ceil() as i32,
        )
    }
    /// Convert to the integer version of this struct by rounding to the nearest integral hex.
    /// Note that this is not the same as rounding each of its components!
    #[must_use]
    pub fn as_ipos_round(&self) -> IPos {
        IPos(self.round().0.as_ivec3())
    }
    /// Convert axial coordinates to regular, square coordinates.
    #[must_use]
    pub fn as_xyz(&self) -> Vec3 {
        QRZ_TO_XYZ
            .get()
            .expect("Pos::setup() was never called!")
            .transform_vector3(self.0)
    }
    /// Round to the nearest integral hex.
    #[must_use]
    pub fn round(&self) -> Self {
        let rounded_q = self.q().round();
        let rounded_r = self.r().round();
        let rounded_s = self.s().round();
        let delta_q = (rounded_q - self.q()).abs();
        let delta_r = (rounded_r - self.r()).abs();
        let delta_s = (rounded_s - self.s()).abs();

        if delta_q > delta_r && delta_q > delta_s {
            Self::new(-rounded_r - rounded_s, rounded_r, self.z().round())
        } else if delta_r > delta_s {
            Self::new(rounded_q, -rounded_q - rounded_s, self.z().round())
        } else {
            Self::new(rounded_q, rounded_r, self.z().round())
        }
    }
    /// Returns the distance on the hexagonal grid.
    #[must_use]
    pub fn distance(a: &Self, b: &Self) -> f32 {
        let delta = a - b;
        delta.q().max(delta.r()).max(delta.z())
    }

    /// Call this once at the start of the application.
    pub(crate) fn setup() {
        XYZ_TO_QRZ.set(Self::setup_xyz_to_qrz())
            .expect("Setup was called twice, but you should only call it once at the start of the application.");
        QRZ_TO_XYZ.set(Self::setup_qrz_to_xyz())
            .expect("Setup was called twice, but you should only call it once at the start of the application.");
    }
    /// Do not call this outside of the setup stage.
    pub(crate) fn setup_xyz_to_qrz() -> Mat4 {
        Mat4::from_cols(
            Vec4::new(2. / 3., -1. / 3., 0., 0.),
            Vec4::new(0., SQRT_THREE / 3., 0., 0.),
            Vec4::new(0., 0., 1., 0.),
            Vec4::new(0., 0., 0., 1.),
        )
        .mul_scalar(1. / RADIUS)
    }
    /// Do not call this outside of the setup stage.
    pub(crate) fn setup_qrz_to_xyz() -> Mat4 {
        Mat4::from_cols(
            Vec4::new(3. / 2., SQRT_THREE / 2., 0., 0.),
            Vec4::new(0., SQRT_THREE, 0., 0.),
            Vec4::new(0., 0., 1., 0.),
            Vec4::new(0., 0., 0., 1.),
        )
        .mul_scalar(RADIUS)
    }
}

/// Order by q first, then r, then z.
///
/// This is implemented purely to make it possible to save level files in a deterministic way.
impl PartialOrd<Self> for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.q() < other.q() {
            Some(Ordering::Less)
        } else if self.q() > other.q() {
            Some(Ordering::Greater)
        } else if self.r() < other.r() {
            Some(Ordering::Less)
        } else if self.r() > other.r() {
            Some(Ordering::Greater)
        } else if self.z() < other.z() {
            Some(Ordering::Less)
        } else if self.z() > other.z() {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Sub for &Pos {
    type Output = Pos;

    fn sub(self, other: &Pos) -> Pos {
        Pos(self.0 - other.0)
    }
}

impl Add for &Pos {
    type Output = Pos;

    fn add(self, other: &Pos) -> Pos {
        Pos(self.0 + other.0)
    }
}
