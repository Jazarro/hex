use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

use bevy::math::*;
use serde::{Deserialize, Serialize};

use crate::game::hex_grid::axial::Pos;

/// A hexagonal coordinate in the flat-topped axial coordinate system.
/// This is the integral version. There is also a floating-point version.
/// For more information, see https://www.redblobgames.com/grids/hexagons/
#[derive(Deserialize, Serialize, Default, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IPos(pub(crate) IVec3);

impl IPos {
    #[must_use]
    pub fn new(q: i32, r: i32, z: i32) -> Self {
        IPos(IVec3::new(q, r, z))
    }
    #[must_use]
    pub fn splat(value: i32) -> Self {
        IPos(IVec3::splat(value))
    }
    #[must_use]
    pub fn delta(&self, delta_q: i32, delta_r: i32, delta_z: i32) -> Self {
        IPos(IVec3::new(
            self.q() + delta_q,
            self.r() + delta_r,
            self.z() + delta_z,
        ))
    }
    #[must_use]
    pub fn from_xyz(xyz: &Vec3) -> Self {
        Pos::from_xyz(xyz).as_ipos_round()
    }
    #[must_use]
    pub fn q(&self) -> i32 {
        self.0.x
    }
    #[must_use]
    pub fn r(&self) -> i32 {
        self.0.y
    }
    /// The third coordinate 's' in the axial system is always equal to minus the sum of q and r.
    #[must_use]
    pub fn s(&self) -> i32 {
        -self.q() - self.r()
    }
    /// The height-coordinate. This is the same as in the xyz-coordinate system.
    #[must_use]
    pub fn z(&self) -> i32 {
        self.0.z
    }
    /// Take only the qr-components of the coordinates.
    #[must_use]
    pub fn qr(&self) -> IVec2 {
        self.0.xy()
    }
    /// Convert to the floating-point version of this struct.
    #[must_use]
    pub fn as_pos(&self) -> Pos {
        Pos(self.0.as_vec3())
    }
    /// Convert axial coordinates to regular, square coordinates.
    #[must_use]
    pub fn as_xyz(&self) -> Vec3 {
        self.as_pos().as_xyz()
    }
    /// Returns the distance on the hexagonal grid.
    #[must_use]
    pub fn distance(a: &Self, b: &Self) -> i32 {
        let delta = a - b;
        delta.q().abs().max(delta.r().abs()).max(delta.z().abs())
    }
    #[must_use]
    pub fn neighbour(&self, neighbour_index: u32) -> Self {
        self + &Self::direction(neighbour_index)
    }
    #[must_use]
    pub fn direction(neighbour_index: u32) -> Self {
        // On the unit circle, the vertex at i=0 is located at (1,0).
        // Vertices are going counter-clockwise around the unit circle.
        // The first neighbour at (q=1, r=0) borders the face between the first two vertices.
        match neighbour_index.rem_euclid(6) {
            0 => IPos::new(1, 0, 0),  // <== North-east neighbour
            1 => IPos::new(0, 1, 0),  // <== North neighbour.
            2 => IPos::new(-1, 1, 0), // <== North-west neighbour.
            3 => IPos::new(-1, 0, 0), // <== South-west neighbour.
            4 => IPos::new(0, -1, 0), // <== South neighbour.
            5 => IPos::new(1, -1, 0), // <== South-east neighbour.
            _ => panic!("Shouldn't happen, compiler doesn't realise this match is exhaustive."),
        }
    }
}

/// Order by q first, then r, then z.
///
/// This is implemented purely to make it possible to save level files in a deterministic way.
impl PartialOrd<Self> for IPos {
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

impl Ord for IPos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Add for IPos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Add for &IPos {
    type Output = IPos;

    fn add(self, other: &IPos) -> IPos {
        IPos(self.0 + other.0)
    }
}

impl Sub for IPos {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl Sub for &IPos {
    type Output = IPos;

    fn sub(self, other: &IPos) -> IPos {
        IPos(self.0 - other.0)
    }
}

impl Mul for IPos {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        IPos(self.0 * other.0)
    }
}

impl Mul<i32> for IPos {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        IPos(self.0 * rhs)
    }
}

impl Div for IPos {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        IPos(self.0 / other.0)
    }
}
