
; ModuleID = 'segfault.0.rs'
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%str_slice = type { i8*, i64 }
%"2.core::fmt::Formatter" = type { i32, i32, i8, %"2.core::option::Option<usize>", %"2.core::option::Option<usize>", { i8*, void (i8*)** }, %"2.core::slice::Iter<core::fmt::ArgumentV1>", { %"2.core::fmt::ArgumentV1"*, i64 } }
%"2.core::option::Option<usize>" = type { i64, [0 x i64], [1 x i64] }
%"2.core::slice::Iter<core::fmt::ArgumentV1>" = type { %"2.core::fmt::ArgumentV1"*, %"2.core::fmt::ArgumentV1"*, %"2.core::marker::PhantomData<&'static core::fmt::ArgumentV1>" }
%"2.core::fmt::ArgumentV1" = type { %"2.core::fmt::Void"*, i8 (%"2.core::fmt::Void"*, %"2.core::fmt::Formatter"*)* }
%"2.core::fmt::Void" = type {}
%"2.core::marker::PhantomData<&'static core::fmt::ArgumentV1>" = type {}
%"9.my_crate::Foo" = type { {} }
%"2.core::fmt::Arguments" = type { { %str_slice*, i64 }, %"2.core::option::Option<&'static [core::fmt::rt::v1::Argument]>", { %"2.core::fmt::ArgumentV1"*, i64 } }
%"2.core::option::Option<&'static [core::fmt::rt::v1::Argument]>" = type { { %"2.core::fmt::rt::v1::Argument"*, i64 } }
%"2.core::fmt::rt::v1::Argument" = type { %"2.core::fmt::rt::v1::Position", %"2.core::fmt::rt::v1::FormatSpec" }
%"2.core::fmt::rt::v1::Position" = type { i64, [0 x i64], [1 x i64] }
%"2.core::fmt::rt::v1::FormatSpec" = type { i32, i8, i32, %"2.core::fmt::rt::v1::Count", %"2.core::fmt::rt::v1::Count" }
%"2.core::fmt::rt::v1::Count" = type { i64, [0 x i64], [1 x i64] }

@const2476 = internal unnamed_addr constant i64 0
@const2480 = internal unnamed_addr constant {}* inttoptr (i64 1 to {}*)
@const2481 = internal unnamed_addr constant i8* inttoptr (i64 1 to i8*)
@const2497 = internal unnamed_addr constant i8 1
@_ZN4heap24check_size_and_alignment14_MSG_FILE_LINE20hc3b2d56120089136JbaE = external global { %str_slice, %str_slice, i32 }
@const2504 = internal unnamed_addr constant i64 64
@const2505 = internal unnamed_addr constant i64 -9223372036854775808
@const2506 = internal unnamed_addr constant i64 9223372036854775807
@const2514 = internal unnamed_addr constant { { i8*, i8* } } zeroinitializer
@_ZN4heap24check_size_and_alignment15__STATIC_FMTSTR20hfa6992380310c546wcaE = external global { %str_slice*, i64 }
@const2621 = internal unnamed_addr constant i8 (i64*, %"2.core::fmt::Formatter"*)* @"_ZN3fmt3num13_$LT$impl$GT$3fmt20h61c12488bb3d5903AZVE"
@_ZN4heap24check_size_and_alignment10_FILE_LINE20hcf2a8ce10e9354d6gcaE = external global { %str_slice, i32 }
@const2632 = internal unnamed_addr constant i64 1
@_ZN4heap24check_size_and_alignment15__STATIC_FMTSTR20hfa6992380310c546qdaE = external global { %str_slice*, i64 }
@_ZN4heap24check_size_and_alignment10_FILE_LINE20hcf2a8ce10e9354d6adaE = external global { %str_slice, i32 }
@const2641 = internal unnamed_addr constant i8* null

