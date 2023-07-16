 go
// foo.go
package main

func main() {
  for i := 0; i < 100000; i++ {
    c := make(chan int)
    go func() { c <- 5 }()
    <-c
  }
}
