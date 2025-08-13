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
	errInvalidDimensions = "为负数的无效参数"
	errEulerViolation    = "不满足欧拉公式"
)

// IsValidDimensions 检查尺寸参数是否为非负数
func IsValidDimensions(dims ...float64) bool {
	for _, dim := range dims {
		if dim < 0 {
			return false
		}
	}
	return true
}

// CylinderSurfaceArea 计算圆柱体表面积
func CylinderSurfaceArea(r, h float64) (float64, error) {
	if !IsValidDimensions(r, h) {
		return 0, errors.New(errInvalidDimensions)
	}
	return 2 * math.Pi * r * (r + h), nil
}

// FrustumVolume 计算台体体积(通用公式，支持圆柱、圆锥)
func FrustumVolume(s1, s2, h float64) (float64, error) {
	if !IsValidDimensions(s1, s2, h) {
		return 0, errors.New(errInvalidDimensions)
	}
	return (s1 + s2 + math.Sqrt(s1*s2)) * h / 3, nil
}

// SphereSurfaceArea 计算球体表面积
func SphereSurfaceArea(r float64) (float64, error) {
	if !IsValidDimensions(r) {
		return 0, errors.New(errInvalidDimensions)
	}
	return 4 * math.Pi * math.Pow(r, 2), nil
}

// SphereVolume 计算球体体积
func SphereVolume(r float64) (float64, error) {
	if !IsValidDimensions(r) {
		return 0, errors.New(errInvalidDimensions)
	}
	return 4 * math.Pi * math.Pow(r, 3) / 3, nil
}

// EulerCharacteristic 验证或计算多面体的欧拉示性数
func EulerCharacteristic(v, e, f uint64) (uint64, error) {
	if v == 0 {
		if e < f || 2-f+e < 0 {
			return 0, errors.New(errInvalidDimensions)
		}
		return 2 - f + e, nil
	}
	if e == 0 {
		if v+f < 2 {
			return 0, errors.New(errInvalidDimensions)
		}
		return v + f - 2, nil
	}
	if f == 0 {
		if e < v-2 {
			return 0, errors.New(errInvalidDimensions)
		}
		return 2 + e - v, nil
	}
	if v-e+f != 2 {
		return 0, errors.New(errEulerViolation)
	}
	return 0, nil
}
