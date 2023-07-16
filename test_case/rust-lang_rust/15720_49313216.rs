
stage1 before the patch
time: 0.216 s   parsing
time: 0.010 s   gated feature checking
time: 0.000 s   crate injection
time: 0.034 s   configuration 1
time: 0.001 s   plugin loading
time: 0.000 s   plugin registration
time: 0.487 s   expansion
time: 0.038 s   configuration 2
time: 0.036 s   maybe building test harness
time: 0.000 s   prelude injection
time: 0.044 s   assigning node ids and indexing ast
time: 0.003 s   checking that all macro invocations are gone
time: 0.008 s   external crate/lib resolution
time: 0.007 s   language item collection
time: 0.095 s   resolution
time: 0.004 s   lifetime resolution
time: 0.000 s   looking for entry point
time: 0.004 s   looking for plugin registrar
time: 0.006 s   freevar finding
time: 0.011 s   region resolution
time: 0.004 s   loop checking
time: 0.008 s   stability index
time: 0.024 s   type collecting
time: 0.007 s   variance inference
time: 0.065 s   coherence checking
time: 1.416 s   type checking
time: 0.005 s   check static items
time: 0.013 s   const marking
time: 0.004 s   const checking
time: 0.041 s   privacy checking
time: 0.007 s   intrinsic checking
time: 0.006 s   effect checking
time: 0.039 s   match checking
time: 0.016 s   liveness checking
time: 0.139 s   borrow checking
time: 0.031 s   kind checking
time: 0.006 s   reachability checking
time: 0.020 s   death checking
time: 0.106 s   lint checking
time: 0.000 s   resolving dependency formats
time: 2.257 s   translation
  time: 0.356 s llvm function passes
  time: 6.334 s llvm module passes
  time: 3.113 s codegen passes
time: 9.950 s   LLVM passes
  time: 0.096 s running linker
time: 0.870 s   linking

real    0m16.262s
user    0m15.487s
sys 0m0.343s

stage1 with the patch
time: 0.226 s   parsing
time: 0.010 s   gated feature checking
time: 0.000 s   crate injection
time: 0.036 s   configuration 1
time: 0.001 s   plugin loading
time: 0.000 s   plugin registration
time: 0.496 s   expansion
time: 0.040 s   configuration 2
time: 0.038 s   maybe building test harness
time: 0.000 s   prelude injection
time: 0.044 s   assigning node ids and indexing ast
time: 0.003 s   checking that all macro invocations are gone
time: 0.009 s   external crate/lib resolution
time: 0.007 s   language item collection
time: 0.091 s   resolution
time: 0.004 s   lifetime resolution
time: 0.000 s   looking for entry point
time: 0.004 s   looking for plugin registrar
time: 0.007 s   freevar finding
time: 0.010 s   region resolution
time: 0.004 s   loop checking
time: 0.007 s   stability index
time: 0.023 s   type collecting
time: 0.007 s   variance inference
time: 0.066 s   coherence checking
time: 1.392 s   type checking
time: 0.005 s   check static items
time: 0.015 s   const marking
time: 0.004 s   const checking
time: 0.042 s   privacy checking
time: 0.007 s   intrinsic checking
time: 0.006 s   effect checking
time: 0.041 s   match checking
time: 0.016 s   liveness checking
time: 0.135 s   borrow checking
time: 0.033 s   kind checking
time: 0.007 s   reachability checking
time: 0.020 s   death checking
time: 0.101 s   lint checking
time: 0.000 s   resolving dependency formats
time: 2.226 s   translation
  time: 0.352 s llvm function passes
  time: 6.361 s llvm module passes
  time: 3.167 s codegen passes
time: 10.025 s  LLVM passes
  time: 0.101 s running linker
time: 0.749 s   linking

real    0m16.169s
user    0m15.493s
sys 0m0.347s
