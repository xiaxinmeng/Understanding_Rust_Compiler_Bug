 LLVM
define void @"_ZN4swap17_8abdbe47da3a37657_0$x2e0E"({ i64, %tydesc*, i8*, i8*, i8 }*, i64*, i64*) #1 {
static_allocas:
  %__arg = alloca i64*
  %__arg1 = alloca i64*
  br label %"function top level"

"function top level":                             ; preds = %static_allocas
  store i64* %1, i64** %__arg
  store i64* %2, i64** %__arg1
  %3 = load i64** %__arg
  %4 = load i64** %__arg1
  call void @"_ZN4util9swap_321717_8abdbe47da3a37657_0$x2e0E"({ i64, %tydesc*, i8*, i8*, i8 }* undef, i64* %3, i64* %4)
  br label %return

return:                                           ; preds = %"function top level"
  ret void
}

define internal void @"_ZN4util9swap_321717_8abdbe47da3a37657_0$x2e0E"({ i64, %tydesc*, i8*, i8*, i8 }*, i64*, i64*) #2 {
static_allocas:
  %__arg = alloca i64*
  %__arg1 = alloca i64*
  %tmp = alloca i64
  %t = alloca i64*
  br label %"function top level"

"function top level":                             ; preds = %static_allocas
  store i64* %1, i64** %__arg
  store i64* %2, i64** %__arg1
  %3 = call i64 @"_ZN8unstable10intrinsics11uninit_321917_95a44b74b1a07b1f7_0$x2e0E"({ i64, %tydesc*, i8*, i8*, i8 }* undef)
  store i64 %3, i64* %tmp
  store i64* %tmp, i64** %t
  %4 = load i64** %t
  %5 = load i64** %__arg
  call void @"_ZN3ptr31copy_nonoverlapping_memory_322216_525ce544f73e4927_0$x2e0E"({ i64, %tydesc*, i8*, i8*, i8 }* undef, i64* %4, i64* %5, i64 1)
  %6 = load i64** %__arg
  %7 = load i64** %__arg1
  call void @"_ZN3ptr31copy_nonoverlapping_memory_322216_525ce544f73e4927_0$x2e0E"({ i64, %tydesc*, i8*, i8*, i8 }* undef, i64* %6, i64* %7, i64 1)
  %8 = load i64** %__arg1
  %9 = load i64** %t
  call void @"_ZN3ptr31copy_nonoverlapping_memory_322216_525ce544f73e4927_0$x2e0E"({ i64, %tydesc*, i8*, i8*, i8 }* undef, i64* %8, i64* %9, i64 1)
  %10 = load i64* %tmp
  call void @"_ZN4cast11forget_322817_36feec33e3f151c97_0$x2e0E"({ i64, %tydesc*, i8*, i8*, i8 }* undef, i64 %10)
  br label %return

return:                                           ; preds = %"function top level"
  ret void
}

define internal i64 @"_ZN8unstable10intrinsics11uninit_321917_95a44b74b1a07b1f7_0$x2e0E"({ i64, %tydesc*, i8*, i8*, i8 }*) {
static_allocas:
  %__make_return_pointer = alloca i64
  br label %"function top level"

return:                                           ; preds = %"function top level"
  %1 = load i64* %__make_return_pointer
  ret i64 %1

"function top level":                             ; preds = %static_allocas
  br label %return
}

define internal void @"_ZN3ptr31copy_nonoverlapping_memory_322216_525ce544f73e4927_0$x2e0E"({ i64, %tydesc*, i8*, i8*, i8 }*, i64*, i64*, i64) #2 {
static_allocas:
  %__arg = alloca i64*
  %__arg1 = alloca i64*
  %__arg2 = alloca i64
  br label %"function top level"

"function top level":                             ; preds = %static_allocas
  store i64* %1, i64** %__arg
  store i64* %2, i64** %__arg1
  store i64 %3, i64* %__arg2
  %4 = load i64** %__arg
  %5 = load i64** %__arg1
  %6 = load i64* %__arg2
  call void @"_ZN8unstable10intrinsics13memcpy64_322517_bdf0af8f1983aa627_0$x2e0E"({ i64, %tydesc*, i8*, i8*, i8 }* undef, i64* %4, i64* %5, i64 %6)
  br label %return

return:                                           ; preds = %"function top level"
  ret void
}

define internal void @"_ZN8unstable10intrinsics13memcpy64_322517_bdf0af8f1983aa627_0$x2e0E"({ i64, %tydesc*, i8*, i8*, i8 }*, i64*, i64*, i64) {
static_allocas:
  br label %"function top level"

return:                                           ; preds = %"function top level"
  ret void

"function top level":                             ; preds = %static_allocas
  %4 = bitcast i64* %1 to i8*
  %5 = bitcast i64* %2 to i8*
  %6 = mul i64 8, %3
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %4, i8* %5, i64 %6, i32 8, i1 false)
  br label %return
}

define internal void @"_ZN4cast11forget_322817_36feec33e3f151c97_0$x2e0E"({ i64, %tydesc*, i8*, i8*, i8 }*, i64) #2 {
static_allocas:
  %__arg = alloca i64
  br label %"function top level"

"function top level":                             ; preds = %static_allocas
  store i64 %1, i64* %__arg
  %2 = load i64* %__arg
  call void @"_ZN8unstable10intrinsics11forget_323116_b1abe21e5555db37_0$x2e0E"({ i64, %tydesc*, i8*, i8*, i8 }* undef, i64 %2)
  br label %return

return:                                           ; preds = %"function top level"
  ret void
}

define internal void @"_ZN8unstable10intrinsics11forget_323116_b1abe21e5555db37_0$x2e0E"({ i64, %tydesc*, i8*, i8*, i8 }*, i64) {
static_allocas:
  br label %"function top level"

return:                                           ; preds = %"function top level"
  ret void

"function top level":                             ; preds = %static_allocas
  br label %return
}

define void @"_ZN4main16_eeec6a0647358e17_0$x2e0E"({ i64, %tydesc*, i8*, i8*, i8 }*) #3 {
static_allocas:
  %x = alloca i64
  %y = alloca i64
  br label %"function top level"

"function top level":                             ; preds = %static_allocas
  store i64 5, i64* %x
  store i64 10, i64* %y
  call void @"_ZN4swap17_8abdbe47da3a37657_0$x2e0E"({ i64, %tydesc*, i8*, i8*, i8 }* undef, i64* %x, i64* %y)
  br label %return

return:                                           ; preds = %"function top level"
  ret void
}

define void @_rust_main({ i64, %tydesc*, i8*, i8*, i8 }*) {
static_allocas:
  br label %"function top level"

return:                                           ; preds = %"function top level"
  ret void

"function top level":                             ; preds = %static_allocas
  call void @"_ZN4main16_eeec6a0647358e17_0$x2e0E"({ i64, %tydesc*, i8*, i8*, i8 }* %0)
  br label %return
}

define i64 @main(i64, i8**) {
top:
  %2 = call i64 @"_ZN8unstable4lang5start17_76d6c774aa357c7a14_0$x2e8$x2dpreE"({ i64, %tydesc*, i8*, i8*, i8 }* null, i8* bitcast (void ({ i64, %tydesc*, i8*, i8*, i8 }*)* @_rust_main to i8*), i64 %0, i8** %1, i8* bitcast ({ i32, i8*, i64, [2 x i64] }* @_rust_crate_map_toplevel to i8*))
  ret i64 %2
}
