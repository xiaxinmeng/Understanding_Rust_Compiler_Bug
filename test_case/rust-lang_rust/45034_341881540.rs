llvm
; ModuleID = 'test.cgu-0.rs'
source_filename = "test.cgu-0.rs"
target datalayout = "e-m:x-p:32:32-i64:64-f80:32-n8:16:32-a:0:32-S32"
target triple = "i686-pc-windows-msvc"

%str_slice = type { i8*, i32 }
%"std::io::error::Error" = type { %"std::io::error::Repr", [0 x i8] }
%"std::io::error::Repr" = type { i8, [3 x i8], [1 x i32] }
%"std::io::error::Custom" = type { i8, [3 x i8], { i8*, void (i8*)** }, [0 x i8] }
%"alloc::vec::Vec<u8>" = type { %"alloc::raw_vec::RawVec<u8, alloc::heap::Heap>", [0 x i8], i32, [0 x i8] }
%"alloc::raw_vec::RawVec<u8, alloc::heap::Heap>" = type { %"core::ptr::Unique<u8>", [0 x i8], i32, [0 x i8], %"alloc::heap::Heap", [0 x i8] }
%"core::ptr::Unique<u8>" = type { %"core::nonzero::NonZero<*const u8>", [0 x i8], %"core::marker::PhantomData<u8>", [0 x i8] }
%"core::nonzero::NonZero<*const u8>" = type { i8*, [0 x i8] }
%"core::marker::PhantomData<u8>" = type {}
%"alloc::heap::Heap" = type {}
%"core::fmt::ArgumentV1" = type { %"core::fmt::Void"*, [0 x i8], i8 (%"core::fmt::Void"*, %"core::fmt::Formatter"*)*, [0 x i8] }
%"core::fmt::Void" = type { {}, [0 x i8] }
%"core::fmt::Formatter" = type { i32, [0 x i8], i32, [0 x i8], %"core::option::Option<usize>", [0 x i8], %"core::option::Option<usize>", [0 x i8], { i8*, void (i8*)** }, [0 x i8], %"core::slice::Iter<core::fmt::ArgumentV1>", [0 x i8], { %"core::fmt::ArgumentV1"*, i32 }, [0 x i8], i8, [3 x i8] }
%"core::option::Option<usize>" = type { i32, [0 x i32], [1 x i32] }
%"core::slice::Iter<core::fmt::ArgumentV1>" = type { %"core::fmt::ArgumentV1"*, [0 x i8], %"core::fmt::ArgumentV1"*, [0 x i8], %"core::marker::PhantomData<&core::fmt::ArgumentV1>", [0 x i8] }
%"core::marker::PhantomData<&core::fmt::ArgumentV1>" = type {}
%"core::fmt::Arguments" = type { { %str_slice*, i32 }, [0 x i8], %"core::option::Option<&[core::fmt::rt::v1::Argument]>", [0 x i8], { %"core::fmt::ArgumentV1"*, i32 }, [0 x i8] }
%"core::option::Option<&[core::fmt::rt::v1::Argument]>" = type { { %"core::fmt::rt::v1::Argument"*, i32 }, [0 x i8] }
%"core::fmt::rt::v1::Argument" = type { %"core::fmt::rt::v1::Position", [0 x i8], %"core::fmt::rt::v1::FormatSpec", [0 x i8] }
%"core::fmt::rt::v1::Position" = type { i32, [0 x i32], [1 x i32] }
%"core::fmt::rt::v1::FormatSpec" = type { i32, [0 x i8], i32, [0 x i8], %"core::fmt::rt::v1::Count", [0 x i8], %"core::fmt::rt::v1::Count", [0 x i8], i8, [3 x i8] }
%"core::fmt::rt::v1::Count" = type { i32, [0 x i32], [1 x i32] }
%"std::fs::DirBuilder" = type { %"std::sys::imp::fs::DirBuilder", [0 x i8], i8, [0 x i8] }
%"std::sys::imp::fs::DirBuilder" = type {}
%"core::result::Result<(), std::io::error::Error>" = type { i32, [0 x i32], [2 x i32] }
%"std::ffi::os_str::OsStr" = type { %"std::sys::imp::os_str::Slice" }
%"std::sys::imp::os_str::Slice" = type { %"std::sys_common::wtf8::Wtf8" }
%"std::sys_common::wtf8::Wtf8" = type { i8 }
%"std::path::Path" = type { %"std::ffi::os_str::OsStr" }

