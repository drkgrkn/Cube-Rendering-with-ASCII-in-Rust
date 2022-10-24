pub mod cube;
pub mod point;
pub mod rotation_matrix;
pub mod space;

pub const NUM_COL: isize = 200;
pub const NUM_ROW: isize = 50;
pub const VERTICAL_LENGTH: f64 = 24.;
pub const HORIZONTAL_LENGTH: f64 = 12.;
pub const POV: f64 = 10.;

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn from_perspective_test() {
        let p1 = point::Point::new(0., 0.5, 0.5);
        let p2 = point::Point::new(0., 0., 1.);
        let p3 = point::Point::new(0., 0.5, -1.);

        assert_eq!(point::Point::new(0., 1., 0.), p1.from_pespective(&p2));
        assert_eq!(point::Point::new(0., 0.25, 0.), p3.from_pespective(&p2));
    }

    #[test]
    fn as_unit_test() {
        let p1 = point::Point::new(0., 1., 0.);
        let p2 = point::Point::new(0., 2., 0.);

        assert_eq!(p1, p2.as_unit());
    }

    #[test]
    fn project_onto_test() {
        let p1 = point::Point::new(0., 0.5, 0.5);
        let p2 = point::Point::new(0., 0., 1.);

        assert_eq!(point::Point::new(0., 0., 0.5), p1.project_onto(&p2));
    }

    #[test]
    fn dot_test() {
        let p1 = point::Point::new(0., 0.5, 0.5);
        let p2 = point::Point::new(0., 0., 1.);

        assert_eq!(0.5, p1.dot(&p2));
    }
}
