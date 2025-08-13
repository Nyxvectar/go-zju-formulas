/**
 * Author:  Nyxvectar Yan
 * Repo:    go-zju-formulas
 * Created: 07/23/2025
 */

package geometry

import (
	"errors"
	"math"
)

const epsilon = 1e-10 // 浮点数比较阈值

// Vec3 表示三维向量（可表示点或方向向量）
type Vec3 struct {
	x float64
	y float64
	z float64
}

// Plane 平面方程为 ax + by + cz + d = 0
type Plane struct {
	a float64
	b float64
	c float64
	d float64
}

var (
	ErrZeroVector       = errors.New("零向量没有方向")
	ErrNotPerpendicular = errors.New("两向量不垂直")
	ErrNotCoplanar      = errors.New("传入的两点不共面")
	ErrNotParallel      = errors.New("两向量不平行")
	ErrInvalidParam     = errors.New("给定的参数无效")
)

// NewVec3 创建三维向量
func NewVec3(x, y, z float64) Vec3 {
	return Vec3{x, y, z}
}

// Add 向量加法
func (v Vec3) Add(u Vec3) Vec3 {
	return Vec3{v.x + u.x, v.y + u.y, v.z + u.z}
}

// Subtract 向量减法
func (v Vec3) Subtract(u Vec3) Vec3 {
	return Vec3{v.x - u.x, v.y - u.y, v.z - u.z}
}

// Scale 向量标量乘法
func (v Vec3) Scale(scalar float64) Vec3 {
	return Vec3{
		v.x * scalar,
		v.y * scalar,
		v.z * scalar,
	}
}

// Dot 点积
func (v Vec3) Dot(u Vec3) float64 {
	return v.x*u.x + v.y*u.y + v.z*u.z
}

// Cross 叉积
func (v Vec3) Cross(u Vec3) Vec3 {
	return Vec3{
		v.y*u.z - v.z*u.y,
		v.z*u.x - v.x*u.z,
		v.x*u.y - v.y*u.x,
	}
}

// Magnitude 向量模长
func (v Vec3) Magnitude() float64 {
	return math.Sqrt(v.x*v.x + v.y*v.y + v.z*v.z)
}

// Normalize 归一化（单位向量）
func (v Vec3) Normalize() (Vec3, error) {
	mag := v.Magnitude()
	if mag < epsilon {
		return Vec3{}, ErrZeroVector
	}
	return Vec3{
		v.x / mag,
		v.y / mag,
		v.z / mag,
	}, nil
}

// IsCollinear 判断两向量是否共线（同向或反向）
func (v Vec3) IsCollinear(u Vec3) (bool, error) {
	vNorm, errV := v.Normalize()
	if errV != nil {
		return false, errV
	}
	uNorm, errU := u.Normalize()
	if errU != nil {
		return false, errU
	}
	dot := vNorm.Dot(uNorm)
	return math.Abs(math.Abs(dot)-1) < epsilon, nil
}

// IsLineParallelToPlane 判断直线是否平行于平面
func IsLineParallelToPlane(line, planeNorm Vec3) (bool, error) {
	if line.Magnitude() < epsilon || planeNorm.Magnitude() < epsilon {
		return false, ErrZeroVector
	}
	return math.Abs(line.Dot(planeNorm)) < epsilon, nil
}

// NewPlane 通过三点创建平面（三点不共面）
func NewPlane(pa, pb, pc Vec3) (Plane, error) {
	vAB := pb.Subtract(pa)
	vAC := pc.Subtract(pa)
	normal := vAB.Cross(vAC)
	if normal.Magnitude() < epsilon {
		return Plane{}, ErrNotCoplanar
	}
	a, b, c := normal.x, normal.y, normal.z
	d := -(a*pa.x + b*pa.y + c*pa.z)
	return Plane{a, b, c, d}, nil
}

// Normal 返回平面法向量
func (p Plane) Normal() Vec3 {
	return Vec3{p.a, p.b, p.c}
}

// ArePlanesParallel 判断两平面是否平行
func ArePlanesParallel(p1, p2 Plane) (bool, error) {
	n1 := p1.Normal()
	n2 := p2.Normal()

	if n1.Magnitude() < epsilon || n2.Magnitude() < epsilon {
		return false, ErrZeroVector
	}
	cross := n1.Cross(n2)
	return cross.Magnitude() < epsilon, nil
}

// AreLinesPerpendicularToSamePlane 判断两直线是否垂直于同一平面且互相平行
func AreLinesPerpendicularToSamePlane(lineDir1, lineDir2 Vec3, plane Plane) (bool, error) {
	perp1, err := IsLinePerpendicularToPlane(lineDir1, plane)
	if err != nil {
		return false, err
	}
	perp2, err := IsLinePerpendicularToPlane(lineDir2, plane)
	if err != nil {
		return false, err
	}
	if !perp1 || !perp2 {
		return false, nil
	}
	cross := lineDir1.Cross(lineDir2)
	return cross.Magnitude() < epsilon, nil
}

// GetPlaneIntersectionDirs 求两平行平面与第三平面的交线方向
func GetPlaneIntersectionDirs(p1, p2, intersectingPlane Plane) (Vec3, Vec3, error) {
	areParallel, err := ArePlanesParallel(p1, p2)
	if err != nil {
		return Vec3{}, Vec3{}, err
	}
	if !areParallel {
		return Vec3{}, Vec3{}, ErrNotParallel
	}
	n1 := p1.Normal()
	n2 := intersectingPlane.Normal()
	lineDir1 := n1.Cross(n2)
	n3 := p2.Normal()
	lineDir2 := n3.Cross(n2)
	return lineDir1, lineDir2, nil
}

