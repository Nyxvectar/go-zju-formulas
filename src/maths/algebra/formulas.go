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
	errEmptySet        = "不得传入空集"
	errNonPositive     = "集合内需全部为正数"
	errCauchyCondition = "实参不满足柯西求最值条件"
)

// CubicDifference 计算立方差：a³ - b³ = (a-b)(a²+ab+b²)
func CubicDifference(a, b float64) float64 {
	return (a - b) * (a*a + a*b + b*b)
}

// SubsetCount 计算子集数量：n个元素的子集总数为2ⁿ
func SubsetCount(n uint64) uint64 {
	return uint64(math.Pow(2, float64(n)))
}

// MeanInequalities 计算均值不等式中的调和、几何、算术、平方平均数
func MeanInequalities(u []float64) (float64, float64, float64, float64, error) {
	if len(u) == 0 {
		return 0, 0, 0, 0, errors.New(errEmptySet)
	}
	for _, num := range u {
		if num <= 0 {
			return 0, 0, 0, 0, errors.New(errNonPositive)
		}
	}
	n := float64(len(u))
	var harmonicSum, geoProduct, squareSum, arithSum float64
	geoProduct = 1
	for _, num := range u {
		harmonicSum += 1 / num
		geoProduct *= num
		squareSum += num * num
		arithSum += num
	}
	H := n / harmonicSum           // 调和平均数
	G := math.Pow(geoProduct, 1/n) // 几何平均数
	A := arithSum / n              // 算术平均数
	Q := math.Sqrt(squareSum / n)  // 平方平均数
	return H, G, A, Q, nil
}

// CauchyEquality 检查柯西不等式等号条件并计算对应值
func CauchyEquality(a, b, c, d float64) (float64, error) {
	if a*d != b*c {
		return 0, errors.New(errCauchyCondition)
	}
	return math.Pow(a*c+b*d, 2), nil
}
