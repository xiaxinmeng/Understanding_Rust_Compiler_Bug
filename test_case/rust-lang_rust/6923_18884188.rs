 llvm
define {} @_ZN4swap16_d0923be40fc71903_00E({ i64, %tydesc*, i8*, i8*, i8 } addrspace(1)*, i64*, i64*) #1 {
static_allocas:
  %3 = alloca {}
  %4 = alloca i64*
  %5 = alloca i64*
  %6 = alloca i64*
  %7 = alloca i64*
  br label %9

return:                                           ; preds = %16
  %8 = load {}* %3
  ret {} %8

; <label>:9                                       ; preds = %static_allocas
  store i64* %1, i64** %4
  store i64* %2, i64** %5
  br label %10

; <label>:10                                      ; preds = %9
  %11 = load i64** %4
  store i64* %11, i64** %6
  %12 = load i64** %6
  %13 = load i64** %5
  store i64* %13, i64** %7
  %14 = load i64** %7
  %15 = call {} @_ZN4util9swap_287416_d0923be40fc71903_00E({ i64, %tydesc*, i8*, i8*, i8 } addrspace(1)* undef, i64* %12, i64* %14)
  br label %16

; <label>:16                                      ; preds = %10
  br label %return
}

define internal {} @_ZN4util9swap_287416_d0923be40fc71903_00E({ i64, %tydesc*, i8*, i8*, i8 } addrspace(1)*, i64*, i64*) #2 {
static_allocas:
  %3 = alloca {}
  %4 = alloca i64*
  %5 = alloca i64*
  %6 = alloca i64
  %7 = alloca i64*
  %8 = alloca i64*
  %9 = alloca i64*
  %10 = alloca i64
  %11 = alloca i64*
  %12 = alloca i64*
  %13 = alloca i64
  %14 = alloca i64*
  %15 = alloca i64*
  %16 = alloca i64
  %17 = alloca i64
  br label %19

return:                                           ; preds = %53
  %18 = load {}* %3
  ret {} %18

; <label>:19                                      ; preds = %static_allocas
  store i64* %1, i64** %4
  store i64* %2, i64** %5
  br label %20

; <label>:20                                      ; preds = %19
  br label %21

; <label>:21                                      ; preds = %20
  %22 = call i64 @_ZN8unstable10intrinsics11uninit_287617_9d40a93ccd5df7ef3_00E({ i64, %tydesc*, i8*, i8*, i8 } addrspace(1)* undef)
  store i64 %22, i64* %6
  br label %23

; <label>:23                                      ; preds = %21
  store i64* %6, i64** %7
  br label %24

; <label>:24                                      ; preds = %23
  %25 = load i64** %7
  store i64* %25, i64** %8
  %26 = load i64** %8
  %27 = load i64** %4
  store i64* %27, i64** %9
  %28 = load i64** %9
  store i64 1, i64* %10
  %29 = load i64* %10
  %30 = call {} @_ZN3ptr31copy_nonoverlapping_memory_287917_cf9a2fc5e9ba32573_00E({ i64, %tydesc*, i8*, i8*, i8 } addrspace(1)* undef, i64* %26, i64* %28, i64 %29)
  br label %31

; <label>:31                                      ; preds = %24
  br label %32

; <label>:32                                      ; preds = %31
  %33 = load i64** %4
  store i64* %33, i64** %11
  %34 = load i64** %11
  %35 = load i64** %5
  store i64* %35, i64** %12
  %36 = load i64** %12
  store i64 1, i64* %13
  %37 = load i64* %13
  %38 = call {} @_ZN3ptr31copy_nonoverlapping_memory_287917_cf9a2fc5e9ba32573_00E({ i64, %tydesc*, i8*, i8*, i8 } addrspace(1)* undef, i64* %34, i64* %36, i64 %37)
  br label %39

; <label>:39                                      ; preds = %32
  br label %40

; <label>:40                                      ; preds = %39
  %41 = load i64** %5
  store i64* %41, i64** %14
  %42 = load i64** %14
  %43 = load i64** %7
  store i64* %43, i64** %15
  %44 = load i64** %15
  store i64 1, i64* %16
  %45 = load i64* %16
  %46 = call {} @_ZN3ptr31copy_nonoverlapping_memory_287917_cf9a2fc5e9ba32573_00E({ i64, %tydesc*, i8*, i8*, i8 } addrspace(1)* undef, i64* %42, i64* %44, i64 %45)
  br label %47

; <label>:47                                      ; preds = %40
  br label %48

; <label>:48                                      ; preds = %47
  %49 = load i64* %6
  store i64 %49, i64* %17
  %50 = load i64* %17
  %51 = call {} @_ZN4cast11forget_288516_b11419b9f83337d3_00E({ i64, %tydesc*, i8*, i8*, i8 } addrspace(1)* undef, i64 %50)
  br label %52

; <label>:52                                      ; preds = %48
  br label %53

