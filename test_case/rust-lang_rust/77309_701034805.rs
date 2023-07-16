
running: "/home/joshua/rustc/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "clippy" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/home/joshua/rustc/library/test/Cargo.toml" "--message-format" "json-render-diagnostics" "--" "--cap-lints" "warn" "--sysroot" "/home/joshua/rustc/build/x86_64-unknown-linux-gnu/stage0-sysroot"
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `/home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/clippy-driver /home/joshua/rustc/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=cfg` (exit code: 1)
  --- stderr
  error: Option 'sysroot' given more than once
