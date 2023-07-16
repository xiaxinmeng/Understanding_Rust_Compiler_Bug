llvm
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"std::string::String" = type { [0 x i64], %"std::vec::Vec<u8>", [0 x i64] }
%"std::vec::Vec<u8>" = type { [0 x i64], { i8*, i64 }, [0 x i64], i64, [0 x i64] }

@0 = private unnamed_addr constant <{ [16 x i8] }> <{ [16 x i8] c"\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00" }>, align 8

define void @abc(%"std::string::String"* %0) unnamed_addr {
start:
  %1 = load i8*, i8** bitcast (<{ [16 x i8] }>* @0 to i8**), align 8
  %ptr = bitcast %"std::string::String"* %0 to i8**
  store i8* %1, i8** %ptr, align 8
  ret void
}

; This works by replacing the pointer type with an integral type

define void @cba(%"std::string::String"* %0) unnamed_addr {
start:
  %1 = load i64, i64* bitcast (<{ [16 x i8] }>* @0 to i64*), align 8
  %ptr = bitcast %"std::string::String"* %0 to i64*
  store i64 %1, i64* %ptr, align 8
  ret void
}

