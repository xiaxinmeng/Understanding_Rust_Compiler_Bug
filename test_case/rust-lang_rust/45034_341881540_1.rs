llvm
; ModuleID = 'test0-317d481089b8c8fe83113de504472633.rs'
source_filename = "test0-317d481089b8c8fe83113de504472633.rs"
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
define internal fastcc void @_ZN4core3ptr13drop_in_place17h0b2d9bad824fa3b0E(%"std::io::error::Error"* nocapture readonly %arg0) unnamed_addr #0 personality i32 (...)* @__CxxFrameHandler3 {
start:
  %0 = getelementptr inbounds %"std::io::error::Error", %"std::io::error::Error"* %arg0, i32 0, i32 0, i32 0
  %1 = load i8, i8* %0, align 1, !range !1
  %switch.i = icmp ult i8 %1, 2
  br i1 %switch.i, label %_ZN4core3ptr13drop_in_place17ha25dfc290e4c6e7eE.exit, label %bb2.i

bb2.i:                                            ; preds = %start
  %2 = getelementptr inbounds %"std::io::error::Error", %"std::io::error::Error"* %arg0, i32 0, i32 0, i32 2
  %3 = bitcast [1 x i32]* %2 to %"std::io::error::Custom"**
  %4 = load %"std::io::error::Custom"*, %"std::io::error::Custom"** %3, align 4, !nonnull !2
  %5 = getelementptr inbounds %"std::io::error::Custom", %"std::io::error::Custom"* %4, i32 0, i32 2
  %6 = getelementptr inbounds { i8*, void (i8*)** }, { i8*, void (i8*)** }* %5, i32 0, i32 0
  %7 = load i8*, i8** %6, align 4, !nonnull !2
  %8 = getelementptr inbounds %"std::io::error::Custom", %"std::io::error::Custom"* %4, i32 0, i32 2, i32 1
  %9 = load void (i8*)**, void (i8*)*** %8, align 4, !nonnull !2
  %10 = load void (i8*)*, void (i8*)** %9, align 4, !invariant.load !2, !nonnull !2
  invoke void %10(i8* nonnull %7)
          to label %_ZN4core3ptr13drop_in_place17hf464a53cef483c2eE.exit.i unwind label %funclet_bb4.i.i.i.i

funclet_bb4.i.i.i.i:                              ; preds = %bb2.i
  %cleanuppad.i.i.i.i = cleanuppad within none []
  %11 = bitcast { i8*, void (i8*)** }* %5 to {}**
  %12 = load {}*, {}** %11, align 4, !nonnull !2
  %13 = bitcast void (i8*)*** %8 to {}**
  %14 = load {}*, {}** %13, align 4, !nonnull !2
; call alloc::heap::box_free
  call fastcc void @_ZN5alloc4heap8box_free17h22526cba032eb000E({}* nonnull %12, {}* nonnull %14) #8 [ "funclet"(token %cleanuppad.i.i.i.i) ]
  %15 = load %"std::io::error::Custom"*, %"std::io::error::Custom"** %3, align 4, !nonnull !2
; call alloc::heap::box_free
  call fastcc void @_ZN5alloc4heap8box_free17h39a5af7e911d8a90E(%"std::io::error::Custom"* nonnull %15) #8 [ "funclet"(token %cleanuppad.i.i.i.i) ]
  cleanupret from %cleanuppad.i.i.i.i unwind to caller

_ZN4core3ptr13drop_in_place17hf464a53cef483c2eE.exit.i: ; preds = %bb2.i
  %16 = bitcast { i8*, void (i8*)** }* %5 to {}**
  %17 = load {}*, {}** %16, align 4, !nonnull !2
  %18 = bitcast void (i8*)*** %8 to {}**
  %19 = load {}*, {}** %18, align 4, !nonnull !2
; call alloc::heap::box_free
  tail call fastcc void @_ZN5alloc4heap8box_free17h22526cba032eb000E({}* nonnull %17, {}* nonnull %19)
  %20 = load %"std::io::error::Custom"*, %"std::io::error::Custom"** %3, align 4, !nonnull !2
