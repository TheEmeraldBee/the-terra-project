use render::{
    mesh::{builder::MeshBuilder, Mesh},
    renderer::Renderer,
};

macro_rules! conditional_tile {
    ($self:expr, $loc:expr, $offset:expr, $builder:expr, $idx:expr, $pos_offset:expr) => {{
        if let Some(pos) = loc($loc, $offset) {
            if $self.get(pos).is_none() {
                $builder.add(
                    [
                        $loc[0] as f32 + $pos_offset[0],
                        $loc[1] as f32 + $pos_offset[1],
                        $loc[2] as f32 + $pos_offset[2],
                    ],
                    $idx,
                );
            }
        } else {
            $builder.add(
                [
                    $loc[0] as f32 + $pos_offset[0],
                    $loc[1] as f32 + $pos_offset[1],
                    $loc[2] as f32 + $pos_offset[2],
                ],
                $idx,
            );
        }
    }};
}

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

    pub fn mesh(&self, renderer: &Renderer, offset: [f32; 3]) -> Mesh {
        let mut builder = MeshBuilder::default();
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let i = x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE;
                    if self.tiles[i].is_some() {
                        // Top
                        conditional_tile!(self, [x, y, z], [0, 1, 0], builder, 0, offset);
                        // Bottom
                        conditional_tile!(self, [x, y, z], [0, -1, 0], builder, 1, offset);
                        // Left
                        conditional_tile!(self, [x, y, z], [-1, 0, 0], builder, 2, offset);
                        // Right
                        conditional_tile!(self, [x, y, z], [1, 0, 0], builder, 3, offset);
                        // Front
                        conditional_tile!(self, [x, y, z], [0, 0, -1], builder, 4, offset);
                        // Back
                        conditional_tile!(self, [x, y, z], [0, 0, 1], builder, 5, offset);
                    }
                }
            }
        }

        builder.build(renderer)
    }
}

fn loc(pos: [usize; 3], offset: [i32; 3]) -> Option<[usize; 3]> {
    let mut pos = [pos[0] as i32, pos[1] as i32, pos[2] as i32];
    pos[0] += offset[0];
    pos[1] += offset[1];
    pos[2] += offset[2];

    for v in pos {
        if v < 0 || v >= CHUNK_SIZE as i32 {
            return None;
        }
    }
    Some([pos[0] as usize, pos[1] as usize, pos[2] as usize])
}
