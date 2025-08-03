/**
 * Author:  Nyxvectar Yan
 * Repo:    guths
 * Created: 07/29/2025
 */

package slice

import "fmt"

func GutsSlice() {
	{
		// 对于一个slice有其底层数组,其衍生slice的
		// slice有len和cap,只有读取len内的值才不算越界访问
		// 当使用make()函数声明slice的时候会分配底层数组
		var slice = make([]int, 2, 5)
		fmt.Print(slice)
		slice[1] = 1
		fmt.Print(slice)
		slice = append(slice, 2)
		fmt.Print(slice)
		// 然而使用new()的时候并不会,需要append()来分配
		var kokosa = new([]string)
		// 此时其data位值是nil,返回值就是其起始地址/&[]
		fmt.Print(*kokosa)
	}
	fmt.Println()
	{
		var arr = [10]int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
		var slice1 = arr[1:4]
		var slice2 = arr[7:]
		fmt.Print(slice1, slice2)
		slice1 = append(slice1, 1, 2, 3, 4)
		fmt.Print(" ", slice2)
		// 观察到slice2的第一个数字也被更改为4
		// 说明了go当中slice可以以如上方式关联
		// 到同一个底层数组上,并且他们之间数组
		// 的修改是会互相影响的.
		{
			// 除此以外还需要观察他们的cap大小(容量大小)
			// 注意到都是从他们的起点开始取到底层数组的末位.
			fmt.Println(cap(slice1), cap(slice2))
		}
		{
			// 那么如果我们append()到了一个更大的切片呢?
			slice1 = append(slice1, 5, 6, 7)
			fmt.Print(slice1, slice2)
			// 观察到此时slice2的值并没有受到slice1的影响
			// 证明了slice1在append到一个更大slice的过程中
			// 复制产生了一个更大的独立的新底层数组.
		}
	}
	// append的slice扩容是有其规则的
	// 当切片尚且没有那么大的时候cap直接翻倍
	// 当切片达到一定程度的时候则会只扩大1/4.
}
