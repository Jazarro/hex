use std::cmp::Ordering;
use std::ops::{Add, Sub};

use bevy::math::*;
use serde::{Deserialize, Serialize};

use crate::game::hex_grid::axial::{IPos, Pos, RADIUS, SQRT_THREE};
use crate::game::hex_grid::chunk::{CHUNK_HEIGHT, CHUNK_RADIUS};

/// Distance away from the origin in a pointy-topped axial coordinate system for every 1m distance
/// along the x-axis in a regular 'square' coordinate system.
const CHUNK_QR_PER_X: Vec2 = Vec2::new(SQRT_THREE / 3., 0.);
/// Distance away from the origin in a pointy-topped axial coordinate system for every 1m distance
/// along the y-axis in a regular 'square' coordinate system.
const CHUNK_QR_PER_Y: Vec2 = Vec2::new(-1. / 3., 2. / 3.);

/// ChunkIds are pointy-topped axial coordinates. They are NOT compatible with block
/// positions (IPos), which use flat-topped axial coordinates.
/// There are functions to convert between them.
#[derive(Deserialize, Serialize, Default, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChunkId(pub(crate) IVec3);

impl ChunkId {
    #[must_use]
    pub fn new(q: i32, r: i32, z: i32) -> Self {
        ChunkId(IVec3::new(q, r, z))
    }
    #[must_use]
    pub fn splat(value: i32) -> Self {
        ChunkId(IVec3::splat(value))
    }
    #[must_use]
    pub fn delta(&self, delta_q: i32, delta_r: i32, delta_z: i32) -> Self {
        ChunkId(IVec3::new(
            self.q() + delta_q,
            self.r() + delta_r,
            self.z() + delta_z,
        ))
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
    pub fn center_pos(&self) -> IPos {
        IPos::new(
            (CHUNK_RADIUS as i32 * 2 + 1) * self.q() + CHUNK_RADIUS as i32 * self.r(),
            CHUNK_RADIUS as i32 * -self.q() + (CHUNK_RADIUS as i32 + 1) * self.r(),
            self.z() * CHUNK_HEIGHT as i32,
        )
    }
    #[must_use]
    pub fn from_xyz(xyz: &Vec3) -> Self {
        let distance_to_edge = (CHUNK_RADIUS as f32 + 0.5) * RADIUS * SQRT_THREE;
        let half_edge = RADIUS * 0.5;
        let chunk_virtual_hex_radius = distance_to_edge.hypot(half_edge);
        let angle = -1. * (half_edge / chunk_virtual_hex_radius).asin();
        let xyz = Vec3::new(
            xyz.x * angle.cos() - xyz.y * angle.sin(),
            xyz.x * angle.sin() + xyz.y * angle.cos(),
            xyz.z,
        );
        let qr = (CHUNK_QR_PER_X * xyz.x + CHUNK_QR_PER_Y * xyz.y) / chunk_virtual_hex_radius;
        let rounded = Pos::new(qr.x, qr.y, 0.).round();
        ChunkId::new(
            rounded.q() as i32,
            rounded.r() as i32,
            (xyz.z / CHUNK_HEIGHT as f32).floor() as i32,
        )
    }
    #[must_use]
    pub fn from_block_pos(block_pos: &IPos) -> Self {
        Self::from_xyz(&block_pos.as_xyz())
    }
}

/// Order by q first, then r, then z.
///
/// This is implemented purely to make it possible to save level files in a deterministic way.
impl PartialOrd<Self> for ChunkId {
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

impl Ord for ChunkId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Sub for ChunkId {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl Add for ChunkId {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Sub for &ChunkId {
    type Output = ChunkId;

    fn sub(self, other: &ChunkId) -> ChunkId {
        ChunkId(self.0 - other.0)
    }
}

impl Add for &ChunkId {
    type Output = ChunkId;

    fn add(self, other: &ChunkId) -> ChunkId {
        ChunkId(self.0 + other.0)
    }
}