// GetLinePlaneIntersectionDir 求直线与平面的交线方向
func GetLinePlaneIntersectionDir(lineDir Vec3, plane Plane) (Vec3, error) {
	normal := plane.Normal()
	if lineDir.Magnitude() < epsilon || normal.Magnitude() < epsilon {
		return Vec3{}, ErrZeroVector
	}
	isParallel, err := IsLineParallelToPlane(lineDir, normal)
	if err != nil {
		return Vec3{}, err
	}
	if !isParallel {
		return Vec3{}, ErrNotParallel
	}
	return lineDir.Cross(normal), nil
}

// ArePlanePerpendicular 判断两平面是否垂直
func ArePlanePerpendicular(p1, p2 Plane) (bool, error) {
	n1 := p1.Normal()
	n2 := p2.Normal()
	if n1.Magnitude() < epsilon || n2.Magnitude() < epsilon {
		return false, ErrZeroVector
	}
	return math.Abs(n1.Dot(n2)) < epsilon, nil
}

// IsLinePerpendicularToPlane 判断直线是否垂直于平面
func IsLinePerpendicularToPlane(lineDir Vec3, plane Plane) (bool, error) {
	if lineDir.Magnitude() < epsilon {
		return false, ErrZeroVector
	}
	normal := plane.Normal()
	if normal.Magnitude() < epsilon {
		return false, ErrZeroVector
	}
	cross := lineDir.Cross(normal)
	return cross.Magnitude() < epsilon, nil
}

// IsLinePerpendicularToPlaneByInters 通过平面交线判断直线是否垂直于平面
func IsLinePerpendicularToPlaneByInters(lineDir Vec3, p1, p2 Plane) (bool, error) {
	arePerpendicular, err := ArePlanePerpendicular(p1, p2)
	if err != nil {
		return false, err
	}
	if !arePerpendicular {
		return false, ErrNotPerpendicular
	}
	n1 := p1.Normal()
	n2 := p2.Normal()
	intersectionDir := n1.Cross(n2)
	if math.Abs(lineDir.Dot(intersectionDir)) > epsilon {
		return false, nil
	}
	return IsLinePerpendicularToPlane(lineDir, p2)
}

// ProjectOntoPlane 计算向量在平面上的投影（垂直于法向量的分量）
func ProjectOntoPlane(v, normal Vec3) (Vec3, error) {
	normalized, err := normal.Normalize()
	if err != nil {
		return Vec3{}, err
	}
	dot := v.Dot(normalized)
	return v.Subtract(normalized.Scale(dot)), nil
}

// ProjectedArea 计算面积在另一平面上的投影
func ProjectedArea(originalArea float64, normal1, normal2 Vec3) (float64, error) {
	if originalArea < 0 {
		return 0, ErrInvalidParam
	}
	if normal1.Magnitude() < epsilon || normal2.Magnitude() < epsilon {
		return 0, ErrZeroVector
	}
	cosTheta := math.Abs(normal1.Dot(normal2)) / (normal1.Magnitude() * normal2.Magnitude())
	return originalArea * cosTheta, nil
}

// MinimumAngleBetweenLineAndPlane 计算直线与平面的最小夹角
func MinimumAngleBetweenLineAndPlane(lineDir Vec3, plane Plane) (float64, error) {
	if lineDir.Magnitude() < epsilon {
		return 0, ErrZeroVector
	}
	normal := plane.Normal()
	if normal.Magnitude() < epsilon {
		return 0, ErrZeroVector
	}
	dot := lineDir.Dot(normal)
	cosTheta := dot / (lineDir.Magnitude() * normal.Magnitude())
	sinAlpha := math.Abs(cosTheta)
	alpha := math.Asin(sinAlpha)
	return alpha, nil
}

// MaximumAngleBetweenSkewLines 计算异面直线的最大夹角
func MaximumAngleBetweenSkewLines(lineDir1, lineDir2 Vec3) (float64, error) {
	if lineDir1.Magnitude() < epsilon || lineDir2.Magnitude() < epsilon {
		return 0, ErrZeroVector
	}
	dot := lineDir1.Dot(lineDir2)
	cosTheta := dot / (lineDir1.Magnitude() * lineDir2.Magnitude())
	theta := math.Acos(math.Abs(cosTheta))
	return theta, nil
}

// IsLinePerpendicularToOblique 判断直线是否垂直于斜线
func IsLinePerpendicularToOblique(lineDir, obliqueDir, planeNormal Vec3) (bool, error) {
	proj, err := ProjectOntoPlane(obliqueDir, planeNormal)
	if err != nil {
		return false, err
	}
	if math.Abs(lineDir.Dot(proj)) > epsilon {
		return false, nil
	}
	return math.Abs(lineDir.Dot(obliqueDir)) < epsilon, nil
}

// ThreeCosineTheorem 三余弦定理
func ThreeCosineTheorem(angleOAB, angleBAC float64) (float64, error) {
	if angleOAB < 0 || angleOAB > math.Pi/2 ||
		angleBAC < 0 || angleBAC > math.Pi/2 {
		return 0, ErrInvalidParam
	}
	return math.Cos(angleOAB) * math.Cos(angleBAC), nil
}

// ThreeSineTheorem 三正弦定理
func ThreeSineTheorem(angleOAC, angleAOC float64) (float64, error) {
	if angleOAC < 0 || angleOAC > math.Pi/2 ||
		angleAOC < 0 || angleAOC > math.Pi/2 {
		return 0, ErrInvalidParam
	}
	return math.Sin(angleOAC) * math.Sin(angleAOC), nil
}
