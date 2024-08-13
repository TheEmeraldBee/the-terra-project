use crate::{vertex, Renderer, Vertex};

use super::Mesh;

#[derive(Default, Clone)]
pub struct MeshBuilder {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    face_count: u32,
}

impl MeshBuilder {
    pub fn with_added(mut self, chord: [f32; 3], face: usize) -> Self {
        self.add(chord, face);
        self
    }

    pub fn add(&mut self, cord: [f32; 3], face: usize) -> &mut Self {
        // Push all vertice faces
        for i in &VERTICES[face] {
            self.vertices.push(*i + cord)
        }

        for i in &INDICES {
            self.indices.push(*i + (4 * self.face_count))
        }

        self.face_count += 1;
        self
    }

    pub fn with_translation(mut self, distance: [f32; 3]) -> Self {
        self.translate(distance);
        self
    }

    pub fn translate(&mut self, distance: [f32; 3]) -> &mut Self {
        for vertex in &mut self.vertices {
            vertex.position[0] += distance[0];
        }
        for vertex in &mut self.vertices {
            vertex.position[1] += distance[1];
        }
        for vertex in &mut self.vertices {
            vertex.position[2] += distance[2];
        }
        self
    }

    pub fn build(self, renderer: &Renderer) -> Mesh {
        Mesh::new(renderer, &self.vertices, &self.indices)
    }
}

const INDICES: [u32; 6] = [0, 1, 2, 2, 1, 3];
const VERTICES: [[Vertex; 4]; 6] = [
    [
        // Top
        vertex(0.0, 1.0, 0.0),
        vertex(0.0, 1.0, 1.0),
        vertex(1.0, 1.0, 0.0),
        vertex(1.0, 1.0, 1.0),
    ],
    [
        // Bottom
        vertex(0.0, 0.0, 1.0),
        vertex(0.0, 0.0, 0.0),
        vertex(1.0, 0.0, 1.0),
        vertex(1.0, 0.0, 0.0),
    ],
    [
        // Left
        vertex(0.0, 0.0, 1.0),
        vertex(0.0, 1.0, 1.0),
        vertex(0.0, 0.0, 0.0),
        vertex(0.0, 1.0, 0.0),
    ],
    [
        // Right
        vertex(1.0, 0.0, 0.0),
        vertex(1.0, 1.0, 0.0),
        vertex(1.0, 0.0, 1.0),
        vertex(1.0, 1.0, 1.0),
    ],
    [
        // Front
        vertex(1.0, 0.0, 0.0),
        vertex(1.0, 1.0, 0.0),
        vertex(0.0, 0.0, 0.0),
        vertex(0.0, 1.0, 0.0),
    ],
    [
        // Back
        vertex(1.0, 0.0, 1.0),
        vertex(1.0, 1.0, 1.0),
        vertex(0.0, 0.0, 1.0),
        vertex(0.0, 1.0, 1.0),
    ],
];
