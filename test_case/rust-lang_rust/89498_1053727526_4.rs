llvm
%struct.Struct = type { i64, i32, [4 x i8] }

; Function Attrs: noinline nounwind optnone
define dso_local void @t(%struct.Struct* noalias sret(%struct.Struct) align 8 %0) #0 {
  %2 = getelementptr inbounds %struct.Struct, %struct.Struct* %0, i32 0, i32 0
  store i64 1000, i64* %2, align 8
  %3 = getelementptr inbounds %struct.Struct, %struct.Struct* %0, i32 0, i32 1
  store i32 3000, i32* %3, align 8
  ret void
}
