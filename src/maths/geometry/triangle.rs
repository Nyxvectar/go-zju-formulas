/**
 * Author:  Nyxvectar Yan 
 * Repo:    guhs
 * Created: 08/17/2025
 */

pub mod geometry {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug, Clone, Copy)]
    pub struct Vector2D {
        pub x: f64,
        pub y: f64,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Triangle {
        pub a: Vector2D,
        pub b: Vector2D,
        pub c: Vector2D,
    }

    #[derive(Debug, PartialEq)]
    pub enum TriangleError {
        LengthNegative,
        ResultNegative,
        AngleNegative,
        AngleOutRange,
        AngleFault,
        CalibrationFail,
        CalculateFail,
    }

    impl fmt::Display for TriangleError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                TriangleError::LengthNegative => write!(f, "三角形不存在 [length<0]"),
                TriangleError::ResultNegative => write!(f, "三角形不存在 [square<0]"),
                TriangleError::AngleNegative => write!(f, "三角形不存在 [angle<0]"),
                TriangleError::AngleOutRange => write!(f, "三角形不存在 [angleOutRange]"),
                TriangleError::AngleFault => write!(f, "三角形不存在 [angleTotal!=Pi]"),
                TriangleError::CalibrationFail => write!(f, "计算结果不一 [present!=previous]"),
                TriangleError::CalculateFail => write!(f, "无法计算     {!}"),
            }
        }
    }

    impl Error for TriangleError {}

    pub fn law_of_sines(a: f64, b: f64, c: f64, a_angle: f64, b_angle: f64, c_angle: f64) -> Result<f64, TriangleError> {
        if a <= 0.0 || b <= 0.0 || c <= 0.0 {
            return Err(TriangleError::LengthNegative);
        }
        if a_angle <= 0.0 || b_angle <= 0.0 || c_angle <= 0.0 {
            return Err(TriangleError::AngleNegative);
        }
        if (a_angle + b_angle + c_angle - std::f64::consts::PI).abs() > 1e-9 {
            return Err(TriangleError::AngleFault);
        }
        let r1 = a / a_angle.sin();
        let r2 = b / b_angle.sin();
        let r3 = c / c_angle.sin();
        if (r1 - r2).abs() > 1e-9 || (r1 - r3).abs() > 1e-9 {
            return Err(TriangleError::CalibrationFail);
        }
        Ok(r1 / 2.0)
    }

    pub fn law_of_cosines(a: f64, b: f64, c_angle: f64) -> Result<f64, TriangleError> {
        if a <= 0.0 || b <= 0.0 {
            return Err(TriangleError::LengthNegative);
        }
        if c_angle <= 0.0 || c_angle >= std::f64::consts::PI {
            return Err(TriangleError::AngleOutRange);
        }
        let c_squared = a * a + b * b - 2.0 * a * b * c_angle.cos();
        if c_squared < 0.0 {
            return Err(TriangleError::ResultNegative);
        }
        Ok(c_squared.sqrt())
    }

    pub fn projection_theorem(a: f64, b: f64, c: f64, b_angle: f64, c_angle: f64) -> Result<bool, TriangleError> {
        if a <= 0.0 || b <= 0.0 || c <= 0.0 {
            return Err(TriangleError::LengthNegative);
        }
        if b_angle <= 0.0 || c_angle <= 0.0 || b_angle + c_angle >= std::f64::consts::PI {
            return Err(TriangleError::AngleOutRange);
        }
        let left = a;
        let right = b * c_angle.cos() + c * b_angle.cos();
        Ok((left - right).abs() < 1e-9)
    }

    pub fn median_length(a: f64, b: f64, c: f64) -> Result<f64, TriangleError> {
        if a <= 0.0 || b <= 0.0 || c <= 0.0 {
            return Err(TriangleError::LengthNegative);
        }
        if a >= b + c || b >= a + c || c >= a + b {
            return Err(TriangleError::CalibrationFail);
        }
        Ok((2.0 * b * b + 2.0 * c * c - a * a).sqrt() / 2.0)
    }

    impl Triangle {
        pub fn centroid(&self) -> Vector2D {
            Vector2D {
                x: (self.a.x + self.b.x + self.c.x) / 3.0,
                y: (self.a.y + self.b.y + self.c.y) / 3.0,
            }
        }

        pub fn incenter(&self) -> Result<Vector2D, TriangleError> {
            let a = distance(self.b, self.c);
            let b = distance(self.a, self.c);
            let c = distance(self.a, self.b);
            if a <= 0.0 || b <= 0.0 || c <= 0.0 {
                return Err(TriangleError::LengthNegative);
            }
            let denominator = a + b + c;
            Ok(Vector2D {
                x: (a * self.a.x + b * self.b.x + c * self.c.x) / denominator,
                y: (a * self.a.y + b * self.b.y + c * self.c.y) / denominator,
            })
        }

        pub fn circumcenter(&self) -> Result<Vector2D, TriangleError> {
            let a = distance(self.b, self.c);
            let b = distance(self.a, self.c);
            let c = distance(self.a, self.b);
            let area = heron_formula(a, b, c)?;
            if area == 0.0 {
                return Err(TriangleError::CalculateFail);
            }
            let d = 2.0 * (self.a.x * (self.b.y - self.c.y) + self.b.x * (self.c.y - self.a.y) + self.c.x * (self.a.y - self.b.y));
            let x = ((self.a.x * self.a.x + self.a.y * self.a.y) * (self.b.y - self.c.y) +
                (self.b.x * self.b.x + self.b.y * self.b.y) * (self.c.y - self.a.y) +
                (self.c.x * self.c.x + self.c.y * self.c.y) * (self.a.y - self.b.y)) / d;
            let y = ((self.a.x * self.a.x + self.a.y * self.a.y) * (self.c.x - self.b.x) +
                (self.b.x * self.b.x + self.b.y * self.b.y) * (self.a.x - self.c.x) +
                (self.c.x * self.c.x + self.c.y * self.c.y) * (self.b.x - self.a.x)) / d;
            Ok(Vector2D { x, y })
        }

        pub fn orthocenter(&self) -> Result<Vector2D, TriangleError> {
            if is_vertical_line(self.a, self.b) {
                return Ok(Vector2D { x: self.a.x, y: self.c.y });
            }
            if is_vertical_line(self.b, self.c) {
                return Ok(Vector2D { x: self.b.x, y: self.a.y });
            }
            if is_vertical_line(self.a, self.c) {
                return Ok(Vector2D { x: self.c.x, y: self.b.y });
            }
            let slope_ab = (self.b.y - self.a.y) / (self.b.x - self.a.x);
            let slope_bc = (self.c.y - self.b.y) / (self.c.x - self.b.x);
            let x = (slope_ab * slope_bc * (self.a.y - self.c.y) + slope_bc * (self.b.x - self.a.x) - slope_ab * (self.c.x - self.b.x)) /
                (slope_bc - slope_ab);
            let y = slope_ab * (x - self.a.x) + self.a.y;
            Ok(Vector2D { x, y })
        }
    }

    pub fn heron_formula(a: f64, b: f64, c: f64) -> Result<f64, TriangleError> {
        if a <= 0.0 || b <= 0.0 || c <= 0.0 {
            return Err(TriangleError::LengthNegative);
        }
        if a >= b + c || b >= a + c || c >= a + b {
            return Err(TriangleError::CalibrationFail);
        }
        let s = (a + b + c) / 2.0;
        let area_squared = s * (s - a) * (s - b) * (s - c);
        if area_squared < 0.0 {
            return Err(TriangleError::LengthNegative);
        }
        Ok(area_squared.sqrt())
    }

    fn distance(p1: Vector2D, p2: Vector2D) -> f64 {
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        (dx * dx + dy * dy).sqrt()
    }

    fn is_vertical_line(p1: Vector2D, p2: Vector2D) -> bool {
        (p2.x - p1.x).abs() < 1e-9
    }
}
