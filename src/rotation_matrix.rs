use std::ops;

use crate::point::Point;

#[derive(Debug)]

pub struct RotationMatrix {
    matrix: [[f64; 3]; 3],
}

pub enum RotationAngle {
    X,
    Y,
    Z,
}

impl RotationMatrix {
    pub fn new(rotation_angle: RotationAngle, theta: f64) -> RotationMatrix {
        match rotation_angle {
            RotationAngle::X => RotationMatrix {
                matrix: [
                    [1., 0., 0.],
                    [0., theta.cos(), -theta.sin()],
                    [0., theta.sin(), theta.cos()],
                ],
            },
            RotationAngle::Y => RotationMatrix {
                matrix: [
                    [theta.cos(), 0., theta.sin()],
                    [0., 1., 0.],
                    [-theta.sin(), 0., theta.cos()],
                ],
            },
            RotationAngle::Z => RotationMatrix {
                matrix: [
                    [theta.cos(), -theta.sin(), 0.],
                    [theta.sin(), theta.cos(), 0.],
                    [0., 0., 1.],
                ],
            },
        }
    }
}

impl ops::Mul<&Point> for &RotationMatrix {
    type Output = Point;

    fn mul(self, v: &Point) -> Self::Output {
        let new_x = self.matrix[0][0] * v.x + self.matrix[0][1] * v.y + self.matrix[0][2] * v.z;
        let new_y = self.matrix[1][0] * v.x + self.matrix[1][1] * v.y + self.matrix[1][2] * v.z;
        let new_z = self.matrix[2][0] * v.x + self.matrix[2][1] * v.y + self.matrix[2][2] * v.z;

        Point::new(new_x, new_y, new_z)
    }
}

impl ops::Mul<&RotationMatrix> for &RotationMatrix {
    type Output = RotationMatrix;

    fn mul(self, b: &RotationMatrix) -> Self::Output {
        RotationMatrix {
            matrix: [
                [
                    self.matrix[0][0] * b.matrix[0][0]
                        + self.matrix[0][1] * b.matrix[1][0]
                        + self.matrix[0][2] * b.matrix[2][0],
                    self.matrix[0][0] * b.matrix[0][1]
                        + self.matrix[0][1] * b.matrix[1][1]
                        + self.matrix[0][2] * b.matrix[2][1],
                    self.matrix[0][0] * b.matrix[0][2]
                        + self.matrix[0][1] * b.matrix[1][2]
                        + self.matrix[0][2] * b.matrix[2][2],
                ],
                [
                    self.matrix[1][0] * b.matrix[0][0]
                        + self.matrix[1][1] * b.matrix[1][0]
                        + self.matrix[1][2] * b.matrix[2][0],
                    self.matrix[1][0] * b.matrix[0][1]
                        + self.matrix[1][1] * b.matrix[1][1]
                        + self.matrix[1][2] * b.matrix[2][1],
                    self.matrix[1][0] * b.matrix[0][2]
                        + self.matrix[1][1] * b.matrix[1][2]
                        + self.matrix[1][2] * b.matrix[2][2],
                ],
                [
                    self.matrix[2][0] * b.matrix[0][0]
                        + self.matrix[2][1] * b.matrix[1][0]
                        + self.matrix[2][2] * b.matrix[2][0],
                    self.matrix[2][0] * b.matrix[0][1]
                        + self.matrix[2][1] * b.matrix[1][1]
                        + self.matrix[2][2] * b.matrix[2][1],
                    self.matrix[2][0] * b.matrix[0][2]
                        + self.matrix[2][1] * b.matrix[1][2]
                        + self.matrix[2][2] * b.matrix[2][2],
                ],
            ],
        }
    }
}
