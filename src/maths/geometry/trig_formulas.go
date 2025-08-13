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

var (
	outDefinition = "函数在此时没有定义"
	outRange      = "正余弦值超出范围"
	outRule       = "Omega不得为零"
)

// isInRange 检查值是否在[min, max]范围内
func isInRange(value, min, max float64) bool {
	return value >= min && value <= max
}

// Sin 计算正弦值（弧度）
func Sin(rad float64) float64 {
	return math.Sin(rad)
}

// Cos 计算余弦值（弧度）
func Cos(rad float64) float64 {
	return math.Cos(rad)
}

// Tan 计算正切值（弧度）
func Tan(rad float64) (float64, error) {
	if math.Abs(math.Cos(rad)) < 1e-10 {
		return 0, errors.New(outDefinition)
	}
	return math.Tan(rad), nil
}

// DegToRad 角度转弧度
func DegToRad(deg float64) float64 {
	return deg * math.Pi / 180
}

// RadToDeg 弧度转角度
func RadToDeg(rad float64) float64 {
	return rad * 180 / math.Pi
}

// SinToCos 由正弦值求余弦值（非负）
func SinToCos(sin float64) (float64, error) {
	if !isInRange(sin, -1, 1) {
		return 0, errors.New(outRange)
	}
	return math.Sqrt(1 - sin*sin), nil
}

// CosToSin 由余弦值求正弦值（非负）
func CosToSin(cos float64) (float64, error) {
	if !isInRange(cos, -1, 1) {
		return 0, errors.New(outRange)
	}
	return math.Sqrt(1 - cos*cos), nil
}

// SinAdd 和角正弦公式：sin(A+B)
func SinAdd(radA, radB float64) float64 {
	return Sin(radA)*Cos(radB) + Cos(radA)*Sin(radB)
}

// SinSub 差角正弦公式：sin(A-B)
func SinSub(radA, radB float64) float64 {
	return Sin(radA)*Cos(radB) - Cos(radA)*Sin(radB)
}

// CosAdd 和角余弦公式：cos(A+B)
func CosAdd(radA, radB float64) float64 {
	return Cos(radA)*Cos(radB) - Sin(radA)*Sin(radB)
}

// CosSub 差角余弦公式：cos(A-B)
func CosSub(radA, radB float64) float64 {
	return Cos(radA)*Cos(radB) + Sin(radA)*Sin(radB)
}

// SinDouble 二倍角正弦公式：sin(2θ)
func SinDouble(rad float64) float64 {
	return 2 * Sin(rad) * Cos(rad)
}

// CosDouble 二倍角余弦公式：cos(2θ)
func CosDouble(rad float64) float64 {
	return 2*Cos(rad)*Cos(rad) - 1
}

// TanDouble 二倍角正切公式：tan(2θ)
func TanDouble(rad float64) (float64, error) {
	tan, err := Tan(rad)
	if err != nil {
		return 0, err
	}
	if math.Abs(1-tan*tan) < 1e-10 {
		return 0, errors.New(outDefinition)
	}
	return (2 * tan) / (1 - tan*tan), nil
}

// SinSumToProduct 正弦和化积：sinA+sinB
func SinSumToProduct(radA, radB float64) (float64, float64) {
	return 2 * Sin((radA+radB)/2) * Cos((radA-radB)/2), 0
}

// SinSubToProduct 正弦差化积：sinA-sinB
func SinSubToProduct(radA, radB float64) (float64, float64) {
	return 2 * Cos((radA+radB)/2) * Sin((radA-radB)/2), 0
}

// CosSumToProduct 余弦和化积：cosA+cosB
func CosSumToProduct(radA, radB float64) (float64, float64) {
	return 2 * Cos((radA+radB)/2) * Cos((radA-radB)/2), 0
}

// CosSubToProduct 余弦差化积：cosA-cosB
func CosSubToProduct(radA, radB float64) (float64, float64) {
	return -2 * Sin((radA+radB)/2) * Sin((radA-radB)/2), 0
}

// SinCosToSum 正弦余弦积化和：sinAcosB
func SinCosToSum(radA, radB float64) (float64, float64) {
	return 0.5 * Sin(radA+radB), 0.5 * Sin(radA-radB)
}

// SinSinToSum 正弦正弦积化和：sinAsinB
func SinSinToSum(radA, radB float64) (float64, float64) {
	return 0.5 * Cos(radA-radB), -0.5 * Cos(radA+radB)
}

// CosCosToSum 余弦余弦积化和：cosAcosB
func CosCosToSum(radA, radB float64) (float64, float64) {
	return 0.5 * Cos(radA-radB), 0.5 * Cos(radA+radB)
}

// SinHalf 半角正弦公式：sin(θ/2)
func SinHalf(cos float64) (float64, error) {
	if !isInRange(cos, -1, 1) {
		return 0, errors.New(outRange)
	}
	return math.Sqrt((1 - cos) / 2), nil
}

// CosHalf 半角余弦公式：cos(θ/2)
func CosHalf(cos float64) (float64, error) {
	if !isInRange(cos, -1, 1) {
		return 0, errors.New(outRange)
	}
	return math.Sqrt((1 + cos) / 2), nil
}

// TanHalf 半角正切公式：tan(θ/2)
func TanHalf(cos float64) (float64, error) {
	if !isInRange(cos, -1, 1) {
		return 0, errors.New(outRange)
	}
	if math.Abs(1+cos) < 1e-10 {
		return 0, errors.New(outDefinition)
	}
	return math.Sqrt((1 - cos) / (1 + cos)), nil
}

// SinFromTanHalf 由半角正切求正弦：sinθ from tan(θ/2)
func SinFromTanHalf(tan float64) float64 {
	return 2 * tan / (1 + tan*tan)
}

// CosFromTanHalf 由半角正切求余弦：cosθ from tan(θ/2)
func CosFromTanHalf(tan float64) float64 {
	return (1 - tan*tan) / (1 + tan*tan)
}

// TanFromTanHalf 由半角正切求正切：tanθ from tan(θ/2)
func TanFromTanHalf(tan float64) float64 {
	return 2 * tan / (1 - tan*tan)
}

// AuxiliaryAngle 辅助角公式：a sinθ + b cosθ = A sin(θ+φ)
func AuxiliaryAngle(a, b float64) (float64, float64, error) {
	if a == 0 && b == 0 {
		return 0, 0, errors.New(outDefinition)
	}
	A := math.Sqrt(a*a + b*b)
	y := math.Atan2(b, a)
	return A, y, nil
}

// InverseAuxiliaryAngle 辅助角逆运算：A sin(θ+φ) 拆分为a sinθ + b cosθ
func InverseAuxiliaryAngle(A, y float64) (float64, float64) {
	return A * math.Cos(y), A * math.Sin(y)
}

// Period 计算周期：2π/ω
func Period(w float64) (float64, error) {
	if w == 0 {
		return 0, errors.New(outRule)
	}
	return 2 * math.Pi / w, nil
}
