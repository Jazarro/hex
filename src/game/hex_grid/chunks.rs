use crate::game::hex_grid::axial::IPos;
use bevy::math::IVec2;
use bevy::utils::HashMap;

use crate::game::hex_grid::chunk::{Chunk, CHUNK_DIMENSION_Q, CHUNK_DIMENSION_R};

#[derive(Default)]
pub struct Chunks {
    pub chunks: HashMap<IVec2, Chunk>,
}

impl Chunks {
    pub fn generate_chunk(&mut self, position: IVec2) {
        let chunk = Chunk::new(position);
        self.chunks.insert(position, chunk);
    }

    pub fn load_chunk_from_disk(position: IVec2) {}

    pub fn save_chunk_to_disk(position: IVec2) {}
}

pub fn qr_to_index(position: IPos) -> usize {
    (position.r() as usize * CHUNK_DIMENSION_Q) + position.q() as usize
}

pub fn index_to_qr(index: usize) -> IPos {
    let q = index as i32 % CHUNK_DIMENSION_Q as i32;
    let r = index as i32 / CHUNK_DIMENSION_Q as i32;
    let z = 0;
    IPos::new(q, r, z)
}

pub fn qrz_to_index(position: IPos) -> usize {
    ((position.z() * CHUNK_DIMENSION_Q as i32 * CHUNK_DIMENSION_R as i32)
        + (position.r() * CHUNK_DIMENSION_Q as i32)
        + position.q()) as usize
}

pub fn index_to_qrz(index: usize) -> IPos {
    let z = index / (CHUNK_DIMENSION_Q * CHUNK_DIMENSION_R);
    let r = (index - (z * CHUNK_DIMENSION_Q * CHUNK_DIMENSION_R)) / CHUNK_DIMENSION_Q;
    let q = index - (z * CHUNK_DIMENSION_Q * CHUNK_DIMENSION_R) - (r * CHUNK_DIMENSION_Q);
    IPos::new(q as i32, r as i32, z as i32)
}

pub fn map_value(value: f64, from_min: f64, from_max: f64, to_min: f64, to_max: f64) -> f64 {
    let from_range = from_max - from_min;
    let to_range = to_max - to_min;
    let value_scaled = (value - from_min) / from_range;
    to_min + (value_scaled * to_range)
}

// References
// 1. 3D -> 1D Array, 1D -> 3D Array
// https://stackoverflow.com/questions/7367770/how-to-flatten-or-index-3d-array-in-1d-array
