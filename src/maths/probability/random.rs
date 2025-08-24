/**
 * Author:  Nyxvectar Yan 
 * Repo:    guhs
 * Created: 08/17/2025
 */

pub mod random_variables {
    const ERR_INVALID_PROBABILITY: &str = "概率值必须在[0, 1]范围内";
    const ERR_EMPTY_DISTRIBUTION: &str = "分布数据不能为空";
    const ERR_PROBABILITY_SUM: &str = "概率总和必须为1";
    const ERR_NEGATIVE_VARIANCE: &str = "方差不能为负数";

    pub fn expected_value(values: &[f64], probabilities: &[f64]) -> Result<f64, &'static str> {
        if values.is_empty() || probabilities.is_empty() {
            return Err(ERR_EMPTY_DISTRIBUTION);
        }
        if values.len() != probabilities.len() {
            return Err("值与概率数量必须相等");
        }
        let mut prob_sum = 0.0;
        for &p in probabilities {
            if p < 0.0 || p > 1.0 {
                return Err(ERR_INVALID_PROBABILITY);
            }
            prob_sum += p;
        }
        if (prob_sum - 1.0).abs() > 1e-9 {
            return Err(ERR_PROBABILITY_SUM);
        }
        Ok(values.iter().zip(probabilities.iter()).map(|(&v, &p)| v * p).sum())
    }

    pub fn variance(values: &[f64], probabilities: &[f64]) -> Result<f64, &'static str> {
        let mean = expected_value(values, probabilities)?;
        let mut variance = 0.0;
        for (&v, &p) in values.iter().zip(probabilities.iter()) {
            variance += (v - mean).powi(2) * p;
        }
        if variance < -1e-9 {
            return Err(ERR_NEGATIVE_VARIANCE);
        }
        Ok(variance.max(0.0))
    }

    pub fn mean_constant(c: f64) -> f64 {
        c
    }

    pub fn mean_scalar_multiple(a: f64, mean_x: f64) -> f64 {
        a * mean_x
    }

    pub fn mean_sum(mean_x: f64, mean_y: f64) -> f64 {
        mean_x + mean_y
    }

    pub fn mean_linear_combination(coefficients: &[f64], means: &[f64], constant: f64) -> Result<f64, &'static str> {
        if coefficients.len() != means.len() {
            return Err("系数与均值数量必须相等");
        }
        let sum: f64 = coefficients.iter().zip(means.iter()).map(|(&c, &m)| c * m).sum();
        Ok(sum + constant)
    }

    pub fn variance_constant(_c: f64) -> f64 {
        0.0
    }

    pub fn variance_scalar_multiple(a: f64, var_x: f64) -> Result<f64, &'static str> {
        if var_x < 0.0 {
            return Err(ERR_NEGATIVE_VARIANCE);
        }
        Ok(a.powi(2) * var_x)
    }

    pub fn variance_sum_independent(var_x: f64, var_y: f64) -> Result<f64, &'static str> {
        if var_x < 0.0 || var_y < 0.0 {
            return Err(ERR_NEGATIVE_VARIANCE);
        }
        Ok(var_x + var_y)
    }

    pub fn variance_linear_combination(coefficients: &[f64], variances: &[f64]) -> Result<f64, &'static str> {
        if coefficients.len() != variances.len() {
            return Err("系数与方差数量必须相等");
        }
        for &v in variances {
            if v < 0.0 {
                return Err(ERR_NEGATIVE_VARIANCE);
            }
        }
        Ok(coefficients.iter().zip(variances.iter()).map(|(&c, &v)| c.powi(2) * v).sum())
    }
}
