
DEBUG:rustc_trans::mir::statement: trans_statement(statement=_14 = ((_11 as Some).0: <Self as iter::iterator::Iterator>::Item))
DEBUG:rustc_trans::debuginfo::source_loc: set_source_location: src/libcore/iter/iterator.rs:254:13: 254:14
DEBUG:rustc_trans::debuginfo::source_loc: setting debug location to 254 0
DEBUG:rustc_trans::mir::rvalue: trans_rvalue(dest.llval=({ i64, [0 x i8], i32, [4 x i8] }*:  %x = alloca { i64, [0 x i8], i32, [4 x i8] }), rvalue=((_11 as Some).0: <Self as iter::iterator::Iterator>::Item))
DEBUG:rustc_trans::mir::operand: trans_operand(operand=((_11 as Some).0: <Self as iter::iterator::Iterator>::Item))
DEBUG:rustc_trans::mir::operand: trans_consume(lvalue=((_11 as Some).0: <Self as iter::iterator::Iterator>::Item))
DEBUG:rustc_trans::mir::lvalue: trans_lvalue(lvalue=((_11 as Some).0: <Self as iter::iterator::Iterator>::Item))
DEBUG:rustc_trans::mir::lvalue: trans_lvalue(lvalue=(_11 as Some))
DEBUG:rustc_trans::mir::lvalue: trans_lvalue(lvalue=_11)
DEBUG:rustc_trans::monomorphize: apply_param_substs(param_substs=Slice([str::CharIndices]), value=Downcast { adt_def: option::Option, substs: Slice([(usize, char)]), variant_index: 1 })
DEBUG:rustc_trans::mir::lvalue: trans_lvalue(lvalue=(_11 as Some)) => LvalueRef { llval: 0x7f059d15cb78, llextra: 0x0, ty: Downcast { adt_def: option::Option, substs: Slice([(usize, char)]), variant_index: 1 }, alignment: AbiAligned }
DEBUG:rustc_trans::monomorphize: apply_param_substs(param_substs=Slice([str::CharIndices]), value=Ty { ty: <Self as iter::iterator::Iterator>::Item })
DEBUG:rustc_trans::adt: struct_llfields: variant: Struct { align: Align { raw: 51 }, primitive_align: Align { raw: 51 }, packed: false, sized: true, offsets: [Size { raw: 0 }, Size { raw: 8 }], memory_index: [0, 1], min_size: Size { raw: 24 } }
DEBUG:rustc_trans::adt: struct_llfields: 0 ty: u64 min_offset: 0 target_offset: 0
DEBUG:rustc_trans::adt: struct_llfields: 1 ty: (usize, char) pad_bytes: 0 min_offset: 8 target_offset: 8
DEBUG:rustc_trans::adt: struct_llfields: pad_bytes: 0 min_offset: 24 min_size: 24 stride: 24

DEBUG:rustc_trans::mir::lvalue: trans_lvalue(lvalue=((_11 as Some).0: <Self as iter::iterator::Iterator>::Item)) => LvalueRef { llval: 0x7f05a0a17f78, llextra: 0x0, ty: Ty { ty: (usize, char) }, alignment: AbiAligned }
DEBUG:rustc_trans::mir::operand: trans_load: ({ i64, [0 x i8], i32, [4 x i8] }*:  %10 = getelementptr inbounds { i64, [0 x i8], { i64, [0 x i8], i32, [4 x i8] }, [0 x i8] }, { i64, [0 x i8], { i64, [0 x i8], i32, [4 x i8] }, [0 x i8] }* %9, i32 0, i32 2, !dbg !73) @ (usize, char)
