bash
RUST_BACKTRACE=1 rustc +dev src/lib.rs -Z treat-err-as-bug=1  > ./short.log 2>&1
