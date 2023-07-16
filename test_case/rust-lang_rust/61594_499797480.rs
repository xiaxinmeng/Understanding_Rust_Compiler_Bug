
thread 'rustc' panicked at 'Forcing query with already existing DepNode.
- query-key: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: (Const { ty: [u32; 16], val: ByRef(AllocId(3181).0x0, Allocation { bytes: [0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0, 7, 0, 0, 0, 8, 0, 0, 0, 9, 0, 0, 0, 10, 0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [18446744073709551615, 0], len: Size { raw: 64 } }, align: Align { pow2: 2 }, mutability: Immutable, extra: () }) }, field[0]) }
- dep-node: const_field(449c8050006f9d1b-207c1d76bc38c220)', src/librustc/ty/query/plumbing.rs:548:9
