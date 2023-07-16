
error: internal compiler error: /rustc/04caa632dd10c2bf64b69524c7f9c4c30a436877/compiler/rustc_codegen_ssa/src/mir/operand.rs:133:38: Deref of by-Ref operand OperandRef(Ref((%"alloc::boxed::Box<core::mem::MaybeUninit<usize>, dynamic_memory_management::Locked<dynamic_memory_management::allocator::Heap>>"*:  %6 = alloca %"alloc::boxed::Box<core::mem::MaybeUninit<usize>, dynamic_memory_management::Locked<dynamic_memory_management::allocator::Heap>>", align 4), None, Align { pow2: 2 }) @ TyAndLayout { ty: alloc::boxed::Box<core::mem::MaybeUninit<usize>, dynamic_memory_management::Locked<dynamic_memory_management::allocator::Heap>>, layout: Layout { fields: Arbitrary { offsets: [Size { raw: 0 }, Size { raw: 4 }], memory_index: [0, 1] }, variants: Single { index: 0 }, abi: Aggregate { sized: true }, largest_niche: Some(Niche { offset: Size { raw: 0 }, scalar: Scalar { value: Pointer, valid_range: 1..=4294967295 } }), align: AbiAndPrefAlign { abi: Align { pow2: 2 }, pref: Align { pow2: 3 } }, size: Size { raw: 32 } } })

thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:958:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (04caa632d 2021-01-30) running on x86_64-apple-darwin

note: compiler flags: -Z unstable-options -C opt-level=3 -C panic=abort -C embed-bitcode=no --crate-type staticlib

note: some of the compiler flags provided by cargo are hidden
