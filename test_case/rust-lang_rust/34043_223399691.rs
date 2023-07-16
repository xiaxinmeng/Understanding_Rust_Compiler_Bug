 llvm
; Function Attrs: naked uwtable
define void @naked_fn(i8*, i8*) unnamed_addr #0 !dbg !10 {
entry-block:
  %a = alloca i8*
  %b = alloca i8*
  store i8* %0, i8** %a
  call void @llvm.dbg.declare(metadata i8** %a, metadata !19, metadata !20), !dbg !21
  store i8* %1, i8** %b
  call void @llvm.dbg.declare(metadata i8** %b, metadata !22, metadata !20), !dbg !21
  unreachable, !dbg !23
}
