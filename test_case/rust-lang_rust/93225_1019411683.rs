plain
running 50 tests
..............................F...................
failures:

---- issue_93224 stdout ----
thread 'issue_93224' panicked at 'assertion failed: `(left == right)`
 right: `0`', compiler/rustc_apfloat/src/ieee.rs:2545:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace



failures:
    issue_93224

test result: FAILED. 49 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '-p rustc_apfloat --test ieee'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_apfloat" "--" "--quiet"


Build completed unsuccessfully in 0:23:44
