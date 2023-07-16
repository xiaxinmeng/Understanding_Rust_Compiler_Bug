plain
[00:02:43]       Memory: 8 GB
[00:02:43]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:02:43]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:43]       SMC Version (system): 2.8f0
[00:02:43]       Serial Number (system): VMgjzHPLe8oW
[00:02:43] 
[00:02:43] hw.ncpu: 4
[00:02:43] hw.byteorder: 1234
[00:02:43] hw.memsize: 8589934592
---
[00:54:55] 
[00:54:55] ---- [ui] ui/issue-26548.rs stdout ----
[00:54:55] diff of stderr:
[00:54:55] 
[00:54:55] - error[E0391]: cycle detected when computing layout of `std::option::Option<S>`
[00:54:55] + error[E0391]: cycle detected when computing layout of `S`
[00:54:55] 2    |
[00:54:55] - note: ...which requires computing layout of `S`...
[00:54:55] -    = note: ...which again requires computing layout of `std::option::Option<S>`, completing the cycle
[00:54:55] + note: ...which requires computing layout of `std::option::Option<S>`...
[00:54:55] +    = note: ...which again requires computing layout of `S`, completing the cycle
[00:54:55] 5 note: cycle used when compile_codegen_unit
[00:54:55] 7 error: aborting due to previous error
[00:54:55] 
[00:54:55] 
[00:54:55] The actual stderr differed from the expected stderr.
[00:54:55] The actual stderr differed from the expected stderr.
[00:54:55] Actual stderr saved to /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui/issue-26548/issue-26548.stderr
[00:54:55] To update references, rerun the tests and pass the `--bless` flag
[00:54:55] To only update this specific test, also pass `--test-args issue-26548.rs`
[00:54:55] error: 1 errors occurred comparing output.
[00:54:55] status: exit code: 101
[00:54:55] status: exit code: 101
[00:54:55] command: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/travis/build/rust-lang/rust/src/test/ui/issue-26548.rs" "--target=x86_64-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui/issue-26548/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/ui/issue-26548/auxiliary" "-A" "unused"
[00:54:55] ------------------------------------------
[00:54:55] 
[00:54:55] ------------------------------------------
[00:54:55] stderr:
[00:54:55] stderr:
[00:54:55] ------------------------------------------
[00:54:55] {"message":"cycle detected when computing layout of `S`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n