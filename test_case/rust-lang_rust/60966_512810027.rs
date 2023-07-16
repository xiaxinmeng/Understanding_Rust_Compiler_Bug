plain
2019-07-18T13:06:31.1433632Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-18T13:06:31.1657318Z ##[command]git config gc.auto 0
2019-07-18T13:06:31.1719745Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-18T13:06:31.1782451Z ##[command]git config --get-all http.proxy
2019-07-18T13:06:31.1922716Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/60966/merge:refs/remotes/pull/60966/merge
---
2019-07-18T13:07:06.8762769Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-18T13:07:06.8762833Z 
2019-07-18T13:07:06.8763566Z   git checkout -b <new-branch-name>
2019-07-18T13:07:06.8763629Z 
2019-07-18T13:07:06.8763923Z HEAD is now at 1a970c388 Merge 6028c1f53b19630bfaab29cfd494003b884ba158 into 21d5b8bf0c26e3ee4c270ce5527df66b1af56513
2019-07-18T13:07:06.8921056Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-18T13:07:06.8923691Z ==============================================================================
2019-07-18T13:07:06.8923742Z Task         : Bash
2019-07-18T13:07:06.8923785Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-18T13:12:32.1378224Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-18T13:12:32.1401290Z Checking std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-18T13:12:32.4998073Z    Compiling cc v1.0.35
2019-07-18T13:12:32.4998394Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-07-18T13:12:34.1046647Z error: cannot determine resolution for the derive macro `Debug`
2019-07-18T13:12:34.1048432Z   --> src/libcore/num/dec2flt/rawfp.rs:30:23
2019-07-18T13:12:34.1048987Z    |
2019-07-18T13:12:34.1049569Z 30 | #[derive(Copy, Clone, Debug)]
2019-07-18T13:12:34.1050652Z    |
2019-07-18T13:12:34.1050652Z    |
2019-07-18T13:12:34.1051180Z    = note: import resolution is stuck, try simplifying macro imports
2019-07-18T13:12:41.0075036Z    Compiling libc v0.2.54
2019-07-18T13:12:41.7938394Z    Compiling autocfg v0.1.4
2019-07-18T13:12:43.4679029Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-07-18T13:12:45.8463191Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-07-18T13:12:45.8463191Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-07-18T13:12:46.1362603Z error: aborting due to previous error
2019-07-18T13:12:46.1366297Z 
2019-07-18T13:12:46.2378484Z error: Could not compile `core`.
2019-07-18T13:12:46.2378902Z warning: build failed, waiting for other jobs to finish...
2019-07-18T13:12:46.3969782Z error: build failed
2019-07-18T13:12:46.3997794Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-18T13:12:46.4010792Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-18T13:12:46.4010890Z Build completed unsuccessfully in 0:02:27
2019-07-18T13:12:46.4010890Z Build completed unsuccessfully in 0:02:27
2019-07-18T13:12:54.5709964Z ##[error]Bash exited with code '1'.
2019-07-18T13:12:54.5740125Z ##[section]Starting: Checkout
2019-07-18T13:12:54.5742241Z ==============================================================================
2019-07-18T13:12:54.5742282Z Task         : Get sources
2019-07-18T13:12:54.5742332Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
