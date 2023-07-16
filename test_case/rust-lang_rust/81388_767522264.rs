plain
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
thread 'rustc' panicked at 'ArgAbi { layout: TyAndLayout { ty: (&[u8], i16), layout: Layout { fields: Arbitrary { offsets: [Size { raw: 0 }, Size { raw: 16 }], memory_index: [0, 1] }, variants: Single { index: 0 }, abi: Aggregate { sized: true }, largest_niche: Some(Niche { offset: Size { raw: 0 }, scalar: Scalar { value: Pointer, valid_range: 1..=18446744073709551615 } }), align: AbiAndPrefAlign { abi: Align { pow2: 3 }, pref: Align { pow2: 3 } }, size: Size { raw: 24 } } }, pad: None, mode: Direct(ArgAttributes { regular: (empty), arg_ext: None, pointee_size: Size { raw: 0 }, pointee_align: None }) }', /checkout/compiler/rustc_middle/src/ty/layout.rs:2848:17

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.51.0-nightly (ff55d49db 2021-01-26) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `core`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
thread 'rustc' panicked at 'ArgAbi { layout: TyAndLayout { ty: unix::group, layout: Layout { fields: Arbitrary { offsets: [Size { raw: 0 }, Size { raw: 8 }, Size { raw: 16 }, Size { raw: 24 }], memory_index: [0, 1, 2, 3] }, variants: Single { index: 0 }, abi: Aggregate { sized: true }, largest_niche: None, align: AbiAndPrefAlign { abi: Align { pow2: 3 }, pref: Align { pow2: 3 } }, size: Size { raw: 32 } } }, pad: None, mode: Direct(ArgAttributes { regular: (empty), arg_ext: None, pointee_size: Size { raw: 0 }, pointee_align: None }) }', /checkout/compiler/rustc_middle/src/ty/layout.rs:2848:17

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.51.0-nightly (ff55d49db 2021-01-26) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
