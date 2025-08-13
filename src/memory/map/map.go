/**
 * Author:  Nyxvectar Yan
 * Repo:    guths
 * Created: 07/30/2025
 */

package _map

import (
	"fmt"
	"unsafe"
)

func GutsMap() {
	type bmap struct {
		tophash [8]uint8 // 存储哈希值的高8位，用于快速比对
		// 实际结构中还包含key、value数组以及overflow指针
		// 但这些字段是通过指针运算访问的，并未显式定义
	}
	// 注意: bmap要先声明于mapextra, 这是顺序敏感的
	// 自定义类型，用于表示hmap.extra字段的指针类型
	type mapextra struct {
		overflow     *[]*bmap // 溢出桶指针
		oldoverflow  *[]*bmap // 扩容时的旧溢出桶
		nextOverflow *bmap    // 下一个可用溢出桶
	}
	// 但是这并不是桶的结构, 而是整体的结构
	// 桶的结构为bmap, 即buckets map.
	// go语言映射的实质就是哈希表
	var a = map[string]string{}
	// a是一个指向hmap结构体的指针, 通过*hmap可知:
	type hmap struct {
		count      int   // 键值对数目
		flags      uint8 // 2^B
		B          uint8
		noverflow  uint16
		hash0      uint32
		buckets    unsafe.Pointer // 桶
		oldbuckets unsafe.Pointer // 旧桶
		nevacuate  uintptr        // 即将迁移的旧桶
		extra      *mapextra
	}
	fmt.Println(a)
	// 等量扩容的意义是让新的值排列更加
	// 紧密, 从而减少溢出桶的使用.
}
