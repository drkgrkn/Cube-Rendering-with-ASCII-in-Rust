use core::time;
use std::{f64::consts::PI, thread, time::Instant};

use cube::{
    cube::Cube,
    point::Point,
    rotation_matrix::{RotationAngle, RotationMatrix},
    space::Space,
};

fn main() {
    let mut cube: Cube = Cube {
        vertices: [
            Point::new(-1., -1., -1.),
            Point::new(-1., -1., 1.),
            Point::new(-1., 1., -1.),
            Point::new(-1., 1., 1.),
            Point::new(1., -1., -1.),
            Point::new(1., -1., 1.),
            Point::new(1., 1., -1.),
            Point::new(1., 1., 1.),
        ],
    };

    let mut space = Space::new();
    space.create_new_frame();
    space.render_frame();

    let mut start = Instant::now();
    loop {
        let delta = start.elapsed();
        start = Instant::now();
        let rotation_matrix =
            RotationMatrix::new(RotationAngle::X, delta.as_secs_f64() * (PI / 4.));
        cube.rotate(&rotation_matrix);
        space.create_new_frame();
        for p in cube.vertices {
            let projected_point = p.from_pespective(&space.pov);
            space.render_point(projected_point);
        }
        thread::sleep(time::Duration::from_millis(100));
        clear_screen();
        space.render_frame();
    }
}
enum Drawable {
    HorizontalEdge,    // -
    VerticalEdge,      // |
    LeftDiagonalEdge,  // /
    RightDiagonalEdge, // \
    Vertex,            // +
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}
