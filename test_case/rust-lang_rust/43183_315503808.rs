
00:16:51] rustc: /checkout/src/llvm/include/llvm/IR/GlobalValue.h:227: void llvm::GlobalValue::setVisibility(llvm::GlobalValue::VisibilityTypes): Assertion `(!hasLocalLinkage() || V == DefaultVisibility) && "local linkage requires default visibility"' failed.
[00:16:51] (B[m[31m[1merror:(B[m Could not compile `core`.
[00:16:51] (B[m
[00:16:51] Caused by:
[00:16:51] (B[m(B[m  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core /checkout/src/libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=a6117b7e5e0db06a -C extra-filename=-a6117b7e5e0db06a --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Zincremental=/tmp/rust-incr-cache` (signal: 6, SIGABRT: process abort signal)
[00:16:51] --- stderr
[00:16:51] rustc: /checkout/src/llvm/include/llvm/IR/GlobalValue.h:227: void llvm::GlobalValue::setVisibility(llvm::GlobalValue::VisibilityTypes): Assertion `(!hasLocalLinkage() || V == DefaultVisibility) && "local linkage requires default visibility"' failed.
[00:16:51] 
[00:16:51] (B[mthread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-j" "4" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:16:51] expected success, got: exit code: 101', /checkout/src/bootstrap/compile.rs:586
[00:16:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
