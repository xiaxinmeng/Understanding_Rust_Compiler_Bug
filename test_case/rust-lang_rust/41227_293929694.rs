
[00:48:34] error: failed to run custom build command for `openssl-sys v0.9.10`
[00:48:34] process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/build/openssl-sys-af1f288687ffbc04/build-script-build` (exit code: 101)
[00:48:34] --- stderr
[00:48:34] thread 'main' panicked at 'OpenSSL library directory does not exist: /checkout/obj/build/x86_64-unknown-linux-gnu/openssl/install/lib', /cargo/registry/src/github.com-1ecc6299db9ec823/openssl-sys-0.9.10/build.rs:55
[00:48:34] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:34] 
[00:48:34] Build failed, waiting for other jobs to finish...
[00:48:36] error: build failed
[00:48:36] 
[00:48:36] 
[00:48:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-j" "4" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--manifest-path" "/checkout/rls/Cargo.toml"
[00:48:36] expected success, got: exit code: 101
[00:48:36] 
[00:48:36] 
[00:48:36] Build completed unsuccessfully in 0:47:31
