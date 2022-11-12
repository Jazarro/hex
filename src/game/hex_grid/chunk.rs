use bevy::math::{IVec2, Vec3Swizzles};

use crate::game::hex_grid::axial::{ChunkId, ColumnId, IPos};
use crate::game::hex_grid::biomes::generate_biomes;
use crate::game::hex_grid::block::{Block, BlockType};
use crate::game::hex_grid::chunks::map_value;
use crate::game::procedural_generation::noise_generation::{
    generate_noise, get_noise_profile, NoiseLayer,
};

pub const CHUNK_RADIUS: usize = 8;
pub const CHUNK_HEIGHT: usize = 32;
pub const CHUNK_BOUNDS: usize = CHUNK_RADIUS * 2 + 1;

pub struct Chunk {
    /// Outer array is Q-coordinate. Second array is R-coordinate. Inner array is Z-coordinate.
    /// Since chunks are hexagonal, not all columns in a square array exist.
    /// That's why the inner array is wrapped in an Option.
    blocks: [[Option<[Block; CHUNK_HEIGHT]>; CHUNK_BOUNDS]; CHUNK_BOUNDS],
}

impl Default for Chunk {
    fn default() -> Self {
        let mut chunk = Self {
            blocks: [[None; CHUNK_BOUNDS]; CHUNK_BOUNDS],
        };
        for pos in Chunk::chunk_columns().iter() {
            let storage_pos = Chunk::column_to_storage_pos(pos);
            *chunk
                .blocks
                .get_mut(storage_pos.q() as usize)
                .unwrap_or_else(|| panic!("{}", Self::index_out_of_bounds(&pos.as_ipos(0))))
                .get_mut(storage_pos.r() as usize)
                .unwrap_or_else(|| panic!("{}", Self::index_out_of_bounds(&pos.as_ipos(0)))) =
                Some([Block::default(); CHUNK_HEIGHT]);
        }
        chunk
    }
}

impl Chunk {
    /// Converts relative position (relative to the chunk center)
    /// to storage position (internal array storage).
    /// This is needed because the relative position can be negative; it spirals out from (0,0).
    fn storage_pos(relative_pos: &IPos) -> IPos {
        relative_pos + &IPos::new(CHUNK_RADIUS as i32, CHUNK_RADIUS as i32, 0)
    }
    /// Converts relative position (relative to the chunk center)
    /// to storage position (internal array storage).
    /// This is needed because the relative position can be negative; it spirals out from (0,0).
    fn column_to_storage_pos(relative_pos: &ColumnId) -> ColumnId {
        relative_pos + &ColumnId::new(CHUNK_RADIUS as i32, CHUNK_RADIUS as i32)
    }
    #[must_use]
    pub fn block(&self, pos: &IPos) -> &Block {
        let storage_pos = Chunk::storage_pos(pos);
        self.blocks
            .get(storage_pos.q() as usize)
            .unwrap_or_else(|| panic!("{}", Self::index_out_of_bounds(pos)))
            .get(storage_pos.r() as usize)
            .unwrap_or_else(|| panic!("{}", Self::index_out_of_bounds(pos)))
            .as_ref()
            .unwrap_or_else(|| panic!("{}", Self::index_out_of_bounds(pos)))
            .get(storage_pos.z() as usize)
            .unwrap_or_else(|| panic!("{}", Self::index_out_of_bounds(pos)))
    }
    pub fn set(&mut self, pos: &IPos, block: Block) {
        let storage_pos = Chunk::storage_pos(pos);
        *self
            .blocks
            .get_mut(storage_pos.q() as usize)
            .unwrap_or_else(|| panic!("{}", Self::index_out_of_bounds(pos)))
            .get_mut(storage_pos.r() as usize)
            .unwrap_or_else(|| panic!("{}", Self::index_out_of_bounds(pos)))
            .as_mut()
            .unwrap_or_else(|| panic!("{}", Self::index_out_of_bounds(pos)))
            .get_mut(storage_pos.z() as usize)
            .unwrap_or_else(|| panic!("{}", Self::index_out_of_bounds(pos))) = block;
    }
    fn index_out_of_bounds(pos: &IPos) -> String {
        format!(
            "Chunk lookup index out of bounds. Tried to access \
        (q={},r={},z={}). Expected axial coordinates relative to chunk; they should not exceed \
        chunk length/width/height.",
            pos.q(),
            pos.r(),
            pos.z(),
        )
    }

    pub fn from_noise(chunk_id: &ChunkId) -> Self {
        let noise_pos = Chunk::storage_pos(&chunk_id.center_pos())
            .as_xyz() // TODO: Do we need storage pos here?
            .xy()
            .as_dvec2();
        let noise_bounds = IVec2::splat(CHUNK_BOUNDS as i32);
        let elevation_noise = generate_noise(
            noise_pos,
            noise_bounds,
            get_noise_profile(NoiseLayer::Elevation),
        );
        let humidity_noise = generate_noise(
            noise_pos,
            noise_bounds,
            get_noise_profile(NoiseLayer::Humidity),
        );
        let temperature_noise = generate_noise(
            noise_pos,
            noise_bounds,
            get_noise_profile(NoiseLayer::Temperature),
        );

        let biomes = generate_biomes(humidity_noise, temperature_noise);
        let mut chunk = Chunk::default();

        let mut min_elevation = 0.0;
        let mut max_elevation = 0.0;
        for ele in elevation_noise.iter() {
            if ele < &min_elevation {
                min_elevation = *ele;
            }
            if ele > &max_elevation {
                max_elevation = *ele;
            }
        }
        // debug!(
        //     "min elevation: {}, max elevation: {}",
        //     min_elevation, max_elevation
        // );

        for qr in Chunk::chunk_columns().iter() {
            let noise_pos = Chunk::column_to_storage_pos(qr);
            let index = (noise_pos.r() as usize * CHUNK_BOUNDS) + noise_pos.q() as usize;
            let elevation = elevation_noise[index];
            let z_elevation = map_value(elevation, -1.0, 1.0, 0.0, CHUNK_HEIGHT as f64);
            // debug!("Elevation: {} \t {}", elevation, z_elevation);
            let biome_type = biomes[index];

            for z in 0..CHUNK_HEIGHT {
                let block_type = if z < z_elevation as usize {
                    BlockType::Stone
                } else {
                    BlockType::Air
                };
                let pos = qr.as_ipos(z as i32);
                chunk.set(
                    &pos,
                    Block {
                        block_type,
                        biome_type,
                    },
                );
            }
        }
        chunk
    }

    /// A Vec of relative positions of all blocks in a chunk.
    /// These are positions relative to the chunk's center.
    pub fn chunk_columns() -> Vec<ColumnId> {
        ColumnId::spiral(CHUNK_RADIUS as u32)
    }
}
