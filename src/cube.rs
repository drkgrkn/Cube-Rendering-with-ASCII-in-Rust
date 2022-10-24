use crate::{point, rotation_matrix::RotationMatrix};

#[derive(Debug)]
pub struct Cube {
    pub vertices: [point::Point; 8],
}

impl Cube {
    pub fn rotate(&mut self, rotation_matrix: &RotationMatrix) {
        self.vertices = [
            rotation_matrix * &self.vertices[0],
            rotation_matrix * &self.vertices[1],
            rotation_matrix * &self.vertices[2],
            rotation_matrix * &self.vertices[3],
            rotation_matrix * &self.vertices[4],
            rotation_matrix * &self.vertices[5],
            rotation_matrix * &self.vertices[6],
            rotation_matrix * &self.vertices[7],
        ];
    }
}
