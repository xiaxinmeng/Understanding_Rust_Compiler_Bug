
; playground::foo
; Function Attrs: nonlazybind uwtable
define void @_ZN10playground3foo17h31b32b7845372f25E(i64 %x) unnamed_addr #0 !dbg !6 {
start:
  %x.dbg.spill = alloca i64, align 8
  store i64 %x, i64* %x.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata i64* %x.dbg.spill, metadata !20, metadata !DIExpression()), !dbg !21
  ret void, !dbg !22
}
