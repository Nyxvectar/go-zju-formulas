/**
 * Author:  Nyxvectar Yan
 * Repo:    guths
 * Created: 07/30/2025
 */

package stack

func GutsArgument() {
	var x = 10
	double(x)
	println(x)
	// x 的输出值并不会改变.
	// 这也是go作用域的结果.
	// 关于返回值其实也同理.
}

func double(num int) {
	num = num * 2
}
