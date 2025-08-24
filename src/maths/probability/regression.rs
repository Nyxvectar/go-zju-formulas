/**
 * Author:  Nyxvectar Yan
 * Repo:    guhs
 * Created: 08/17/2025
 */

pub mod statistics {
    const ERR_INSUFFICIENT_DATA: &str = "数据点数量必须至少为2";
    const ERR_MISMATCHED_DATA: &str = "x和y数据点数量必须相等";
    const ERR_ZERO_VARIANCE: &str = "x值不能全相同";
    const ERR_INVALID_OBSERVED: &str = "观测值不能为负数";
    const ERR_INVALID_EXPECTED: &str = "期望值必须为正数";

    pub fn least_squares(x: &[f64], y: &[f64]) -> Result<(f64, f64), &'static str> {
        if x.len() < 2 || y.len() < 2 {
            return Err(ERR_INSUFFICIENT_DATA);
        }
        if x.len() != y.len() {
            return Err(ERR_MISMATCHED_DATA);
        }

        let n = x.len() as f64;
        let sum_x: f64 = x.iter().sum();
        let sum_y: f64 = y.iter().sum();
        let sum_xy: f64 = x.iter().zip(y.iter()).map(|(&a, &b)| a * b).sum();
        let sum_x2: f64 = x.iter().map(|&a| a * a).sum();

        let denominator = n * sum_x2 - sum_x * sum_x;
        if denominator.abs() < 1e-9 {
            return Err(ERR_ZERO_VARIANCE);
        }

        let slope = (n * sum_xy - sum_x * sum_y) / denominator;
        let intercept = (sum_y - slope * sum_x) / n;

        Ok((slope, intercept))
    }

    pub fn empirical_regression(x: f64, slope: f64, intercept: f64) -> f64 {
        slope * x + intercept
    }

    pub fn chi_squared(observed: &[f64], expected: &[f64]) -> Result<f64, &'static str> {
        if observed.len() != expected.len() {
            return Err(ERR_MISMATCHED_DATA);
        }
        if observed.is_empty() {
            return Err(ERR_INSUFFICIENT_DATA);
        }

        for &o in observed {
            if o < 0.0 {
                return Err(ERR_INVALID_OBSERVED);
            }
        }
        for &e in expected {
            if e <= 0.0 {
                return Err(ERR_INVALID_EXPECTED);
            }
        }

        let mut chi2 = 0.0;
        for (&o, &e) in observed.iter().zip(expected.iter()) {
            chi2 += (o - e).powi(2) / e;
        }

        Ok(chi2)
    }
}
