
[01:50:36] Building stage2 tool miri (x86_64-unknown-linux-gnu)

[01:50:37] error: Package `miri v0.1.0 (/checkout/src/tools/miri)` does not have these features: `rustc-workspace-hack`

[01:50:37] 

[01:50:37] 

[01:50:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/miri/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json"

[01:50:37] expected success, got: exit code: 101

[01:50:37] Unable to build miri, skipping dist

[01:57:40] Build completed successfully in 1:53:28
