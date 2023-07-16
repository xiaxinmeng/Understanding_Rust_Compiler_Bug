plain
    
    --- stdout
    
    running 6 tests
    test C:\Users\RUNNER~1\AppData\Local\Temp\mdbook-5VCFHM\rust-2021/disjoint-capture-in-closures.md - Disjoint_capture_in_closures::Details (line 19) ... ignored
    test C:\Users\RUNNER~1\AppData\Local\Temp\mdbook-5VCFHM\rust-2021/disjoint-capture-in-closures.md - Disjoint_capture_in_closures::Migration::Wild_Card_Patterns (line 68) ... ok
    test C:\Users\RUNNER~1\AppData\Local\Temp\mdbook-5VCFHM\rust-2021/disjoint-capture-in-closures.md - Disjoint_capture_in_closures::Migration::Drop_Order (line 102) ... ok
    test C:\Users\RUNNER~1\AppData\Local\Temp\mdbook-5VCFHM\rust-2021/disjoint-capture-in-closures.md - Disjoint_capture_in_closures::Migration::Drop_Order (line 89) ... ok
    test C:\Users\RUNNER~1\AppData\Local\Temp\mdbook-5VCFHM\rust-2021/disjoint-capture-in-closures.md - Disjoint_capture_in_closures::Migration (line 51) ... ok
    test C:\Users\RUNNER~1\AppData\Local\Temp\mdbook-5VCFHM\rust-2021/disjoint-capture-in-closures.md - Disjoint_capture_in_closures::Migration::Trait_implementations (line 150) ... FAILED
    failures:
    
    
    ---- C:\Users\RUNNER~1\AppData\Local\Temp\mdbook-5VCFHM\rust-2021/disjoint-capture-in-closures.md - Disjoint_capture_in_closures::Migration::Trait_implementations (line 150) stdout ----
    Test executable failed (exit code -1073740791).
    
    failures:
    failures:
        C:\Users\RUNNER~1\AppData\Local\Temp\mdbook-5VCFHM\rust-2021/disjoint-capture-in-closures.md - Disjoint_capture_in_closures::Migration::Trait_implementations (line 150)
    test result: FAILED. 4 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 1.83s
    
    
    --- stderr
---
Verifying status of rls...
Verifying status of miri...
Verifying status of embedded-book...
Cloning into 'rust-toolstate'...
error: Tool `edition-guide` has regressed from test-pass to test-fail during beta week.
{"rls":"test-pass","rust-by-example":"test-pass","nomicon":"test-pass","rustbook":"test-fail","miri":"test-pass","reference":"test-pass","embedded-book":"test-pass","book":"test-pass","cargo-miri":"test-fail","edition-guide":"test-fail"}Build completed unsuccessfully in 0:00:01
