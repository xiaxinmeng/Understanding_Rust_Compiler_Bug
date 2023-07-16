
Duplicate integer as switch case
  switch i32 %1, label %match_else [
    i32 0, label %match_case
    i32 0, label %match_case2
  ], !dbg !22
i32 0
LLVM ERROR: Broken function found, compilation aborted!
playpen: application terminated with error code 1