@str.0 = internal constant [43 x i8] c"called `Result::unwrap()` on an `Err` value"
@str.1 = internal constant [0 x i8] zeroinitializer
@str.2 = internal constant [2 x i8] c": "
@ref.3 = internal unnamed_addr constant [2 x %str_slice] [%str_slice { i8* getelementptr inbounds ([0 x i8], [0 x i8]* @str.1, i32 0, i32 0), i32 0 }, %str_slice { i8* getelementptr inbounds ([2 x i8], [2 x i8]* @str.2, i32 0, i32 0), i32 2 }], align 4
@str.4 = internal constant [21 x i8] c"src\5Clibcore\5Cresult.rs"
@ref.5 = internal unnamed_addr constant { %str_slice, i32, i32 } { %str_slice { i8* getelementptr inbounds ([21 x i8], [21 x i8]* @str.4, i32 0, i32 0), i32 21 }, i32 906, i32 4 }, align 4
@str.7 = internal constant [1 x i8] c"a"

; core::ptr::drop_in_place
; Function Attrs: minsize optsize uwtable
define internal fastcc void @_ZN4core3ptr13drop_in_place17h98c613591a196845E(%"std::io::error::Error"* nocapture readonly) unnamed_addr #0 personality i32 (...)* @__CxxFrameHandler3 {
start:
  %1 = getelementptr inbounds %"std::io::error::Error", %"std::io::error::Error"* %0, i32 0, i32 0, i32 0
  %2 = load i8, i8* %1, align 1, !range !1
  %switch.i = icmp ult i8 %2, 2
  br i1 %switch.i, label %_ZN4core3ptr13drop_in_place17hee4fa26f1c722335E.exit, label %bb2.i

bb2.i:                                            ; preds = %start
  %3 = getelementptr inbounds %"std::io::error::Error", %"std::io::error::Error"* %0, i32 0, i32 0, i32 2
  %4 = bitcast [1 x i32]* %3 to %"std::io::error::Custom"**
  %5 = load %"std::io::error::Custom"*, %"std::io::error::Custom"** %4, align 4, !nonnull !2
  %6 = getelementptr inbounds %"std::io::error::Custom", %"std::io::error::Custom"* %5, i32 0, i32 2
  %7 = getelementptr inbounds { i8*, void (i8*)** }, { i8*, void (i8*)** }* %6, i32 0, i32 0
  %8 = load i8*, i8** %7, align 4, !nonnull !2
  %9 = getelementptr inbounds %"std::io::error::Custom", %"std::io::error::Custom"* %5, i32 0, i32 2, i32 1
  %10 = load void (i8*)**, void (i8*)*** %9, align 4, !nonnull !2
  %11 = load void (i8*)*, void (i8*)** %10, align 4, !invariant.load !2, !nonnull !2
  invoke void %11(i8* nonnull %8)
          to label %_ZN4core3ptr13drop_in_place17h366e013c72603a51E.exit.i unwind label %funclet_bb4.i.i.i.i

funclet_bb4.i.i.i.i:                              ; preds = %bb2.i
  %cleanuppad.i.i.i.i = cleanuppad within none []
  %12 = bitcast { i8*, void (i8*)** }* %6 to {}**
  %13 = load {}*, {}** %12, align 4, !nonnull !2
  %14 = bitcast void (i8*)*** %9 to {}**
  %15 = load {}*, {}** %14, align 4, !nonnull !2
; call alloc::heap::box_free
  call fastcc void @_ZN5alloc4heap8box_free17h22449e8a34a6f623E({}* nonnull %13, {}* nonnull %15) #9 [ "funclet"(token %cleanuppad.i.i.i.i) ]
  %16 = load %"std::io::error::Custom"*, %"std::io::error::Custom"** %4, align 4, !nonnull !2
; call alloc::heap::box_free
  call fastcc void @_ZN5alloc4heap8box_free17hc9672664816bdccdE(%"std::io::error::Custom"* nonnull %16) #9 [ "funclet"(token %cleanuppad.i.i.i.i) ]
  cleanupret from %cleanuppad.i.i.i.i unwind to caller

_ZN4core3ptr13drop_in_place17h366e013c72603a51E.exit.i: ; preds = %bb2.i
  %17 = bitcast { i8*, void (i8*)** }* %6 to {}**
  %18 = load {}*, {}** %17, align 4, !nonnull !2
  %19 = bitcast void (i8*)*** %9 to {}**
  %20 = load {}*, {}** %19, align 4, !nonnull !2
; call alloc::heap::box_free
  tail call fastcc void @_ZN5alloc4heap8box_free17h22449e8a34a6f623E({}* nonnull %18, {}* nonnull %20)
  %21 = load %"std::io::error::Custom"*, %"std::io::error::Custom"** %4, align 4, !nonnull !2
