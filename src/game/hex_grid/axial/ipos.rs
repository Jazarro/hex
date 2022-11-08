use std::cmp::Ordering;
use std::ops::{Add, Sub};

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
        delta.q().max(delta.r()).max(delta.z())
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

impl Sub for IPos {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl Add for IPos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Sub for &IPos {
    type Output = IPos;

    fn sub(self, other: &IPos) -> IPos {
        IPos(self.0 - other.0)
    }
}

impl Add for &IPos {
    type Output = IPos;

    fn add(self, other: &IPos) -> IPos {
        IPos(self.0 + other.0)
    }
}