; call alloc::heap::box_free
  tail call fastcc void @_ZN5alloc4heap8box_free17h39a5af7e911d8a90E(%"std::io::error::Custom"* nonnull %20)
  br label %_ZN4core3ptr13drop_in_place17ha25dfc290e4c6e7eE.exit

_ZN4core3ptr13drop_in_place17ha25dfc290e4c6e7eE.exit: ; preds = %start, %_ZN4core3ptr13drop_in_place17hf464a53cef483c2eE.exit.i
  ret void
}

; core::ptr::drop_in_place
; Function Attrs: minsize nounwind optsize uwtable
define internal fastcc void @_ZN4core3ptr13drop_in_place17h9c3948f7e82fa850E(%"alloc::vec::Vec<u8>"* nocapture readonly %arg0) unnamed_addr #1 personality i32 (...)* @__CxxFrameHandler3 {
bb4:
  %0 = getelementptr inbounds %"alloc::vec::Vec<u8>", %"alloc::vec::Vec<u8>"* %arg0, i32 0, i32 0, i32 2
  %1 = load i32, i32* %0, align 4, !alias.scope !3, !noalias !6
  %2 = icmp eq i32 %1, 0
  br i1 %2, label %_ZN4core3ptr13drop_in_place17h359ad3002181084dE.exit, label %bb6.i.i.i

bb6.i.i.i:                                        ; preds = %bb4
  %3 = getelementptr inbounds %"alloc::vec::Vec<u8>", %"alloc::vec::Vec<u8>"* %arg0, i32 0, i32 0, i32 0, i32 0, i32 0
  %_2.sroa.0.0.copyload3.i.i.i.i = load i8*, i8** %3, align 4, !alias.scope !8
  tail call void @__rust_dealloc(i8* %_2.sroa.0.0.copyload3.i.i.i.i, i32 %1, i32 1) #9, !noalias !11
  br label %_ZN4core3ptr13drop_in_place17h359ad3002181084dE.exit

_ZN4core3ptr13drop_in_place17h359ad3002181084dE.exit: ; preds = %bb4, %bb6.i.i.i
  ret void
}

