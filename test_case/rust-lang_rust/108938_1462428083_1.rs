bash
RUST_BACKTRACE=full rustc +dev src/lib.rs -Z treat-err-as-bug=1  > ./full.log 2>&1
