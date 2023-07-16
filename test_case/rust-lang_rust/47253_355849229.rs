llvm
%"alloc::vec::Vec<u8>" = type { [0 x i64], { i8*, i64 }, [0 x i64], i64, [0 x i64] }
%"core::option::Option<alloc::allocator::Layout>" = type { [3 x i64] }
%"core::option::Option<alloc::allocator::Layout>::None" = type {}
%"core::option::Option<alloc::allocator::Layout>::Some" = type { [1 x i64], { i64, i64 }, [0 x i64] }
%"alloc::heap::Heap" = type {}
%"main::{{closure}}" = type { [0 x i64], %Foo, [0 x i32], i32, [1 x i32] }
%Foo = type { [0 x i64], %"core::option::Option<alloc::string::String>", [0 x i8], i8, [7 x i8] }
%"core::option::Option<alloc::string::String>" = type { [0 x i64], {}*, [2 x i64] }
%"core::option::Option<main::{{closure}}>" = type { [24 x i8], i8, [15 x i8] }
%"core::option::Option<main::{{closure}}>::Some" = type { [0 x i64], %"main::{{closure}}", [0 x i64] }
%"alloc::string::String" = type { [0 x i64], %"alloc::vec::Vec<u8>", [0 x i64] }
%"core::option::Option<alloc::string::String>::Some" = type { [0 x i64], %"alloc::string::String", [0 x i64] }

define internal void @_ZN6uninit4main17h832ef423163e0fb1E() unnamed_addr #1 {
start:
  %_3 = alloca %"main::{{closure}}", align 8
  %_2 = alloca %"core::option::Option<main::{{closure}}>", align 8
  %g = alloca %"main::{{closure}}", align 8
  %_0 = alloca {}, align 1
  %0 = getelementptr inbounds %"main::{{closure}}", %"main::{{closure}}"* %g, i32 0, i32 3
  store i32 0, i32* %0
  %1 = bitcast %"main::{{closure}}"* %g to i8*
  %2 = bitcast %"main::{{closure}}"* %_3 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %2, i8* %1, i64 40, i32 8, i1 false)
  %3 = bitcast %"core::option::Option<main::{{closure}}>"* %_2 to %"core::option::Option<main::{{closure}}>::Some"*
  %4 = bitcast %"core::option::Option<main::{{closure}}>::Some"* %3 to %"main::{{closure}}"*
  %5 = bitcast %"main::{{closure}}"* %_3 to i8*
  %6 = bitcast %"main::{{closure}}"* %4 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %6, i8* %5, i64 40, i32 8, i1 false)
  %7 = getelementptr inbounds %"core::option::Option<main::{{closure}}>", %"core::option::Option<main::{{closure}}>"* %_2, i32 0, i32 1
  %8 = load i8, i8* %7, !range !3
  %9 = icmp eq i8 %8, 2
  %10 = select i1 %9, i64 0, i64 1
  switch i64 %10, label %bb3 [
    i64 0, label %bb1
    i64 1, label %bb2
  ]

bb1:                                              ; preds = %start
  br label %bb4

bb2:                                              ; preds = %start
  br label %bb4

bb3:                                              ; preds = %start
  unreachable

bb4:                                              ; preds = %bb1, %bb2
; call core::ptr::drop_in_place
  call void @_ZN4core3ptr13drop_in_place17h2c816151c807a52eE(%"core::option::Option<main::{{closure}}>"* %_2)
  br label %bb5

bb5:                                              ; preds = %bb4
  ret void
}
