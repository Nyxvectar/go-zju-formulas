/**
 * Author:  Nyxvectar Yan
 * Repo:    guths
 * Created: 07/31/2025
 */

package closure

import "fmt"

// TODO: 不同的变量情况,Go编译器在内存层面都做了些什么操作?

func GutsClosure() {
	{
		// Go语言当中的函数是头等对象, 她可以作为参数传递. (**其一**)
		tests()
		var B = A(tests)
		// 还可以绑定到变量. (**其三**)
		B()
		// 这样的函数参数,变量,返回值被称为是function value.
		// 其传递的实际上并不是函数本身,而是一个指向了函数入
		// 口地址的指针, 存储则是使用funcval(function value)
		// 结构体 (为了实现闭包). 闭包则适用于动态地创建和调
		// 用函数, 在外部的整体函数结束以后,闭包所捕获的变量
		// 不会被清理从内存中清理掉,所以闭包函数仍然可以正常
		// 的被使用. 注意, 其中的变量是会被保持修改状态的.如
		counter := makeCounter()
		fmt.Println(counter()) // 第一次数：1
		fmt.Println(counter()) // 第二次数：2
		// 从结构上看, 闭包的函数通常有一个外层函数和一个内
		// 层函数构成, 内层函数会用到外层函数里所定义的变量
		// 而这些变量并不在内层函数自己的作用域里, 并且, 内
		// 部的函数会被返回, 同时展现出记忆住变量状态的特征
		// 最核心的特征上讲, 即使外层函数已经执行完了,它里
		// 面的变量也不会消失, 因为被返回的内层函数抓着不放
		// 所以这些变量还能继续用.
	}
	// 一般变量是放在栈当中的, 但是闭包长期使用, 于是被迫向
	// 堆转移, 这个现象就是变量逃逸. 而传入的参数也会因为长
	// 期使用的需要而被迫转移到堆当中, 一般称之为参数堆分配
	// 另外需要注意的一点是, 每个闭包所带的变量都是独立的,
	// 他们将分开记录, 也就是说, 如果外部的函数被多次调用,
	// 每次都会产生一个这样独立的内部函数, 从而产生不同的结
	// 果,而且不同闭包之间的变量常常是互不干扰的.
	// 通过 -gcflags="-m" 参数可以实现对应的观测.
	{
		var inter1 = creater()
		var inter2 = creater()
		fmt.Println("inter1", inter1())
		fmt.Println("inter2", inter2())
		fmt.Println("inter1", inter1())
	}
}

func makeCounter() func() int {
	count := 0
	// 这个被内层函数使用的count variable不会在makeCounter生命
	// 周期结束以后消失, 而是会一直存在直到内层不再被调用,这个
	// count变量一般称之为捕获变量.
	return func() int {
		count++
		return count
	}
}

func tests() {
	var sentence = "tests函数是一个符合接受要求的函数"
	println(sentence)
}

func A(a func()) func() {
	println("A接受了一个函数参数")
	// 虽然接受了tests,但是sentence并不可以在这里
	// 直接调用, 这是受到了作用域的影响, 她严格遵
	// 守{}的束缚.
	a()
	return a
	// 也可以作为返回值. (**其二**)
}

// 栈：临时,函数执行完,里面的变量就消失,速度快.
// 堆：长期,就算函数执行完,变量也能保留,速度稍慢.

func creater() func() int {
	var a = 0
	return func() int {
		a++
		return a
	}
}
