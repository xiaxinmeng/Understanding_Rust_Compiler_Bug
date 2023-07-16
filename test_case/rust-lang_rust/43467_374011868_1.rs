llvm
%struct.c_void = type opaque

define void @call_malloc() #0 {
  %1 = call %struct.c_void* @malloc(i64 1)
  ret void
}

declare %struct.c_void* @malloc(i64) #1