; call alloc::heap::box_free
  tail call fastcc void @_ZN5alloc4heap8box_free17hc9672664816bdccdE(%"std::io::error::Custom"* nonnull %21)
  br label %_ZN4core3ptr13drop_in_place17hee4fa26f1c722335E.exit

_ZN4core3ptr13drop_in_place17hee4fa26f1c722335E.exit: ; preds = %start, %_ZN4core3ptr13drop_in_place17h366e013c72603a51E.exit.i
  ret void
}

; core::ptr::drop_in_place
; Function Attrs: minsize nounwind optsize uwtable
define internal fastcc void @_ZN4core3ptr13drop_in_place17heb21502d2037c2c4E(%"alloc::vec::Vec<u8>"* nocapture readonly) unnamed_addr #1 personality i32 (...)* @__CxxFrameHandler3 {
bb4:
  %1 = getelementptr inbounds %"alloc::vec::Vec<u8>", %"alloc::vec::Vec<u8>"* %0, i32 0, i32 0, i32 2
  %2 = load i32, i32* %1, align 4, !alias.scope !3, !noalias !6
  %3 = icmp eq i32 %2, 0
  br i1 %3, label %_ZN4core3ptr13drop_in_place17h697c4224b2db9e90E.exit, label %bb6.i.i.i

bb6.i.i.i:                                        ; preds = %bb4
  %4 = getelementptr inbounds %"alloc::vec::Vec<u8>", %"alloc::vec::Vec<u8>"* %0, i32 0, i32 0, i32 0, i32 0, i32 0
  %_3.sroa.0.0.copyload3.i.i.i.i = load i8*, i8** %4, align 4, !alias.scope !8
  tail call void @__rust_dealloc(i8* %_3.sroa.0.0.copyload3.i.i.i.i, i32 %2, i32 1) #8, !noalias !11
  br label %_ZN4core3ptr13drop_in_place17h697c4224b2db9e90E.exit

_ZN4core3ptr13drop_in_place17h697c4224b2db9e90E.exit: ; preds = %bb4, %bb6.i.i.i
  ret void
}

