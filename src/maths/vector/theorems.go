/**
 * Author:  Nyxvectar Yan
 * Repo:    go-zju-formulas
 * Created: 07/23/2025
 */

package vector

import "math"

type Vector struct {
	X float64
	Y float64
	Z float64
}

// Magnitude 计算向量模长
func (v Vector) Magnitude() float64 {
	return math.Sqrt(v.X*v.X + v.Y*v.Y + v.Z*v.Z)
}

// AreCollinear 判断两向量是否共线
func AreCollinear(a, b Vector) bool {
	if a.isZero() || b.isZero() {
		return true
	}
	return almostEqual(a.X*b.Y, a.Y*b.X) &&
		almostEqual(a.X*b.Z, a.Z*b.X) &&
		almostEqual(a.Y*b.Z, a.Z*b.Y)
}

// DotProduct 计算向量点积
func DotProduct(a, b Vector) float64 {
	return a.X*b.X + a.Y*b.Y + a.Z*b.Z
}

// CosAngle 计算两向量夹角余弦值
func CosAngle(a, b Vector) float64 {
	if a.isZero() || b.isZero() {
		return 0
	}
	return DotProduct(a, b) / (a.Magnitude() * b.Magnitude())
}

// CrossProduct 计算向量叉积
func CrossProduct(a, b Vector) Vector {
	return Vector{
		a.Y*b.Z - a.Z*b.Y,
		a.Z*b.X - a.X*b.Z,
		a.X*b.Y - a.Y*b.X,
	}
}

// isZero 判断向量是否为零向量
func (v Vector) isZero() bool {
	const epsilon = 1e-10
	return math.Abs(v.X) < epsilon &&
		math.Abs(v.Y) < epsilon &&
		math.Abs(v.Z) < epsilon
}

// almostEqual 判断两个浮点数是否近似相等
func almostEqual(a, b float64) bool {
	const epsilon = 1e-10
	return math.Abs(a-b) < epsilon ||
		math.Abs(a-b) < epsilon*math.Max(math.Abs(a), math.Abs(b))
}
