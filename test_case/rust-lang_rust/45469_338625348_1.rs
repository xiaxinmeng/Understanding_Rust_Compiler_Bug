llvm
; ModuleID = '/tmp/cargo-incr-workdir/incr-evacuated/regex-1swkmi2uqg1oi/s-euq7nl90oz-1xvesab-2wo0aafxuj664/cgu-regex-literals.volatile.bc'
source_filename = "regex-literals.volatile-4bdbb09c6b6a4ee0f854d53a30967c07.rs"
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%str_slice = type { i8*, i64 }
%"literals::SingleSearch" = type { %"alloc::vec::Vec<u8>", [0 x i8], i64, [0 x i8], i64, [0 x i8], i64, [0 x i8], i8, [0 x i8], i8, [6 x i8] }
%"alloc::vec::Vec<u8>" = type { %"alloc::raw_vec::RawVec<u8, alloc::heap::Heap>", [0 x i8], i64, [0 x i8] }
%"alloc::raw_vec::RawVec<u8, alloc::heap::Heap>" = type { %"core::ptr::Unique<u8>", [0 x i8], i64, [0 x i8], %"alloc::heap::Heap", [0 x i8] }
%"core::ptr::Unique<u8>" = type { %"core::nonzero::NonZero<*const u8>", [0 x i8], %"core::marker::PhantomData<u8>", [0 x i8] }
%"core::nonzero::NonZero<*const u8>" = type { i8*, [0 x i8] }
%"core::marker::PhantomData<u8>" = type {}
%"alloc::heap::Heap" = type {}
%"core::fmt::Formatter" = type { %"core::option::Option<usize>", [0 x i8], %"core::option::Option<usize>", [0 x i8], { i8*, void (i8*)** }, [0 x i8], %"core::slice::Iter<core::fmt::ArgumentV1>", [0 x i8], { %"core::fmt::ArgumentV1"*, i64 }, [0 x i8], i32, [0 x i8], i32, [0 x i8], i8, [7 x i8] }
%"core::option::Option<usize>" = type { i64, [0 x i64], [1 x i64] }
%"core::slice::Iter<core::fmt::ArgumentV1>" = type { %"core::fmt::ArgumentV1"*, [0 x i8], %"core::fmt::ArgumentV1"*, [0 x i8], %"core::marker::PhantomData<&core::fmt::ArgumentV1>", [0 x i8] }
%"core::fmt::ArgumentV1" = type { %"core::fmt::Void"*, [0 x i8], i8 (%"core::fmt::Void"*, %"core::fmt::Formatter"*)*, [0 x i8] }
%"core::fmt::Void" = type { {}, [0 x i8] }
%"core::marker::PhantomData<&core::fmt::ArgumentV1>" = type {}
%"core::result::Result<(), core::fmt::Error>" = type { i8, [0 x i8], [0 x i8] }
%"literals::Matcher" = type { i64, [0 x i64], [12 x i64] }
%"literals::SingleByteSet" = type { %"alloc::vec::Vec<bool>", [0 x i8], %"alloc::vec::Vec<u8>", [0 x i8], i8, [7 x i8] }
%"alloc::vec::Vec<bool>" = type { %"alloc::raw_vec::RawVec<bool, alloc::heap::Heap>", [0 x i8], i64, [0 x i8] }
%"alloc::raw_vec::RawVec<bool, alloc::heap::Heap>" = type { %"core::ptr::Unique<bool>", [0 x i8], i64, [0 x i8], %"alloc::heap::Heap", [0 x i8] }
%"core::ptr::Unique<bool>" = type { %"core::nonzero::NonZero<*const bool>", [0 x i8], %"core::marker::PhantomData<bool>", [0 x i8] }
%"core::nonzero::NonZero<*const bool>" = type { i8*, [0 x i8] }
%"core::marker::PhantomData<bool>" = type {}
%"literals::LiteralSearcher" = type { %"literals::SingleSearch", [0 x i8], %"literals::SingleSearch", [0 x i8], %"literals::Matcher", [0 x i8], i8, [7 x i8] }
%"literals::LiteralIter" = type { i64, [0 x i64], [2 x i64] }
%closure.1 = type { %"literals::SingleSearch"**, [0 x i8] }
%closure = type { i8*, [0 x i8] }
%closure.0 = type { i8*, [0 x i8] }