; core::result::unwrap_failed
; Function Attrs: cold minsize noinline noreturn optsize uwtable
define internal fastcc void @_ZN4core6result13unwrap_failed17h8e5b53e5dd2dce6cE(%"std::io::error::Error"* noalias nocapture readonly dereferenceable(8)) unnamed_addr #2 personality i32 (...)* @__CxxFrameHandler3 {
bb5:
  %_12 = alloca [2 x %"core::fmt::ArgumentV1"], align 4
  %_5 = alloca %"core::fmt::Arguments", align 4
  %error = alloca i64, align 4
  %msg = alloca %str_slice, align 4
  %1 = bitcast %str_slice* %msg to i8*
  call void @llvm.lifetime.start(i64 8, i8* nonnull %1)
  %2 = getelementptr inbounds %str_slice, %str_slice* %msg, i32 0, i32 0
  store i8* getelementptr inbounds ([43 x i8], [43 x i8]* @str.0, i32 0, i32 0), i8** %2, align 4
  %3 = getelementptr inbounds %str_slice, %str_slice* %msg, i32 0, i32 1
  store i32 43, i32* %3, align 4
  %4 = bitcast i64* %error to i8*
  call void @llvm.lifetime.start(i64 8, i8* nonnull %4)
  %5 = bitcast %"std::io::error::Error"* %0 to i64*
  %6 = load i64, i64* %5, align 4
  store i64 %6, i64* %error, align 4
  %7 = bitcast %"core::fmt::Arguments"* %_5 to i8*
  call void @llvm.lifetime.start(i64 24, i8* nonnull %7)
  %8 = bitcast [2 x %"core::fmt::ArgumentV1"]* %_12 to i8*
  call void @llvm.lifetime.start(i64 16, i8* nonnull %8)
  %9 = ptrtoint %str_slice* %msg to i32
  %10 = ptrtoint i64* %error to i32
  %11 = bitcast [2 x %"core::fmt::ArgumentV1"]* %_12 to i32*
  store i32 %9, i32* %11, align 4
  %12 = getelementptr inbounds [2 x %"core::fmt::ArgumentV1"], [2 x %"core::fmt::ArgumentV1"]* %_12, i32 0, i32 0, i32 2
  %13 = bitcast i8 (%"core::fmt::Void"*, %"core::fmt::Formatter"*)** %12 to i32*
  store i32 ptrtoint (i8 (%str_slice*, %"core::fmt::Formatter"*)* @"_ZN55_$LT$$RF$$u27$a$u20$T$u20$as$u20$core..fmt..Display$GT$3fmt17hc6c7c1e09ea80fc6E" to i32), i32* %13, align 4
  %14 = getelementptr inbounds [2 x %"core::fmt::ArgumentV1"], [2 x %"core::fmt::ArgumentV1"]* %_12, i32 0, i32 1
  %15 = bitcast %"core::fmt::ArgumentV1"* %14 to i32*
  store i32 %10, i32* %15, align 4
  %16 = getelementptr inbounds [2 x %"core::fmt::ArgumentV1"], [2 x %"core::fmt::ArgumentV1"]* %_12, i32 0, i32 1, i32 2
  %17 = bitcast i8 (%"core::fmt::Void"*, %"core::fmt::Formatter"*)** %16 to i32*
  store i32 ptrtoint (i8 (%"std::io::error::Error"*, %"core::fmt::Formatter"*)* @"_ZN58_$LT$std..io..error..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h1c15ba7dbd8385a7E" to i32), i32* %17, align 4
  %18 = getelementptr inbounds [2 x %"core::fmt::ArgumentV1"], [2 x %"core::fmt::ArgumentV1"]* %_12, i32 0, i32 0
  %19 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_5, i32 0, i32 0, i32 0
  store %str_slice* getelementptr inbounds ([2 x %str_slice], [2 x %str_slice]* @ref.3, i32 0, i32 0), %str_slice** %19, align 4, !alias.scope !14, !noalias !17
  %20 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_5, i32 0, i32 0, i32 1
  store i32 2, i32* %20, align 4, !alias.scope !14, !noalias !17
  %_6.sroa.0.0..sroa_idx.i = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_5, i32 0, i32 2, i32 0, i32 0
  store %"core::fmt::rt::v1::Argument"* null, %"core::fmt::rt::v1::Argument"** %_6.sroa.0.0..sroa_idx.i, align 4, !alias.scope !14, !noalias !17
  %21 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_5, i32 0, i32 4, i32 0
  store %"core::fmt::ArgumentV1"* %18, %"core::fmt::ArgumentV1"** %21, align 4, !alias.scope !14, !noalias !17
  %22 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_5, i32 0, i32 4, i32 1
  store i32 2, i32* %22, align 4, !alias.scope !14, !noalias !17
; invoke core::panicking::panic_fmt
  invoke void @_ZN4core9panicking9panic_fmt17h68ecfc4b67eef9b6E(%"core::fmt::Arguments"* noalias nocapture nonnull dereferenceable(24) %_5, { %str_slice, [0 x i8], i32, [0 x i8], i32, [0 x i8] }* noalias readonly dereferenceable(16) bitcast ({ %str_slice, i32, i32 }* @ref.5 to { %str_slice, [0 x i8], i32, [0 x i8], i32, [0 x i8] }*))
          to label %unreachable unwind label %funclet_bb3

funclet_bb3:                                      ; preds = %bb5
  %cleanuppad = cleanuppad within none []
  %tmpcast = bitcast i64* %error to %"std::io::error::Error"*
; call core::ptr::drop_in_place
  call fastcc void @_ZN4core3ptr13drop_in_place17h98c613591a196845E(%"std::io::error::Error"* nonnull %tmpcast) #9 [ "funclet"(token %cleanuppad) ]
  cleanupret from %cleanuppad unwind to caller

unreachable:                                      ; preds = %bb5
  unreachable
}

; <&'a T as core::fmt::Display>::fmt
; Function Attrs: minsize optsize uwtable
define internal i8 @"_ZN55_$LT$$RF$$u27$a$u20$T$u20$as$u20$core..fmt..Display$GT$3fmt17hc6c7c1e09ea80fc6E"(%str_slice* noalias nocapture readonly dereferenceable(8), %"core::fmt::Formatter"* dereferenceable(52)) unnamed_addr #0 {
start:
  %2 = getelementptr inbounds %str_slice, %str_slice* %0, i32 0, i32 0
  %3 = load i8*, i8** %2, align 4, !nonnull !2
  %4 = getelementptr inbounds %str_slice, %str_slice* %0, i32 0, i32 1
  %5 = load i32, i32* %4, align 4
; call <str as core::fmt::Display>::fmt
  %6 = tail call i8 @"_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17he5ed7498531768bdE"(i8* noalias nonnull readonly %3, i32 %5, %"core::fmt::Formatter"* nonnull dereferenceable(52) %1)
  ret i8 %6
}

