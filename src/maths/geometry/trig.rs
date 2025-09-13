/**
 * Author:  Raye Lattice  
 * Repo:    guhs
 * Created: 08/17/2025
 */

pub mod geometry {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug, PartialEq)]
    pub enum TrigError {
        OutDefinition,
        OutRange,
        OutRule,
    }

    impl fmt::Display for TrigError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                TrigError::OutDefinition => write!(f, "函数在此时没有定义"),
                TrigError::OutRange => write!(f, "正余弦值超出范围"),
                TrigError::OutRule => write!(f, "Omega不得为零"),
            }
        }
    }

    impl Error for TrigError {}

    fn is_in_range(value: f64, min: f64, max: f64) -> bool {
        value >= min && value <= max
    }

    pub fn sin(rad: f64) -> f64 {
        rad.sin()
    }

    pub fn cos(rad: f64) -> f64 {
        rad.cos()
    }

    pub fn tan(rad: f64) -> Result<f64, TrigError> {
        if cos(rad).abs() < 1e-10 {
            return Err(TrigError::OutDefinition);
        }
        Ok(rad.tan())
    }

    pub fn deg_to_rad(deg: f64) -> f64 {
        deg * std::f64::consts::PI / 180.0
    }

    pub fn rad_to_deg(rad: f64) -> f64 {
        rad * 180.0 / std::f64::consts::PI
    }

    pub fn sin_to_cos(sin: f64) -> Result<f64, TrigError> {
        if !is_in_range(sin, -1.0, 1.0) {
            return Err(TrigError::OutRange);
        }
        Ok((1.0 - sin * sin).sqrt())
    }

    pub fn cos_to_sin(cos: f64) -> Result<f64, TrigError> {
        if !is_in_range(cos, -1.0, 1.0) {
            return Err(TrigError::OutRange);
        }
        Ok((1.0 - cos * cos).sqrt())
    }

    pub fn sin_add(rad_a: f64, rad_b: f64) -> f64 {
        sin(rad_a) * cos(rad_b) + cos(rad_a) * sin(rad_b)
    }

    pub fn sin_sub(rad_a: f64, rad_b: f64) -> f64 {
        sin(rad_a) * cos(rad_b) - cos(rad_a) * sin(rad_b)
    }

    pub fn cos_add(rad_a: f64, rad_b: f64) -> f64 {
        cos(rad_a) * cos(rad_b) - sin(rad_a) * sin(rad_b)
    }

    pub fn cos_sub(rad_a: f64, rad_b: f64) -> f64 {
        cos(rad_a) * cos(rad_b) + sin(rad_a) * sin(rad_b)
    }

    pub fn sin_double(rad: f64) -> f64 {
        2.0 * sin(rad) * cos(rad)
    }

    pub fn cos_double(rad: f64) -> f64 {
        2.0 * cos(rad) * cos(rad) - 1.0
    }

    pub fn tan_double(rad: f64) -> Result<f64, TrigError> {
        let tan_val = tan(rad)?;
        if (1.0 - tan_val * tan_val).abs() < 1e-10 {
            return Err(TrigError::OutDefinition);
        }
        Ok((2.0 * tan_val) / (1.0 - tan_val * tan_val))
    }

    pub fn sin_sum_to_product(rad_a: f64, rad_b: f64) -> (f64, f64) {
        (2.0 * sin((rad_a + rad_b) / 2.0) * cos((rad_a - rad_b) / 2.0), 0.0)
    }

    pub fn sin_sub_to_product(rad_a: f64, rad_b: f64) -> (f64, f64) {
        (2.0 * cos((rad_a + rad_b) / 2.0) * sin((rad_a - rad_b) / 2.0), 0.0)
    }

    pub fn cos_sum_to_product(rad_a: f64, rad_b: f64) -> (f64, f64) {
        (2.0 * cos((rad_a + rad_b) / 2.0) * cos((rad_a - rad_b) / 2.0), 0.0)
    }

    pub fn cos_sub_to_product(rad_a: f64, rad_b: f64) -> (f64, f64) {
        (-2.0 * sin((rad_a + rad_b) / 2.0) * sin((rad_a - rad_b) / 2.0), 0.0)
    }

    pub fn sin_cos_to_sum(rad_a: f64, rad_b: f64) -> (f64, f64) {
        (0.5 * sin(rad_a + rad_b), 0.5 * sin(rad_a - rad_b))
    }

    pub fn sin_sin_to_sum(rad_a: f64, rad_b: f64) -> (f64, f64) {
        (0.5 * cos(rad_a - rad_b), -0.5 * cos(rad_a + rad_b))
    }

    pub fn cos_cos_to_sum(rad_a: f64, rad_b: f64) -> (f64, f64) {
        (0.5 * cos(rad_a - rad_b), 0.5 * cos(rad_a + rad_b))
    }

    pub fn sin_half(cos: f64) -> Result<f64, TrigError> {
        if !is_in_range(cos, -1.0, 1.0) {
            return Err(TrigError::OutRange);
        }
        Ok(((1.0 - cos) / 2.0).sqrt())
    }

    pub fn cos_half(cos: f64) -> Result<f64, TrigError> {
        if !is_in_range(cos, -1.0, 1.0) {
            return Err(TrigError::OutRange);
        }
        Ok(((1.0 + cos) / 2.0).sqrt())
    }

    pub fn tan_half(cos: f64) -> Result<f64, TrigError> {
        if !is_in_range(cos, -1.0, 1.0) {
            return Err(TrigError::OutRange);
        }
        if (1.0 + cos).abs() < 1e-10 {
            return Err(TrigError::OutDefinition);
        }
        Ok(((1.0 - cos) / (1.0 + cos)).sqrt())
    }

    pub fn sin_from_tan_half(tan: f64) -> f64 {
        2.0 * tan / (1.0 + tan * tan)
    }

    pub fn cos_from_tan_half(tan: f64) -> f64 {
        (1.0 - tan * tan) / (1.0 + tan * tan)
    }

    pub fn tan_from_tan_half(tan: f64) -> f64 {
        2.0 * tan / (1.0 - tan * tan)
    }

    pub fn auxiliary_angle(a: f64, b: f64) -> Result<(f64, f64), TrigError> {
        if a == 0.0 && b == 0.0 {
            return Err(TrigError::OutDefinition);
        }
        let a_val = (a * a + b * b).sqrt();
        let y = b.atan2(a);
        Ok((a_val, y))
    }

    pub fn inverse_auxiliary_angle(a: f64, y: f64) -> (f64, f64) {
        (a * cos(y), a * sin(y))
    }

    pub fn period(w: f64) -> Result<f64, TrigError> {
        if w == 0.0 {
            return Err(TrigError::OutRule);
        }
        Ok(2.0 * std::f64::consts::PI / w)
    }
}