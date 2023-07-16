
PHINode should have one entry for each predecessor of its parent basic block!
  %4 = phi i64 [ 0, %entry-block ], [ %5, %expr_repeat ]
LLVM ERROR: Broken function found, compilation aborted!
