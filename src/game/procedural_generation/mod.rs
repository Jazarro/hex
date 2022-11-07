use bevy::prelude::*;
use bevy::utils::HashMap;

mod biomes;
mod chunk;
mod noise_generation;

use biomes::*;
use chunk::*;
use noise_generation::*;

// x
const CHUNK_WIDTH: usize = 32;
// y
const CHUNK_HEIGHT: usize = 32;
// z
const CHUNK_DEPTH: usize = 64;

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

pub fn setup_chunks(mut commands: Commands) {
    let chunks = Chunks {
        chunks: Default::default(),
    };
    commands.insert_resource(chunks);
}

pub fn xy_to_index(position: IVec2) -> usize {
    (position.y as usize * CHUNK_WIDTH) + position.x as usize
}

pub fn index_to_xy(index: usize) -> IVec2 {
    let x = index as i32 % CHUNK_WIDTH as i32;
    let y = index as i32 / CHUNK_WIDTH as i32;
    IVec2 { x, y }
}

pub fn xyz_to_index(position: IVec3) -> usize {
    ((position.z * CHUNK_WIDTH as i32 * CHUNK_HEIGHT as i32)
        + (position.y * CHUNK_WIDTH as i32)
        + position.x) as usize
}

pub fn index_to_xyz(index: usize) -> IVec3 {
    let z = index / (CHUNK_WIDTH * CHUNK_HEIGHT);
    let y = (index - (z * CHUNK_WIDTH * CHUNK_HEIGHT)) / CHUNK_WIDTH;
    let x = index - (z * CHUNK_WIDTH * CHUNK_HEIGHT) - (y * CHUNK_WIDTH);
    IVec3::new(x as i32, y as i32, z as i32)
}

pub fn map_value(value: f64, from_min: f64, from_max: f64, to_min: f64, to_max: f64) -> f64 {
    let from_range = from_max - from_min;
    let to_range = to_max - to_min;
    let value_scaled = (value - from_min) / from_range;
    to_min + (value_scaled * to_range)
}