@str.0 = internal constant [15 x i8] c"src/literals.rs"
@str.1 = internal constant [28 x i8] c"attempt to add with overflow"
@panic_loc.2 = internal unnamed_addr constant { %str_slice, %str_slice, i32, i32 } { %str_slice { i8* getelementptr inbounds ([28 x i8], [28 x i8]* @str.1, i32 0, i32 0), i64 28 }, %str_slice { i8* getelementptr inbounds ([15 x i8], [15 x i8]* @str.0, i32 0, i32 0), i64 15 }, i32 104, i32 59 }, align 8

; Function Attrs: uwtable
define hidden i8 @"_ZN53_$LT$$RF$$u27$a$u20$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h4f9a8251953d8bc5E"(%"literals::SingleSearch"** noalias readonly dereferenceable(8), %"core::fmt::Formatter"* dereferenceable(96)) unnamed_addr #0 {
start:
  %abi_cast = alloca i8
  %_0 = alloca %"core::result::Result<(), core::fmt::Error>"
  %2 = load %"literals::SingleSearch"*, %"literals::SingleSearch"** %0, !nonnull !0
  %3 = call i8 @"_ZN66_$LT$regex..literals..SingleSearch$u20$as$u20$core..fmt..Debug$GT$3fmt17h6f635f18f5162119E"(%"literals::SingleSearch"* noalias readonly dereferenceable(56) %2, %"core::fmt::Formatter"* dereferenceable(96) %1)
  store i8 %3, i8* %abi_cast
  %4 = bitcast %"core::result::Result<(), core::fmt::Error>"* %_0 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %4, i8* %abi_cast, i64 1, i32 1, i1 false)
  br label %bb1

bb1:                                              ; preds = %start
  %5 = bitcast %"core::result::Result<(), core::fmt::Error>"* %_0 to i8*
  %6 = load i8, i8* %5, align 1
  ret i8 %6
}

; Function Attrs: uwtable
define hidden i8 @"_ZN53_$LT$$RF$$u27$a$u20$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h89cad6f5345ba815E"(%"literals::Matcher"** noalias readonly dereferenceable(8), %"core::fmt::Formatter"* dereferenceable(96)) unnamed_addr #0 {
start:
  %abi_cast = alloca i8
  %_0 = alloca %"core::result::Result<(), core::fmt::Error>"
  %2 = load %"literals::Matcher"*, %"literals::Matcher"** %0, !nonnull !0
  %3 = call i8 @"_ZN61_$LT$regex..literals..Matcher$u20$as$u20$core..fmt..Debug$GT$3fmt17hd34c150aff41902eE"(%"literals::Matcher"* noalias readonly dereferenceable(104) %2, %"core::fmt::Formatter"* dereferenceable(96) %1)
  store i8 %3, i8* %abi_cast
  %4 = bitcast %"core::result::Result<(), core::fmt::Error>"* %_0 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %4, i8* %abi_cast, i64 1, i32 1, i1 false)
  br label %bb1

bb1:                                              ; preds = %start
  %5 = bitcast %"core::result::Result<(), core::fmt::Error>"* %_0 to i8*
  %6 = load i8, i8* %5, align 1
  ret i8 %6
}

; Function Attrs: uwtable
define hidden i8 @"_ZN53_$LT$$RF$$u27$a$u20$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h97dd76aa1e7521a3E"(%"literals::SingleByteSet"** noalias readonly dereferenceable(8), %"core::fmt::Formatter"* dereferenceable(96)) unnamed_addr #0 {
start:
  %abi_cast = alloca i8
  %_0 = alloca %"core::result::Result<(), core::fmt::Error>"
  %2 = load %"literals::SingleByteSet"*, %"literals::SingleByteSet"** %0, !nonnull !0
  %3 = call i8 @"_ZN67_$LT$regex..literals..SingleByteSet$u20$as$u20$core..fmt..Debug$GT$3fmt17h18e05412653f2485E"(%"literals::SingleByteSet"* noalias readonly dereferenceable(56) %2, %"core::fmt::Formatter"* dereferenceable(96) %1)
  store i8 %3, i8* %abi_cast
  %4 = bitcast %"core::result::Result<(), core::fmt::Error>"* %_0 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %4, i8* %abi_cast, i64 1, i32 1, i1 false)
  br label %bb1

bb1:                                              ; preds = %start
  %5 = bitcast %"core::result::Result<(), core::fmt::Error>"* %_0 to i8*
  %6 = load i8, i8* %5, align 1
  ret i8 %6
}

