/**
 * Author:  Nyxvectar Yan
 * Repo:    guths
 * Created: 07/31/2025
 */

package method

import (
	"fmt"
	"reflect"
)

type A struct {
	nyx string
	age uint8
}

var nyxvectar = A{
	nyx: "Nyxvectar Yan",
	age: 15,
}

func GutsMethod() {
	{
		fmt.Println(nyxvectar.name())  // 常用的是这种简化写法
		fmt.Println(A.name(nyxvectar)) // Go编译器会转化为这种
		// 要解释后者的这种显示写法, 可以理解为调用 A 这个类型
		// 自带的 name 方法，并且让这个方法去处理 nyxvectar 这
		// 个具体的 A 类型实例. 我们看到, 方法是函数的一种变体
		var t1 = reflect.TypeOf(NameOfA)
		var t2 = reflect.TypeOf(A.name)
		fmt.Println(t1 == t2) // 观察发现是true,证明了上述结论
		// 同时我们知道, 接受者就是隐含的第一个传入参数.
		var pa = &nyxvectar
		fmt.Println(pa.getName())
		// 实际上Go的编译器会自动解引用而无需开发者手动解引用
		// 上面这一种写法会被自动编译为 *pa.getName(), 是一种
		// 发生在编译期间的语法糖, 也是因为这个, 显式的字符串
		// 等等都是不可以被这样自动化处理的. 事实上, 地址操作
		// 符也无法取得字符串字面量, 数字字面量以及布尔值字面
		// 量的地址, 诸如 &42 和 &"Nyxvectar Yan"  这样的语句
		// 将导致 Go 报 invalid indirect 错误, 无法通过编译.
	}
	// 由于前文我们提到过 Go 语言的方法实质上就是函数一种变体
	// 所以函数所适用的闭包特性对于方法来说也是通用的, 观察:
	{
		var c1 = crimsonet()
		var c2 = crimsonet()
		fmt.Println(c1())
		fmt.Println(c2())
		fmt.Println(c1())
	}
}

func (a A) name() string {
	a.nyx = "Hi! " + a.nyx
	return a.nyx
}

func (a *A) getName() string {
	a.nyx = "Hi! " + a.nyx
	return a.nyx
}

// 注意, Go语言文档并不推荐对于同一个结构同时实现
// 值的方法和指针的方法, 原因是指针的方法会改变底
// 层的值而值的方法则会改变副本的值, 由于可能每次
// 都创建一个新的副本, 那么这样的混合使用并不利于
// 复杂情形的处理, 容易造成错误且难以排查, 弱警告
//      Struct A has methods on  both  value  and
//      pointer receivers .  Such  usage  is  not
//      recommended by the Go Documentation.

func NameOfA(a A) string {
	a.nyx = "Hi! " + a.nyx
	return a.nyx
}

// Go语言只能给当前包当中所定义的类型来定义方法
// 例如 int 是系统内置的类型而不是我自定义,这里
// GoLand就报错:
//      Invalid receiver type 'int'
//      ('int' is a non-local type)

func (a A) grow() uint8 {
	a.age++
	return a.age
}

func crimsonet() func() uint8 {
	var k = nyxvectar
	return func() uint8 {
		// 每一次调用闭包, 都会让 k 调用grow并更新自身
		// 由于grow是值接收者,需要重新赋值才能保存修改
		k.age = k.grow()
		return k.age
	}
}