; Function Attrs: uwtable
define noalias %"9.my_crate::Foo"* @_ZN7box_foo20hf984887980016f2cgaaE() unnamed_addr #0 {
entry-block:
  %0 = call %"9.my_crate::Foo" @"_ZN13_$LT$impl$GT$3new20h7a0183b89d396887kaaE"()
  %1 = call i8* @_ZN4heap15exchange_malloc20h496cd3d208c78b0aJfaE(i64 0, i64 1)
  %2 = bitcast i8* %1 to %"9.my_crate::Foo"*
  ret %"9.my_crate::Foo"* %2
}

; Function Attrs: inlinehint uwtable
define internal i8* @_ZN4heap15exchange_malloc20h496cd3d208c78b0aJfaE(i64, i64) unnamed_addr #1 {
entry-block:
  %sret_slot = alloca i8*
  %size = alloca i64
  %align = alloca i64
  %ptr = alloca i8*
  %2 = alloca i8
  store i64 %0, i64* %size, align 8
  store i64 %1, i64* %align, align 8
  %3 = load i64, i64* %size, align 8
  %4 = icmp eq i64 %3, 0
  br i1 %4, label %then-block-41-, label %else-block

then-block-41-:                                   ; preds = %entry-block
  store i8* inttoptr (i64 1 to i8*), i8** %sret_slot, align 8
  br label %join

else-block:                                       ; preds = %entry-block
  %5 = load i64, i64* %size, align 8
  %6 = load i64, i64* %align, align 8
  %7 = call i8* @_ZN4heap8allocate20h8f9906da8b1122b2PdaE(i64 %5, i64 %6)
  store i8* %7, i8** %ptr, align 8
  %8 = load i8*, i8** %ptr, align 8
  %9 = call zeroext i1 @"_ZN3ptr13_$LT$impl$GT$7is_null7is_null21h14748829460356738718E"(i8* %8)
  %10 = zext i1 %9 to i8
  store i8 %10, i8* %2, align 1
  %11 = load i8, i8* %2, align 1, !range !0
  %12 = trunc i8 %11 to i1
  br i1 %12, label %then-block-59-, label %next-block

then-block-59-:                                   ; preds = %else-block
  call void @_ZN3oom20hd9170b7f1397402fatbE()
  unreachable

next-block:                                       ; preds = %else-block
  %13 = load i8*, i8** %ptr, align 8
  store i8* %13, i8** %sret_slot, align 8
  br label %join

join:                                             ; preds = %next-block, %then-block-41-
  %14 = load i8*, i8** %sret_slot, align 8
  ret i8* %14
}

; Function Attrs: inlinehint uwtable
define internal i8* @_ZN4heap8allocate20h8f9906da8b1122b2PdaE(i64, i64) unnamed_addr #1 {
entry-block:
  %size.i = alloca i64
  %align.i = alloca i64
  %2 = alloca %"2.core::fmt::Arguments"
  %3 = alloca { %str_slice*, i64 }
  %4 = alloca [1 x %"2.core::fmt::ArgumentV1"]
  %5 = alloca { i64* }
  %match.i = alloca { i64* }
  %__llmatch.i = alloca i64**
  %__arg0.i = alloca i64*
  %__coerce_target.i = alloca { %"2.core::fmt::ArgumentV1"*, i64 }
  %6 = alloca i8
  %7 = alloca %"2.core::fmt::Arguments"
  %8 = alloca { %str_slice*, i64 }
  %9 = alloca [1 x %"2.core::fmt::ArgumentV1"]
  %10 = alloca { i64* }
  %match2.i = alloca { i64* }
  %__llmatch4.i = alloca i64**
  %__arg05.i = alloca i64*
  %__coerce_target7.i = alloca { %"2.core::fmt::ArgumentV1"*, i64 }
  %size = alloca i64
  %align = alloca i64
  %__arg = alloca i64
  %__arg1 = alloca i64
  store i64 %0, i64* %size, align 8
  store i64 %1, i64* %align, align 8
  %11 = load i64, i64* %size, align 8
  %12 = load i64, i64* %align, align 8
  store i64 %11, i64* %size.i, align 8
  store i64 %12, i64* %align.i, align 8
  %13 = load i64, i64* %size.i, align 8
  %14 = icmp ne i64 %13, 0
  %15 = xor i1 %14, true
  br i1 %15, label %then-block-112-.i, label %next-block.i

