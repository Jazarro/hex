use crate::game::hex_grid::biomes::BiomeType;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlockType {
    Air,
    Stone,
    Dirt,
    Grass,
    Water,
}

#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub block_type: BlockType,
    pub biome_type: BiomeType,
}

impl Default for Block {
    fn default() -> Self {
        Block {
            block_type: BlockType::Stone,
            biome_type: BiomeType::Grassland,
        }
    }
}

impl Block {
    pub fn is_solid(&self) -> bool {
        self.block_type != BlockType::Air
    }
}
