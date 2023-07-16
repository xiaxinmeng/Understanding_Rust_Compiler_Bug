llvm
define void @call_malloc() #0 {
  %1 = call i8* @malloc(i64 1)
  ret void
}

declare i8* @malloc(i64) #1
