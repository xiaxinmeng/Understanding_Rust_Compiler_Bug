
; Function Attrs: convergent noinline nounwind optnone mustprogress
define dso_local %struct.foo @_Z6deviceh(i8 zeroext %0) #0 {
  %2 = alloca %struct.foo, align 1
  %3 = alloca i8, align 1
  store i8 %0, i8* %3, align 1
  %4 = getelementptr inbounds %struct.foo, %struct.foo* %2, i32 0, i32 0
  %5 = load i8, i8* %3, align 1
  store i8 %5, i8* %4, align 1
  %6 = getelementptr inbounds %struct.foo, %struct.foo* %2, i32 0, i32 1
  %7 = load i8, i8* %3, align 1
  store i8 %7, i8* %6, align 1
  %8 = getelementptr inbounds %struct.foo, %struct.foo* %2, i32 0, i32 2
  %9 = load i8, i8* %3, align 1
  store i8 %9, i8* %8, align 1
  %10 = load %struct.foo, %struct.foo* %2, align 1
  ret %struct.foo %10
}
