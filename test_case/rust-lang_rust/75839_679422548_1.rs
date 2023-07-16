llvm
%S = type { [0 x i32], i32, [0 x i32], float, [0 x i32], i32, [0 x i32] }
[...]
; foreign_truncated_arguments::main
; Function Attrs: nonlazybind uwtable
define internal void @_ZN27foreign_truncated_arguments4main17h8a09fc9d120f0135E() unnamed_addr #1 {
start:
  %_26 = alloca i32*, align 8
  %_24 = alloca i32*, align 8
  %_22 = alloca { i64*, i64* }, align 8
  %_21 = alloca [2 x { i8*, i64* }], align 8
  %_14 = alloca %"core::fmt::Arguments", align 8
  %_4 = alloca %S, align 4
  %_3 = alloca i32, align 4
  %_1 = alloca { i32*, i32* }, align 8
  %0 = bitcast %S* %_4 to i32*
  store i32 0, i32* %0, align 4
  %1 = getelementptr inbounds %S, %S* %_4, i32 0, i32 3
  store float 0.000000e+00, float* %1, align 4
  %2 = getelementptr inbounds %S, %S* %_4, i32 0, i32 5
  store i32 42, i32* %2, align 4
  %3 = bitcast %S* %_4 to { i64, i32 }*
  %4 = load { i64, i32 }, { i64, i32 }* %3, align 4
  %5 = call i32 @test({ i64, i32 } %4)
  store i32 %5, i32* %_3, align 4
  br label %bb1
