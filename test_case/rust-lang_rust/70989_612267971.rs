llvm
 cmpxchg instructions must be atomic.
  %10 = cmpxchg i64* %9, i64 0, i64 0 unordered not_atomic
in function __llvm_memcpy_element_unordered_atomic_8
LLVM ERROR: Broken function found, compilation aborted!
error: could not compile `compiler_builtins`.
