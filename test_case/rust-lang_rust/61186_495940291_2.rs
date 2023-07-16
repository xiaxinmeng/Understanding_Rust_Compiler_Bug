go
package main

import (
	"net"
	"fmt"
)

func main() {
	fmt.Println(net.ParseIP("01.01.01.01")) // prints 1.1.1.1
}
