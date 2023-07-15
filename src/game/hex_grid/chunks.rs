use bevy::math::IVec2;
use bevy::utils::{HashMap, HashSet};

use crate::game::hex_grid::axial::{ChunkId, IPos};
use crate::game::hex_grid::block::Block;
use crate::game::hex_grid::chunk::{Chunk, CHUNK_BOUNDS};
use bevy::ecs::system::Resource;
#[derive(Default, Resource)]
pub struct Chunks {
    chunks: HashMap<ChunkId, Chunk>,
}

impl Chunks {
    #[must_use]
    pub fn block(&self, pos: &IPos) -> Option<&Block> {
        let chunk_id = ChunkId::from_block_pos(pos);
        let pos_relative = pos - &chunk_id.center_pos();
        self.chunks
            .get(&chunk_id)
            .map(|chunk| chunk.block(&pos_relative))
    }
    #[must_use]
    pub fn is_solid(&self, pos_absolute: &IPos) -> bool {
        self.block(pos_absolute)
            .map(|block| block.is_solid())
            .unwrap_or(false)
    }
    /// Removes chunks from memory that are not in the given whitelist.
    pub fn cull_chunks(&mut self, allowed: &HashSet<ChunkId>) {
        let keys: Vec<_> = self.chunks.keys().filter(|&key| !allowed.contains(key))
            .collect();
        for &key in keys.iter() {
            self.chunks.remove(key);
        }
    }
    #[must_use]
    pub fn contains(&self, id: &ChunkId) -> bool {
        self.chunks.contains_key(id)
    }

    pub fn generate_chunk(&mut self, chunk_id: ChunkId) {
        let chunk = Chunk::from_noise(&chunk_id);
        self.chunks.insert(chunk_id, chunk);
    }

    pub fn get_chunk(&self, chunk_id: &ChunkId) -> &Chunk {
        self.chunks.get(chunk_id).unwrap() // TODO: fix this unsafe unwrap?
    }

    #[allow(dead_code)] // Will be implemented later.
    pub fn load_chunk_from_disk(position: IVec2) {}

    #[allow(dead_code)] // Will be implemented later.
    pub fn save_chunk_to_disk(position: IVec2) {}
}

pub fn qr_to_index(position: &IPos) -> usize {
    (position.r() as usize * CHUNK_BOUNDS) + position.q() as usize
}

pub fn index_to_qr(index: usize) -> IPos {
    let q = index as i32 % CHUNK_BOUNDS as i32;
    let r = index as i32 / CHUNK_BOUNDS as i32;
    let z = 0;
    IPos::new(q, r, z)
}

pub fn qrz_to_index(position: IPos) -> usize {
    ((position.z() * CHUNK_BOUNDS as i32 * CHUNK_BOUNDS as i32)
        + (position.r() * CHUNK_BOUNDS as i32)
        + position.q()) as usize
}

pub fn index_to_qrz(index: usize) -> IPos {
    let z = index / (CHUNK_BOUNDS * CHUNK_BOUNDS);
    let r = (index - (z * CHUNK_BOUNDS * CHUNK_BOUNDS)) / CHUNK_BOUNDS;
    let q = index - (z * CHUNK_BOUNDS * CHUNK_BOUNDS) - (r * CHUNK_BOUNDS);
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
