
$ cargo +nightly rustc --release -- -Ztime-passes -Ccodegen-units=1
   Compiling whee v0.1.0 (file:///home/lampam/asd/clone/cpp_demangle/whee)
  time: 0.000; rss: 46MB        parsing
  time: 0.000; rss: 46MB        recursion limit
  time: 0.000; rss: 46MB        crate injection
  time: 0.000; rss: 46MB        plugin loading
  time: 0.000; rss: 46MB        plugin registration
    time: 0.017; rss: 71MB      expand crate
    time: 0.000; rss: 71MB      check unused macros
  time: 0.017; rss: 71MB        expansion
  time: 0.000; rss: 71MB        maybe building test harness
  time: 0.000; rss: 71MB        maybe creating a macro crate
  time: 0.000; rss: 71MB        creating allocators
  time: 0.000; rss: 71MB        AST validation
  time: 0.001; rss: 72MB        name resolution
  time: 0.000; rss: 72MB        complete gated feature checking
  time: 0.000; rss: 74MB        lowering ast -> hir
  time: 0.000; rss: 74MB        early lint checks
  time: 0.000; rss: 74MB        indexing hir
  time: 0.000; rss: 74MB        load query result cache
  time: 0.000; rss: 74MB        looking for entry point
  time: 0.000; rss: 74MB        looking for plugin registrar
  time: 0.000; rss: 74MB        loop checking
  time: 0.000; rss: 77MB        attribute checking
  time: 0.000; rss: 77MB        stability checking
  time: 0.000; rss: 77MB        type collecting
  time: 0.000; rss: 77MB        outlives testing
  time: 0.000; rss: 77MB        impl wf inference
  time: 0.000; rss: 77MB        coherence checking
  time: 0.000; rss: 77MB        variance testing
  time: 0.000; rss: 77MB        wf checking
  time: 0.000; rss: 77MB        item-types checking
  time: 0.047; rss: 120MB       item-bodies checking
  time: 0.003; rss: 121MB       rvalue promotion
  time: 0.000; rss: 121MB       privacy checking
  time: 0.000; rss: 121MB       intrinsic checking
  time: 0.000; rss: 122MB       match checking
  time: 0.000; rss: 122MB       liveness checking
  time: 0.005; rss: 124MB       borrow checking
  time: 0.000; rss: 124MB       MIR borrow checking
  time: 0.000; rss: 124MB       dumping chalk-like clauses
  time: 0.000; rss: 124MB       MIR effect checking
  time: 0.000; rss: 124MB       death checking
  time: 0.000; rss: 124MB       unused lib feature checking
  time: 0.000; rss: 124MB       lint checking
  time: 0.000; rss: 124MB       resolving dependency formats
    time: 0.000; rss: 124MB     write metadata
    time: 0.057; rss: 134MB     translation item collection
    time: 0.001; rss: 134MB     codegen unit partitioning
    time: 0.000; rss: 134MB     write allocator module
    time: 0.060; rss: 143MB     translate to LLVM IR
    time: 0.000; rss: 143MB     assert dep graph
    time: 0.000; rss: 143MB     serialize dep graph
  time: 0.124; rss: 143MB       translation
    time: 0.034; rss: 142MB     llvm function passes [whee0]
    time: 6.867; rss: 172MB     llvm module passes [whee0]
    time: 55.740; rss: 165MB    codegen passes [whee0]
  time: 62.665; rss: 144MB      LLVM passes
  time: 0.000; rss: 144MB       serialize work products
    time: 0.245; rss: 144MB     running linker
  time: 0.245; rss: 144MB       linking
    Finished release [optimized] target(s) in 63.23 secs
