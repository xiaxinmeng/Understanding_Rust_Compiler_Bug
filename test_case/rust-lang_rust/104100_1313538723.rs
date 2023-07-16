plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.82
   Compiling unwind v0.0.0 (/checkout/library/unwind)
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v1.0.0
   Compiling adler v1.0.2
   Compiling rustc-demangle v0.1.21
---
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.82
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: missing `~const` qualifier for specialization
    |
    |
634 | impl<A: ~const Step + TrustedStep + ~const Destruct> const RangeIteratorImpl for ops::Range<A> {

error: could not compile `core` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:03:01
