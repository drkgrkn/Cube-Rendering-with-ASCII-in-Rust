use std::f32::consts::PI;

use crate::{point::Point, HORIZONTAL_LENGTH, NUM_COL, NUM_ROW, POV, VERTICAL_LENGTH};

pub struct Space {
    pub frame: Vec<Vec<char>>,
    pub origin: (isize, isize),
    pub pov: Point,
}

impl Space {
    pub fn new() -> Space {
        Space {
            frame: Vec::with_capacity((NUM_COL * NUM_ROW) as usize),
            origin: (NUM_ROW / 2, NUM_COL / 2),
            pov: Point::new(0., 0., POV),
        }
    }

    pub fn create_new_frame(&mut self) {
        let mut new_frame: Vec<Vec<char>> = Vec::with_capacity(NUM_ROW as usize * NUM_COL as usize);
        for _ in 0..NUM_ROW {
            let mut row: Vec<char> = vec![];
            for _ in 0..NUM_COL {
                row.push(' ');
            }
            new_frame.push(row);
        }
        self.frame = new_frame;
    }

    pub fn render_frame(&self) {
        for row in &self.frame {
            for c in row {
                print!("{}", c);
            }
            print!("\n");
        }
    }

    pub fn render_point(&mut self, point: Point) {
        let p = point.from_pespective(&self.pov);
        let coord = p.space_to_plane();
        let int_coord = self.plane_to_frame(coord);

        self.frame[(self.origin.0 + int_coord.0) as usize][(self.origin.1 + int_coord.1) as usize] =
            '+'
    }

    pub fn plane_to_frame(&self, p: (f64, f64)) -> (isize, isize) {
        (
            (HORIZONTAL_LENGTH * p.0).round() as isize,
            (VERTICAL_LENGTH * p.1).round() as isize,
        )
    }

    pub fn draw_line(&mut self, p0: (isize, isize), p1: (isize, isize)) {
        let line = (p0.0 - p1.0, p0.1 - p1.1);
        let angle = ((line.1 / line.0) as f64).atan();

        let symbol = self.get_ascii(angle);

        if line.0 > line.1 {
            let step_size = line.0 / line.1;
            println!("{:?}", step_size);
            for i in p0.0..p1.0 {
                for j in 0..step_size {
                    println!("{:?} {:?}", i, j);
                    self.frame[i as usize][(p0.0 + j) as usize] = symbol;
                }
            }
        } else {
            let step_size = line.1 / line.0;
            println!("{:?}", step_size);
            for i in p0.1..p1.1 {
                for j in 0..step_size {
                    println!("{:?} {:?}", i, j);
                    self.frame[(p0.0 + i + j) as usize][i as usize] = symbol;
                }
            }
        }
    }

    pub fn get_ascii(&self, angle: f64) -> char {
        if angle >= (PI * 3. / 8.).into() {
            '|'
        } else if angle >= (PI / 8.).into() {
            '/'
        } else if angle >= (-PI / 8.).into() {
            '-'
        } else {
            '\\'
        }
    }
}
