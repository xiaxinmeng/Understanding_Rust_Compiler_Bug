go
package main

import "fmt"

type S struct {}

func (*S) String() string {
	return "S"
}

func main() {
	var s fmt.Stringer = (*S)(nil)
	fmt.Println(s == nil) // false
}
