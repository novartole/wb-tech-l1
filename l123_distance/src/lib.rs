#![allow(dead_code)]

mod point {
    pub struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        pub fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }

        pub fn dist(&self, point: &Self) -> f64 {
            let a = point.x - self.x;
            let b = point.y - self.y;

            f64::sqrt(a * a + b * b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::point::Point;

    use std::f64::consts::SQRT_2;

    #[test]
    fn dist_sqrt_2() {
        let a = Point::new(-0.5, -0.5);
        let b = Point::new(0.5, 0.5);

        assert!((a.dist(&b) - SQRT_2).abs() < f64::EPSILON);
    }

    #[test]
    fn dist_0() {
        let a = Point::new(-0.0, 0.0);
        let b = Point::new(0.0, -0.0);

        assert!((a.dist(&b) - 0.0).abs() < f64::EPSILON);
    }
}
