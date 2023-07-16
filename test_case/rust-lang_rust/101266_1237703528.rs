plain
Caused by:
  could not parse input as TOML

Caused by:
  TOML parse error at line 19, column 1
     |
  19 | rustc_target = { path = "../rustc_target" }
     | ^
  Duplicate key `rustc_target` in table `dependencies`

thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--no-deps" "--manifest-path" "/checkout/Cargo.toml"
expected success, got: exit status: 101', metadata.rs:39:18
Build completed unsuccessfully in 0:01:06
