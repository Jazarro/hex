use bevy::log::info;
use rand::distributions::Bernoulli;
use rand::distributions::Distribution;
use rand::Rng;

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
    pub fn get(&self, q: usize, r: usize, z: usize) -> Option<Block> {
        let err = || {
            format!(
                "Chunk lookup index out of bounds. Tried to look up \
        (q={},r={},z={}). Expected axial coordinates relative to chunk, so they should not exceed \
        chunk length/width/height.",
                q, r, z
            )
        };
        self.blocks
            .get(z)
            .expect(&err())
            .get(r)
            .expect(&err())
            .get(q)
            .expect(&err())
            .clone()
    }
    pub fn set(&mut self, q: usize, r: usize, z: usize, block: Option<Block>) {
        let err = || {
            format!(
                "Chunk lookup index out of bounds. Tried to look up \
        (q={},r={},z={}). Expected axial coordinates relative to chunk, so they should not exceed \
        chunk length/width/height.",
                q, r, z
            )
        };
        *self
            .blocks
            .get_mut(z)
            .expect(&err())
            .get_mut(r)
            .expect(&err())
            .get_mut(q)
            .expect(&err()) = block;
    }

    /// Generate a chunk consisting of columns of random height. Only for testing.
    pub fn random() -> Self {
        let mut rand = rand::thread_rng();
        let mut distribution = Bernoulli::new(0.8).unwrap();
        let mut chunk = Chunk::default();
        let mut count_true = 0;
        let mut count_false = 0;
        (0..CHUNK_DIMENSION_Z).for_each(|z| {
            (0..CHUNK_DIMENSION_R).for_each(|r| {
                (0..CHUNK_DIMENSION_Q).for_each(|q| {
                    let hmm = distribution.sample(&mut rand::thread_rng());
                    if hmm {
                        count_true += 1;
                    } else {
                        count_false += 1;
                    }
                    if z == 0 || (chunk.get(q, r, z - 1).is_some() && hmm) {
                        chunk.set(q, r, z, Some(Block::default()));
                    }
                });
            });
        });
        info!("Count is true={}, false={}", count_true, count_false);
        chunk
    }
}