; Function Attrs: uwtable
define hidden i8 @"_ZN53_$LT$$RF$$u27$a$u20$T$u20$as$u20$core..fmt..Debug$GT$3fmt17haddc4a3305db6d13E"(%"literals::LiteralSearcher"** noalias readonly dereferenceable(8), %"core::fmt::Formatter"* dereferenceable(96)) unnamed_addr #0 {
start:
  %abi_cast = alloca i8
  %_0 = alloca %"core::result::Result<(), core::fmt::Error>"
  %2 = load %"literals::LiteralSearcher"*, %"literals::LiteralSearcher"** %0, !nonnull !0
  %3 = call i8 @"_ZN69_$LT$regex..literals..LiteralSearcher$u20$as$u20$core..fmt..Debug$GT$3fmt17hf410c8f8ba753d00E"(%"literals::LiteralSearcher"* noalias readonly dereferenceable(224) %2, %"core::fmt::Formatter"* dereferenceable(96) %1)
  store i8 %3, i8* %abi_cast
  %4 = bitcast %"core::result::Result<(), core::fmt::Error>"* %_0 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %4, i8* %abi_cast, i64 1, i32 1, i1 false)
  br label %bb1

bb1:                                              ; preds = %start
  %5 = bitcast %"core::result::Result<(), core::fmt::Error>"* %_0 to i8*
  %6 = load i8, i8* %5, align 1
  ret i8 %6
}

; Function Attrs: uwtable
define hidden void @"_ZN54_$LT$I$u20$as$u20$core..iter..traits..IntoIterator$GT$9into_iter17hae67dd3e34ef9bd7E"(%"literals::LiteralIter"* noalias nocapture sret dereferenceable(24), %"literals::LiteralIter"* noalias nocapture dereferenceable(24)) unnamed_addr #0 {
start:
  %_3 = alloca %"literals::LiteralIter"
  %self = alloca %"literals::LiteralIter"
  %2 = bitcast %"literals::LiteralIter"* %1 to i8*
  %3 = bitcast %"literals::LiteralIter"* %self to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %3, i8* %2, i64 24, i32 8, i1 false)
  %4 = bitcast %"literals::LiteralIter"* %self to i8*
  %5 = bitcast %"literals::LiteralIter"* %_3 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %5, i8* %4, i64 24, i32 8, i1 false)
  %6 = bitcast %"literals::LiteralIter"* %_3 to i8*
  %7 = bitcast %"literals::LiteralIter"* %0 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %7, i8* %6, i64 24, i32 8, i1 false)
  ret void
}

; Function Attrs: uwtable
define hidden void @"_ZN5regex8literals15LiteralSearcher4find28_$u7b$$u7b$closure$u7d$$u7d$17ha1270b7a84f05815E"({ i64, [0 x i8], i64, [0 x i8] }* noalias nocapture sret dereferenceable(16), i64, i64) unnamed_addr #0 {
start:
  %abi_cast = alloca i64
  %arg0 = alloca %closure.1
  store i64 %1, i64* %abi_cast
  %3 = bitcast %closure.1* %arg0 to i8*
  %4 = bitcast i64* %abi_cast to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %3, i8* %4, i64 8, i32 8, i1 false)
  %5 = getelementptr inbounds %closure.1, %closure.1* %arg0, i32 0, i32 0
  %6 = load %"literals::SingleSearch"**, %"literals::SingleSearch"*** %5, !nonnull !0
  %7 = load %"literals::SingleSearch"*, %"literals::SingleSearch"** %6, !nonnull !0
  %8 = call i64 @_ZN5regex8literals12SingleSearch3len17hc5d27086c6d5a41dE(%"literals::SingleSearch"* noalias readonly dereferenceable(56) %7)
  br label %bb1

bb1:                                              ; preds = %start
  %9 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %2, i64 %8)
  %10 = extractvalue { i64, i1 } %9, 0
  %11 = extractvalue { i64, i1 } %9, 1
  %12 = call i1 @llvm.expect.i1(i1 %11, i1 false)
  br i1 %12, label %panic, label %bb2

bb2:                                              ; preds = %bb1
  %13 = getelementptr inbounds { i64, [0 x i8], i64, [0 x i8] }, { i64, [0 x i8], i64, [0 x i8] }* %0, i32 0, i32 0
  store i64 %2, i64* %13
  %14 = getelementptr inbounds { i64, [0 x i8], i64, [0 x i8] }, { i64, [0 x i8], i64, [0 x i8] }* %0, i32 0, i32 2
  store i64 %10, i64* %14
  ret void

