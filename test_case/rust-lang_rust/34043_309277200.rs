llvm
; Function Attrs: naked nounwind uwtable
define void @naked(i64) unnamed_addr #0 !dbg !4 {
start:
  %_arg = alloca i64
  %arg0 = alloca i64
  store i64 %0, i64* %arg0
  call void @llvm.dbg.declare(metadata i64* %arg0, metadata !10, metadata !11), !dbg !12
  call void @llvm.dbg.declare(metadata i64* %_arg, metadata !13, metadata !11), !dbg !15
  %1 = load i64, i64* %arg0, !dbg !16
  store i64 %1, i64* %_arg, !dbg !16
  call void asm inteldialect "ret", "~{dirflag},~{fpsr},~{flags}"(), !dbg !17, !srcloc !18
  ret void, !dbg !19
}

; Function Attrs: nounwind uwtable
define void @non_naked(i64) unnamed_addr #1 !dbg !20 {
start:
  %_arg = alloca i64
  %arg0 = alloca i64
  store i64 %0, i64* %arg0
  call void @llvm.dbg.declare(metadata i64* %arg0, metadata !21, metadata !11), !dbg !22
  call void @llvm.dbg.declare(metadata i64* %_arg, metadata !23, metadata !11), !dbg !25
  %1 = load i64, i64* %arg0, !dbg !26
  store i64 %1, i64* %_arg, !dbg !26
  ret void, !dbg !27
}
