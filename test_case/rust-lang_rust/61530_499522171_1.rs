
Updating crates.io index
   Compiling packed_simd v0.3.3
   Compiling cfg-if v0.1.9
   Compiling curve25519-dalek v1.1.4 (/tmp/tmp/curve25519-dalek)
thread 'rustc' panicked at 'Forcing query with already existing DepNode.
- query-key: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: (Const { ty: [u32; 8], val: ByRef(AllocId(172).0x0, Allocation { bytes: [0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0, 7, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [4294967295], len: Size { raw: 32 } }, align: Align { pow2: 2 }, mutability: Immutable, extra: () }) }, field[0]) }
- dep-node: const_field(eb99858e4874f291-9a55d263e1a1abdf)', src/librustc/ty/query/plumbing.rs:548:9
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.37.0-nightly (7cdaffd79 2019-06-05) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `curve25519-dalek`.

To learn more, run the command again with --verbose.
