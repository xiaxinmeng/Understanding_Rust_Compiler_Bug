llvm
; ModuleID = '<stdin>'
source_filename = "uw.rs"
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

declare void @foo(i64*) unnamed_addr

; Function Attrs: uwtable
define void @bar() unnamed_addr #0 !dbg !4 {
  %1 = alloca i64, align 8, !dbg !8
  store i64 0, i64* %1, align 8, !dbg !8
  br label %l

l:                                                ; preds = %0
  call void @foo(i64* %1), !dbg !8
  ret void, !dbg !9
}

attributes #0 = { uwtable "no-frame-pointer-elim"="true" "probe-stack"="__rust_probestack" }

!llvm.dbg.cu = !{!0}
!llvm.module.flags = !{!3}

!0 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !1, producer: "clang LLVM (rustc version 1.25.0-nightly (b1f8e6fb0 2018-02-22))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, enums: !2)
!1 = !DIFile(filename: "uw.rs", directory: "/home/bs/src/rust-play")
!2 = !{}
!3 = !{i32 2, !"Debug Info Version", i32 3}
!4 = distinct !DISubprogram(name: "test", linkageName: "bar", scope: !5, file: !1, line: 5, type: !6, isLocal: false, isDefinition: true, scopeLine: 5, flags: DIFlagPrototyped, isOptimized: false, unit: !0, templateParams: !2, variables: !2)
!5 = !DINamespace(name: "uw", scope: null)
!6 = !DISubroutineType(types: !7)
!7 = !{null}
!8 = !DILocation(line: 6, scope: !4)
!9 = !DILocation(line: 8, scope: !4)
