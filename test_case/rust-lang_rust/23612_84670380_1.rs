 llvm
define internal zeroext i1 @_ZN4main11closure.178E(%closure* noalias readonly, i32, i32) unnamed_addr #3 {
entry-block:
  %tupled_args = alloca { i32, i32 }
  %argtuple = alloca { i32, i32 }
  %x = alloca i32
  %y = alloca i32
  %3 = getelementptr inbounds { i32, i32 }* %tupled_args, i32 0, i32 0
  store i32 %1, i32* %3
  %4 = getelementptr inbounds { i32, i32 }* %tupled_args, i32 0, i32 1
  store i32 %2, i32* %4
  %5 = bitcast { i32, i32 }* %tupled_args to i64*
  %6 = load i64* %5
  %7 = bitcast { i32, i32 }* %argtuple to i64*
  store i64 %6, i64* %7
  %8 = getelementptr inbounds { i32, i32 }* %argtuple, i32 0, i32 0
  %9 = load i32* %8
  store i32 %9, i32* %x
  %10 = getelementptr inbounds { i32, i32 }* %argtuple, i32 0, i32 1
  %11 = load i32* %10
  store i32 %11, i32* %y
  %12 = load i32* %x
  %13 = load i32* %y
  %14 = icmp eq i32 %12, %13
  %15 = zext i1 %14 to i8
  %16 = trunc i8 %15 to i1
  ret i1 %16
}
