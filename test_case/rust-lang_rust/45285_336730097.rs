
Building stage2 tool clippy-driver (x86_64-unknown-linux-gnu)
[00:36:22] error: the lock file needs to be updated but --locked was passed to prevent this
[00:36:22] 
[00:36:22] 
[00:36:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/clippy/Cargo.toml"
[00:36:22] expected success, got: exit code: 101
[00:36:22] 
[00:36:22] 
[00:36:22] You can disable the tool in `src/tools/toolstate.toml`
[00:36:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:36:22] Build completed unsuccessfully in 0:33:05
[00:36:22] make: *** [all] Error 1
[00:36:22] Makefile:22: recipe for target 'all' failed
