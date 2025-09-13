/**
 * Author:  Raye Lattice  
 * Repo:    guhs
 * Created: 08/15/2025
 */

pub mod derivatives {
    const ERR_DIVISION_BY_ZERO: &str = "除数不能为零";
    const ERR_NEGATIVE_BASE: &str = "对数的底数必须为正数且不等于1";
    const ERR_LOGARITHM_DOMAIN: &str = "对数的真数必须为正数";
    const ERR_SQUARE_ROOT_DOMAIN: &str = "开平方的数必须为非负数";

    pub fn constant_deriv(_c: f64) -> f64 {
        0.0
    }

    pub fn power_deriv(x: f64, n: f64) -> f64 {
        n * x.powf(n - 1.0)
    }

    pub fn exp_deriv(x: f64) -> f64 {
        x.exp()
    }

    pub fn ln_deriv(x: f64) -> Result<f64, &'static str> {
        if x <= 0.0 {
            return Err(ERR_LOGARITHM_DOMAIN);
        }
        Ok(1.0 / x)
    }

    pub fn log_deriv(x: f64, base: f64) -> Result<f64, &'static str> {
        if base <= 0.0 || base == 1.0 {
            return Err(ERR_NEGATIVE_BASE);
        }
        if x <= 0.0 {
            return Err(ERR_LOGARITHM_DOMAIN);
        }
        Ok(1.0 / (x * base.ln()))
    }

    pub fn sin_deriv(x: f64) -> f64 {
        x.cos()
    }

    pub fn cos_deriv(x: f64) -> f64 {
        -x.sin()
    }

    pub fn tan_deriv(x: f64) -> f64 {
        1.0 / x.cos().powi(2)
    }

    pub fn add_deriv(f_deriv: f64, g_deriv: f64) -> f64 {
        f_deriv + g_deriv
    }

    pub fn subtract_deriv(f_deriv: f64, g_deriv: f64) -> f64 {
        f_deriv - g_deriv
    }

    pub fn multiply_deriv(f: f64, f_deriv: f64, g: f64, g_deriv: f64) -> f64 {
        f_deriv * g + f * g_deriv
    }

    pub fn divide_deriv(f: f64, f_deriv: f64, g: f64, g_deriv: f64) -> Result<f64, &'static str> {
        if g.abs() < 1e-9 {
            return Err(ERR_DIVISION_BY_ZERO);
        }
        Ok((f_deriv * g - f * g_deriv) / (g * g))
    }

    pub fn composite_deriv(outer_deriv: f64, inner_deriv: f64) -> f64 {
        outer_deriv * inner_deriv
    }

    pub fn power_composite_deriv(inner: f64, inner_deriv: f64, n: f64) -> f64 {
        n * inner.powf(n - 1.0) * inner_deriv
    }

    pub fn exp_composite_deriv(inner: f64, inner_deriv: f64) -> f64 {
        inner.exp() * inner_deriv
    }

    pub fn sin_composite_deriv(inner: f64, inner_deriv: f64) -> f64 {
        inner.cos() * inner_deriv
    }

    pub fn cos_composite_deriv(inner: f64, inner_deriv: f64) -> f64 {
        -inner.sin() * inner_deriv
    }
}
