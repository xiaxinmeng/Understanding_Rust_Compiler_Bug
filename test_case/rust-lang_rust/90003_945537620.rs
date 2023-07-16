go
package main

import "fmt"

func main() {
var a int64 = -25252734735360000 * 365 - int64(25252734735360000 * 0.2425)
var b int64 = -25252734735360000 * 365.2425 
fmt.Println(a==b)
}
