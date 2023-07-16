
warning: build failed, waiting for other jobs to finish...
error: internal compiler error: src/librustc/dep_graph/graph.rs:687: DepNode Hir(bend[3a80]::{{impl}}[5]::from_request_inner[0]::{{misc}}[0]::{{misc}}[0]) should have been pre-allocated but wasn't.

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:650:9
stack backtrace:
   0: <unknown>
   1: <unknown>
   2: <unknown>
   3: <unknown>
   4: <unknown>
   5: <unknown>
   6: <unknown>
   7: <unknown>
   8: <unknown>
   9: <unknown>
  10: <unknown>
  11: <unknown>
  12: <unknown>
  13: <unknown>
  14: <unknown>
  15: <unknown>
  16: <unknown>
  17: <unknown>
  18: <unknown>
  19: <unknown>
  20: <unknown>
  21: <unknown>
  22: <unknown>
  23: <unknown>
  24: <unknown>
  25: <unknown>
  26: <unknown>
  27: <unknown>
  28: <unknown>
  29: <unknown>
  30: <unknown>
  31: <unknown>
  32: <unknown>
  33: <unknown>
  34: <unknown>
  35: <unknown>
  36: <unknown>
  37: <unknown>
query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error