; core::result::unwrap_failed
; Function Attrs: cold minsize noinline noreturn optsize uwtable
define internal fastcc void @_ZN4core6result13unwrap_failed17h8a5e75b67d4ac1fcE(%"std::io::error::Error"* noalias nocapture dereferenceable(8) %error) unnamed_addr #2 personality i32 (...)* @__CxxFrameHandler3 {
bb5:
  %_10 = alloca [2 x %"core::fmt::ArgumentV1"], align 4
  %_3 = alloca %"core::fmt::Arguments", align 4
  %msg = alloca %str_slice, align 4
  %0 = getelementptr inbounds %str_slice, %str_slice* %msg, i32 0, i32 0
  store i8* getelementptr inbounds ([43 x i8], [43 x i8]* @str.0, i32 0, i32 0), i8** %0, align 4
  %1 = getelementptr inbounds %str_slice, %str_slice* %msg, i32 0, i32 1
  store i32 43, i32* %1, align 4
  %2 = bitcast %"core::fmt::Arguments"* %_3 to i8*
  call void @llvm.lifetime.start(i64 24, i8* nonnull %2)
  %3 = bitcast [2 x %"core::fmt::ArgumentV1"]* %_10 to i8*
  call void @llvm.lifetime.start(i64 16, i8* nonnull %3)
  %4 = ptrtoint %str_slice* %msg to i32
  %5 = ptrtoint %"std::io::error::Error"* %error to i32
  %6 = bitcast [2 x %"core::fmt::ArgumentV1"]* %_10 to i32*
  store i32 %4, i32* %6, align 4
  %7 = getelementptr inbounds [2 x %"core::fmt::ArgumentV1"], [2 x %"core::fmt::ArgumentV1"]* %_10, i32 0, i32 0, i32 2
  %8 = bitcast i8 (%"core::fmt::Void"*, %"core::fmt::Formatter"*)** %7 to i32*
  store i32 ptrtoint (i8 (%str_slice*, %"core::fmt::Formatter"*)* @"_ZN55_$LT$$RF$$u27$a$u20$T$u20$as$u20$core..fmt..Display$GT$3fmt17h2f014e77e91383feE" to i32), i32* %8, align 4
  %9 = getelementptr inbounds [2 x %"core::fmt::ArgumentV1"], [2 x %"core::fmt::ArgumentV1"]* %_10, i32 0, i32 1
  %10 = bitcast %"core::fmt::ArgumentV1"* %9 to i32*
  store i32 %5, i32* %10, align 4
  %11 = getelementptr inbounds [2 x %"core::fmt::ArgumentV1"], [2 x %"core::fmt::ArgumentV1"]* %_10, i32 0, i32 1, i32 2
  %12 = bitcast i8 (%"core::fmt::Void"*, %"core::fmt::Formatter"*)** %11 to i32*
  store i32 ptrtoint (i8 (%"std::io::error::Error"*, %"core::fmt::Formatter"*)* @"_ZN58_$LT$std..io..error..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h67058a23b522aa15E" to i32), i32* %12, align 4
  %13 = getelementptr inbounds [2 x %"core::fmt::ArgumentV1"], [2 x %"core::fmt::ArgumentV1"]* %_10, i32 0, i32 0
  %14 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_3, i32 0, i32 0, i32 0
  store %str_slice* getelementptr inbounds ([2 x %str_slice], [2 x %str_slice]* @ref.3, i32 0, i32 0), %str_slice** %14, align 4, !alias.scope !14, !noalias !17
  %15 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_3, i32 0, i32 0, i32 1
  store i32 2, i32* %15, align 4, !alias.scope !14, !noalias !17
  %_4.sroa.0.0..sroa_idx.i = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_3, i32 0, i32 2, i32 0, i32 0
  store %"core::fmt::rt::v1::Argument"* null, %"core::fmt::rt::v1::Argument"** %_4.sroa.0.0..sroa_idx.i, align 4, !alias.scope !14, !noalias !17
  %16 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_3, i32 0, i32 4, i32 0
  store %"core::fmt::ArgumentV1"* %13, %"core::fmt::ArgumentV1"** %16, align 4, !alias.scope !14, !noalias !17
  %17 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_3, i32 0, i32 4, i32 1
  store i32 2, i32* %17, align 4, !alias.scope !14, !noalias !17
; invoke core::panicking::panic_fmt
  invoke void @_ZN4core9panicking9panic_fmt17h338347f1b5680408E(%"core::fmt::Arguments"* noalias nocapture nonnull dereferenceable(24) %_3, { %str_slice, [0 x i8], i32, [0 x i8], i32, [0 x i8] }* noalias readonly dereferenceable(16) bitcast ({ %str_slice, i32, i32 }* @ref.5 to { %str_slice, [0 x i8], i32, [0 x i8], i32, [0 x i8] }*))
          to label %unreachable unwind label %funclet_bb3

funclet_bb3:                                      ; preds = %bb5
  %cleanuppad = cleanuppad within none []
; call core::ptr::drop_in_place
  call fastcc void @_ZN4core3ptr13drop_in_place17h0b2d9bad824fa3b0E(%"std::io::error::Error"* nonnull %error) #8 [ "funclet"(token %cleanuppad) ]
  cleanupret from %cleanuppad unwind to caller

unreachable:                                      ; preds = %bb5
  unreachable
}

; <&'a T as core::fmt::Display>::fmt
; Function Attrs: minsize optsize uwtable
define internal i8 @"_ZN55_$LT$$RF$$u27$a$u20$T$u20$as$u20$core..fmt..Display$GT$3fmt17h2f014e77e91383feE"(%str_slice* noalias nocapture readonly dereferenceable(8) %self, %"core::fmt::Formatter"* dereferenceable(52) %f) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds %str_slice, %str_slice* %self, i32 0, i32 0
  %1 = load i8*, i8** %0, align 4, !nonnull !2
  %2 = getelementptr inbounds %str_slice, %str_slice* %self, i32 0, i32 1
  %3 = load i32, i32* %2, align 4
