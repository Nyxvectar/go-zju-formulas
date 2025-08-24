/**
 * Author:  Nyxvectar Yan 
 * Repo:    guhs
 * Created: 08/15/2025
 */

pub mod combinatorics {
    const ERR_NEGATIVE_VALUE: &str = "数值不能为负数";
    const ERR_INVALID_INPUT: &str = "n必须大于等于k";
    const ERR_EMPTY_OPTIONS: &str = "选项列表不能为空";

    fn factorial(n: u64) -> u64 {
        (1..=n).product()
    }

    pub fn permutation(n: u64, k: u64) -> Result<u64, &'static str> {
        if n < k {
            return Err(ERR_INVALID_INPUT);
        }
        Ok(factorial(n) / factorial(n - k))
    }

    pub fn combination(n: u64, k: u64) -> Result<u64, &'static str> {
        if n < k {
            return Err(ERR_INVALID_INPUT);
        }
        let k = k.min(n - k);
        let numerator: u64 = (n - k + 1..=n).product();
        Ok(numerator / factorial(k))
    }

    pub fn combination_identity(n: u64, k: u64) -> Result<u64, &'static str> {
        if k == 0 || k == n {
            return Ok(1);
        }
        if k > n {
            return Err(ERR_INVALID_INPUT);
        }
        combination(n - 1, k - 1).and_then(|a| {
            combination(n - 1, k).map(|b| a + b)
        })
    }

    pub fn derangement(n: u64) -> Result<u64, &'static str> {
        if n == 0 {
            return Ok(1);
        }
        if n == 1 {
            return Ok(0);
        }
        let n_minus_1 = n - 1;
        derangement(n_minus_1).and_then(|d| {
            derangement(n_minus_1 - 1).map(|d_prev| (n_minus_1) * (d + d_prev))
        })
    }

    pub fn binomial_theorem(n: u64, k: u64) -> Result<u64, &'static str> {
        combination(n, k)
    }

    pub fn classification_addition_principle(options: &[u64]) -> Result<u64, &'static str> {
        if options.is_empty() {
            return Err(ERR_EMPTY_OPTIONS);
        }
        Ok(options.iter().sum())
    }

    pub fn step_multiplication_principle(steps: &[u64]) -> Result<u64, &'static str> {
        if steps.is_empty() {
            return Err(ERR_EMPTY_OPTIONS);
        }
        Ok(steps.iter().product())
    }
}
