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
    Checking alloc v0.0.0 (/checkout/library/alloc)
error[E0599]: `std::cmp::Ordering` is not an iterator
   --> library/core/tests/cmp.rs:74:24
    |
74  |     assert_eq!(Greater.cmp(&Less), Greater);
    |                        ^^^ `std::cmp::Ordering` is not an iterator
   ::: /checkout/library/core/src/cmp.rs:340:1
    |
340 | pub enum Ordering {
    | ----------------- doesn't satisfy `std::cmp::Ordering: std::iter::Iterator`
