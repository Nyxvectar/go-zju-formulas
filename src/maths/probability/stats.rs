/**
 * Author:  Nyxvectar Yan 
 * Repo:    guhs
 * Created: 08/17/2025
 */

pub mod probability {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug, PartialEq)]
    pub enum ProbabilityError {
        DntPositive,
        DntExist,
    }

    impl fmt::Display for ProbabilityError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ProbabilityError::DntPositive => write!(f, "实参须为正数"),
                ProbabilityError::DntExist => write!(f, "切片须为非空集合"),
            }
        }
    }

    impl Error for ProbabilityError {}

    fn checker(a: f64, b: f64) -> bool {
        a >= 0.0 && b >= 0.0
    }

    pub fn percentile(p: f64, data: &[f64]) -> Result<f64, ProbabilityError> {
        if checker(p, 0.0) {
            let mut sorted_data = data.to_vec();
            sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let p_normalized = p / 100.0;
            let i = (sorted_data.len() as f64) * p_normalized;
            if (i - i.floor()) < 1e-10 {
                let idx = i as usize;
                Ok((sorted_data[idx] + sorted_data[idx + 1]) / 2.0)
            } else {
                let idx = i as usize;
                Ok(sorted_data[idx + 1])
            }
        } else {
            Err(ProbabilityError::DntPositive)
        }
    }

    pub fn sample_mean(sample: &[f64]) -> Result<f64, ProbabilityError> {
        if sample.is_empty() {
            return Err(ProbabilityError::DntExist);
        }
        let sum: f64 = sample.iter().sum();
        Ok(sum / sample.len() as f64)
    }

    pub fn sample_variance(sample: &[f64]) -> Result<f64, ProbabilityError> {
        if sample.is_empty() {
            return Err(ProbabilityError::DntExist);
        }
        let (square_sum, sum): (f64, f64) = sample.iter().fold((0.0, 0.0), |(s_sq, s), &num| {
            (s_sq + num * num, s + num)
        });
        let avg = sum / sample.len() as f64;
        let square_avg = square_sum / sample.len() as f64;
        Ok(square_avg - avg * avg)
    }
}