then-block-112-.i:                                ; preds = %entry-block
  call void @_ZN9panicking5panic20h52df14a965b04688KKKE({ %str_slice, %str_slice, i32 }* noalias readonly dereferenceable(40) @_ZN4heap24check_size_and_alignment14_MSG_FILE_LINE20hc3b2d56120089136JbaE)
  unreachable

next-block.i:                                     ; preds = %entry-block
  %16 = load i64, i64* %size.i, align 8
  %17 = icmp ule i64 %16, 9223372036854775807
  %18 = xor i1 %17, true
  br i1 %18, label %then-block-145-.i, label %next-block1.i

then-block-145-.i:                                ; preds = %next-block.i
  %19 = bitcast { %str_slice*, i64 }* %3 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %19, i8* bitcast ({ %str_slice*, i64 }* @_ZN4heap24check_size_and_alignment15__STATIC_FMTSTR20hfa6992380310c546wcaE to i8*), i64 16, i32 8, i1 false)
  %20 = getelementptr inbounds { %str_slice*, i64 }, { %str_slice*, i64 }* %3, i32 0, i32 0
  %21 = load %str_slice*, %str_slice** %20
  %22 = getelementptr inbounds { %str_slice*, i64 }, { %str_slice*, i64 }* %3, i32 0, i32 1
  %23 = load i64, i64* %22
  %24 = getelementptr inbounds { i64* }, { i64* }* %5, i32 0, i32 0
  store i64* %size.i, i64** %24, align 8
  %25 = bitcast { i64* }* %5 to i64*
  %26 = load i64, i64* %25, align 8
  %27 = bitcast { i64* }* %match.i to i64*
  store i64 %26, i64* %27, align 8
  %28 = getelementptr inbounds { i64* }, { i64* }* %match.i, i32 0, i32 0
  store i64** %28, i64*** %__llmatch.i
  %29 = load i64**, i64*** %__llmatch.i
  %30 = load i64*, i64** %29, align 8, !nonnull !1
  store i64* %30, i64** %__arg0.i, align 8
  %31 = getelementptr inbounds [1 x %"2.core::fmt::ArgumentV1"], [1 x %"2.core::fmt::ArgumentV1"]* %4, i32 0, i32 0
  %32 = load i64*, i64** %__arg0.i, align 8, !nonnull !1
  call void @"_ZN3fmt13_$LT$impl$GT$3new3new20h4621647584932494472E"(%"2.core::fmt::ArgumentV1"* noalias nocapture sret dereferenceable(16) %31, i64* noalias readonly dereferenceable(8) %32, i8 (i64*, %"2.core::fmt::Formatter"*)* @"_ZN3fmt3num13_$LT$impl$GT$3fmt20h61c12488bb3d5903AZVE")
  %33 = bitcast [1 x %"2.core::fmt::ArgumentV1"]* %4 to %"2.core::fmt::ArgumentV1"*
  %34 = getelementptr inbounds { %"2.core::fmt::ArgumentV1"*, i64 }, { %"2.core::fmt::ArgumentV1"*, i64 }* %__coerce_target.i, i32 0, i32 0
  store %"2.core::fmt::ArgumentV1"* %33, %"2.core::fmt::ArgumentV1"** %34
  %35 = getelementptr inbounds { %"2.core::fmt::ArgumentV1"*, i64 }, { %"2.core::fmt::ArgumentV1"*, i64 }* %__coerce_target.i, i32 0, i32 1
  store i64 1, i64* %35
  %36 = getelementptr inbounds { %"2.core::fmt::ArgumentV1"*, i64 }, { %"2.core::fmt::ArgumentV1"*, i64 }* %__coerce_target.i, i32 0, i32 0
  %37 = load %"2.core::fmt::ArgumentV1"*, %"2.core::fmt::ArgumentV1"** %36
  %38 = getelementptr inbounds { %"2.core::fmt::ArgumentV1"*, i64 }, { %"2.core::fmt::ArgumentV1"*, i64 }* %__coerce_target.i, i32 0, i32 1
  %39 = load i64, i64* %38
  call void @"_ZN3fmt13_$LT$impl$GT$6new_v120h5e9764a905143b40HJWE"(%"2.core::fmt::Arguments"* noalias nocapture sret dereferenceable(48) %2, %str_slice* noalias nonnull readonly %21, i64 %23, %"2.core::fmt::ArgumentV1"* noalias nonnull readonly %37, i64 %39)
  call void @_ZN9panicking9panic_fmt20h9106226f5abb116edMKE(%"2.core::fmt::Arguments"* noalias nocapture dereferenceable(48) %2, { %str_slice, i32 }* noalias readonly dereferenceable(24) @_ZN4heap24check_size_and_alignment10_FILE_LINE20hcf2a8ce10e9354d6gcaE)
  unreachable

