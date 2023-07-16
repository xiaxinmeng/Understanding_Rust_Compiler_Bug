
failures:
    test_decode_option_none

test result: FAILED. 69 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass '-p rustc_serialize --test json'


command did not execute successfully: "/home/joshua/rustc/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--features" " llvm" "--manifest-path" "/home/joshua/rustc/compiler/rustc/Cargo.toml" "-p" "rustc_serialize" "--" "--quiet"
expected success, got: exit status: 101


note: failed while building bootstrap::test::Crate
help: to replicate this failure, run `./x.py test library/alloc`
