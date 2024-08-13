use render::{
    mesh::{builder::MeshBuilder, Mesh},
    renderer::Renderer,
};

use crate::tile::Tile;

pub const CHUNK_SIZE: usize = 32;
const CHUNK_VOLUME: usize = CHUNK_SIZE.pow(3);

pub struct Chunk {
    tiles: Vec<Option<Tile>>,
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}

impl Chunk {
    pub fn new() -> Self {
        let tiles = Vec::from_iter(std::iter::repeat_with(|| None).take(CHUNK_VOLUME));
        Self { tiles }
    }

    pub fn set(&mut self, loc: [usize; 3], tile: Option<Tile>) {
        self.tiles[loc[0] + loc[1] * CHUNK_SIZE + loc[2] * CHUNK_SIZE * CHUNK_SIZE] = tile;
    }

    pub fn get(&self, loc: [usize; 3]) -> Option<&Tile> {
        self.tiles[loc[0] + loc[1] * CHUNK_SIZE + loc[2] * CHUNK_SIZE * CHUNK_SIZE].as_ref()
    }

    pub fn mesh(&self, renderer: &Renderer) -> Mesh {
        let mut builder = MeshBuilder::default();
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let i = x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE;
                    if self.tiles[i].is_some() {
                        builder.add_range([x as f32, y as f32, z as f32], 0..6);
                    }
                }
            }
        }

        builder.build(renderer)
    }
}