; call <str as core::fmt::Display>::fmt
  %4 = tail call i8 @"_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hcf4b0d7c8930933fE"(i8* noalias nonnull readonly %1, i32 %3, %"core::fmt::Formatter"* nonnull dereferenceable(52) %f)
  ret i8 %4
}

; alloc::heap::box_free
; Function Attrs: inlinehint minsize nounwind optsize uwtable
define internal fastcc void @_ZN5alloc4heap8box_free17h22526cba032eb000E({}*, {}* nocapture readonly) unnamed_addr #3 {
start:
  %2 = bitcast {}* %1 to i32*
  %3 = getelementptr inbounds i32, i32* %2, i32 1
  %4 = load i32, i32* %3, align 4, !invariant.load !2
  %5 = icmp eq i32 %4, 0
  br i1 %5, label %bb7, label %bb3

bb3:                                              ; preds = %start
  %6 = getelementptr inbounds i32, i32* %2, i32 2
  %7 = load i32, i32* %6, align 4, !invariant.load !2
  %ptr.ptr = bitcast {}* %0 to i8*
  tail call void @__rust_dealloc(i8* %ptr.ptr, i32 %4, i32 %7) #9, !noalias !19
  br label %bb7

bb7:                                              ; preds = %start, %bb3
  ret void
}

; alloc::heap::box_free
; Function Attrs: inlinehint minsize nounwind optsize uwtable
define internal fastcc void @_ZN5alloc4heap8box_free17h39a5af7e911d8a90E(%"std::io::error::Custom"* %ptr) unnamed_addr #3 {
start:
  %0 = getelementptr inbounds %"std::io::error::Custom", %"std::io::error::Custom"* %ptr, i32 0, i32 0
  tail call void @__rust_dealloc(i8* %0, i32 12, i32 4) #9, !noalias !22
  ret void
}

; test::main
; Function Attrs: minsize optsize uwtable
define internal void @_ZN4test4main17h0eddcc39ebf1578fE() unnamed_addr #0 personality i32 (...)* @__CxxFrameHandler3 {
start:
  %_8.i = alloca i64, align 4
  %_3.i = alloca %"std::fs::DirBuilder", align 4
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
  %3 = invoke { %"std::ffi::os_str::OsStr"*, i32 } @"_ZN3std3ffi6os_str85_$LT$impl$u20$core..convert..AsRef$LT$std..ffi..os_str..OsStr$GT$$u20$for$u20$str$GT$6as_ref17hb155917dda2e3c8aE"(i8* noalias nonnull readonly getelementptr inbounds ([1 x i8], [1 x i8]* @str.7, i32 0, i32 0), i32 1)
          to label %bb2 unwind label %funclet_bb4

bb2:                                              ; preds = %start
  %4 = bitcast %"std::fs::DirBuilder"* %_3.i to i8*
  call void @llvm.lifetime.start(i64 1, i8* nonnull %4), !noalias !28
; invoke std::fs::DirBuilder::new
  %5 = invoke i8 @_ZN3std2fs10DirBuilder3new17h05238cf76b618a10E()
          to label %.noexc unwind label %funclet_bb4

.noexc:                                           ; preds = %bb2
  %6 = extractvalue { %"std::ffi::os_str::OsStr"*, i32 } %3, 0
  %7 = getelementptr inbounds %"std::ffi::os_str::OsStr", %"std::ffi::os_str::OsStr"* %6, i32 0, i32 0, i32 0, i32 0
  %8 = extractvalue { %"std::ffi::os_str::OsStr"*, i32 } %3, 1
  store i8 %5, i8* %4, align 4, !noalias !28
; invoke <std::path::Path as core::convert::AsRef<std::path::Path>>::as_ref
  %9 = invoke { %"std::path::Path"*, i32 } @"_ZN79_$LT$std..path..Path$u20$as$u20$core..convert..AsRef$LT$std..path..Path$GT$$GT$6as_ref17hece874b213dfc22cE"(i8* noalias nonnull readonly %7, i32 %8)
          to label %.noexc1 unwind label %funclet_bb4

