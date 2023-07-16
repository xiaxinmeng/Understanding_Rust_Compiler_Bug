go
package main

import "fmt"

func main() {
	fmt.Println(interface{}(int(2)) == interface{}(int(2)))
	fmt.Println(interface{}(int(2)) == interface{}(uint(2)))
}
