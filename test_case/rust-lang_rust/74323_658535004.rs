
     Running `build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo-miri miri setup`
ERROR: the sysroot can't be built for the Beta channel. Switch to nightly.
fatal error: Failed to run xargo


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "run" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/miri/cargo-miri/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--" "miri" "setup"
expected success, got: exit code: 1


[TIMING] Miri { stage: 2, host: "x86_64-unknown-linux-gnu" } -- 33.283

2 command(s) did not execute successfully:

  - "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustfmt/Cargo.toml" "--features" "rustc-workspace-hack/all-static"

  - "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "run" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/miri/cargo-miri/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--" "miri" "setup"

failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --no-fail-fast src/doc/book src/doc/nomicon src/doc/reference src/doc/rust-by-example src/doc/embedded-book src/doc/edition-guide src/tools/rls src/tools/rustfmt src/tools/miri
Build completed unsuccessfully in 0:33:37
