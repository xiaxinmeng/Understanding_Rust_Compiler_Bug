
error: internal compiler error: /rustc/e100ec5bc7cd768ec17d75448b29c9ab4a39272b\compiler\rustc_codegen_ssa\src\mir\operand.rs:132:38: Deref of by-Ref operand OperandRef(Ref((%"alloc::boxed::Box<[u8], &alloc::alloc::Global>"*:  %4 = alloca %"alloc::boxed::Box<[u8], &alloc::alloc::Global>", align 8), None, Align 
{ pow2: 3 }) @ TyAndLayout { ty: std::boxed::Box<[u8], &std::alloc::Global>, layout: Layout { fields: Arbitrary { offsets: [Size { raw: 0 }, Size { raw: 16 }], memory_index: [0, 1] }, variants: Single { index: 0 }, abi: Aggregate { sized: true }, largest_niche: Some(Niche { offset: Size { raw: 0 }, scalar: Scalar { value: Pointer, valid_range: 1..=18446744073709551615 } }), align: AbiAndPrefAlign { abi: Align { pow2: 3 }, pref: Align { pow2: 3 } }, size: Size { raw: 24 } } })

thread 'rustc' panicked at 'Box<dyn Any>', compiler\rustc_errors\src\lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (e100ec5bc 2021-12-21) running on x86_64-pc-windows-msvc

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
