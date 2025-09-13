/**
 * Author:  Raye Lattice  
 * Repo:    guhs
 * Created: 08/15/2025
 */

pub mod lines {
    const ERR_COINCIDENT_POINTS: &str = "两点不能重合";
    const ERR_VERTICAL_LINE: &str = "垂直直线斜率不存在";
    const ERR_INVALID_LINE_PARAMS: &str = "直线参数A和B不能同时为0";

    pub fn slope(x1: f64, y1: f64, x2: f64, y2: f64) -> Result<f64, &'static str> {
        if x1 == x2 && y1 == y2 {
            return Err(ERR_COINCIDENT_POINTS);
        }
        if x1 == x2 {
            return Err(ERR_VERTICAL_LINE);
        }
        Ok((y2 - y1) / (x2 - x1))
    }

    pub fn parametric_equation(x0: f64, y0: f64, a: f64, b: f64, t: f64) -> (f64, f64) {
        (x0 + a * t, y0 + b * t)
    }

    pub fn point_to_line_distance(x0: f64, y0: f64, a: f64, b: f64, c: f64) -> Result<f64, &'static str> {
        if a == 0.0 && b == 0.0 {
            return Err(ERR_INVALID_LINE_PARAMS);
        }
        let numerator = (a * x0 + b * y0 + c).abs();
        let denominator = (a * a + b * b).sqrt();
        Ok(numerator / denominator)
    }
}
