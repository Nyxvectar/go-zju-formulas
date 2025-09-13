/**
 * Author:  Raye Lattice  
 * Repo:    guhs
 * Created: 08/13/2025
 */

pub mod vector {
    #[derive(Debug, Clone, Copy)]
    pub struct Vector {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    const EPSILON: f64 = 1e-10;

    impl Vector {
        pub fn magnitude(&self) -> f64 {
            (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
        }

        fn is_zero(&self) -> bool {
            self.x.abs() < EPSILON && self.y.abs() < EPSILON && self.z.abs() < EPSILON
        }
    }

    pub fn are_collinear(a: Vector, b: Vector) -> bool {
        if a.is_zero() || b.is_zero() {
            return true;
        }
        almost_equal(a.x * b.y, a.y * b.x) &&
            almost_equal(a.x * b.z, a.z * b.x) &&
            almost_equal(a.y * b.z, a.z * b.y)
    }

    pub fn dot_product(a: Vector, b: Vector) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cos_angle(a: Vector, b: Vector) -> f64 {
        if a.is_zero() || b.is_zero() {
            return 0.0;
        }
        dot_product(a, b) / (a.magnitude() * b.magnitude())
    }

    pub fn cross_product(a: Vector, b: Vector) -> Vector {
        Vector {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    fn almost_equal(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON || (a - b).abs() < EPSILON * a.abs().max(b.abs())
    }
}