next-block1.i:                                    ; preds = %next-block.i
  %40 = load i64, i64* %align.i, align 8
  %41 = call zeroext i1 @"_ZN3num13_$LT$impl$GT$15is_power_of_two20hcabd0a0fb3e6b19ar8iE"(i64 %40)
  %42 = zext i1 %41 to i8
  store i8 %42, i8* %6, align 1
  %43 = load i8, i8* %6, align 1, !range !0
  %44 = trunc i8 %43 to i1
  %45 = xor i1 %44, true
  br i1 %45, label %then-block-201-.i, label %_ZN4heap24check_size_and_alignment20hfe4e6278eec1ce19mbaE.exit

then-block-201-.i:                                ; preds = %next-block1.i
  %46 = bitcast { %str_slice*, i64 }* %8 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %46, i8* bitcast ({ %str_slice*, i64 }* @_ZN4heap24check_size_and_alignment15__STATIC_FMTSTR20hfa6992380310c546qdaE to i8*), i64 16, i32 8, i1 false)
  %47 = getelementptr inbounds { %str_slice*, i64 }, { %str_slice*, i64 }* %8, i32 0, i32 0
  %48 = load %str_slice*, %str_slice** %47
  %49 = getelementptr inbounds { %str_slice*, i64 }, { %str_slice*, i64 }* %8, i32 0, i32 1
  %50 = load i64, i64* %49
  %51 = getelementptr inbounds { i64* }, { i64* }* %10, i32 0, i32 0
  store i64* %align.i, i64** %51, align 8
  %52 = bitcast { i64* }* %10 to i64*
  %53 = load i64, i64* %52, align 8
  %54 = bitcast { i64* }* %match2.i to i64*
  store i64 %53, i64* %54, align 8
  %55 = getelementptr inbounds { i64* }, { i64* }* %match2.i, i32 0, i32 0
  store i64** %55, i64*** %__llmatch4.i
  %56 = load i64**, i64*** %__llmatch4.i
  %57 = load i64*, i64** %56, align 8, !nonnull !1
  store i64* %57, i64** %__arg05.i, align 8
  %58 = getelementptr inbounds [1 x %"2.core::fmt::ArgumentV1"], [1 x %"2.core::fmt::ArgumentV1"]* %9, i32 0, i32 0
  %59 = load i64*, i64** %__arg05.i, align 8, !nonnull !1
  call void @"_ZN3fmt13_$LT$impl$GT$3new3new20h4621647584932494472E"(%"2.core::fmt::ArgumentV1"* noalias nocapture sret dereferenceable(16) %58, i64* noalias readonly dereferenceable(8) %59, i8 (i64*, %"2.core::fmt::Formatter"*)* @"_ZN3fmt3num13_$LT$impl$GT$3fmt20h61c12488bb3d5903AZVE")
  %60 = bitcast [1 x %"2.core::fmt::ArgumentV1"]* %9 to %"2.core::fmt::ArgumentV1"*
  %61 = getelementptr inbounds { %"2.core::fmt::ArgumentV1"*, i64 }, { %"2.core::fmt::ArgumentV1"*, i64 }* %__coerce_target7.i, i32 0, i32 0
  store %"2.core::fmt::ArgumentV1"* %60, %"2.core::fmt::ArgumentV1"** %61
  %62 = getelementptr inbounds { %"2.core::fmt::ArgumentV1"*, i64 }, { %"2.core::fmt::ArgumentV1"*, i64 }* %__coerce_target7.i, i32 0, i32 1
  store i64 1, i64* %62
  %63 = getelementptr inbounds { %"2.core::fmt::ArgumentV1"*, i64 }, { %"2.core::fmt::ArgumentV1"*, i64 }* %__coerce_target7.i, i32 0, i32 0
  %64 = load %"2.core::fmt::ArgumentV1"*, %"2.core::fmt::ArgumentV1"** %63
  %65 = getelementptr inbounds { %"2.core::fmt::ArgumentV1"*, i64 }, { %"2.core::fmt::ArgumentV1"*, i64 }* %__coerce_target7.i, i32 0, i32 1
  %66 = load i64, i64* %65
  call void @"_ZN3fmt13_$LT$impl$GT$6new_v120h5e9764a905143b40HJWE"(%"2.core::fmt::Arguments"* noalias nocapture sret dereferenceable(48) %7, %str_slice* noalias nonnull readonly %48, i64 %50, %"2.core::fmt::ArgumentV1"* noalias nonnull readonly %64, i64 %66)
  call void @_ZN9panicking9panic_fmt20h9106226f5abb116edMKE(%"2.core::fmt::Arguments"* noalias nocapture dereferenceable(48) %7, { %str_slice, i32 }* noalias readonly dereferenceable(24) @_ZN4heap24check_size_and_alignment10_FILE_LINE20hcf2a8ce10e9354d6adaE)
  unreachable

