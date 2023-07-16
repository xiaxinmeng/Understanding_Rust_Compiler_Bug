
define i64 @test(i64* %p) {
  %x = atomicrmw sub i64* %p, i64 1 seq_cst 
  ret i64 %x 
} 
