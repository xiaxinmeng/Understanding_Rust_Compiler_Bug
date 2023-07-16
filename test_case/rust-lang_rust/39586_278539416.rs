
DEBUG:rustc_trans::mir::operand: trans_operand(operand=_15)
DEBUG:rustc_trans::mir::operand: trans_consume(lvalue=_15)
DEBUG:rustc_trans::mir::lvalue: trans_lvalue(lvalue=(*(*_4))[_15]) => LvalueRef { llval: 0x154f0e0, llextra: 0x0, ty: Ty { ty: Unaligned<u32> }, alignment: AbiAligned }
DEBUG:rustc_trans::mir::operand: trans_load: (%"Unaligned<u32>"*:  %26 = getelementptr inbounds %"Unaligned<u32>", %"Unaligned<u32>"* %23, i64 %13, !dbg !85) @ Unaligned<u32>
DEBUG:rustc_trans::mir::operand: store_operand: operand=OperandRef(Immediate((%"Unaligned<u32>" = type <{ i32 }>:  %27 = load %"Unaligned<u32>", %"Unaligned<u32>"* %26, !dbg !85)) @ Unaligned<u32>), align=None
DEBUG:rustc_trans::builder: Store (%"Unaligned<u32>" = type <{ i32 }>:  %27 = load %"Unaligned<u32>", %"Unaligned<u32>"* %26, !dbg !85) -> (%"Unaligned<u32>"*:  %_14 = alloca %"Unaligned<u32>")
