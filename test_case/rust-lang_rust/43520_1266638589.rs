go
package main

type EmptyFunc func()

type FuncInterface interface {
	EmptyFunc
}

func bar(f EmptyFunc) EmptyFunc { return f }

func foo[T FuncInterface](flag bool, fp T) {
	if flag {
		foo(false, bar(func() {}))
	}
}

func main() {
	foo(false, bar(func() {}))
}
