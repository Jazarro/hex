use bevy::math::{IVec2, Vec3Swizzles};
use rand::distributions::Bernoulli;
use rand::distributions::Distribution;

use crate::game::hex_grid::axial::Pos;
use crate::game::hex_grid::chunks::{map_value, xy_to_index};
use crate::game::procedural_generation::biomes::generate_biomes;
use crate::game::procedural_generation::block::{Block, BlockType};
use crate::game::procedural_generation::noise_generation::{generate_noise, get_noise_profile, NoiseLayer};

pub const CHUNK_DIMENSION_Q: usize = 16;
pub const CHUNK_DIMENSION_R: usize = 16;
pub const CHUNK_DIMENSION_Z: usize = 16;

#[derive(Default)]
pub struct Chunk {
    blocks: [[[Option<Block>; CHUNK_DIMENSION_Q]; CHUNK_DIMENSION_R]; CHUNK_DIMENSION_Z],
}

impl Chunk {
    pub fn get(&self, q: usize, r: usize, z: usize) -> &Option<Block> {
        self.blocks
            .get(z)
            .expect(&Self::index_out_of_bounds(q, r, z))
            .get(r)
            .expect(&Self::index_out_of_bounds(q, r, z))
            .get(q)
            .expect(&Self::index_out_of_bounds(q, r, z))
    }
    pub fn set(&mut self, q: usize, r: usize, z: usize, block: Option<Block>) {
        *self
            .blocks
            .get_mut(z)
            .expect(&Self::index_out_of_bounds(q, r, z))
            .get_mut(r)
            .expect(&Self::index_out_of_bounds(q, r, z))
            .get_mut(q)
            .expect(&Self::index_out_of_bounds(q, r, z)) = block;
    }
    fn index_out_of_bounds(q: usize, r: usize, z: usize) -> String {
        format!(
            "Chunk lookup index out of bounds. Tried to access \
        (q={},r={},z={}). Expected axial coordinates relative to chunk; they should not exceed \
        chunk length/width/height.",
            q, r, z
        )
    }
    /// Generate a chunk consisting of columns of random height. Only for testing.
    pub fn random() -> Self {
        let distribution = Bernoulli::new(0.8).unwrap();
        let mut chunk = Chunk::default();
        (0..CHUNK_DIMENSION_Z).for_each(|z| {
            (0..CHUNK_DIMENSION_R).for_each(|r| {
                (0..CHUNK_DIMENSION_Q).for_each(|q| {
                    if z == 0
                        || (chunk.get(q, r, z - 1).is_some()
                            && distribution.sample(&mut rand::thread_rng()))
                    {
                        chunk.set(q, r, z, Some(Block::default()));
                    }
                });
            });
        });
        chunk
    }
    pub fn new(position: IVec2) -> Self {
        let xy_position = Pos::new(position.x as f32, position.y as f32, 0.).as_xyz().xy().as_ivec2();
        let elevation_noise = generate_noise(xy_position, get_noise_profile(NoiseLayer::Elevation));
        let humidity_noise = generate_noise(xy_position, get_noise_profile(NoiseLayer::Humidity));
        let temperature_noise =
            generate_noise(xy_position, get_noise_profile(NoiseLayer::Temperature));

        let biomes = generate_biomes(humidity_noise, temperature_noise);
        let mut chunk = Chunk::default();

        for q in 0..CHUNK_DIMENSION_Q {
            for r in 0..CHUNK_DIMENSION_R {
                let xy = Pos::new(q as f32, r as f32, 0.).as_xyz().xy().as_ivec2();
                let elevation = elevation_noise[xy_to_index(xy)];
                let z_elevation = map_value(elevation, -1.0, 1.0, 0.0, CHUNK_DIMENSION_Z as f64);
                let biome_type = biomes[xy_to_index(xy)];

                for z in 0..CHUNK_DIMENSION_Z {
                    let block_type: BlockType;
                    if z < z_elevation as usize {
                        block_type = BlockType::Stone;
                    } else {
                        block_type = BlockType::Air;
                    }

                    if matches!(block_type, BlockType::Air) {
                        chunk.set(q, r, z, None);
                    } else {
                        chunk.set(q, r, z, Some(Block {
                            block_type,
                            biome_type,
                        }));
                    }
                }
            }
        }

        chunk
    }
}
