use bevy::math::IVec2;
use bevy::utils::HashMap;

use crate::game::hex_grid::axial::IPos;
use crate::game::hex_grid::block::Block;
use crate::game::hex_grid::chunk::{
    Chunk, CHUNK_DIMENSION_Q, CHUNK_DIMENSION_R, CHUNK_DIMENSION_Z,
};

#[derive(Default)]
pub struct Chunks {
    pub chunks: HashMap<IPos, Chunk>,
}

impl Chunks {
    pub fn pos_block_to_chunk(pos: &IPos) -> IPos {
        let pos_f = pos.as_pos();
        IPos::new(
            (pos_f.q() / CHUNK_DIMENSION_Q as f32).floor() as i32,
            (pos_f.r() / CHUNK_DIMENSION_R as f32).floor() as i32,
            (pos_f.z() / CHUNK_DIMENSION_Z as f32).floor() as i32,
        )
    }
    pub fn get(&self, pos: &IPos) -> Option<Block> {
        // TODO:...
        None
    }
    /// pos: An absolute position.
    pub fn is_solid(&self, pos_absolute: &IPos) -> bool {
        let pos_relative = IPos::new(
            pos_absolute.q().rem_euclid(CHUNK_DIMENSION_Q as i32),
            pos_absolute.r().rem_euclid(CHUNK_DIMENSION_R as i32),
            pos_absolute.z().rem_euclid(CHUNK_DIMENSION_Z as i32),
        );
        self.chunks
            .get(&Self::pos_block_to_chunk(&pos_absolute))
            .map(|chunk| chunk.get(&pos_relative).is_solid())
            .unwrap_or(false)
    }

    pub fn generate_chunk(&mut self, position: IPos) {
        let chunk = Chunk::new(IVec2::new(position.q(), position.r()));
        self.chunks.insert(position, chunk);
    }

    pub fn get_chunk(&self, position: &IPos) -> &Chunk {
        &self.chunks.get(position).unwrap()
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
