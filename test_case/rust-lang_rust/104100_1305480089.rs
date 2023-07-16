plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.82
   Compiling unwind v0.0.0 (/checkout/library/unwind)
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking cfg-if v1.0.0
    Checking adler v0.2.3
    Checking rustc-demangle v0.1.21
---
    Checking core v0.0.0 (/checkout/library/core)
error: any use of this value will cause an error
   --> /checkout/library/core/src/iter/range.rs:816:9
    |
816 |         self.spec_next()
    |         |
    |         |
    |         calling non-const function `<std::ops::Range<usize> as std::iter::range::RangeIteratorImpl>::spec_next`
    |         inside `std::iter::range::<impl std::iter::Iterator for std::ops::Range<usize>>::next` at /checkout/library/core/src/iter/range.rs:816:9
    |         inside `iter::test_const_iter_range::X` at library/core/tests/iter/mod.rs:123:18
   ::: library/core/tests/iter/mod.rs:121:5
    |
121 |     const X: usize = {
    |     --------------
---

error: any use of this value will cause an error
   --> library/core/tests/iter/mod.rs:129:27
    |
129 |     const _: () = assert!(X == 10);
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