_ZN4heap24check_size_and_alignment20hfe4e6278eec1ce19mbaE.exit: ; preds = %next-block1.i
  %67 = load i64, i64* %size, align 8
  %68 = load i64, i64* %align, align 8
  store i64 %67, i64* %__arg, align 8
  %69 = load i64, i64* %__arg
  store i64 %68, i64* %__arg1, align 8
  %70 = load i64, i64* %__arg1
  %71 = call i8* @__rust_allocate(i64 %69, i64 %70)
  ret i8* %71
}

; Function Attrs: cold noinline noreturn
declare void @_ZN9panicking5panic20h52df14a965b04688KKKE({ %str_slice, %str_slice, i32 }* noalias readonly dereferenceable(40)) unnamed_addr #2

; Function Attrs: cold noinline noreturn
declare void @_ZN9panicking9panic_fmt20h9106226f5abb116edMKE(%"2.core::fmt::Arguments"* noalias nocapture dereferenceable(48), { %str_slice, i32 }* noalias readonly de
referenceable(24)) unnamed_addr #2

; Function Attrs: inlinehint uwtable
define internal void @"_ZN3fmt13_$LT$impl$GT$6new_v120h5e9764a905143b40HJWE"(%"2.core::fmt::Arguments"* noalias nocapture sret dereferenceable(48), %str_slice* noalias nonnull readonly, i64, %"2.core::fmt::ArgumentV1"* noalias nonnull readonly, i64) unnamed_addr #1 {
entry-block:
  %pieces = alloca { %str_slice*, i64 }
  %args = alloca { %"2.core::fmt::ArgumentV1"*, i64 }
  %5 = getelementptr inbounds { %str_slice*, i64 }, { %str_slice*, i64 }* %pieces, i32 0, i32 0
  store %str_slice* %1, %str_slice** %5
  %6 = getelementptr inbounds { %str_slice*, i64 }, { %str_slice*, i64 }* %pieces, i32 0, i32 1
  store i64 %2, i64* %6
  %7 = getelementptr inbounds { %"2.core::fmt::ArgumentV1"*, i64 }, { %"2.core::fmt::ArgumentV1"*, i64 }* %args, i32 0, i32 0
  store %"2.core::fmt::ArgumentV1"* %3, %"2.core::fmt::ArgumentV1"** %7
  %8 = getelementptr inbounds { %"2.core::fmt::ArgumentV1"*, i64 }, { %"2.core::fmt::ArgumentV1"*, i64 }* %args, i32 0, i32 1
  store i64 %4, i64* %8
  %9 = getelementptr inbounds %"2.core::fmt::Arguments", %"2.core::fmt::Arguments"* %0, i32 0, i32 0
  %10 = bitcast { %str_slice*, i64 }* %pieces to i8*
  %11 = bitcast { %str_slice*, i64 }* %9 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %11, i8* %10, i64 16, i32 8, i1 false)
  %12 = getelementptr inbounds %"2.core::fmt::Arguments", %"2.core::fmt::Arguments"* %0, i32 0, i32 1
  %13 = bitcast %"2.core::option::Option<&'static [core::fmt::rt::v1::Argument]>"* %12 to { { i8*, i8* } }*
  %14 = bitcast { { i8*, i8* } }* %13 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %14, i8* bitcast ({ { i8*, i8* } }* @const2514 to i8*), i64 16, i32 8, i1 false)
  %15 = getelementptr inbounds %"2.core::fmt::Arguments", %"2.core::fmt::Arguments"* %0, i32 0, i32 2
  %16 = bitcast { %"2.core::fmt::ArgumentV1"*, i64 }* %args to i8*
  %17 = bitcast { %"2.core::fmt::ArgumentV1"*, i64 }* %15 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %17, i8* %16, i64 16, i32 8, i1 false)
  ret void
}

