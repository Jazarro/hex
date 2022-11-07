use rand::distributions::Bernoulli;
use rand::distributions::Distribution;

pub const CHUNK_DIMENSION_Q: usize = 16;
pub const CHUNK_DIMENSION_R: usize = 16;
pub const CHUNK_DIMENSION_Z: usize = 16;

#[derive(Default)]
pub struct Chunk {
    blocks: [[[Option<Block>; CHUNK_DIMENSION_Q]; CHUNK_DIMENSION_R]; CHUNK_DIMENSION_Z],
}

#[derive(Default, Clone)]
pub struct Block {}

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
}
