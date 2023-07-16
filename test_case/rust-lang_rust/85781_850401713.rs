plain
platform support check
   Compiling tier-check v0.1.0 (/checkout/src/tools/tier-check)
    Finished release [optimized] target(s) in 0.84s
     Running `build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/tier-check /checkout/src/doc/rustc/src/platform-support.md /checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc`
error: target `aarch64-apple-ios-sim` is missing from platform-support.md
If this is a new target, please add it to /checkout/src/doc/rustc/src/platform-support.md.

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "run" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tier-check/Cargo.toml" "/checkout/src/doc/rustc/src/platform-support.md" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc"
expected success, got: exit status: 1