.noexc1:                                          ; preds = %.noexc
  %10 = extractvalue { %"std::path::Path"*, i32 } %9, 0
  %11 = extractvalue { %"std::path::Path"*, i32 } %9, 1
  %12 = getelementptr inbounds %"std::path::Path", %"std::path::Path"* %10, i32 0, i32 0, i32 0, i32 0, i32 0
; invoke <std::path::Path as core::convert::AsRef<std::path::Path>>::as_ref
  %13 = invoke { %"std::path::Path"*, i32 } @"_ZN79_$LT$std..path..Path$u20$as$u20$core..convert..AsRef$LT$std..path..Path$GT$$GT$6as_ref17hece874b213dfc22cE"(i8* noalias nonnull readonly %12, i32 %11)
          to label %.noexc2 unwind label %funclet_bb4

.noexc2:                                          ; preds = %.noexc1
  %14 = extractvalue { %"std::path::Path"*, i32 } %13, 0
  %15 = extractvalue { %"std::path::Path"*, i32 } %13, 1
  %16 = getelementptr inbounds %"std::path::Path", %"std::path::Path"* %14, i32 0, i32 0, i32 0, i32 0, i32 0
; invoke std::fs::DirBuilder::_create
  invoke void @_ZN3std2fs10DirBuilder7_create17h57e12c60209658d4E(%"core::result::Result<(), std::io::error::Error>"* noalias nocapture nonnull sret dereferenceable(12) %_3, %"std::fs::DirBuilder"* noalias nonnull readonly dereferenceable(1) %_3.i, i8* noalias nonnull readonly %16, i32 %15)
          to label %bb5 unwind label %funclet_bb4

bb5:                                              ; preds = %.noexc2
  call void @llvm.lifetime.end(i64 1, i8* nonnull %4), !noalias !28
  %17 = getelementptr inbounds %"core::result::Result<(), std::io::error::Error>", %"core::result::Result<(), std::io::error::Error>"* %_3, i32 0, i32 0
  %18 = load i32, i32* %17, align 4, !range !32, !alias.scope !33
  %cond.i = icmp eq i32 %18, 0
  br i1 %cond.i, label %bb6, label %bb2.i

bb2.i:                                            ; preds = %bb5
  %tmpcast10.i = bitcast i64* %_8.i to %"std::io::error::Error"*
  %e.sroa.0.0..sroa_idx3.i = getelementptr inbounds %"core::result::Result<(), std::io::error::Error>", %"core::result::Result<(), std::io::error::Error>"* %_3, i32 0, i32 2
  %19 = bitcast [2 x i32]* %e.sroa.0.0..sroa_idx3.i to i64*
  %20 = load i64, i64* %19, align 4, !alias.scope !33
  %21 = bitcast i64* %_8.i to i8*
  call void @llvm.lifetime.start(i64 8, i8* nonnull %21), !noalias !33
  store i64 %20, i64* %_8.i, align 4, !noalias !33
; invoke core::result::unwrap_failed
  invoke fastcc void @_ZN4core6result13unwrap_failed17h8a5e75b67d4ac1fcE(%"std::io::error::Error"* noalias nocapture nonnull dereferenceable(8) %tmpcast10.i)
          to label %.noexc4 unwind label %funclet_bb4

.noexc4:                                          ; preds = %bb2.i
  unreachable

bb6:                                              ; preds = %bb5
  call void @llvm.lifetime.end(i64 12, i8* nonnull %2)
; call core::ptr::drop_in_place
  call fastcc void @_ZN4core3ptr13drop_in_place17h9c3948f7e82fa850E(%"alloc::vec::Vec<u8>"* nonnull %_vec)
  call void @llvm.lifetime.end(i64 12, i8* nonnull %0)
  ret void

funclet_bb4:                                      ; preds = %bb2.i, %.noexc2, %.noexc1, %.noexc, %bb2, %start
  %cleanuppad = cleanuppad within none []
