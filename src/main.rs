use cube::{cube::Cube, point::Point, space::Space};

fn main() {
    let cube: Cube = Cube {
        vertices: [
            Point::new(-1.5, -1.5, -1.5),
            Point::new(-1.5, -1.5, 1.5),
            Point::new(-1.5, 1.5, -1.5),
            Point::new(-1.5, 1.5, 1.5),
            Point::new(1.5, -1.5, -1.5),
            Point::new(1.5, -1.5, 1.5),
            Point::new(1.5, 1.5, -1.5),
            Point::new(1.5, 1.5, 1.5),
        ],
    };

    let mut space = Space::new();
    space.create_new_frame();
    clear_screen();
    space.render_frame();

    for p in cube.vertices {
        let projected_point = p.from_pespective(&space.pov);
        space.render_point(projected_point);
    }

    //clear_screen();
    space.render_frame();
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
