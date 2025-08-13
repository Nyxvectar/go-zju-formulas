/**
 * Author:  Nyxvectar Yan
 * Repo:    go-zju-formulas
 * Created: 07/23/2025
 */

package algebra

import "math"

type Complex struct {
	Real      float64
	Imaginary float64
}

var (
	errDivideByZero = "Panic: 试图除以零或者及其接近零的数"
)

// Add 复数加法
func Add(a, b Complex) Complex {
	return Complex{
		Real:      a.Real + b.Real,
		Imaginary: a.Imaginary + b.Imaginary,
	}
}

// Multiply 复数乘法
func Multiply(a, b Complex) Complex {
	return Complex{
		Real:      a.Real*b.Real - a.Imaginary*b.Imaginary,
		Imaginary: a.Real*b.Imaginary + a.Imaginary*b.Real,
	}
}

// Divide 复数除法
func Divide(a, b Complex) Complex {
	denominator := b.Real*b.Real + b.Imaginary*b.Imaginary
	if math.Abs(denominator) < 1e-10 {
		panic(errDivideByZero)
	}
	return Complex{
		Real:      (a.Real*b.Real + a.Imaginary*b.Imaginary) / denominator,
		Imaginary: (a.Imaginary*b.Real - a.Real*b.Imaginary) / denominator,
	}
}

// Conjugate 复数共轭
func Conjugate(a Complex) Complex {
	return Complex{
		Real:      a.Real,
		Imaginary: -a.Imaginary,
	}
}

// Modulus 复数模长
func Modulus(a Complex) float64 {
	return math.Sqrt(a.Real*a.Real + a.Imaginary*a.Imaginary)
}