; Function Attrs: nounwind
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* nocapture, i8* nocapture readonly, i64, i32, i1) #3

; Function Attrs: uwtable
define internal void @"_ZN3fmt13_$LT$impl$GT$3new3new20h4621647584932494472E"(%"2.core::fmt::ArgumentV1"* noalias nocapture sret dereferenceable(16), i64* noalias readonly dereferenceable(8), i8 (i64*, %"2.core::fmt::Formatter"*)*) unnamed_addr #0 {
entry-block:
  %x = alloca i64*
  %f = alloca i8 (i64*, %"2.core::fmt::Formatter"*)*
  store i64* %1, i64** %x, align 8
  store i8 (i64*, %"2.core::fmt::Formatter"*)* %2, i8 (i64*, %"2.core::fmt::Formatter"*)** %f, align 8
  %3 = getelementptr inbounds %"2.core::fmt::ArgumentV1", %"2.core::fmt::ArgumentV1"* %0, i32 0, i32 1
  %4 = load i8 (i64*, %"2.core::fmt::Formatter"*)*, i8 (i64*, %"2.core::fmt::Formatter"*)** %f, align 8
  %5 = bitcast i8 (i64*, %"2.core::fmt::Formatter"*)* %4 to i8 (%"2.core::fmt::Void"*, %"2.core::fmt::Formatter"*)*
  store i8 (%"2.core::fmt::Void"*, %"2.core::fmt::Formatter"*)* %5, i8 (%"2.core::fmt::Void"*, %"2.core::fmt::Formatter"*)** %3
  %6 = getelementptr inbounds %"2.core::fmt::ArgumentV1", %"2.core::fmt::ArgumentV1"* %0, i32 0, i32 0
  %7 = load i64*, i64** %x, align 8, !nonnull !1
  %8 = bitcast i64* %7 to %"2.core::fmt::Void"*
  store %"2.core::fmt::Void"* %8, %"2.core::fmt::Void"** %6
  ret void
}

declare i8 @"_ZN3fmt3num13_$LT$impl$GT$3fmt20h61c12488bb3d5903AZVE"(i64* noalias readonly dereferenceable(8), %"2.core::fmt::Formatter"* noalias dereferenceable(96)) unnamed_addr

