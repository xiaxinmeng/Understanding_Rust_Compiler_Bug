
$ ./x.py clippy src/libstd                    
Updating only changed submodules
Submodules updated in 0.02 seconds
   Compiling bootstrap v0.0.0 (/home/mateusz/Projects/rust/rust/src/bootstrap)
    Finished dev [unoptimized] target(s) in 6.40s
Checking std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Checking core v0.0.0 (/home/mateusz/Projects/rust/rust/src/libcore)
   Compiling compiler_builtins v0.1.18
   Compiling libc v0.2.60
error: Option 'error-format' given more than once

error: Could not compile `core`.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/home/mateusz/Projects/rust/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "clippy" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/home/mateusz/Projects/rust/rust/src/libstd/Cargo.toml" "--message-format" "json-render-diagnostics" "--" "--cap-lints" "warn"
expected success, got: exit code: 101
failed to run: /home/mateusz/Projects/rust/rust/build/bootstrap/debug/bootstrap clippy src/libstd
