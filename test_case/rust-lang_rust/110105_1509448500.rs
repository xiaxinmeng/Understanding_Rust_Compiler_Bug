plain
    
    --- stdout
    
    running 5 tests
    test /tmp/mdbook-scKKm5/exotic-sizes.md - Exotically_Sized_Types::Dynamically_Sized_Types__DSTs_ (line 35) ... ok
    test /tmp/mdbook-scKKm5/exotic-sizes.md - Exotically_Sized_Types::Zero_Sized_Types__ZSTs_ (line 72) ... ok
    test /tmp/mdbook-scKKm5/exotic-sizes.md - Exotically_Sized_Types::Empty_Types (line 122) ... ok
    test /tmp/mdbook-scKKm5/exotic-sizes.md - Exotically_Sized_Types::Empty_Types (line 142) - compile fail ... FAILED
    test /tmp/mdbook-scKKm5/exotic-sizes.md - Exotically_Sized_Types::Dynamically_Sized_Types__DSTs_ (line 47) ... ok
    failures:
    
    
    ---- /tmp/mdbook-scKKm5/exotic-sizes.md - Exotically_Sized_Types::Empty_Types (line 142) stdout ----
    Test compiled successfully, but it's marked `compile_fail`.
    failures:
    failures:
        /tmp/mdbook-scKKm5/exotic-sizes.md - Exotically_Sized_Types::Empty_Types (line 142)
    test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s
    
    
    --- stderr
---
 finished in 1.356 seconds

1 command(s) did not execute successfully:

  - LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin:/node-v14.20.0-linux-x64/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin" RUSTC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc" RUSTC_BOOTSTRAP="1" RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustbook" "test" "/checkout/src/doc/nomicon"
Build completed unsuccessfully in 0:10:27
{"embedded-book":"test-pass","reference":"test-pass","rust-by-example":"test-pass","rustbook":"test-fail","nomicon":"test-fail","edition-guide":"test-pass","book":"test-pass"}Building bootstrap
    Finished dev [unoptimized] target(s) in 0.03s
Verifying status of book...
Verifying status of book...
Verifying status of nomicon...
Verifying status of reference...
Verifying status of rust-by-example...
Verifying status of edition-guide...
Verifying status of embedded-book...
Cloning into 'rust-toolstate'...
error: Tool `nomicon` has regressed from test-pass to test-fail during beta week.
