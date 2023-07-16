
ASAN_OPTIONS=detect_stack_use_after_return=1 RUSTFLAGS="-Zsanitizer=address -Copt-level=2" cargo +nightly run
