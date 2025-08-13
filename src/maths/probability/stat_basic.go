/**
 * Author:  Nyxvectar Yan
 * Repo:    go-zju-formulas
 * Created: 07/23/2025
 */

package probability

import (
	"errors"
	"math"
	"sort"
)

var (
	dntPositive = "实参须为正数"
	dntExist    = "切片须为非空集合"
)

func checker(a, b float64) bool {
	if a >= 0 && b >= 0 {
		return true
	} else {
		return false
	}
}

func Percentile(p float64, data []float64) (float64, error) {
	if checker(p, 0) {
		sort.Float64s(data)
		p /= 100
		var i = float64(len(data)) * p
		if i-math.Floor(i) < 1e-10 {
			return (data[int(i)] + data[int(i)+1]) / 2, nil
		} else {
			return data[int(i)+1], nil
		}
	} else {
		return 0, errors.New(dntPositive)
	}
}

func SampleMean(sample []float64) (float64, error) {
	var sum float64
	if len(sample) == 0 {
		return 0, errors.New(dntExist)
	} else {
		for _, num := range sample {
			sum += num
		}
		return sum / float64(len(sample)), nil
	}
}

func SampleVariance(sample []float64) (float64, error) {
	var squareSum float64
	var sum float64
	if len(sample) == 0 {
		return 0, errors.New(dntExist)
	} else {
		for _, num := range sample {
			squareSum += num * num
			sum += num
		}
		var avgSquare = sum / float64(len(sample))
		var squareAvg = squareSum / float64(len(sample))
		return squareAvg - avgSquare, nil
	}
}
