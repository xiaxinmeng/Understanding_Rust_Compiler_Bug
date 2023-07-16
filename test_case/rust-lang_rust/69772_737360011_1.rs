go
package main

import (
	"fmt"
	"net"
)

func main() {
	fmt.Println(net.ParseIP(`::ffff:127.0.0.1`).IsLoopback())
}
