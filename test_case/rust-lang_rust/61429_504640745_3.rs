
error: internal compiler error: src/librustc_codegen_ssa/mir/operand.rs:128: not immediate: OperandRef(Pair(([0 x i8]*:  %10 = load [0 x i8]*, [0 x i8]** %9, align 8, !dbg !32, !nonnull !4), (i64:  %12 = load i64, i64* %11, align 8, !dbg !32)) @ TyLayout { ty: &str, details: LayoutDetails { variants: Single { index: 0 }, fields: Arbitrary { offsets: [Size { raw: 0 }, Size { raw: 8 }], memory_index: [0, 1] }, abi: ScalarPair(Scalar { value: Pointer, valid_range: 1..=18446744073709551615 }, Scalar { value: Int(I64, false), valid_range: 0..=18446744073709551615 }), align: AbiAndPrefAlign { abi: Align { pow2: 3 }, pref: Align { pow2: 3 } }, size: Size { raw: 16 } } })

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:637:9
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.36.0-nightly (e70d5386d 2019-05-27) running on x86_64-unknown-linux-gnu

