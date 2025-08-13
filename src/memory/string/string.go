/**
 * Author:  Nyxvectar Yan
 * Repo:    go-zju-formulas
 * Created: 07/29/2025
 */

package string

import "fmt"

func GutsString() {
	// 为了判定字符，给每个字符设置一个编码。
	// 出于读取的需要，派定一个不同区间适用的格式，一般标头。
	// 于是得到诸如UTF-8编码，这也是Go语言的默认编码格式。
	var stringDemo = "Hello, Golang!"
	// 在Go语言当中的字符串默认是不会被修改的，
	// 上述这种方法所得到的string将分配进入只读内存段，
	// 这也是为什么我们不可以直接 sD[] = ...修改其值的缘由。
	stringDemo = "Hello, GNU!"
	// 这样并不会修改原来的内存，而是赋予了新的内存段
	stringExam := []byte(stringDemo)
	stringExam[2] = 'a'
	fmt.Printf("原始字符串: %s\n", stringDemo)
	fmt.Printf("修改后的字节切片: %s\n", stringExam)
	// 注意，将字符串转换为字节切片会复制数据
	// 所以修改stringExam不会影响原始的stringDemo
}
