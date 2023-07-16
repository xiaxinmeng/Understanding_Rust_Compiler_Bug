
target triple = "x86_64-unknown-linux-gnu"
 
define void @test1() "target-features"="+avx" {
  call void @test2() 
  ret void
} 
 
define void @test2() {
  call void @test3(<4 x i64> <i64 0, i64 1, i64 2, i64 3>)
  ret void
} 
 
define void @test3(<4 x i64> %arg) noinline {
  ret void 
}  