; alloc::heap::box_free
; Function Attrs: inlinehint minsize nounwind optsize uwtable
define internal fastcc void @_ZN5alloc4heap8box_free17h22449e8a34a6f623E({}*, {}* nocapture readonly) unnamed_addr #3 {
start:
  %2 = bitcast {}* %1 to i32*
  %3 = getelementptr inbounds i32, i32* %2, i32 1
  %4 = load i32, i32* %3, align 4, !invariant.load !2
  %5 = icmp eq i32 %4, 0
  br i1 %5, label %bb7, label %bb3

bb3:                                              ; preds = %start
  %6 = getelementptr inbounds i32, i32* %2, i32 2
  %7 = load i32, i32* %6, align 4, !invariant.load !2
  %8 = bitcast {}* %0 to i8*
  tail call void @__rust_dealloc(i8* %8, i32 %4, i32 %7) #8, !noalias !19
  br label %bb7

bb7:                                              ; preds = %start, %bb3
  ret void
}

; alloc::heap::box_free
; Function Attrs: inlinehint minsize nounwind optsize uwtable
define internal fastcc void @_ZN5alloc4heap8box_free17hc9672664816bdccdE(%"std::io::error::Custom"*) unnamed_addr #3 {
start:
  %1 = getelementptr inbounds %"std::io::error::Custom", %"std::io::error::Custom"* %0, i32 0, i32 0
  tail call void @__rust_dealloc(i8* %1, i32 12, i32 4) #8, !noalias !22
  ret void
}

; test::main
; Function Attrs: minsize optsize uwtable
define internal void @_ZN4test4main17h7dd2d5a121102563E() unnamed_addr #0 personality i32 (...)* @__CxxFrameHandler3 {
start:
  %_9.i = alloca i64, align 4
  %_4.i = alloca %"std::fs::DirBuilder", align 4
  %_3 = alloca %"core::result::Result<(), std::io::error::Error>", align 4
  %_vec = alloca %"alloc::vec::Vec<u8>", align 4
  %0 = bitcast %"alloc::vec::Vec<u8>"* %_vec to i8*
  call void @llvm.lifetime.start(i64 12, i8* nonnull %0)
  %.sroa_cast.i = bitcast %"alloc::vec::Vec<u8>"* %_vec to i32*
  store i32 1, i32* %.sroa_cast.i, align 4, !alias.scope !25
  %.sroa_idx.i = getelementptr inbounds %"alloc::vec::Vec<u8>", %"alloc::vec::Vec<u8>"* %_vec, i32 0, i32 0, i32 2
  store i32 0, i32* %.sroa_idx.i, align 4, !alias.scope !25
  %1 = getelementptr inbounds %"alloc::vec::Vec<u8>", %"alloc::vec::Vec<u8>"* %_vec, i32 0, i32 2
  store i32 0, i32* %1, align 4, !alias.scope !25
  %2 = bitcast %"core::result::Result<(), std::io::error::Error>"* %_3 to i8*
  call void @llvm.lifetime.start(i64 12, i8* nonnull %2)
; invoke std::ffi::os_str::<impl core::convert::AsRef<std::ffi::os_str::OsStr> for str>::as_ref
  %3 = invoke { %"std::ffi::os_str::OsStr"*, i32 } @"_ZN3std3ffi6os_str85_$LT$impl$u20$core..convert..AsRef$LT$std..ffi..os_str..OsStr$GT$$u20$for$u20$str$GT$6as_ref17hdf08e315569900f3E"(i8* noalias nonnull readonly getelementptr inbounds ([1 x i8], [1 x i8]* @str.7, i32 0, i32 0), i32 1)
          to label %bb2 unwind label %funclet_bb4

bb2:                                              ; preds = %start
  %4 = extractvalue { %"std::ffi::os_str::OsStr"*, i32 } %3, 0
  %5 = icmp ne %"std::ffi::os_str::OsStr"* %4, null
  tail call void @llvm.assume(i1 %5)
  %6 = bitcast %"std::fs::DirBuilder"* %_4.i to i8*
  call void @llvm.lifetime.start(i64 1, i8* nonnull %6), !noalias !28
; invoke std::fs::DirBuilder::new
  %7 = invoke i8 @_ZN3std2fs10DirBuilder3new17hef1ff9c701332e7fE()
          to label %.noexc unwind label %funclet_bb4

