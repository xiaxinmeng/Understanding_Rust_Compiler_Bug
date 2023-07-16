
   Compiling qt_workspace v0.1.0 (file:///D:/sources/personal/rust/qt_workspace)
warning: the option `Z` is unstable and should only be used on the nightly compiler, but it is currently accepted for backwards compatibility; this will soon change, see issue #31847 for more details

time: 0.001; rss: 16MB  parsing
time: 0.000; rss: 16MB  configuration
time: 0.000; rss: 16MB  recursion limit
time: 0.000; rss: 16MB  crate injection
time: 0.000; rss: 16MB  plugin loading
time: 0.000; rss: 16MB  plugin registration
time: 0.034; rss: 48MB  expansion
time: 0.000; rss: 48MB  maybe building test harness
time: 0.000; rss: 48MB  assigning node ids
time: 0.000; rss: 48MB  checking for inline asm in case the target doesn't support it
time: 0.000; rss: 48MB  complete gated feature checking
time: 0.000; rss: 48MB  collecting defs
time: 0.234; rss: 217MB external crate/lib resolution
time: 0.000; rss: 217MB early lint checks
time: 0.000; rss: 217MB AST validation
time: 0.002; rss: 219MB name resolution
time: 0.000; rss: 220MB lowering ast -> hir
time: 0.000; rss: 220MB indexing hir
time: 0.000; rss: 220MB attribute checking
time: 0.000; rss: 220MB language item collection
time: 0.000; rss: 220MB lifetime resolution
time: 0.000; rss: 220MB looking for entry point
time: 0.000; rss: 220MB looking for plugin registrar
time: 0.000; rss: 220MB region resolution
time: 0.000; rss: 220MB loop checking
time: 0.000; rss: 220MB static item recursion checking
time: 0.000; rss: 220MB load_dep_graph
time: 0.001; rss: 221MB type collecting
time: 0.000; rss: 221MB variance inference
time: 0.020; rss: 224MB coherence checking
time: 0.002; rss: 225MB wf checking
time: 0.000; rss: 225MB item-types checking
time: 0.025; rss: 229MB item-bodies checking
time: 0.000; rss: 229MB drop-impl checking
time: 0.001; rss: 229MB const checking
time: 0.000; rss: 229MB privacy checking
time: 0.000; rss: 229MB stability index
time: 0.000; rss: 229MB intrinsic checking
time: 0.000; rss: 229MB effect checking
time: 0.000; rss: 229MB match checking
time: 0.000; rss: 229MB liveness checking
time: 0.000; rss: 229MB rvalue checking
time: 0.001; rss: 230MB MIR dump
time: 0.000; rss: 230MB MIR passes
time: 0.000; rss: 231MB borrow checking
time: 0.000; rss: 231MB reachability checking
time: 0.000; rss: 231MB death checking
time: 0.000; rss: 231MB stability checking
time: 0.000; rss: 231MB unused lib feature checking
warning: unused variable: `text`, #[warn(unused_variables)] on by default
  --> main.rs:26:7
   |
26 |   let text = from_qt_string(&btn.text(AsStruct));
   |       ^^^^

warning: unused variable: `text2`, #[warn(unused_variables)] on by default
  --> main.rs:27:7
   |
27 |   let text2 = from_qt_string(&btn2.text(AsStruct));
   |       ^^^^^

warning: unused variable: `ret`, #[warn(unused_variables)] on by default
  --> main.rs:30:7
   |
30 |   let ret = Application::exec();
   |       ^^^

time: 0.006; rss: 231MB lint checking
time: 0.002; rss: 231MB resolving dependency formats
time: 0.001; rss: 231MB Prepare MIR codegen passes
  time: 0.000; rss: 231MB       write metadata
  time: 0.026; rss: 239MB       translation item collection
  time: 0.000; rss: 239MB       codegen unit partitioning
  time: 0.001; rss: 244MB       internalize symbols
time: 0.164; rss: 244MB translation
time: 0.000; rss: 244MB assert dep graph
time: 0.000; rss: 244MB serialize dep graph
  time: 0.001; rss: 239MB       llvm function passes [0]
  time: 0.000; rss: 239MB       llvm module passes [0]
  time: 0.018; rss: 242MB       codegen passes [0]
  time: 0.000; rss: 242MB       codegen passes [0]
time: 0.021; rss: 242MB LLVM passes
time: 0.000; rss: 242MB serialize work products
  time: 34.444; rss: 242MB      running linker
time: 34.446; rss: 242MB        linking
    Finished debug [unoptimized + debuginfo] target(s) in 35.5 secs
