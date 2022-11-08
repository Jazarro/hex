use bevy::math::IVec2;
use rand::distributions::Bernoulli;

use crate::game::hex_grid::axial::IPos;
use crate::game::hex_grid::biomes::generate_biomes;
use crate::game::hex_grid::block::{Block, BlockType};
use crate::game::hex_grid::chunks::{map_value, qr_to_index};
use crate::game::procedural_generation::noise_generation::{
    generate_noise, get_noise_profile, NoiseLayer,
};

pub const CHUNK_DIMENSION_Q: usize = 32;
pub const CHUNK_DIMENSION_R: usize = 32;
pub const CHUNK_DIMENSION_Z: usize = 64;

pub struct Chunk {
    blocks: [[[Block; CHUNK_DIMENSION_Q]; CHUNK_DIMENSION_R]; CHUNK_DIMENSION_Z],
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk {
            blocks: [[[Block::default(); CHUNK_DIMENSION_Q]; CHUNK_DIMENSION_R]; CHUNK_DIMENSION_Z],
        }
    }
}

impl Chunk {
    pub fn get(&self, q: usize, r: usize, z: usize) -> &Block {
        self.blocks
            .get(z)
            .unwrap_or_else(|| panic!("{}", Self::index_out_of_bounds(q, r, z)))
            .get(r)
            .unwrap_or_else(|| panic!("{}", Self::index_out_of_bounds(q, r, z)))
            .get(q)
            .unwrap_or_else(|| panic!("{}", Self::index_out_of_bounds(q, r, z)))
    }
    pub fn set(&mut self, q: usize, r: usize, z: usize, block: Block) {
        *self
            .blocks
            .get_mut(z)
            .unwrap_or_else(|| panic!("{}", Self::index_out_of_bounds(q, r, z)))
            .get_mut(r)
            .unwrap_or_else(|| panic!("{}", Self::index_out_of_bounds(q, r, z)))
            .get_mut(q)
            .unwrap_or_else(|| panic!("{}", Self::index_out_of_bounds(q, r, z))) = block;
    }
    fn index_out_of_bounds(q: usize, r: usize, z: usize) -> String {
        format!(
            "Chunk lookup index out of bounds. Tried to access \
        (q={},r={},z={}). Expected axial coordinates relative to chunk; they should not exceed \
        chunk length/width/height.",
            q, r, z
        )
    }

    pub fn new(position: IVec2) -> Self {
        let elevation_noise = generate_noise(position, get_noise_profile(NoiseLayer::Elevation));
        let humidity_noise = generate_noise(position, get_noise_profile(NoiseLayer::Humidity));
        let temperature_noise =
            generate_noise(position, get_noise_profile(NoiseLayer::Temperature));

        let biomes = generate_biomes(humidity_noise, temperature_noise);
        let mut chunk = Chunk::default();

        for q in 0..CHUNK_DIMENSION_Q {
            for r in 0..CHUNK_DIMENSION_R {
                let elevation = elevation_noise[qr_to_index(IPos::new(q as i32, r as i32, 0))];
                let z_elevation = map_value(elevation, -1.0, 1.0, 0.0, CHUNK_DIMENSION_Z as f64);
                let biome_type = biomes[qr_to_index(IPos::new(q as i32, r as i32, 0))];

                for z in 0..CHUNK_DIMENSION_Z {
                    let block_type = if z < z_elevation as usize {
                        BlockType::Stone
                    } else {
                        BlockType::Air
                    };

                    chunk.set(
                        q,
                        r,
                        z,
                        Block {
                            block_type,
                            biome_type,
                        },
                    );
                }
            }
        }

        chunk
    }

    /// Generate a chunk consisting of columns of random height. Only for testing.
    pub fn random() -> Self {
        let distribution = Bernoulli::new(0.8).unwrap();
        let mut chunk = Chunk::default();
        (0..CHUNK_DIMENSION_Z).for_each(|z| {
            (0..CHUNK_DIMENSION_R).for_each(|r| {
                (0..CHUNK_DIMENSION_Q).for_each(|q| {
                    if z == 0 {
                        chunk.set(q, r, z, Block::default());
                    }
                });
            });
        });
        chunk
    }
}
