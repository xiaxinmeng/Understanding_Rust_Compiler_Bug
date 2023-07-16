
warning: struct is never used: `Headers`, #[warn(dead_code)] on by default
   --> ethcore\src\verification\queue\kind.rs:167:2
    |
167 |   pub struct Headers;
    |   ^^^^^^^^^^^^^^^^^^^

EH pad must be jumped to via an unwind edge
  %cleanuppad13 = cleanuppad within none []
  br i1 %1927, label %bb99, label %bb102_cleanup_trampoline_bb99, !dbg !727554
LLVM ERROR: Broken function found, compilation aborted!
error: Could not compile `ethcore`.
