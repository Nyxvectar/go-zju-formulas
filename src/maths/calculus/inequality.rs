/**
 * Author:  Raye Lattice  
 * Repo:    guhs
 * Created: 08/15/2025
 */

pub mod analysis {
    const ERR_NON_POSITIVE_VALUE: &str = "值必须为正数";
    const ERR_EQUAL_VALUES: &str = "两个值不能相等";
    const ERR_EMPTY_POINTS: &str = "点集不能为空";
    const ERR_INVALID_LHOSPITAL: &str = "不满足洛必达法则适用条件";
    const ERR_NEGATIVE_ORDER: &str = "展开阶数不能为负数";
    const ERR_DERIVATIVE_ZERO: &str = "导数不能为零";

    pub fn logarithmic_mean_inequality(a: f64, b: f64) -> Result<(f64, f64, f64), &'static str> {
        if a <= 0.0 || b <= 0.0 {
            return Err(ERR_NON_POSITIVE_VALUE);
        }
        if a == b {
            return Err(ERR_EQUAL_VALUES);
        }
        let mean = (a - b) / (a.ln() - b.ln());
        let geo_mean = (a * b).sqrt();
        let arith_mean = (a + b) / 2.0;
        Ok((geo_mean, mean, arith_mean))
    }

    pub fn jensen_inequality<F>(f: F, is_convex: bool, points: &[f64]) -> Result<(f64, f64), &'static str>
    where
        F: Fn(f64) -> f64,
    {
        if points.is_empty() {
            return Err(ERR_EMPTY_POINTS);
        }
        let n = points.len() as f64;
        let mean = points.iter().sum::<f64>() / n;
        let f_mean = f(mean);
        let mean_f = points.iter().map(|&x| f(x)).sum::<f64>() / n;
        Ok((f_mean, mean_f))
    }

    pub fn lhospital_rule<F, G, DF, DG>(f: F, g: G, df: DF, dg: DG, x0: f64) -> Result<f64, &'static str>
    where
        F: Fn(f64) -> f64,
        G: Fn(f64) -> f64,
        DF: Fn(f64) -> f64,
        DG: Fn(f64) -> f64,
    {
        let fx0 = f(x0);
        let gx0 = g(x0);
        let is_0_0 = (fx0.abs() < 1e-9) && (gx0.abs() < 1e-9);
        let is_inf_inf = (fx0.is_infinite() && gx0.is_infinite()) && (fx0.signum() == gx0.signum());

        if !is_0_0 && !is_inf_inf {
            return Err(ERR_INVALID_LHOSPITAL);
        }

        let dfx0 = df(x0);
        let dgx0 = dg(x0);
        if dgx0.abs() < 1e-9 {
            return Err(ERR_DERIVATIVE_ZERO);
        }

        Ok(dfx0 / dgx0)
    }

    pub fn taylor_series<F>(f: F, derivatives: &[F], x0: f64, x: f64, n: usize) -> Result<f64, &'static str>
    where
        F: Fn(f64) -> f64,
    {
        if n == 0 {
            return Ok(f(x0));
        }
        if n > derivatives.len() {
            return Err("导数数量不足");
        }
        if n < 0 {
            return Err(ERR_NEGATIVE_ORDER);
        }

        let mut result = f(x0);
        let dx = x - x0;
        for k in 1..=n {
            let fact = (1..=k).product::<usize>() as f64;
            let deriv = derivatives[k - 1](x0);
            result += deriv * dx.powi(k as i32) / fact;
        }
        Ok(result)
    }
}
