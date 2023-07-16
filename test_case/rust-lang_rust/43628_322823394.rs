
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:15:41] error: failed to run `rustc` to learn about target-specific information
[00:15:41] 
[00:15:41] Caused by:
[00:15:41]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names --crate-type bin --crate-type dylib --crate-type rlib --target x86_64-unknown-linux-gnu` (exit code: 101)
[00:15:41] --- stderr
[00:15:41] error: unknown debugging option: `emit-mir-validate`
[00:15:41] 
[00:15:41] 
[00:15:41] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-j" "4" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
