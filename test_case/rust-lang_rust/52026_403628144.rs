llvm
; ModuleID = 'tmp1-317d481089b8c8fe83113de504472633.rs'
source_filename = "tmp1-317d481089b8c8fe83113de504472633.rs"
target datalayout = "e-m:x-p:32:32-i64:64-f80:32-n8:16:32-a:0:32-S32"
target triple = "i686-pc-windows-msvc"

%S = type { [0 x i32], [2 x i32], [0 x i32] }

; Function Attrs: uwtable
define internal void @_ZN3tmp4main17hcd108164c8e55929E() unnamed_addr {
  %1 = call { [0 x %S]*, i32 } @"_ZN68_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h968a5bfabac70869E"()
  %2 = extractvalue { [0 x %S]*, i32 } %1, 0
  %3 = extractvalue { [0 x %S]*, i32 } %1, 1
  %4 = getelementptr inbounds [0 x %S], [0 x %S]* %2, i32 0, i32 0, i32 0, i32 0
  %5 = getelementptr inbounds [0 x %S], [0 x %S]* %2, i32 0, i32 %3, i32 0, i32 0
  %6 = bitcast i32* %4 to %S*
  %7 = bitcast i32* %5 to %S*
  %8 = getelementptr inbounds %S, %S* %7, i32 -1
  %9 = bitcast %S* %8 to i32*
  %10 = bitcast i32* %9 to %S*
  %11 = icmp eq %S* %10, %6
  br i1 %11, label %12, label %13

; <label>:12:                                     ; preds = %24
  ret void

; <label>:13:                                     ; preds = %12, %50
  ret void
}

declare hidden { [0 x %S]*, i32 } @"_ZN68_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h968a5bfabac70869E"() unnamed_addr
