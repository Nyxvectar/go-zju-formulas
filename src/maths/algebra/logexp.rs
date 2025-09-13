/**
 * Author:  Raye Lattice  
 * Repo:    guhs
 * Created: 08/15/2025
 */

pub mod algebra {
    const ERR_INVALID_BASE: &str = "对数的底须大于0且不为1";
    const ERR_INVALID_X: &str = "对数的真数须大于0";
    const ERR_ZERO_PREVIOUS: &str = "基期值不得为零";

    pub fn check_log_validity(base: f64, x: f64) -> Result<bool, &'static str> {
        if base <= 0.0 || base == 1.0 {
            return Err(ERR_INVALID_BASE);
        }
        if x <= 0.0 {
            return Err(ERR_INVALID_X);
        }
        Ok(true)
    }

    pub fn log(base: f64, x: f64) -> Result<f64, &'static str> {
        check_log_validity(base, x)?;
        Ok(f64::ln(x) / f64::ln(base))
    }

    pub fn average_growth_rate(present: f64, previous: f64) -> Result<f64, &'static str> {
        if previous == 0.0 {
            return Err(ERR_ZERO_PREVIOUS);
        }
        Ok((present - previous) / previous)
    }
}
