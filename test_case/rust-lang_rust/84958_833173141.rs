cpp
/// When inlining a call site that has !llvm.mem.parallel_loop_access,
/// !llvm.access.group, !alias.scope or !noalias metadata, that metadata should
/// be propagated to all memory-accessing cloned instructions.
