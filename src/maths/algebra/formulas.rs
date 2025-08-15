/**
 * Author:  Nyxvectar Yan 
 * Repo:    guhs
 * Created: 08/15/2025
 */

pub mod algebra {
    const ERR_EMPTY_SET: &str = "不得传入空集";
    const ERR_NON_POSITIVE: &str = "集合内需全部为正数";
    const ERR_CAUCHY_CONDITION: &str = "实参不满足柯西求最值条件";

    pub fn cubic_difference(a: f64, b: f64) -> f64 {
        (a - b) * (a * a + a * b + b * b)
    }

    pub fn subset_count(n: u64) -> u64 {
        (2.0_f64).powi(n as i32) as u64
    }

    pub fn mean_inequalities(u: &[f64]) -> Result<(f64, f64, f64, f64), &'static str> {
        if u.is_empty() {
            return Err(ERR_EMPTY_SET);
        }
        for &num in u {
            if num <= 0.0 {
                return Err(ERR_NON_POSITIVE);
            }
        }
        let n = u.len() as f64;
        let mut harmonic_sum = 0.0;
        let mut geo_product = 1.0;
        let mut square_sum = 0.0;
        let mut arith_sum = 0.0;
        for &num in u {
            harmonic_sum += 1.0 / num;
            geo_product *= num;
            square_sum += num * num;
            arith_sum += num;
        }
        let h = n / harmonic_sum;
        let g = geo_product.powf(1.0 / n);
        let a = arith_sum / n;
        let q = (square_sum / n).sqrt();
        Ok((h, g, a, q))
    }

    pub fn cauchy_equality(a: f64, b: f64, c: f64, d: f64) -> Result<f64, &'static str> {
        if a * d != b * c {
            return Err(ERR_CAUCHY_CONDITION);
        }
        Ok((a * c + b * d).powi(2))
    }
}
