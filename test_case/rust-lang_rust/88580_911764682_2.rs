
  time: 0.000; rss: 48MB        parsing
  time: 0.000; rss: 53MB        recursion limit
  time: 0.000; rss: 53MB        crate injection
  time: 0.000; rss: 53MB        plugin loading
  time: 0.000; rss: 53MB        plugin registration
  time: 0.012; rss: 70MB        expansion
  time: 0.000; rss: 70MB        maybe building test harness
  time: 0.000; rss: 70MB        maybe creating a macro crate
  time: 0.000; rss: 70MB        creating allocators
  time: 0.000; rss: 70MB        checking for inline asm in case the target doesn't support it
  time: 0.000; rss: 74MB        AST validation
  time: 0.002; rss: 78MB        name resolution
  time: 0.000; rss: 78MB        complete gated feature checking
  time: 0.000; rss: 78MB        lowering ast -> hir
  time: 0.000; rss: 78MB        early lint checks
  time: 0.000; rss: 78MB        indexing hir
  time: 0.000; rss: 78MB        attribute checking
  time: 0.000; rss: 78MB        load query result cache
  time: 0.000; rss: 78MB        looking for entry point
  time: 0.000; rss: 78MB        looking for plugin registrar
  time: 0.000; rss: 78MB        loop checking
  time: 0.000; rss: 78MB        static item recursion checking
  time: 0.000; rss: 78MB        stability checking
  time: 0.000; rss: 82MB        type collecting
  time: 0.000; rss: 82MB        outlives testing
  time: 0.000; rss: 82MB        impl wf inference
  time: 0.000; rss: 82MB        coherence checking
  time: 0.000; rss: 82MB        variance testing
  time: 0.000; rss: 82MB        wf checking
  time: 0.000; rss: 82MB        item-types checking
  time: 0.013; rss: 105MB       item-bodies checking
  time: 0.000; rss: 105MB       const checking
  time: 0.000; rss: 105MB       privacy checking
  time: 0.000; rss: 105MB       intrinsic checking
  time: 0.000; rss: 105MB       match checking
  time: 0.000; rss: 105MB       liveness checking
  time: 0.001; rss: 105MB       borrow checking
  time: 0.000; rss: 105MB       MIR borrow checking
  time: 0.000; rss: 105MB       MIR effect checking
  time: 0.000; rss: 105MB       death checking
  time: 0.000; rss: 105MB       unused lib feature checking
  time: 0.000; rss: 105MB       lint checking
  time: 0.000; rss: 105MB       resolving dependency formats
    time: 0.000; rss: 105MB     write metadata
    time: 0.007; rss: 105MB     translation item collection
    time: 0.000; rss: 105MB     codegen unit partitioning
    time: 0.001; rss: 124MB     write allocator module
    time: 0.002; rss: 135MB     llvm function passes [a3]
    time: 0.000; rss: 141MB     llvm function passes [a1]
    time: 0.002; rss: 143MB     llvm module passes [a1]
    time: 0.013; rss: 145MB     translate to LLVM IR
    time: 0.000; rss: 143MB     assert dep graph
    time: 0.000; rss: 143MB     serialize dep graph
    time: 0.001; rss: 143MB     llvm function passes [a2]
  time: 0.067; rss: 143MB       translation
    time: 0.004; rss: 143MB     llvm function passes [a0]
    time: 0.001; rss: 148MB     llvm function passes [a4]
    time: 0.001; rss: 147MB     llvm module passes [a2]
    time: 0.000; rss: 145MB     llvm function passes [a5]
    time: 0.001; rss: 145MB     llvm module passes [a5]
    time: 0.014; rss: 145MB     llvm module passes [a3]
    time: 0.009; rss: 147MB     llvm module passes [a4]
    time: 0.012; rss: 145MB     llvm module passes [a0]
    time: 0.001; rss: 145MB     LTO passes
    time: 0.001; rss: 145MB     LTO passes
    time: 0.001; rss: 145MB     codegen passes [a5-317d481089b8c8fe83113de504472633.rs]
    time: 0.002; rss: 145MB     LTO passes
    time: 0.002; rss: 145MB     codegen passes [a2-317d481089b8c8fe83113de504472633.rs]
    time: 0.002; rss: 148MB     codegen passes [a1-317d481089b8c8fe83113de504472633.rs]
    time: 0.007; rss: 148MB     LTO passes
    time: 0.011; rss: 148MB     LTO passes
    time: 0.005; rss: 148MB     codegen passes [a0-317d481089b8c8fe83113de504472633.rs]
    time: 0.006; rss: 150MB     codegen passes [a3-317d481089b8c8fe83113de504472633.rs]
    time: 240.300; rss: 190MB   LTO passes
    time: 6.298; rss: 186MB     codegen passes [a4-317d481089b8c8fe83113de504472633.rs]
  time: 246.644; rss: 160MB     LLVM passes
  time: 0.000; rss: 156MB       serialize work products
    time: 0.246; rss: 156MB     running linker
  time: 0.246; rss: 156MB       linking
