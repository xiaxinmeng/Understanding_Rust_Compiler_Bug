
$ ./x.py clippy library/std -v
Checking std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
running: "/home/joshua/rustc/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "clippy" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/home/joshua/rustc/library/test/Cargo.toml" "--message-format" "json-render-diagnostics" "--" "--cap-lints" "warn"

... many thousands of lines of output ...

error: duplicate lang item in crate `core` (which `rustc_std_workspace_core` depends on): `bool`.
  |
  = note: the lang item is first defined in crate `core` (which `getrandom` depends on)
  = note: first definition in `core` loaded from /home/joshua/.local/lib/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-2675a9a46b5cec89.rlib
  = note: second definition in `core` loaded from /home/joshua/rustc/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libcore-04b26fe1c15aa843.rmeta
