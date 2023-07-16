plain
   Compiling rustc_index v0.0.0 (/checkout/compiler/rustc_index)
error: unreachable expression
  --> compiler/rustc_index/src/interval.rs:73:9
   |
73 | /         let Some(mut end) = inclusive_end(self.domain, range) else {
74 | |             // empty range
   | |             ------------ any code following this expression is unreachable
76 | |         };
   | |__________^ unreachable expression
   |
   |
   = note: `-D unreachable-code` implied by `-D warnings`
error: unused variable: `val`
  |
  |
  = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `rustc_index` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:05:52
