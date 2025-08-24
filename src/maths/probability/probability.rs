/**
 * Author:  Nyxvectar Yan 
 * Repo:    guhs
 * Created: 08/17/2025
 */

pub mod probability {
    const ERR_INVALID_PROBABILITY: &str = "概率值必须在[0, 1]范围内";
    const ERR_EMPTY_SAMPLE_SPACE: &str = "样本空间不能为空";
    const ERR_EMPTY_EVENTS: &str = "事件列表不能为空";
    const ERR_ZERO_PROBABILITY: &str = "条件事件的概率不能为0";

    pub fn is_independent(p_a: f64, p_b: f64, p_ab: f64) -> Result<bool, &'static str> {
        if p_a < 0.0 || p_a > 1.0 || p_b < 0.0 || p_b > 1.0 || p_ab < 0.0 || p_ab > 1.0 {
            return Err(ERR_INVALID_PROBABILITY);
        }
        Ok((p_ab - p_a * p_b).abs() < 1e-9)
    }

    pub fn classical_probability(favorable: usize, total: usize) -> Result<f64, &'static str> {
        if total == 0 {
            return Err(ERR_EMPTY_SAMPLE_SPACE);
        }
        if favorable > total {
            return Err("有利样本数不能超过总样本数");
        }
        Ok(favorable as f64 / total as f64)
    }

    pub fn conditional_probability(p_ab: f64, p_b: f64) -> Result<f64, &'static str> {
        if p_ab < 0.0 || p_ab > 1.0 || p_b < 0.0 || p_b > 1.0 {
            return Err(ERR_INVALID_PROBABILITY);
        }
        if p_b < 1e-9 {
            return Err(ERR_ZERO_PROBABILITY);
        }
        Ok(p_ab / p_b)
    }

    pub fn multiplication_rule(p_a: f64, p_b_given_a: f64) -> Result<f64, &'static str> {
        if p_a < 0.0 || p_a > 1.0 || p_b_given_a < 0.0 || p_b_given_a > 1.0 {
            return Err(ERR_INVALID_PROBABILITY);
        }
        Ok(p_a * p_b_given_a)
    }

    pub fn total_probability(partition_probs: &[f64], conditional_probs: &[f64]) -> Result<f64, &'static str> {
        if partition_probs.is_empty() || conditional_probs.is_empty() {
            return Err(ERR_EMPTY_EVENTS);
        }
        if partition_probs.len() != conditional_probs.len() {
            return Err("划分概率与条件概率数量必须相等");
        }
        for &p in partition_probs {
            if p < 0.0 || p > 1.0 {
                return Err(ERR_INVALID_PROBABILITY);
            }
        }
        for &p in conditional_probs {
            if p < 0.0 || p > 1.0 {
                return Err(ERR_INVALID_PROBABILITY);
            }
        }
        let total: f64 = partition_probs.iter()
            .zip(conditional_probs.iter())
            .map(|(&p, &cp)| p * cp)
            .sum();
        Ok(total)
    }

    pub fn bayes_theorem(prior: f64, likelihood: f64, evidence: f64) -> Result<f64, &'static str> {
        if prior < 0.0 || prior > 1.0 || likelihood < 0.0 || likelihood > 1.0 || evidence < 0.0 || evidence > 1.0 {
            return Err(ERR_INVALID_PROBABILITY);
        }
        if evidence < 1e-9 {
            return Err(ERR_ZERO_PROBABILITY);
        }
        Ok((prior * likelihood) / evidence)
    }
}
