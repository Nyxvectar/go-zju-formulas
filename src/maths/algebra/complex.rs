/**
 * Author:  Nyxvectar Yan 
 * Repo:    guhs
 * Created: 08/15/2025
 */

pub mod algebra {
    use std::f64::consts::SQRT_2;

    #[derive(Debug, Clone, Copy)]
    pub struct Complex {
        pub real: f64,
        pub imaginary: f64,
    }

    const DIVIDE_BY_ZERO_ERROR: &str = "Panic: 试图除以零或者及其接近零的数";

    pub fn add(a: Complex, b: Complex) -> Complex {
        Complex {
            real: a.real + b.real,
            imaginary: a.imaginary + b.imaginary,
        }
    }

    pub fn multiply(a: Complex, b: Complex) -> Complex {
        Complex {
            real: a.real * b.real - a.imaginary * b.imaginary,
            imaginary: a.real * b.imaginary + a.imaginary * b.real,
        }
    }

    pub fn divide(a: Complex, b: Complex) -> Complex {
        let denominator = b.real * b.real + b.imaginary * b.imaginary;
        if denominator.abs() < 1e-10 {
            panic!("{}", DIVIDE_BY_ZERO_ERROR);
        }
        Complex {
            real: (a.real * b.real + a.imaginary * b.imaginary) / denominator,
            imaginary: (a.imaginary * b.real - a.real * b.imaginary) / denominator,
        }
    }

    pub fn conjugate(a: Complex) -> Complex {
        Complex {
            real: a.real,
            imaginary: -a.imaginary,
        }
    }

    pub fn modulus(a: Complex) -> f64 {
        (a.real * a.real + a.imaginary * a.imaginary).sqrt()
    }
}
