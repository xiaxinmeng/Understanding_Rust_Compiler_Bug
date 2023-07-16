
thread 'main' panicked at 'assertion failed: position <= slice.len()', libserialize/leb128.rs:97:1
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Could not compile `rustc_llvm`.

Caused by:
  process didn't exit successfully: `/Users/alex/Software/rust/build/bootstrap/debug/rustc --crate-name build_script_build librustc_llvm/build.rs --error-format json --crate-type bin --emit=dep-info,link -C opt-level=2 -C metadata=74f2a810ad96be1d -C extra-filename=-74f2a810ad96be1d --out-dir /Users/alex/Software/rust/build/x86_64-apple-darwin/stage1-rustc/release/build/rustc_llvm-74f2a810ad96be1d -L dependency=/Users/alex/Software/rust/build/x86_64-apple-darwin/stage1-rustc/release/deps --extern build_helper=/Users/alex/Software/rust/build/x86_64-apple-darwin/stage1-rustc/release/deps/libbuild_helper-89aaac40d3077cd7.rlib --extern cc=/Users/alex/Software/rust/build/x86_64-apple-darwin/stage1-rustc/release/deps/libcc-ead7d4af4a69e776.rlib` (exit code: 101)
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/Users/alex/Software/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-j" "8" "--release" "--manifest-path" "/Users/alex/Software/rust/src/librustc_trans/Cargo.toml" "--features" " jemalloc" "--message-format" "json"
expected success, got: exit code: 101
thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1085:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
failed to run: /Users/alex/Software/rust/build/bootstrap/debug/bootstrap -i build
