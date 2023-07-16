
[01:07:43] running 23 tests
[01:07:43] ....FFFFFF.F.FFF.......
[01:07:43] failures:
[01:07:43]
[01:07:43] ---- test::glb_bound_free_infer stdout ----
[01:07:43]  thread 'test::glb_bound_free_infer' panicked at 'unexpected error computing LUB: RegionsInsufficientlyPolymorphic(BrAnon(1), '_#1r)', /checkout/src/librustc_driver/test.rs:418:22
[01:07:43] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:07:43]
[01:07:43] ---- test::glb_bound_free stdout ----
[01:07:43]  thread 'test::glb_bound_free' panicked at 'unexpected error computing LUB: RegionsInsufficientlyPolymorphic(BrAnon(1), ReFree(DefId { krate: CrateNum(0), node: DefIndex(0) => test_crate/8cd878b }, BrAnon(1)))', /checkout/src/l
ibrustc_driver/test.rs:418:22
[01:07:43]
[01:07:43] ---- test::glb_bound_static stdout ----
[01:07:43]  thread 'test::glb_bound_static' panicked at 'unexpected error computing LUB: RegionsInsufficientlyPolymorphic(BrAnon(1), ReStatic)', /checkout/src/librustc_driver/test.rs:418:22
[01:07:43]
[01:07:43] ---- test::glb_free_free_with_common_scope stdout ----
[01:07:43]  thread 'test::glb_free_free_with_common_scope' panicked at 'Unexpected error: mismatched types Expected: []', /checkout/src/librustc_driver/test.rs:78:12
[01:07:43]
[01:07:43] ---- test::lub_bound_bound_inverse_order stdout ----
[01:07:43]  thread 'test::lub_bound_bound_inverse_order' panicked at 'unexpected error in LUB: expected bound lifetime parameterBrAnon(2), found concrete lifetime', /checkout/src/librustc_driver/test.rs:410:26
[01:07:43]
[01:07:43] ---- test::lub_bound_free stdout ----
[01:07:43]  thread 'test::lub_bound_free' panicked at 'unexpected error in LUB: expected bound lifetime parameterBrAnon(1), found concrete lifetime', /checkout/src/librustc_driver/test.rs:410:26
[01:07:43]
[01:07:43] ---- test::lub_bound_static stdout ----
[01:07:43]  thread 'test::lub_bound_static' panicked at 'unexpected error in LUB: expected bound lifetime parameterBrAnon(1), found concrete lifetime', /checkout/src/librustc_driver/test.rs:410:26
[01:07:43]
[01:07:43] ---- test::lub_free_bound_infer stdout ----
[01:07:43]  thread 'test::lub_free_bound_infer' panicked at 'unexpected error in LUB: expected bound lifetime parameterBrAnon(1), found concrete lifetime', /checkout/src/librustc_driver/test.rs:410:26
[01:07:43]
[01:07:43] ---- test::lub_returning_scope stdout ----
[01:07:43]  thread 'test::lub_returning_scope' panicked at 'Unexpected error: mismatched types Expected: []', /checkout/src/librustc_driver/test.rs:78:12
[01:07:43]
[01:07:43] ---- test::lub_free_free stdout ----
[01:07:43]  thread 'test::lub_free_free' panicked at 'Unexpected error: mismatched types Expected: []', /checkout/src/librustc_driver/test.rs:78:12
[01:07:43]
[01:07:43]
[01:07:43] failures:
[01:07:43]     test::glb_bound_free
[01:07:43]     test::glb_bound_free_infer
[01:07:43]     test::glb_bound_static
[01:07:43]     test::glb_free_free_with_common_scope
[01:07:43]     test::lub_bound_bound_inverse_order
[01:07:43]     test::lub_bound_free
[01:07:43]     test::lub_bound_static
[01:07:43]     test::lub_free_bound_infer
[01:07:43]     test::lub_free_free
[01:07:43]     test::lub_returning_scope