; call core::ptr::drop_in_place
  call fastcc void @_ZN4core3ptr13drop_in_place17h9c3948f7e82fa850E(%"alloc::vec::Vec<u8>"* nonnull %_vec) #8 [ "funclet"(token %cleanuppad) ]
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
declare void @_ZN3std2fs10DirBuilder7_create17h57e12c60209658d4E(%"core::result::Result<(), std::io::error::Error>"* noalias nocapture sret dereferenceable(12), %"std::fs::DirBuilder"* noalias readonly dereferenceable(1), i8* noalias nonnull readonly, i32) unnamed_addr #5

; std::fs::DirBuilder::new
; Function Attrs: minsize optsize
declare i8 @_ZN3std2fs10DirBuilder3new17h05238cf76b618a10E() unnamed_addr #5

; std::ffi::os_str::<impl core::convert::AsRef<std::ffi::os_str::OsStr> for str>::as_ref
; Function Attrs: minsize optsize
declare { %"std::ffi::os_str::OsStr"*, i32 } @"_ZN3std3ffi6os_str85_$LT$impl$u20$core..convert..AsRef$LT$std..ffi..os_str..OsStr$GT$$u20$for$u20$str$GT$6as_ref17hb155917dda2e3c8aE"(i8* noalias nonnull readonly, i32) unnamed_addr #5

; <std::io::error::Error as core::fmt::Debug>::fmt
; Function Attrs: minsize optsize
declare i8 @"_ZN58_$LT$std..io..error..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h67058a23b522aa15E"(%"std::io::error::Error"* noalias readonly dereferenceable(8), %"core::fmt::Formatter"* dereferenceable(52)) unnamed_addr #5

; core::panicking::panic_fmt
; Function Attrs: cold minsize noinline noreturn optsize
declare void @_ZN4core9panicking9panic_fmt17h338347f1b5680408E(%"core::fmt::Arguments"* noalias nocapture dereferenceable(24), { %str_slice, [0 x i8], i32, [0 x i8], i32, [0 x i8] }* noalias readonly dereferenceable(16)) unnamed_addr #6

; <str as core::fmt::Display>::fmt
; Function Attrs: minsize optsize
declare i8 @"_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hcf4b0d7c8930933fE"(i8* noalias nonnull readonly, i32, %"core::fmt::Formatter"* dereferenceable(52)) unnamed_addr #5

; Function Attrs: minsize nounwind optsize
declare void @__rust_dealloc(i8*, i32, i32) unnamed_addr #7

; <std::path::Path as core::convert::AsRef<std::path::Path>>::as_ref
; Function Attrs: minsize optsize
declare { %"std::path::Path"*, i32 } @"_ZN79_$LT$std..path..Path$u20$as$u20$core..convert..AsRef$LT$std..path..Path$GT$$GT$6as_ref17hece874b213dfc22cE"(i8* noalias nonnull readonly, i32) unnamed_addr #5

; Function Attrs: minsize optsize
define i32 @main(i32, i8**) unnamed_addr #5 {
top:
; call std::rt::lang_start
  %2 = tail call i32 @_ZN3std2rt10lang_start17h81baecf0c352613dE(void ()* nonnull @_ZN4test4main17h0eddcc39ebf1578fE, i32 %0, i8** %1)
  ret i32 %2
}

; std::rt::lang_start
; Function Attrs: minsize optsize
declare i32 @_ZN3std2rt10lang_start17h81baecf0c352613dE(void ()*, i32, i8**) unnamed_addr #5

attributes #0 = { minsize optsize uwtable }
attributes #1 = { minsize nounwind optsize uwtable }
attributes #2 = { cold minsize noinline noreturn optsize uwtable }
attributes #3 = { inlinehint minsize nounwind optsize uwtable }
attributes #4 = { argmemonly nounwind }
attributes #5 = { minsize optsize }
attributes #6 = { cold minsize noinline noreturn optsize }
attributes #7 = { minsize nounwind optsize }
attributes #8 = { noinline }
attributes #9 = { nounwind }

!llvm.module.flags = !{!0}

