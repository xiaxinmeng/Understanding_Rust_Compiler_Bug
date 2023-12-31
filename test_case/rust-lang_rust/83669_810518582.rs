plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---
    
    --- stdout
    
    running 3 tests
    test /tmp/mdbook-xBa6QW/attributes/limits.md - Limits::The_ (line 49) ... ignored
    test /tmp/mdbook-xBa6QW/attributes/limits.md - Limits::The_ (line 14) ... ok
    test /tmp/mdbook-xBa6QW/attributes/limits.md - Limits::The_ (line 29) ... FAILED
    failures:
    
    
    ---- /tmp/mdbook-xBa6QW/attributes/limits.md - Limits::The_ (line 29) stdout ----
    Test compiled successfully, but it's marked `compile_fail`.
    failures:
    failures:
        /tmp/mdbook-xBa6QW/attributes/limits.md - Limits::The_ (line 29)
    test result: FAILED. 1 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.17s
    
    
    --- stderr
---
error: tests/compile-fail/unaligned_pointers/reference_to_packed.rs:18: unexpected warning: '18:26: 18:32: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'

error: 2 unexpected errors found, 0 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/unaligned_pointers/reference_to_packed.rs" "-L" "/tmp/compiletest78VQq9" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest78VQq9/unaligned_pointers/reference_to_packed.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-validation" "-Zmiri-disable-stacked-borrows" "-L" "/tmp/compiletest78VQq9/unaligned_pointers/reference_to_packed.stage-id.aux"
    Error {
        line_num: 18,
        kind: Some(
thread '[compile-fail] compile-fail/unaligned_pointers/reference_to_packed.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.5.0/src/runtest.rs:1091:13
---
This PR updated 'src/doc/nomicon', verifying if status is 'test-pass'...
Verifying status of reference...
This PR updated 'src/doc/reference', verifying if status is 'test-pass'...

We detected that this PR updated 'reference', but its tests failed.

If you do intend to update 'reference', please check the error messages above and
commit another update.

If you do NOT intend to update 'reference', please ensure you did not accidentally
change the submodule at 'src/doc/reference'. You may ask your reviewer for the
proper steps.
{"reference":"test-fail","rust-by-example":"test-pass","rustbook":"test-fail","nomicon":"test-pass","edition-guide":"test-pass","rls":"build-fail","miri":"test-fail","book":"test-pass","embedded-book":"test-pass","rustfmt":"build-fail","cargo-miri":"test-fail"}failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 check-tools
