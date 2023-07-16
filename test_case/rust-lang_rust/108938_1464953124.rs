bash
RUST_BACKTRACE=1 rustc +dev tests/run-make/short-ice/src/lib.rs -Z treat-err-as-bug=1
