
$ cargo +nightly rustc --release -- -Ztime-passes
   Compiling whee v0.1.0 (file:///home/lampam/asd/clone/cpp_demangle/whee)
  time: 0.000; rss: 46MB        parsing
  time: 0.000; rss: 46MB        recursion limit
  time: 0.000; rss: 46MB        crate injection
  time: 0.000; rss: 46MB        plugin loading
  time: 0.000; rss: 46MB        plugin registration
    time: 0.020; rss: 71MB      expand crate
    time: 0.000; rss: 71MB      check unused macros
  time: 0.020; rss: 71MB        expansion
  time: 0.000; rss: 71MB        maybe building test harness
  time: 0.000; rss: 71MB        maybe creating a macro crate
  time: 0.000; rss: 71MB        creating allocators
  time: 0.000; rss: 71MB        AST validation
  time: 0.002; rss: 72MB        name resolution
  time: 0.000; rss: 72MB        complete gated feature checking
  time: 0.001; rss: 73MB        lowering ast -> hir
  time: 0.000; rss: 73MB        early lint checks
  time: 0.000; rss: 73MB        indexing hir
  time: 0.000; rss: 73MB        load query result cache
  time: 0.000; rss: 73MB        looking for entry point
  time: 0.000; rss: 73MB        looking for plugin registrar
  time: 0.000; rss: 73MB        loop checking
  time: 0.000; rss: 76MB        attribute checking
  time: 0.000; rss: 76MB        stability checking
  time: 0.000; rss: 76MB        type collecting
  time: 0.000; rss: 76MB        outlives testing
  time: 0.000; rss: 76MB        impl wf inference
  time: 0.000; rss: 76MB        coherence checking
  time: 0.000; rss: 76MB        variance testing
  time: 0.000; rss: 80MB        wf checking
  time: 0.000; rss: 80MB        item-types checking
  time: 0.048; rss: 120MB       item-bodies checking
  time: 0.003; rss: 121MB       rvalue promotion
  time: 0.000; rss: 121MB       privacy checking
  time: 0.000; rss: 121MB       intrinsic checking
  time: 0.000; rss: 122MB       match checking
  time: 0.000; rss: 122MB       liveness checking
  time: 0.005; rss: 123MB       borrow checking
  time: 0.000; rss: 123MB       MIR borrow checking
  time: 0.000; rss: 123MB       dumping chalk-like clauses
  time: 0.000; rss: 123MB       MIR effect checking
  time: 0.000; rss: 123MB       death checking
  time: 0.000; rss: 123MB       unused lib feature checking
  time: 0.000; rss: 123MB       lint checking
  time: 0.000; rss: 123MB       resolving dependency formats
    time: 0.000; rss: 123MB     write metadata
    time: 0.056; rss: 133MB     translation item collection
    time: 0.001; rss: 133MB     codegen unit partitioning
    time: 0.000; rss: 134MB     write allocator module
    time: 0.015; rss: 142MB     llvm function passes [whee1]
    time: 0.003; rss: 146MB     llvm function passes [whee13]
    time: 0.017; rss: 146MB     llvm function passes [whee0]
    time: 0.004; rss: 148MB     llvm function passes [whee4]
    time: 0.022; rss: 149MB     llvm module passes [whee4]
    time: 0.007; rss: 149MB     llvm function passes [whee3]
    time: 0.021; rss: 150MB     llvm module passes [whee3]
    time: 0.092; rss: 150MB     llvm module passes [whee13]
    time: 0.007; rss: 150MB     llvm function passes [whee5]
    time: 0.002; rss: 150MB     llvm function passes [whee2]
    time: 0.011; rss: 150MB     llvm module passes [whee2]
    time: 0.002; rss: 151MB     llvm function passes [whee7]
    time: 0.006; rss: 151MB     llvm module passes [whee7]
    time: 0.002; rss: 152MB     llvm function passes [whee9]
    time: 0.011; rss: 152MB     llvm module passes [whee9]
    time: 0.002; rss: 152MB     llvm function passes [whee11]
    time: 0.056; rss: 152MB     llvm module passes [whee5]
    time: 0.008; rss: 152MB     llvm module passes [whee11]
    time: 0.001; rss: 152MB     llvm function passes [whee10]
    time: 0.002; rss: 153MB     llvm function passes [whee6]
    time: 0.004; rss: 153MB     llvm module passes [whee10]
    time: 0.007; rss: 153MB     llvm module passes [whee6]
    time: 0.002; rss: 154MB     llvm function passes [whee8]
    time: 0.003; rss: 154MB     llvm module passes [whee8]
    time: 0.001; rss: 154MB     llvm function passes [whee12]
    time: 0.090; rss: 154MB     translate to LLVM IR
    time: 0.000; rss: 154MB     assert dep graph
    time: 0.000; rss: 154MB     serialize dep graph
  time: 0.288; rss: 154MB       translation
    time: 0.002; rss: 153MB     llvm module passes [whee12]
    time: 0.001; rss: 151MB     llvm function passes [whee14]
    time: 0.001; rss: 152MB     llvm module passes [whee14]
    time: 0.001; rss: 152MB     llvm function passes [whee15]
    time: 0.002; rss: 152MB     llvm module passes [whee15]
    time: 0.190; rss: 152MB     llvm module passes [whee0]
    time: 6.645; rss: 183MB     llvm module passes [whee1]
    time: 0.021; rss: 159MB     LTO passes
    time: 0.013; rss: 161MB     codegen passes [whee9-faa1bfdce812aee96e5ffecf666ddd72.rs]
    time: 0.036; rss: 160MB     LTO passes
    time: 0.003; rss: 161MB     LTO passes
    time: 0.004; rss: 162MB     codegen passes [whee12-faa1bfdce812aee96e5ffecf666ddd72.rs]
    time: 0.015; rss: 164MB     codegen passes [whee5-faa1bfdce812aee96e5ffecf666ddd72.rs]
    time: 0.011; rss: 164MB     LTO passes
    time: 0.007; rss: 165MB     LTO passes
    time: 0.012; rss: 167MB     codegen passes [whee2-faa1bfdce812aee96e5ffecf666ddd72.rs]
    time: 0.007; rss: 167MB     codegen passes [whee4-faa1bfdce812aee96e5ffecf666ddd72.rs]
    time: 0.073; rss: 166MB     LTO passes
    time: 0.008; rss: 170MB     LTO passes
    time: 0.020; rss: 168MB     LTO passes
    time: 0.008; rss: 168MB     codegen passes [whee7-faa1bfdce812aee96e5ffecf666ddd72.rs]
    time: 0.010; rss: 167MB     codegen passes [whee13-faa1bfdce812aee96e5ffecf666ddd72.rs]
    time: 0.006; rss: 167MB     LTO passes
    time: 0.004; rss: 167MB     codegen passes [whee3-faa1bfdce812aee96e5ffecf666ddd72.rs]
    time: 0.004; rss: 168MB     LTO passes
    time: 0.004; rss: 171MB     codegen passes [whee10-faa1bfdce812aee96e5ffecf666ddd72.rs]
    time: 0.005; rss: 172MB     LTO passes
    time: 0.003; rss: 173MB     LTO passes
    time: 0.006; rss: 173MB     codegen passes [whee6-faa1bfdce812aee96e5ffecf666ddd72.rs]
    time: 0.004; rss: 173MB     codegen passes [whee15-faa1bfdce812aee96e5ffecf666ddd72.rs]
    time: 0.002; rss: 173MB     LTO passes
    time: 0.003; rss: 174MB     codegen passes [whee8-faa1bfdce812aee96e5ffecf666ddd72.rs]
    time: 0.060; rss: 173MB     codegen passes [whee0-faa1bfdce812aee96e5ffecf666ddd72.rs]
    time: 0.004; rss: 170MB     LTO passes
    time: 0.001; rss: 170MB     LTO passes
    time: 0.002; rss: 170MB     codegen passes [whee11-faa1bfdce812aee96e5ffecf666ddd72.rs]
    time: 0.001; rss: 170MB     codegen passes [whee14-faa1bfdce812aee96e5ffecf666ddd72.rs]
    time: 3.249; rss: 180MB     LTO passes
    time: 51.579; rss: 174MB    codegen passes [whee1-faa1bfdce812aee96e5ffecf666ddd72.rs]
  time: 61.759; rss: 154MB      LLVM passes
  time: 0.000; rss: 151MB       serialize work products
    time: 0.250; rss: 150MB     running linker
  time: 0.250; rss: 150MB       linking
    Finished release [optimized] target(s) in 62.30 secs
