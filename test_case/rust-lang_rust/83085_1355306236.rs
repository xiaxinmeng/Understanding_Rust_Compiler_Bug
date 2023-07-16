`
thread '<unnamed>' panicked at 'forcing query with already existing `DepNode`
- query-key: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing, constness: NotConst }, value: Val(Scalar(alloc5), &[u8; 0]) }
- dep-node: deref_mir_constant(58168a8023dae660-4b49ccbada1af725)', /rustc/ec56537c4325ce5b798fc3628cbdd48ba4949ae5/compiler/rustc_query_system/src/dep_graph/graph.rs:316:9
stack backtrace:
