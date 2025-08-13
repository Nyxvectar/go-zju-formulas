/**
 * Author:  Nyxvectar Yan
 * Repo:    go-zju-formulas
 * Created: 07/23/2025
 */

package algebra

import (
	"errors"
	"math"
)

var (
	errInvalidBase  = "对数的底须大于0且不为1"
	errInvalidX     = "对数的真数须大于0"
	errZeroPrevious = "基期值不得为零"
)

// CheckLogValidity 检查对数的底和真数是否有效
func CheckLogValidity(base, x float64) (bool, error) {
	if base <= 0 || base == 1 {
		return false, errors.New(errInvalidBase)
	}
	if x <= 0 {
		return false, errors.New(errInvalidX)
	}
	return true, nil
}

// Log 计算以base为底x的对数
func Log(base, x float64) (float64, error) {
	if ok, err := CheckLogValidity(base, x); !ok {
		return 0, err
	}
	return math.Log(x) / math.Log(base), nil
}

// AverageGrowthRate 计算平均增长率（(现值-基期值)/基期值）
func AverageGrowthRate(present, previous float64) (float64, error) {
	if previous == 0 {
		return 0, errors.New(errZeroPrevious)
	}
	return (present - previous) / previous, nil
}
