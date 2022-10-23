use crate::{point::Point, HORIZONTAL_LENGTH, NUM_COL, NUM_ROW, VERTICAL_LENGTH};

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
            pov: Point::new(0., 0., 5.),
        }
    }

    pub fn create_new_frame(&mut self) {
        for _ in 0..NUM_ROW {
            let mut row: Vec<char> = vec![];
            for _ in 0..NUM_COL {
                row.push(' ');
            }
            self.frame.push(row);
        }
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
        let coord = p.space_to_frame();
        let mut int_coord: (isize, isize) = (0, 0);

        println!("coord {:?}", coord);
        println!("hl {:?}", HORIZONTAL_LENGTH);
        println!("vl {:?}", VERTICAL_LENGTH);
        int_coord.0 = HORIZONTAL_LENGTH * coord.0.round() as isize;
        int_coord.1 = VERTICAL_LENGTH * coord.1.round() as isize;

        println!("int_coord {:?}", int_coord);

        println!("{:?}", (self.origin.0 + int_coord.0));
        println!("{:?}", (self.origin.1 + int_coord.1));
        self.frame[(self.origin.0 + int_coord.0) as usize][(self.origin.1 + int_coord.1) as usize] =
            '+'
    }
}
