
thread 'rustc' panicked at 'Forcing query with already existing DepNode.
- query-key: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: (Const { ty: [u32; 2], val: ByRef(AllocId(3).0x0, Allocation { bytes: [0, 0, 0, 0, 1, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [255], len: Size { raw: 8 } }, align: Align { pow2: 2 }, mutability: Immutable, extra: () }) }, field[0]) }
- dep-node: const_field(67586bc8d6719bbd-8210aca0f024cb22)', src/librustc/ty/query/plumbing.rs:548:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.37.0-nightly (5eeb567a2 2019-06-06) running on x86_64-apple-darwin

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden
