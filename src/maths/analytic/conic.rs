/**
 * Author:  Raye Lattice  
 * Repo:    guhs
 * Created: 08/15/2025
 */

pub mod conic_sections {
    const ERR_INVALID_ELLIPSE_PARAMS: &str = "椭圆参数需满足a > b > 0";
    const ERR_INVALID_HYPERBOLA_PARAMS: &str = "双曲线参数需满足a > 0, b > 0";
    const ERR_INVALID_PARABOLA_PARAM: &str = "抛物线参数p需大于0";
    const ERR_INVALID_ECCENTRICITY: &str = "离心率需满足椭圆0 < e < 1, 双曲线e > 1, 抛物线e = 1";
    const ERR_DIVISION_BY_ZERO: &str = "除数不能为0";

    pub fn eccentricity_ellipse(a: f64, b: f64) -> Result<f64, &'static str> {
        if a <= b || b <= 0.0 {
            return Err(ERR_INVALID_ELLIPSE_PARAMS);
        }
        let c = (a * a - b * b).sqrt();
        Ok(c / a)
    }

    pub fn eccentricity_hyperbola(a: f64, b: f64) -> Result<f64, &'static str> {
        if a <= 0.0 || b <= 0.0 {
            return Err(ERR_INVALID_HYPERBOLA_PARAMS);
        }
        let c = (a * a + b * b).sqrt();
        Ok(c / a)
    }

    pub fn eccentricity_parabola() -> Result<f64, &'static str> {
        Ok(1.0)
    }

    pub fn parametric_ellipse(a: f64, b: f64, t: f64) -> Result<(f64, f64), &'static str> {
        if a <= b || b <= 0.0 {
            return Err(ERR_INVALID_ELLIPSE_PARAMS);
        }
        Ok((a * t.cos(), b * t.sin()))
    }

    pub fn parametric_hyperbola(a: f64, b: f64, t: f64) -> Result<(f64, f64), &'static str> {
        if a <= 0.0 || b <= 0.0 {
            return Err(ERR_INVALID_HYPERBOLA_PARAMS);
        }
        Ok((a / t.cos(), b * t.tan()))
    }

    pub fn parametric_parabola(p: f64, t: f64) -> Result<(f64, f64), &'static str> {
        if p <= 0.0 {
            return Err(ERR_INVALID_PARABOLA_PARAM);
        }
        Ok((p * t * t, 2.0 * p * t))
    }

    pub fn focal_radius_ellipse(a: f64, e: f64, theta: f64) -> Result<f64, &'static str> {
        if e <= 0.0 || e >= 1.0 {
            return Err(ERR_INVALID_ECCENTRICITY);
        }
        if a <= 0.0 {
            return Err(ERR_INVALID_ELLIPSE_PARAMS);
        }
        Ok((a * (1.0 - e * e)) / (1.0 - e * theta.cos()))
    }

    pub fn focal_radius_hyperbola(a: f64, e: f64, theta: f64) -> Result<f64, &'static str> {
        if e <= 1.0 {
            return Err(ERR_INVALID_ECCENTRICITY);
        }
        if a <= 0.0 {
            return Err(ERR_INVALID_HYPERBOLA_PARAMS);
        }
        Ok((a * (e * e - 1.0)) / (1.0 - e * theta.cos()).abs())
    }

    pub fn point_difference_method_ellipse(x0: f64, y0: f64, a: f64, b: f64) -> Result<f64, &'static str> {
        if a <= b || b <= 0.0 {
            return Err(ERR_INVALID_ELLIPSE_PARAMS);
        }
        if x0 == 0.0 {
            return Err(ERR_DIVISION_BY_ZERO);
        }
        Ok(-(b * b * x0) / (a * a * y0))
    }

    pub fn tangent_chord_ellipse(x0: f64, y0: f64, a: f64, b: f64) -> Result<(f64, f64, f64), &'static str> {
        if a <= b || b <= 0.0 {
            return Err(ERR_INVALID_ELLIPSE_PARAMS);
        }
        Ok((x0 / (a * a), y0 / (b * b), 1.0))
    }

    pub fn tangent_chord_hyperbola(x0: f64, y0: f64, a: f64, b: f64) -> Result<(f64, f64, f64), &'static str> {
        if a <= 0.0 || b <= 0.0 {
            return Err(ERR_INVALID_HYPERBOLA_PARAMS);
        }
        Ok((x0 / (a * a), -y0 / (b * b), 1.0))
    }

    pub fn tangent_chord_parabola(x0: f64, y0: f64, p: f64) -> Result<(f64, f64, f64), &'static str> {
        if p <= 0.0 {
            return Err(ERR_INVALID_PARABOLA_PARAM);
        }
        Ok((1.0, -y0, p * x0))
    }
}
