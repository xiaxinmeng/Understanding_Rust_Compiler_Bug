
$ LD_LIBRARY_PATH=~/rust/build/x86_64-unknown-linux-gnu/stage1/lib/ ~/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc nostd.rs -Z time-passes
time: 0.001 s   parsing
time: 0.000 s   gated feature checking
time: 0.000 s   crate injection
time: 0.000 s   configuration 1
time: 0.000 s   plugin loading
time: 0.000 s   plugin registration
time: 6.028 s   expansion
time: 0.688 s   configuration 2
time: 0.732 s   maybe building test harness
time: 0.000 s   prelude injection
time: 0.983 s   assigning node ids and indexing ast
time: 0.079 s   checking that all macro invocations are gone
time: 0.084 s   external crate/lib resolution
time: 0.145 s   language item collection
time: 1.835 s   resolution
time: 0.077 s   lifetime resolution
time: 0.076 s   looking for entry point
time: 0.076 s   looking for plugin registrar
time: 0.215 s   freevar finding
time: 0.466 s   region resolution
time: 0.084 s   loop checking
time: 0.126 s   stability index
time: 0.084 s   type collecting
time: 0.195 s   variance inference
time: 0.184 s   coherence checking
time: 4.880 s   type checking
time: 0.091 s   check static items
time: 0.473 s   const marking
time: 0.074 s   const checking
time: 0.975 s   privacy checking
time: 0.206 s   intrinsic checking
time: 0.252 s   effect checking
time: 0.282 s   match checking
time: 10.996 s  liveness checking
time: 6.158 s   borrow checking
time: 0.519 s   kind checking
time: 0.000 s   reachability checking
time: 0.549 s   death checking
time: 1.482 s   lint checking
time: 0.000 s   resolving dependency formats
time: 4.426 s   translation
  time: 0.380 s llvm function passes
  time: 0.295 s llvm module passes
  time: 8.808 s codegen passes
time: 9.884 s   LLVM passes
  time: 0.152 s running linker
time: 0.169 s   linking