.noexc:                                           ; preds = %bb2
  %8 = getelementptr inbounds %"std::ffi::os_str::OsStr", %"std::ffi::os_str::OsStr"* %4, i32 0, i32 0, i32 0, i32 0
  %9 = extractvalue { %"std::ffi::os_str::OsStr"*, i32 } %3, 1
  store i8 %7, i8* %6, align 4, !noalias !28
; invoke <std::path::Path as core::convert::AsRef<std::path::Path>>::as_ref
  %10 = invoke { %"std::path::Path"*, i32 } @"_ZN79_$LT$std..path..Path$u20$as$u20$core..convert..AsRef$LT$std..path..Path$GT$$GT$6as_ref17heea68d994f9ec736E"(i8* noalias nonnull readonly %8, i32 %9)
          to label %.noexc1 unwind label %funclet_bb4

.noexc1:                                          ; preds = %.noexc
  %11 = extractvalue { %"std::path::Path"*, i32 } %10, 0
  %12 = extractvalue { %"std::path::Path"*, i32 } %10, 1
  %13 = getelementptr inbounds %"std::path::Path", %"std::path::Path"* %11, i32 0, i32 0, i32 0, i32 0, i32 0
; invoke <std::path::Path as core::convert::AsRef<std::path::Path>>::as_ref
  %14 = invoke { %"std::path::Path"*, i32 } @"_ZN79_$LT$std..path..Path$u20$as$u20$core..convert..AsRef$LT$std..path..Path$GT$$GT$6as_ref17heea68d994f9ec736E"(i8* noalias nonnull readonly %13, i32 %12)
          to label %.noexc2 unwind label %funclet_bb4

.noexc2:                                          ; preds = %.noexc1
  %15 = extractvalue { %"std::path::Path"*, i32 } %14, 0
  %16 = extractvalue { %"std::path::Path"*, i32 } %14, 1
  %17 = getelementptr inbounds %"std::path::Path", %"std::path::Path"* %15, i32 0, i32 0, i32 0, i32 0, i32 0
; invoke std::fs::DirBuilder::_create
  invoke void @_ZN3std2fs10DirBuilder7_create17h1bf27d7f4df68242E(%"core::result::Result<(), std::io::error::Error>"* noalias nocapture nonnull sret dereferenceable(12) %_3, %"std::fs::DirBuilder"* noalias nonnull readonly dereferenceable(1) %_4.i, i8* noalias nonnull readonly %17, i32 %16)
          to label %bb5 unwind label %funclet_bb4

bb5:                                              ; preds = %.noexc2
  call void @llvm.lifetime.end(i64 1, i8* nonnull %6), !noalias !28
  %self.i.sroa.0.0..sroa_idx = getelementptr inbounds %"core::result::Result<(), std::io::error::Error>", %"core::result::Result<(), std::io::error::Error>"* %_3, i32 0, i32 0
  %self.i.sroa.0.0.copyload = load i32, i32* %self.i.sroa.0.0..sroa_idx, align 4
  %cond.i = icmp eq i32 %self.i.sroa.0.0.copyload, 0
  br i1 %cond.i, label %bb6, label %bb2.i

bb2.i:                                            ; preds = %bb5
  %self.i.sroa.4.0..sroa_idx6 = getelementptr inbounds %"core::result::Result<(), std::io::error::Error>", %"core::result::Result<(), std::io::error::Error>"* %_3, i32 0, i32 2
  %self.i.sroa.4.0..sroa_cast7 = bitcast [2 x i32]* %self.i.sroa.4.0..sroa_idx6 to i64*
  %self.i.sroa.4.0.copyload = load i64, i64* %self.i.sroa.4.0..sroa_cast7, align 4
  %tmpcast10.i = bitcast i64* %_9.i to %"std::io::error::Error"*
  %18 = bitcast i64* %_9.i to i8*
  call void @llvm.lifetime.start(i64 8, i8* nonnull %18), !noalias !32
  store i64 %self.i.sroa.4.0.copyload, i64* %_9.i, align 4, !noalias !32
; invoke core::result::unwrap_failed
  invoke fastcc void @_ZN4core6result13unwrap_failed17h8e5b53e5dd2dce6cE(%"std::io::error::Error"* noalias nocapture nonnull dereferenceable(8) %tmpcast10.i)
          to label %.noexc4 unwind label %funclet_bb4

.noexc4:                                          ; preds = %bb2.i
  unreachable

bb6:                                              ; preds = %bb5
  call void @llvm.lifetime.end(i64 12, i8* nonnull %2)
; call core::ptr::drop_in_place
  call fastcc void @_ZN4core3ptr13drop_in_place17heb21502d2037c2c4E(%"alloc::vec::Vec<u8>"* nonnull %_vec)
  call void @llvm.lifetime.end(i64 12, i8* nonnull %0)
  ret void