; <label>:53                                      ; preds = %52
  br label %return
}

define internal i64 @_ZN8unstable10intrinsics11uninit_287617_9d40a93ccd5df7ef3_00E({ i64, %tydesc*, i8*, i8*, i8 } addrspace(1)*) {
static_allocas:
  %1 = alloca i64
  br label %3

return:                                           ; preds = %3
  %2 = load i64* %1
  ret i64 %2

; <label>:3                                       ; preds = %static_allocas
  br label %return
}

define internal {} @_ZN3ptr31copy_nonoverlapping_memory_287917_cf9a2fc5e9ba32573_00E({ i64, %tydesc*, i8*, i8*, i8 } addrspace(1)*, i64*, i64*, i64) #2 {
static_allocas:
  %4 = alloca {}
  %5 = alloca i64*
  %6 = alloca i64*
  %7 = alloca i64
  %8 = alloca i64*
  %9 = alloca i64*
  %10 = alloca i64
  br label %12

return:                                           ; preds = %21
  %11 = load {}* %4
  ret {} %11

; <label>:12                                      ; preds = %static_allocas
  store i64* %1, i64** %5
  store i64* %2, i64** %6
  store i64 %3, i64* %7
  br label %13

; <label>:13                                      ; preds = %12
  %14 = load i64** %5
  store i64* %14, i64** %8
  %15 = load i64** %8
  %16 = load i64** %6
  store i64* %16, i64** %9
  %17 = load i64** %9
  %18 = load i64* %7
  store i64 %18, i64* %10
  %19 = load i64* %10
  %20 = call {} @_ZN8unstable10intrinsics13memcpy64_288216_e3b16c8f2d61a2b3_00E({ i64, %tydesc*, i8*, i8*, i8 } addrspace(1)* undef, i64* %15, i64* %17, i64 %19)
  br label %21

; <label>:21                                      ; preds = %13
  br label %return
}

define internal {} @_ZN8unstable10intrinsics13memcpy64_288216_e3b16c8f2d61a2b3_00E({ i64, %tydesc*, i8*, i8*, i8 } addrspace(1)*, i64*, i64*, i64) {
static_allocas:
  %4 = alloca {}
  br label %6

return:                                           ; preds = %6
  %5 = load {}* %4
  ret {} %5

; <label>:6                                       ; preds = %static_allocas
  %7 = bitcast i64* %1 to i8*
  %8 = bitcast i64* %2 to i8*
  %9 = mul i64 8, %3
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %7, i8* %8, i64 %9, i32 8, i1 false)
  br label %return
}

define internal {} @_ZN4cast11forget_288516_b11419b9f83337d3_00E({ i64, %tydesc*, i8*, i8*, i8 } addrspace(1)*, i64) #2 {
static_allocas:
  %2 = alloca {}
  %3 = alloca i64
  %4 = alloca i64
  br label %6

return:                                           ; preds = %11
  %5 = load {}* %2
  ret {} %5

; <label>:6                                       ; preds = %static_allocas
  store i64 %1, i64* %3
  br label %7

; <label>:7                                       ; preds = %6
  %8 = load i64* %3
  store i64 %8, i64* %4
  %9 = load i64* %4
  %10 = call {} @_ZN8unstable10intrinsics11forget_288816_dd7eac2f264cb903_00E({ i64, %tydesc*, i8*, i8*, i8 } addrspace(1)* undef, i64 %9)
  br label %11

; <label>:11                                      ; preds = %7
  br label %return
}

define internal {} @_ZN8unstable10intrinsics11forget_288816_dd7eac2f264cb903_00E({ i64, %tydesc*, i8*, i8*, i8 } addrspace(1)*, i64) {
static_allocas:
  %2 = alloca {}
  br label %4

return:                                           ; preds = %4
  %3 = load {}* %2
  ret {} %3

; <label>:4                                       ; preds = %static_allocas
  br label %return
}

define internal {} @_ZN4main17_8030a1726da236673_00E({ i64, %tydesc*, i8*, i8*, i8 } addrspace(1)*) #1 {
static_allocas:
  %1 = alloca {}
  %2 = alloca i64
  %3 = alloca i64
  %4 = alloca i64*
  %5 = alloca i64*
  br label %7

return:                                           ; preds = %12
  %6 = load {}* %1
  ret {} %6

; <label>:7                                       ; preds = %static_allocas
  store i64 5, i64* %2
  store i64 10, i64* %3
  br label %8

; <label>:8                                       ; preds = %7
  store i64* %2, i64** %4
  %9 = load i64** %4
  store i64* %3, i64** %5
  %10 = load i64** %5
  %11 = call {} @_ZN4swap16_d0923be40fc71903_00E({ i64, %tydesc*, i8*, i8*, i8 } addrspace(1)* undef, i64* %9, i64* %10)
  br label %12

; <label>:12                                      ; preds = %8
  br label %return
}
