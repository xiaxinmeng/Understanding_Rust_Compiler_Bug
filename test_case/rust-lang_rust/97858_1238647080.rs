
thread 'rustc' panicked at 'forcing query with already existing `DepNode`
- query-key: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing, constness: NotConst }, value: Val(ByRef { alloc: Allocation { bytes: [0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0], provenance: ProvenanceMap(SortedMap { data: [(Size(0 bytes), alloc5), (Size(16 bytes), alloc27)] }), init_mask: InitMask { blocks: [4294967295], len: Size(32 bytes) }, align: Align(8 bytes), mutability: Not, extra: () }, offset: Size(0 bytes) }, instant_xml::Id) }
- dep-node: try_destructure_mir_constant(9361c990efa81df2-565a9ce6901e1ee1)', /rustc/9243168fa5615ec8ebe9164c6bc2fdcccffd08b6/compiler/rustc_query_system/src/dep_graph/graph.rs:316:9
