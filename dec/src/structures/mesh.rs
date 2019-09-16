extern crate nalgebra_glm as glm;
extern crate tobj;
use std::path::Path;

struct HalfEdge {
    pub vertex: u32,
    pub face: i32,
    pub next: i32,
}

struct Vertex {
    // One HE connected to it
    pub he: i32,
    // Vertex's position in Euclidian space
    pub position: u32,
}

struct Face {
    pub he: i32,
}

/// Mesh structure represented by half edges.
pub struct Mesh {
    // HE are always stored in pairs. The twin of the HE with odd
    // index o is o - 1 and the twin of the HE with even index e
    // is e + 1. **half_edges** stores the face connected to each
    // he following the index rule described above.
    half_edges: Vec<HalfEdge>,
    // Each face points to one of the HE connected to it.
    faces: Vec<Face>,
    // Each vertex holds its position in Euclidian space and a HE
    // index that is connected to it.
    vertices: Vec<Vertex>,
    positions: Vec<glm::Vec3>,
}

fn load_obj(file: &String) {
    let obj = tobj::load_obj(&Path::new(file));
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            half_edges: Vec::new(),
            faces: Vec::new(),
            vertices: Vec::new(),
            positions: Vec::new(),
        }
    }
    pub fn twin(&self, he: u32) -> i32 {
        if he % 2 == 0 {
            return (he + 1) as i32;
        }
        (he - 1) as i32
    }
    pub fn next(&self, he: u32) {}
    pub fn add_vertex(&mut self, position: glm::Vec3) -> u32 {
        let size = self.vertices.len();
        let position_index = self.positions.len();
        self.positions.push(position);
        self.vertices.push(Vertex {
            he: -1,
            position: position_index as u32,
        });
        size as u32
    }
    pub fn add_edge(&mut self, i: usize, j: usize) -> u32 {
        let size = self.half_edges.len();
        self.vertices[i].he = size as i32;
        self.vertices[j].he = (size + 1) as i32;
        self.half_edges.push(HalfEdge {
            face: -1,
            vertex: i as u32,
            next: -1,
        });
        self.half_edges.push(HalfEdge {
            face: -1,
            vertex: j as u32,
            next: -1,
        });
        size as u32
    }
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }
    pub fn edge_count(&self) -> usize {
        self.half_edges.len() / 2
    }
}

#[cfg(test)]
mod tests {
    use crate::structures::Mesh;
    extern crate nalgebra_glm as glm;
    #[test]
    fn empty_mesh() {
        let mut m: Mesh = Mesh::new();
        assert_eq!(0 as usize, m.vertex_count());
        for _ in 0..10 {
            m.add_vertex(glm::vec3(0f32, 0f32, 0f32));
        }
        assert_eq!(10 as usize, m.vertex_count());
    }
}
