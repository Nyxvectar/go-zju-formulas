/**
 * Author:  Raye Lattice 
 * Repo:    guhs
 * Created: 08/17/2025
 */

pub mod geometry {
    use std::f64::consts::PI;
    use std::error::Error;
    use std::fmt;

    #[derive(Debug, PartialEq)]
    pub enum GeometryError {
        InvalidDimensions,
        EulerViolation,
    }

    impl fmt::Display for GeometryError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                GeometryError::InvalidDimensions => write!(f, "为负数的无效参数"),
                GeometryError::EulerViolation => write!(f, "不满足欧拉公式"),
            }
        }
    }

    impl Error for GeometryError {}

    fn is_valid_dimensions(dims: &[f64]) -> bool {
        dims.iter().all(|&dim| dim >= 0.0)
    }

    pub fn cylinder_surface_area(r: f64, h: f64) -> Result<f64, GeometryError> {
        if !is_valid_dimensions(&[r, h]) {
            return Err(GeometryError::InvalidDimensions);
        }
        Ok(2.0 * PI * r * (r + h))
    }

    pub fn frustum_volume(s1: f64, s2: f64, h: f64) -> Result<f64, GeometryError> {
        if !is_valid_dimensions(&[s1, s2, h]) {
            return Err(GeometryError::InvalidDimensions);
        }
        Ok((s1 + s2 + (s1 * s2).sqrt()) * h / 3.0)
    }

    pub fn sphere_surface_area(r: f64) -> Result<f64, GeometryError> {
        if !is_valid_dimensions(&[r]) {
            return Err(GeometryError::InvalidDimensions);
        }
        Ok(4.0 * PI * r.powi(2))
    }

    pub fn sphere_volume(r: f64) -> Result<f64, GeometryError> {
        if !is_valid_dimensions(&[r]) {
            return Err(GeometryError::InvalidDimensions);
        }
        Ok(4.0 * PI * r.powi(3) / 3.0)
    }

    pub fn euler_characteristic(v: u64, e: u64, f: u64) -> Result<i64, GeometryError> {
        let result = v as i64 - e as i64 + f as i64;

        if v == 0 && e == 0 && f == 0 {
            return Err(GeometryError::InvalidDimensions);
        }

        if v > 0 && e > 0 && f > 0 && result != 2 {
            return Err(GeometryError::EulerViolation);
        }

        Ok(result)
    }
}