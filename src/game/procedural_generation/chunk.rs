use crate::game::procedural_generation::xy_to_index;
use bevy::prelude::*;

use super::{
    generate_biomes, generate_noise, get_noise_profile, map_value, BiomeType, NoiseLayer,
    CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH,
};

pub struct Chunk {
    blocks: Vec<Block>,
}

impl Chunk {
    pub fn get_block(position: IVec3) -> Block {
        Block {
            block_type: BlockType::Air,
            biome_type: BiomeType::Forest,
        }
    }

    pub fn set_block(position: IVec3) {}

    pub fn new(position: IVec2) -> Self {
        let elevation_noise = generate_noise(position, get_noise_profile(NoiseLayer::Elevation));
        let humidity_noise = generate_noise(position, get_noise_profile(NoiseLayer::Humidity));
        let temperature_noise =
            generate_noise(position, get_noise_profile(NoiseLayer::Temperature));

        let biomes = generate_biomes(humidity_noise, temperature_noise);

        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                let elevation = elevation_noise[xy_to_index(IVec2::new(x as i32, y as i32))];

                let biome_type = biomes[xy_to_index(IVec2::new(x as i32, y as i32))];

                for z in 0..CHUNK_DEPTH {
                    let block_type: BlockType;
                    let z_elevation = map_value(elevation, -1.0, 1.0, 0.0, CHUNK_DEPTH as f64);
                    if z < z_elevation as usize {
                        block_type = BlockType::Stone;
                    } else {
                        block_type = BlockType::Air;
                    }

                    let block = Block {
                        block_type,
                        biome_type,
                    };
                }
            }
        }

        Chunk { blocks: Vec::new() }
    }
}

pub struct Block {
    pub block_type: BlockType,
    pub biome_type: BiomeType,
}

pub enum BlockType {
    Air,
    Stone,
    Dirt,
    Grass,
    Water,
}

// References
// 1. 3D -> 1D Array, 1D -> 3D Array
// https://stackoverflow.com/questions/7367770/how-to-flatten-or-index-3d-array-in-1d-array