!0 = !{i32 1, !"PIE Level", i32 2}
!1 = !{i8 0, i8 3}
!2 = !{}
!3 = !{!4}
!4 = distinct !{!4, !5, !"_ZN49_$LT$alloc..raw_vec..RawVec$LT$T$C$$u20$A$GT$$GT$14current_layout17h66e9c34e20f357dfE: %self"}
!5 = distinct !{!5, !"_ZN49_$LT$alloc..raw_vec..RawVec$LT$T$C$$u20$A$GT$$GT$14current_layout17h66e9c34e20f357dfE"}
!6 = !{!7}
!7 = distinct !{!7, !5, !"_ZN49_$LT$alloc..raw_vec..RawVec$LT$T$C$$u20$A$GT$$GT$14current_layout17h66e9c34e20f357dfE: argument 0"}
!8 = !{!9}
!9 = distinct !{!9, !10, !"_ZN49_$LT$alloc..raw_vec..RawVec$LT$T$C$$u20$A$GT$$GT$3ptr17hbeae0d551a499cbeE: %self"}
!10 = distinct !{!10, !"_ZN49_$LT$alloc..raw_vec..RawVec$LT$T$C$$u20$A$GT$$GT$3ptr17hbeae0d551a499cbeE"}
!11 = !{!12}
!12 = distinct !{!12, !13, !"_ZN61_$LT$alloc..heap..Heap$u20$as$u20$alloc..allocator..Alloc$GT$7dealloc17h1c7f85766a2ae242E: %layout"}
!13 = distinct !{!13, !"_ZN61_$LT$alloc..heap..Heap$u20$as$u20$alloc..allocator..Alloc$GT$7dealloc17h1c7f85766a2ae242E"}
!14 = !{!15}
!15 = distinct !{!15, !16, !"_ZN4core3fmt9Arguments6new_v117hea388adf06eedfdfE: argument 0"}
!16 = distinct !{!16, !"_ZN4core3fmt9Arguments6new_v117hea388adf06eedfdfE"}
!17 = !{!18}
!18 = distinct !{!18, !16, !"_ZN4core3fmt9Arguments6new_v117hea388adf06eedfdfE: %args.ptr"}
!19 = !{!20}
!20 = distinct !{!20, !21, !"_ZN61_$LT$alloc..heap..Heap$u20$as$u20$alloc..allocator..Alloc$GT$7dealloc17h1c7f85766a2ae242E: %layout"}
!21 = distinct !{!21, !"_ZN61_$LT$alloc..heap..Heap$u20$as$u20$alloc..allocator..Alloc$GT$7dealloc17h1c7f85766a2ae242E"}
!22 = !{!23}
!23 = distinct !{!23, !24, !"_ZN61_$LT$alloc..heap..Heap$u20$as$u20$alloc..allocator..Alloc$GT$7dealloc17h1c7f85766a2ae242E: %layout"}
!24 = distinct !{!24, !"_ZN61_$LT$alloc..heap..Heap$u20$as$u20$alloc..allocator..Alloc$GT$7dealloc17h1c7f85766a2ae242E"}
!25 = !{!26}
!26 = distinct !{!26, !27, !"_ZN33_$LT$alloc..vec..Vec$LT$T$GT$$GT$3new17he9c2dcd06a6bebefE: argument 0"}
!27 = distinct !{!27, !"_ZN33_$LT$alloc..vec..Vec$LT$T$GT$$GT$3new17he9c2dcd06a6bebefE"}
!28 = !{!29, !31}
!29 = distinct !{!29, !30, !"_ZN3std2fs10create_dir17hf5d0cd3f19e63cc2E: argument 0"}
!30 = distinct !{!30, !"_ZN3std2fs10create_dir17hf5d0cd3f19e63cc2E"}
!31 = distinct !{!31, !30, !"_ZN3std2fs10create_dir17hf5d0cd3f19e63cc2E: argument 1"}
!32 = !{i32 0, i32 2}
!33 = !{!34}
!34 = distinct !{!34, !35, !"_ZN47_$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$6unwrap17h64a617f69504e1b7E: %self"}
!35 = distinct !{!35, !"_ZN47_$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$6unwrap17h64a617f69504e1b7E"}