funclet_bb4:                                      ; preds = %bb2.i, %.noexc2, %.noexc1, %.noexc, %bb2, %start
  %cleanuppad = cleanuppad within none []
; call core::ptr::drop_in_place
  call fastcc void @_ZN4core3ptr13drop_in_place17heb21502d2037c2c4E(%"alloc::vec::Vec<u8>"* nonnull %_vec) #9 [ "funclet"(token %cleanuppad) ]
  cleanupret from %cleanuppad unwind to caller
}

; Function Attrs: argmemonly nounwind
declare void @llvm.lifetime.start(i64, i8* nocapture) #4

; Function Attrs: argmemonly nounwind
declare void @llvm.lifetime.end(i64, i8* nocapture) #4

; Function Attrs: minsize optsize
declare i32 @__CxxFrameHandler3(...) unnamed_addr #5

; std::fs::DirBuilder::_create
; Function Attrs: minsize optsize
declare void @_ZN3std2fs10DirBuilder7_create17h1bf27d7f4df68242E(%"core::result::Result<(), std::io::error::Error>"* noalias nocapture sret dereferenceable(12), %"std::fs::DirBuilder"* noalias readonly dereferenceable(1), i8* noalias nonnull readonly, i32) unnamed_addr #5

; std::fs::DirBuilder::new
; Function Attrs: minsize optsize
declare i8 @_ZN3std2fs10DirBuilder3new17hef1ff9c701332e7fE() unnamed_addr #5

; std::ffi::os_str::<impl core::convert::AsRef<std::ffi::os_str::OsStr> for str>::as_ref
; Function Attrs: minsize optsize
declare { %"std::ffi::os_str::OsStr"*, i32 } @"_ZN3std3ffi6os_str85_$LT$impl$u20$core..convert..AsRef$LT$std..ffi..os_str..OsStr$GT$$u20$for$u20$str$GT$6as_ref17hdf08e315569900f3E"(i8* noalias nonnull readonly, i32) unnamed_addr #5

; <std::io::error::Error as core::fmt::Debug>::fmt
; Function Attrs: minsize optsize
declare i8 @"_ZN58_$LT$std..io..error..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h1c15ba7dbd8385a7E"(%"std::io::error::Error"* noalias readonly dereferenceable(8), %"core::fmt::Formatter"* dereferenceable(52)) unnamed_addr #5

; core::panicking::panic_fmt
; Function Attrs: cold minsize noinline noreturn optsize
declare void @_ZN4core9panicking9panic_fmt17h68ecfc4b67eef9b6E(%"core::fmt::Arguments"* noalias nocapture dereferenceable(24), { %str_slice, [0 x i8], i32, [0 x i8], i32, [0 x i8] }* noalias readonly dereferenceable(16)) unnamed_addr #6

; <str as core::fmt::Display>::fmt
; Function Attrs: minsize optsize
declare i8 @"_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17he5ed7498531768bdE"(i8* noalias nonnull readonly, i32, %"core::fmt::Formatter"* dereferenceable(52)) unnamed_addr #5

; Function Attrs: minsize nounwind optsize
declare void @__rust_dealloc(i8*, i32, i32) unnamed_addr #7

; <std::path::Path as core::convert::AsRef<std::path::Path>>::as_ref
; Function Attrs: minsize optsize
declare { %"std::path::Path"*, i32 } @"_ZN79_$LT$std..path..Path$u20$as$u20$core..convert..AsRef$LT$std..path..Path$GT$$GT$6as_ref17heea68d994f9ec736E"(i8* noalias nonnull readonly, i32) unnamed_addr #5

; Function Attrs: nounwind
declare void @llvm.assume(i1) #8

; Function Attrs: minsize optsize
define i32 @main(i32, i8**) unnamed_addr #5 {
top:
; call std::rt::lang_start
  %2 = tail call i32 @_ZN3std2rt10lang_start17hf7da77336d035e3eE(void ()* nonnull @_ZN4test4main17h7dd2d5a121102563E, i32 %0, i8** %1)
  ret i32 %2
}

; std::rt::lang_start
; Function Attrs: minsize optsize
declare i32 @_ZN3std2rt10lang_start17hf7da77336d035e3eE(void ()*, i32, i8**) unnamed_addr #5