panic:                                            ; preds = %bb1
  call void @_ZN4core9panicking5panic17he92b0c828833c113E({ %str_slice, [0 x i8], %str_slice, [0 x i8], i32, [0 x i8], i32, [0 x i8] }* noalias readonly dereferenceable(40) bitcast ({ %str_slice, %str_slice, i32, i32 }* @panic_loc.2 to { %str_slice, [0 x i8], %str_slice, [0 x i8], i32, [0 x i8], i32, [0 x i8] }*))
  unreachable
}

; Function Attrs: uwtable
define hidden zeroext i1 @"_ZN5regex8literals12SingleSearch3new28_$u7b$$u7b$closure$u7d$$u7d$17he2994c84694e4a6cE"(%closure* dereferenceable(8), i8* noalias readonly dereferenceable(1)) unnamed_addr #0 {
start:
  %2 = load i8, i8* %1
  %3 = getelementptr inbounds %closure, %closure* %0, i32 0, i32 0
  %4 = load i8*, i8** %3, !nonnull !0
  %5 = load i8, i8* %4
  %6 = icmp eq i8 %2, %5
  ret i1 %6
}

; Function Attrs: uwtable
define hidden zeroext i1 @"_ZN5regex8literals12SingleSearch3new28_$u7b$$u7b$closure$u7d$$u7d$17h74bced1bd365f1a6E"(%closure.0* dereferenceable(8), i8* noalias readonly dereferenceable(1)) unnamed_addr #0 {
start:
  %2 = load i8, i8* %1
  %3 = getelementptr inbounds %closure.0, %closure.0* %0, i32 0, i32 0
  %4 = load i8*, i8** %3, !nonnull !0
  %5 = load i8, i8* %4
  %6 = icmp eq i8 %2, %5
  ret i1 %6
}

declare i8 @"_ZN66_$LT$regex..literals..SingleSearch$u20$as$u20$core..fmt..Debug$GT$3fmt17h6f635f18f5162119E"(%"literals::SingleSearch"* noalias readonly dereferenceable(56), %"core::fmt::Formatter"* dereferenceable(96)) unnamed_addr #1

; Function Attrs: argmemonly nounwind
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* nocapture writeonly, i8* nocapture readonly, i64, i32, i1) #2

declare i8 @"_ZN61_$LT$regex..literals..Matcher$u20$as$u20$core..fmt..Debug$GT$3fmt17hd34c150aff41902eE"(%"literals::Matcher"* noalias readonly dereferenceable(104), %"core::fmt::Formatter"* dereferenceable(96)) unnamed_addr #1

declare i8 @"_ZN67_$LT$regex..literals..SingleByteSet$u20$as$u20$core..fmt..Debug$GT$3fmt17h18e05412653f2485E"(%"literals::SingleByteSet"* noalias readonly dereferenceable(56), %"core::fmt::Formatter"* dereferenceable(96)) unnamed_addr #1

declare i8 @"_ZN69_$LT$regex..literals..LiteralSearcher$u20$as$u20$core..fmt..Debug$GT$3fmt17hf410c8f8ba753d00E"(%"literals::LiteralSearcher"* noalias readonly dereferenceable(224), %"core::fmt::Formatter"* dereferenceable(96)) unnamed_addr #1

declare i64 @_ZN5regex8literals12SingleSearch3len17hc5d27086c6d5a41dE(%"literals::SingleSearch"* noalias readonly dereferenceable(56)) unnamed_addr #1

; Function Attrs: nounwind readnone
declare { i64, i1 } @llvm.uadd.with.overflow.i64(i64, i64) #3

; Function Attrs: nounwind readnone
declare i1 @llvm.expect.i1(i1, i1) #3

; Function Attrs: cold noinline noreturn
declare void @_ZN4core9panicking5panic17he92b0c828833c113E({ %str_slice, [0 x i8], %str_slice, [0 x i8], i32, [0 x i8], i32, [0 x i8] }* noalias readonly dereferenceable(40)) unnamed_addr #4

attributes #0 = { uwtable "probe-stack"="__rust_probestack" }
attributes #1 = { "probe-stack"="__rust_probestack" }
attributes #2 = { argmemonly nounwind }
attributes #3 = { nounwind readnone }
attributes #4 = { cold noinline noreturn "probe-stack"="__rust_probestack" }

!0 = !{}
