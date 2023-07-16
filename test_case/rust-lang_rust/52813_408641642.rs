plain
[00:04:15]    Compiling cc v1.0.18
[00:04:15]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:04:15]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:04:15]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:04:18] error[E0425]: cannot find value `nanos` in this scope
[00:04:18]     |
[00:04:18]     |
[00:04:18] 521 |         if !nanos.is_finite() {
[00:04:18]     |             ^^^^^ help: try: `self.nanos`
[00:04:18] 
[00:04:18] error[E0425]: cannot find value `nanos` in this scope
[00:04:18]     |
[00:04:18]     |
[00:04:18] 524 |         if nanos > (u128::MAX as f64) {
[00:04:18]     |            ^^^^^ help: try: `self.nanos`
[00:04:18] 
[00:04:18] error[E0425]: cannot find value `nanos` in this scope
[00:04:18]     |
[00:04:18]     |
[00:04:18] 527 |         let nanos_u128 = nanos.round() as u128;
[00:04:18]     |                          ^^^^^ help: try: `self.nanos`
[00:04:18] 
[00:04:18] error[E0425]: cannot find value `nanos` in this scope
[00:04:18]     |
[00:04:18]     |
[00:04:18] 547 |         if !nanos.is_finite() {
[00:04:18] 
[00:04:18] 
[00:04:18] error[E0425]: cannot find value `nanos` in this scope
[00:04:18]     |
[00:04:18]     |
[00:04:18] 550 |         if nanos > (u128::MAX as f64) {
[00:04:18] 
[00:04:18] 
[00:04:18] error[E0425]: cannot find value `nanos` in this scope
[00:04:18]     |
[00:04:18]     |
[00:04:18] 553 |         let nanos_u128 = nanos.round() as u128;
[00:04:18] 
[00:04:18] 
[00:04:18] error[E0425]: cannot find value `nanos` in this scope
[00:04:18]     |
[00:04:18]     |
[00:04:18] 596 |         if !nanos.is_finite() {
[00:04:18]     |             ^^^^^ help: try: `self.nanos`
[00:04:18] 
[00:04:18] error[E0425]: cannot find value `nanos` in this scope
[00:04:18]     |
[00:04:18]     |
[00:04:18] 599 |         if nanos > (u128::MAX as f64) {
[00:04:18]     |            ^^^^^ help: try: `self.nanos`
[00:04:18] 
[00:04:18] error[E0425]: cannot find value `nanos` in this scope
[00:04:18]     |
[00:04:18]     |
[00:04:18] 602 |         let nanos_u128 = nanos.round() as u128;
[00:04:18]     |                          ^^^^^ help: try: `self.nanos`
[00:04:22]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:04:22]    Compiling cmake v0.1.31
[00:04:22]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:04:26]    Compiling std v0.0.0 (file:///checkout/src/libstd)
