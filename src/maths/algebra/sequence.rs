/**
 * Author:  Nyxvectar Yan 
 * Repo:    guhs
 * Created: 08/15/2025
 */

pub mod sequences {
    const ERR_NEGATIVE_N: &str = "项数n不能为负数";
    const ERR_ZERO_COMMON_RATIO: &str = "等比数列公比不能为0";
    const ERR_INVALID_INDEX: &str = "索引必须大于等于0";
    const ERR_DIVISION_BY_ZERO: &str = "除数不能为0";

    pub fn arithmetic_term(a1: f64, d: f64, n: i32) -> Result<f64, &'static str> {
        if n < 1 {
            return Err(ERR_NEGATIVE_N);
        }
        Ok(a1 + (n - 1) as f64 * d)
    }

    pub fn arithmetic_sum(a1: f64, d: f64, n: i32) -> Result<f64, &'static str> {
        if n < 1 {
            return Err(ERR_NEGATIVE_N);
        }
        let an = arithmetic_term(a1, d, n)?;
        Ok((n as f64) * (a1 + an) / 2.0)
    }

    pub fn geometric_term(a1: f64, r: f64, n: i32) -> Result<f64, &'static str> {
        if n < 1 {
            return Err(ERR_NEGATIVE_N);
        }
        if r == 0.0 {
            return Err(ERR_ZERO_COMMON_RATIO);
        }
        Ok(a1 * r.powi(n - 1))
    }

    pub fn geometric_sum(a1: f64, r: f64, n: i32) -> Result<f64, &'static str> {
        if n < 1 {
            return Err(ERR_NEGATIVE_N);
        }
        if r == 0.0 {
            return Err(ERR_ZERO_COMMON_RATIO);
        }
        if r == 1.0 {
            return Ok(a1 * n as f64);
        }
        Ok(a1 * (1.0 - r.powi(n)) / (1.0 - r))
    }

    pub fn recurrence_term(initial: &[f64], coeffs: &[f64], n: usize) -> Result<f64, &'static str> {
        let k = initial.len();
        if coeffs.len() != k {
            return Err("系数数量必须与初始项数量一致");
        }
        if n < k {
            if n >= initial.len() {
                return Err(ERR_INVALID_INDEX);
            }
            return Ok(initial[n]);
        }

        let mut terms = initial.to_vec();
        for i in k..=n {
            let mut sum = 0.0;
            for (j, &coeff) in coeffs.iter().enumerate() {
                sum += coeff * terms[i - 1 - j];
            }
            terms.push(sum);
        }
        Ok(terms[n])
    }
}
