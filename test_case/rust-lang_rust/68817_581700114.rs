llvm
DBG_VALUE %stack.0, $noreg, !"stack_val_ref",
    !DIExpression(),
    debug-location !31; src/test/debuginfo/borrowed-struct.rs:77:8 line no:77

DBG_VALUE $noreg, $noreg, !"stack_val_interior_ref_1",
     !DIExpression(),
     debug-location !33; src/test/debuginfo/borrowed-struct.rs:78:8 line no:78

DBG_VALUE $noreg, $noreg, !"stack_val_interior_ref_2",
     !DIExpression(DW_OP_plus_uconst, 8, DW_OP_stack_value),
     debug-location !35; src/test/debuginfo/borrowed-struct.rs:79:8 line no:79
