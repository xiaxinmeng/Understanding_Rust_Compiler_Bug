
error: internal compiler error: librustc_codegen_llvm/mir/operand.rs:145: not immediate: OperandRef(Pair(([0 x i8]*:[0 x i8]* bitcast (<{ [9 x i8] }>* @1 to [0 x i8]*)), (i64:i64 9)) @ TyLayout { ty: &str, details: LayoutDetails { variants: Single { index: 0 }, fields: Arbitrary { offsets: [Size { raw: 0 }, Size { raw: 8 }], memory_index: [0, 1] }, abi: ScalarPair(Scalar { value: Pointer, valid_range: 1..=18446744073709551615 }, Scalar { value: Int(I64, false), valid_range: 0..=18446744073709551615 }), align: Align { abi_pow2: 3, pref_pow2: 3 }, size: Size { raw: 16 } } })

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:587:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: aborting due to previous error
