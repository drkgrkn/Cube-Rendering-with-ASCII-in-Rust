use std::{cmp, ops};

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }

    pub fn rotate(&mut self, matrix: [[f64; 3]; 3]) {
        let new_x = matrix[0][0] * self.x + matrix[0][1] * self.y + matrix[0][2] * self.z;
        let new_y = matrix[1][0] * self.x + matrix[1][1] * self.y + matrix[1][2] * self.z;
        let new_z = matrix[2][0] * self.x + matrix[2][1] * self.y + matrix[2][2] * self.z;

        self.x = new_x;
        self.y = new_y;
        self.z = new_z;
    }

    pub fn from_pespective(&self, perspective_point: &Point) -> Point {
        let projected_self: Point = self.project_onto(perspective_point);

        let normal_self: Point = self - &projected_self;

        // let ratio = if projected_self.dot(&projected_self.as_unit()) < 0. {
        //     &(perspective_point.abs() + &projected_self.abs()) / perspective_point.abs()
        // } else {
        //     &(perspective_point.abs() - &projected_self.abs()) / perspective_point.abs()
        // };

        let ratio: f64 = if (perspective_point + &projected_self).abs()
            > (perspective_point - &projected_self).abs()
        {
            // println!(
            //     "{:?} / {:?} + {:?}",
            //     perspective_point.abs(),
            //     perspective_point.abs(),
            //     projected_self.abs()
            // );
            perspective_point.abs() / &(perspective_point.abs() - &projected_self.abs())
        } else {
            // println!(
            //     "{:?} / {:?} - {:?}",
            //     perspective_point.abs(),
            //     perspective_point.abs(),
            //     projected_self.abs()
            // );
            perspective_point.abs() / &(perspective_point.abs() + &projected_self.abs())
        };

        &normal_self * &ratio
    }

    pub fn dot(&self, p2: &Point) -> f64 {
        self.x * p2.x + self.y * p2.y + self.z * p2.z
    }

    pub fn abs(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn as_unit(&self) -> Point {
        let abs = self.abs();
        Point {
            x: self.x / abs,
            y: self.y / abs,
            z: self.z / abs,
        }
    }

    pub fn project_onto(&self, p2: &Point) -> Point {
        p2 * &(self.dot(p2) / p2.dot(p2))
    }

    pub fn origin() -> Point {
        Point {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn space_to_frame(self) -> (f64, f64) {
        (self.x, self.y)
    }
}

impl ops::Add<&Point> for &Point {
    type Output = Point;

    fn add(self, p2: &Point) -> Self::Output {
        Point {
            x: self.x + p2.x,
            y: self.y + p2.y,
            z: self.z + p2.z,
        }
    }
}

impl ops::Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, p2: &Point) -> Self::Output {
        Point {
            x: self.x - p2.x,
            y: self.y - p2.y,
            z: self.z - p2.z,
        }
    }
}

impl ops::Mul<&f64> for &Point {
    type Output = Point;

    fn mul(self, alpha: &f64) -> Self::Output {
        Point {
            x: self.x * alpha,
            y: self.y * alpha,
            z: self.z * alpha,
        }
    }
}

impl ops::Div<&f64> for &Point {
    type Output = Point;

    fn div(self, alpha: &f64) -> Self::Output {
        Point {
            x: self.x / alpha,
            y: self.y / alpha,
            z: self.z / alpha,
        }
    }
}

impl cmp::PartialEq<Point> for Point {
    fn eq(&self, p2: &Point) -> bool {
        if self.x == p2.x && self.y == p2.y && self.z == p2.z {
            return true;
        }
        false
    }
}
