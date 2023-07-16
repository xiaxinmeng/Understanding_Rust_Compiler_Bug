
> package main
> 
> import "testing"
> 
> func BenchmarkClone(b *testing.B) {
> 	b.StopTimer()
> 	var str = "dfasdfas"
> 	var target=[]byte{}
> 	b.StartTimer()
> 	for i := 0; i < b.N; i++ {
> 		copy([]byte(str),target)
> 	}
> }
> //BenchmarkClone-8   	200000000	         7.29 ns/op
> //PASS
> 