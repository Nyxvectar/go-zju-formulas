/**
 * Author:  Raye Lattice  
 * Repo:    guhs
 * Created: 08/17/2025
 */

pub mod geometry {
    use std::error::Error;
    use std::fmt;

    const EPSILON: f64 = 1e-10;

    #[derive(Debug, Clone, Copy)]
    pub struct Vec3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Plane {
        pub a: f64,
        pub b: f64,
        pub c: f64,
        pub d: f64,
    }

    #[derive(Debug, PartialEq)]
    pub enum GeometryError {
        ZeroVector,
        NotPerpendicular,
        NotCoplanar,
        NotParallel,
        InvalidParam,
    }

    impl fmt::Display for GeometryError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                GeometryError::ZeroVector => write!(f, "零向量没有方向"),
                GeometryError::NotPerpendicular => write!(f, "两向量不垂直"),
                GeometryError::NotCoplanar => write!(f, "传入的两点不共面"),
                GeometryError::NotParallel => write!(f, "两向量不平行"),
                GeometryError::InvalidParam => write!(f, "给定的参数无效"),
            }
        }
    }

    impl Error for GeometryError {}

    impl Vec3 {
        pub fn new(x: f64, y: f64, z: f64) -> Self {
            Vec3 { x, y, z }
        }

        pub fn add(self, other: Vec3) -> Vec3 {
            Vec3 {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }

        pub fn subtract(self, other: Vec3) -> Vec3 {
            Vec3 {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }

        pub fn scale(self, scalar: f64) -> Vec3 {
            Vec3 {
                x: self.x * scalar,
                y: self.y * scalar,
                z: self.z * scalar,
            }
        }

        pub fn dot(self, other: Vec3) -> f64 {
            self.x * other.x + self.y * other.y + self.z * other.z
        }

        pub fn cross(self, other: Vec3) -> Vec3 {
            Vec3 {
                x: self.y * other.z - self.z * other.y,
                y: self.z * other.x - self.x * other.z,
                z: self.x * other.y - self.y * other.x,
            }
        }

        pub fn magnitude(self) -> f64 {
            (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
        }

        pub fn normalize(self) -> Result<Vec3, GeometryError> {
            let mag = self.magnitude();
            if mag < EPSILON {
                return Err(GeometryError::ZeroVector);
            }
            Ok(Vec3 {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            })
        }

        pub fn is_collinear(self, other: Vec3) -> Result<bool, GeometryError> {
            let v_norm = self.normalize()?;
            let u_norm = other.normalize()?;
            let dot = v_norm.dot(u_norm);
            Ok((dot.abs() - 1.0).abs() < EPSILON)
        }
    }

    pub fn is_line_parallel_to_plane(line: Vec3, plane_norm: Vec3) -> Result<bool, GeometryError> {
        if line.magnitude() < EPSILON || plane_norm.magnitude() < EPSILON {
            return Err(GeometryError::ZeroVector);
        }
        Ok(line.dot(plane_norm).abs() < EPSILON)
    }

    pub fn new_plane(pa: Vec3, pb: Vec3, pc: Vec3) -> Result<Plane, GeometryError> {
        let v_ab = pb.subtract(pa);
        let v_ac = pc.subtract(pa);
        let normal = v_ab.cross(v_ac);

        if normal.magnitude() < EPSILON {
            return Err(GeometryError::NotCoplanar);
        }

        let a = normal.x;
        let b = normal.y;
        let c = normal.z;
        let d = -(a * pa.x + b * pa.y + c * pa.z);

        Ok(Plane { a, b, c, d })
    }

    impl Plane {
        pub fn normal(self) -> Vec3 {
            Vec3 {
                x: self.a,
                y: self.b,
                z: self.c,
            }
        }
    }

    pub fn are_planes_parallel(p1: Plane, p2: Plane) -> Result<bool, GeometryError> {
        let n1 = p1.normal();
        let n2 = p2.normal();

        if n1.magnitude() < EPSILON || n2.magnitude() < EPSILON {
            return Err(GeometryError::ZeroVector);
        }

        let cross = n1.cross(n2);
        Ok(cross.magnitude() < EPSILON)
    }

    pub fn are_lines_perpendicular_to_same_plane(
        line_dir1: Vec3,
        line_dir2: Vec3,
        plane: Plane,
    ) -> Result<bool, GeometryError> {
        let perp1 = is_line_perpendicular_to_plane(line_dir1, plane)?;
        let perp2 = is_line_perpendicular_to_plane(line_dir2, plane)?;

        if !perp1 || !perp2 {
            return Ok(false);
        }

        let cross = line_dir1.cross(line_dir2);
        Ok(cross.magnitude() < EPSILON)
    }

    pub fn get_plane_intersection_dirs(
        p1: Plane,
        p2: Plane,
        intersecting_plane: Plane,
    ) -> Result<(Vec3, Vec3), GeometryError> {
        if !are_planes_parallel(p1, p2)? {
            return Err(GeometryError::NotParallel);
        }

        let n1 = p1.normal();
        let n2 = intersecting_plane.normal();
        let line_dir1 = n1.cross(n2);

        let n3 = p2.normal();
        let line_dir2 = n3.cross(n2);

        Ok((line_dir1, line_dir2))
    }

    pub fn get_line_plane_intersection_dir(
        line_dir: Vec3,
        plane: Plane,
    ) -> Result<Vec3, GeometryError> {
        let normal = plane.normal();

        if line_dir.magnitude() < EPSILON || normal.magnitude() < EPSILON {
            return Err(GeometryError::ZeroVector);
        }

        if !is_line_parallel_to_plane(line_dir, normal)? {
            return Err(GeometryError::NotParallel);
        }

        Ok(line_dir.cross(normal))
    }

    pub fn are_plane_perpendicular(p1: Plane, p2: Plane) -> Result<bool, GeometryError> {
        let n1 = p1.normal();
        let n2 = p2.normal();
        if n1.magnitude() < EPSILON || n2.magnitude() < EPSILON {
            return Err(GeometryError::ZeroVector);
        }
        Ok(n1.dot(n2).abs() < EPSILON)
    }

    pub fn is_line_perpendicular_to_plane(line_dir: Vec3, plane: Plane) -> Result<bool, GeometryError> {
        if line_dir.magnitude() < EPSILON {
            return Err(GeometryError::ZeroVector);
        }
        let normal = plane.normal();
        if normal.magnitude() < EPSILON {
            return Err(GeometryError::ZeroVector);
        }
        let cross = line_dir.cross(normal);
        Ok(cross.magnitude() < EPSILON)
    }

    pub fn is_line_perpendicular_to_plane_by_inters(line_dir: Vec3, p1: Plane, p2: Plane) -> Result<bool, GeometryError> {
        let are_perpendicular = are_plane_perpendicular(p1, p2)?;
        if !are_perpendicular {
            return Err(GeometryError::NotPerpendicular);
        }
        let n1 = p1.normal();
        let n2 = p2.normal();
        let intersection_dir = n1.cross(n2);
        if line_dir.dot(intersection_dir).abs() > EPSILON {
            return Ok(false);
        }
        is_line_perpendicular_to_plane(line_dir, p2)
    }

    pub fn project_onto_plane(v: Vec3, normal: Vec3) -> Result<Vec3, GeometryError> {
        let normalized = normal.normalize()?;
        let dot = v.dot(normalized);
        Ok(v.subtract(normalized.scale(dot)))
    }

    pub fn projected_area(original_area: f64, normal1: Vec3, normal2: Vec3) -> Result<f64, GeometryError> {
        if original_area < 0.0 {
            return Err(GeometryError::InvalidParam);
        }
        if normal1.magnitude() < EPSILON || normal2.magnitude() < EPSILON {
            return Err(GeometryError::ZeroVector);
        }
        let cos_theta = normal1.dot(normal2).abs() / (normal1.magnitude() * normal2.magnitude());
        Ok(original_area * cos_theta)
    }

    pub fn minimum_angle_between_line_and_plane(line_dir: Vec3, plane: Plane) -> Result<f64, GeometryError> {
        if line_dir.magnitude() < EPSILON {
            return Err(GeometryError::ZeroVector);
        }
        let normal = plane.normal();
        if normal.magnitude() < EPSILON {
            return Err(GeometryError::ZeroVector);
        }
        let dot = line_dir.dot(normal);
        let cos_theta = dot / (line_dir.magnitude() * normal.magnitude());
        let sin_alpha = cos_theta.abs();
        Ok(sin_alpha.asin())
    }

    pub fn maximum_angle_between_skew_lines(line_dir1: Vec3, line_dir2: Vec3) -> Result<f64, GeometryError> {
        if line_dir1.magnitude() < EPSILON || line_dir2.magnitude() < EPSILON {
            return Err(GeometryError::ZeroVector);
        }
        let dot = line_dir1.dot(line_dir2);
        let cos_theta = dot / (line_dir1.magnitude() * line_dir2.magnitude());
        Ok(cos_theta.abs().acos())
    }

    pub fn is_line_perpendicular_to_oblique(line_dir: Vec3, oblique_dir: Vec3, plane_normal: Vec3) -> Result<bool, GeometryError> {
        let proj = project_onto_plane(oblique_dir, plane_normal)?;
        if line_dir.dot(proj).abs() > EPSILON {
            return Ok(false);
        }
        Ok(line_dir.dot(oblique_dir).abs() < EPSILON)
    }

    pub fn three_cosine_theorem(angle_oab: f64, angle_bac: f64) -> Result<f64, GeometryError> {
        if angle_oab < 0.0 || angle_oab > std::f64::consts::PI / 2.0 ||
            angle_bac < 0.0 || angle_bac > std::f64::consts::PI / 2.0 {
            return Err(GeometryError::InvalidParam);
        }
        Ok(angle_oab.cos() * angle_bac.cos())
    }

    pub fn three_sine_theorem(angle_oac: f64, angle_aoc: f64) -> Result<f64, GeometryError> {
        if angle_oac < 0.0 || angle_oac > std::f64::consts::PI / 2.0 ||
            angle_aoc < 0.0 || angle_aoc > std::f64::consts::PI / 2.0 {
            return Err(GeometryError::InvalidParam);
        }
        Ok(angle_oac.sin() * angle_aoc.sin())
    }
}