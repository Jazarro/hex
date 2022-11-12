use std::ops::{Add, Mul};

use bevy::math::IVec2;

use crate::game::hex_grid::axial::{ChunkId, IPos};

#[derive(Default, Copy, Clone)]
pub struct ColumnId(IVec2);

impl ColumnId {
    #[must_use]
    pub fn new(q: i32, r: i32) -> Self {
        Self(IVec2::new(q, r))
    }
    #[must_use]
    pub fn splat(value: i32) -> Self {
        ColumnId(IVec2::splat(value))
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
    #[must_use]
    pub fn as_ipos(&self, z: i32) -> IPos {
        IPos::new(self.q(), self.r(), z)
    }
    #[must_use]
    pub fn as_chunk_id(&self, z: i32) -> ChunkId {
        ChunkId::new(self.q(), self.r(), z)
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
            0 => ColumnId::new(1, 0),  // <== North-east neighbour
            1 => ColumnId::new(0, 1),  // <== North neighbour.
            2 => ColumnId::new(-1, 1), // <== North-west neighbour.
            3 => ColumnId::new(-1, 0), // <== South-west neighbour.
            4 => ColumnId::new(0, -1), // <== South neighbour.
            5 => ColumnId::new(1, -1), // <== South-east neighbour.
            _ => panic!("Shouldn't happen, compiler doesn't realise this match is exhaustive."),
        }
    }
    /// Returns a Vec of all ColumnIds in a hexagon of a given radius, centered on (0, 0).
    /// It does this by repeatedly calling ColumnId::ring() in a spiral pattern.
    #[must_use]
    pub fn spiral(radius: u32) -> Vec<ColumnId> {
        let center = ColumnId::default();
        let mut results = vec![center];
        (1..=radius).for_each(|ring_index| {
            results.append(&mut Self::ring(ring_index));
        });
        results
    }
    /// Returns a Vec of all ColumnIds in a hexagonal ring of a given radius around (0, 0).
    #[must_use]
    pub fn ring(radius: u32) -> Vec<ColumnId> {
        let mut results = vec![];
        // Loop through the six segments of the ring:
        (0..6).for_each(|i| {
            let mut pos_on_ring = ColumnId::direction(i) * radius as i32;
            // Loop through each block on the ring segment.
            // This happens to be exactly radius steps.
            (0..radius).for_each(|j| {
                results.push(pos_on_ring);
                pos_on_ring = pos_on_ring.neighbour(i + 2);
            });
        });
        results
    }
}

impl Add<ColumnId> for ColumnId {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Add<&ColumnId> for &ColumnId {
    type Output = ColumnId;

    fn add(self, other: &ColumnId) -> ColumnId {
        ColumnId(self.0 + other.0)
    }
}

impl Add<IPos> for ColumnId {
    type Output = IPos;

    fn add(self, other: IPos) -> IPos {
        self.as_ipos(0) + other
    }
}

impl Add<IPos> for &ColumnId {
    type Output = IPos;

    fn add(self, other: IPos) -> IPos {
        self.as_ipos(0) + other
    }
}

impl Add<ChunkId> for ColumnId {
    type Output = ChunkId;

    fn add(self, other: ChunkId) -> ChunkId {
        self.as_chunk_id(0) + other
    }
}

impl Mul<i32> for ColumnId {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        ColumnId(self.0 * rhs)
    }
}
