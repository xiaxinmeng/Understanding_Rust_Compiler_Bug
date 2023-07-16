plain
[00:03:28]       Memory: 8 GB
[00:03:28]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:03:28]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:28]       SMC Version (system): 2.8f0
[00:03:28]       Serial Number (system): VMomKGnSy3aJ
[00:03:28] 
[00:03:28] hw.ncpu: 4
[00:03:28] hw.byteorder: 1234
[00:03:28] hw.memsize: 8589934592
---
[01:01:27] diff of stderr:
[01:01:27] 
[01:01:27] 2   --> $DIR/issue-54006.rs:16:5
[01:01:27] 3    |
[01:01:27] 4 LL | use alloc::vec;
[01:01:27] -    |     ^^^^^ Did you mean `std::alloc`?
[01:01:27] +    |     ^^^^^ Did you mean `core::alloc`?
[01:01:27] 7 error: cannot determine resolution for the macro `vec`
[01:01:27] 8   --> $DIR/issue-54006.rs:20:18
[01:01:27] 
[01:01:27] 
[01:01:27] 
[01:01:27] The actual stderr differed from the expected stderr.
[01:01:27] Actual stderr saved to /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/ui/rust-2018/issue-54006/issue-54006.stderr
[01:01:27] To update references, rerun the tests and pass the `--bless` flag
[01:01:27] To only update this specific test, also pass `--test-args rust-2018/issue-54006.rs`
[01:01:27] error: 1 errors occurred comparing output.
[01:01:27] status: exit code: 1
[01:01:27] status: exit code: 1
[01:01:27] command: "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/bin/rustc" "/Users/travis/build/rust-lang/rust/src/test/ui/rust-2018/issue-54006.rs" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/ui/rust-2018/issue-54006/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/native/rust-test-helpers" "--edition=2018" "-L" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/ui/rust-2018/issue-54006/auxiliary" "-A" "unused"
[01:01:27] ------------------------------------------
[01:01:27] 
[01:01:27] ------------------------------------------
[01:01:27] stderr:
[01:01:27] stderr:
[01:01:27] ------------------------------------------
[01:01:27] {"message":"unresolved import `alloc`","code":{"code":"E0432","explanation":"\nAn import was unresolved.\n\nErroneous code example:\n\n