attributes #0 = { minsize optsize uwtable }
attributes #1 = { minsize nounwind optsize uwtable }
attributes #2 = { cold minsize noinline noreturn optsize uwtable }
attributes #3 = { inlinehint minsize nounwind optsize uwtable }
attributes #4 = { argmemonly nounwind }
attributes #5 = { minsize optsize }
attributes #6 = { cold minsize noinline noreturn optsize }
attributes #7 = { minsize nounwind optsize }
attributes #8 = { nounwind }
attributes #9 = { noinline }

!llvm.module.flags = !{!0}

!0 = !{i32 1, !"PIE Level", i32 2}
!1 = !{i8 0, i8 3}
!2 = !{}
!3 = !{!4}
!4 = distinct !{!4, !5, !"_ZN49_$LT$alloc..raw_vec..RawVec$LT$T$C$$u20$A$GT$$GT$14current_layout17hb4938f5273136433E: argument 1"}
!5 = distinct !{!5, !"_ZN49_$LT$alloc..raw_vec..RawVec$LT$T$C$$u20$A$GT$$GT$14current_layout17hb4938f5273136433E"}
!6 = !{!7}
!7 = distinct !{!7, !5, !"_ZN49_$LT$alloc..raw_vec..RawVec$LT$T$C$$u20$A$GT$$GT$14current_layout17hb4938f5273136433E: argument 0"}
!8 = !{!9}
!9 = distinct !{!9, !10, !"_ZN49_$LT$alloc..raw_vec..RawVec$LT$T$C$$u20$A$GT$$GT$3ptr17hc1b2028d4e7e3ff7E: argument 0"}
!10 = distinct !{!10, !"_ZN49_$LT$alloc..raw_vec..RawVec$LT$T$C$$u20$A$GT$$GT$3ptr17hc1b2028d4e7e3ff7E"}
!11 = !{!12}
!12 = distinct !{!12, !13, !"_ZN61_$LT$alloc..heap..Heap$u20$as$u20$alloc..allocator..Alloc$GT$7dealloc17h1043c71d85f57bbaE: argument 0"}
!13 = distinct !{!13, !"_ZN61_$LT$alloc..heap..Heap$u20$as$u20$alloc..allocator..Alloc$GT$7dealloc17h1043c71d85f57bbaE"}
!14 = !{!15}
!15 = distinct !{!15, !16, !"_ZN4core3fmt9Arguments6new_v117hee3a8e38dd625e6fE: argument 0"}
!16 = distinct !{!16, !"_ZN4core3fmt9Arguments6new_v117hee3a8e38dd625e6fE"}
!17 = !{!18}
!18 = distinct !{!18, !16, !"_ZN4core3fmt9Arguments6new_v117hee3a8e38dd625e6fE: argument 1"}
!19 = !{!20}
!20 = distinct !{!20, !21, !"_ZN61_$LT$alloc..heap..Heap$u20$as$u20$alloc..allocator..Alloc$GT$7dealloc17h1043c71d85f57bbaE: argument 0"}
!21 = distinct !{!21, !"_ZN61_$LT$alloc..heap..Heap$u20$as$u20$alloc..allocator..Alloc$GT$7dealloc17h1043c71d85f57bbaE"}
!22 = !{!23}
!23 = distinct !{!23, !24, !"_ZN61_$LT$alloc..heap..Heap$u20$as$u20$alloc..allocator..Alloc$GT$7dealloc17h1043c71d85f57bbaE: argument 0"}
!24 = distinct !{!24, !"_ZN61_$LT$alloc..heap..Heap$u20$as$u20$alloc..allocator..Alloc$GT$7dealloc17h1043c71d85f57bbaE"}
!25 = !{!26}
!26 = distinct !{!26, !27, !"_ZN33_$LT$alloc..vec..Vec$LT$T$GT$$GT$3new17h1513d4ae52d82779E: argument 0"}
!27 = distinct !{!27, !"_ZN33_$LT$alloc..vec..Vec$LT$T$GT$$GT$3new17h1513d4ae52d82779E"}
!28 = !{!29, !31}
!29 = distinct !{!29, !30, !"_ZN3std2fs10create_dir17hf8b1bac220fca5b5E: argument 0"}
!30 = distinct !{!30, !"_ZN3std2fs10create_dir17hf8b1bac220fca5b5E"}
!31 = distinct !{!31, !30, !"_ZN3std2fs10create_dir17hf8b1bac220fca5b5E: argument 1"}
!32 = !{!33}
!33 = distinct !{!33, !34, !"_ZN47_$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$6unwrap17h91690cf7c1f7441fE: argument 0"}
!34 = distinct !{!34, !"_ZN47_$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$6unwrap17h91690cf7c1f7441fE"}
