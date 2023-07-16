
rustc -Cpanic=unwind -Cdebuginfo=1 a.rs && env RUST_BACKTRACE=full ./a