; Function Attrs: inlinehint uwtable
define internal zeroext i1 @"_ZN3num13_$LT$impl$GT$15is_power_of_two20hcabd0a0fb3e6b19ar8iE"(i64) unnamed_addr #1 {
entry-block:
  %self = alloca i64
  %1 = alloca i64
  %2 = alloca i64
  %3 = alloca i64
  %4 = alloca i64
  store i64 %0, i64* %self, align 8
  %5 = load i64, i64* %self, align 8
  %6 = call i64 @"_ZN3num13_$LT$impl$GT$3one20h790da5113c87c0645lhE"()
  store i64 %6, i64* %2, align 8
  %7 = load i64, i64* %2, align 8
  %8 = call i64 @"_ZN3num13_$LT$impl$GT$12wrapping_sub20h362106a5499b8ea7s5iE"(i64 %5, i64 %7)
  store i64 %8, i64* %1, align 8
  %9 = load i64, i64* %1, align 8
  %10 = load i64, i64* %self, align 8
  %11 = and i64 %9, %10
  %12 = call i64 @"_ZN3num13_$LT$impl$GT$4zero20hc4254551c8413675WlhE"()
  store i64 %12, i64* %3, align 8
  %13 = load i64, i64* %3, align 8
  %14 = icmp eq i64 %11, %13
  br i1 %14, label %before_rhs, label %join

join:                                             ; preds = %before_rhs, %entry-block
  %15 = phi i1 [ %14, %entry-block ], [ %22, %before_rhs ]
  %16 = zext i1 %15 to i8
  %17 = trunc i8 %16 to i1
  ret i1 %17

before_rhs:                                       ; preds = %entry-block
  %18 = load i64, i64* %self, align 8
  %19 = call i64 @"_ZN3num13_$LT$impl$GT$4zero20hc4254551c8413675WlhE"()
  store i64 %19, i64* %4, align 8
  %20 = load i64, i64* %4, align 8
  %21 = icmp eq i64 %18, %20
  %22 = xor i1 %21, true
  br label %join
}

; Function Attrs: inlinehint uwtable
define internal i64 @"_ZN3num13_$LT$impl$GT$12wrapping_sub20h362106a5499b8ea7s5iE"(i64, i64) unnamed_addr #1 {
entry-block:
  %self = alloca i64
  %rhs = alloca i64
  store i64 %0, i64* %self, align 8
  store i64 %1, i64* %rhs, align 8
  %2 = load i64, i64* %self, align 8
  %3 = load i64, i64* %rhs, align 8
  %4 = sub i64 %2, %3
  ret i64 %4
}

; Function Attrs: inlinehint uwtable
define internal i64 @"_ZN3num13_$LT$impl$GT$3one20h790da5113c87c0645lhE"() unnamed_addr #1 {
entry-block:
  ret i64 1
}

; Function Attrs: inlinehint uwtable
define internal i64 @"_ZN3num13_$LT$impl$GT$4zero20hc4254551c8413675WlhE"() unnamed_addr #1 {
entry-block:
  ret i64 0
}

; Function Attrs: nounwind
declare noalias i8* @__rust_allocate(i64, i64) unnamed_addr #3

; Function Attrs: inlinehint uwtable
define internal zeroext i1 @"_ZN3ptr13_$LT$impl$GT$7is_null7is_null21h14748829460356738718E"(i8*) unnamed_addr #1 {
entry-block:
  %self = alloca i8*
  store i8* %0, i8** %self, align 8
  %1 = load i8*, i8** %self, align 8
  %2 = icmp eq i8* %1, null
  %3 = zext i1 %2 to i8
  %4 = trunc i8 %3 to i1
  ret i1 %4
}

; Function Attrs: cold noinline noreturn
declare void @_ZN3oom20hd9170b7f1397402fatbE() unnamed_addr #2

declare %"9.my_crate::Foo" @"_ZN13_$LT$impl$GT$3new20h7a0183b89d396887kaaE"() unnamed_addr

attributes #0 = { uwtable }
attributes #1 = { inlinehint uwtable }
attributes #2 = { cold noinline noreturn }
attributes #3 = { nounwind }

!0 = !{i8 0, i8 2}
!1 = !{}
