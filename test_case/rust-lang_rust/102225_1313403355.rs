plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.82
   Compiling unwind v0.0.0 (/checkout/library/unwind)
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking cfg-if v1.0.0
    Checking adler v1.0.2
    Checking rustc-demangle v0.1.21
---
    |
108 |         for a in Some(1) {
    |                  ^^^^^^^
    |
    = note: `-D for-loops-over-fallibles` implied by `-D warnings`
    |
    |
108 |         while let Some(a) = Some(1) {
help: consider using `if let` to clear intent
    |
    |
108 |         if let Some(a) = Some(1) {

error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:02:55
