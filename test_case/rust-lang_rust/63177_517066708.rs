plain
2019-07-31T23:57:05.7413860Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-31T23:57:05.7605218Z ##[command]git config gc.auto 0
2019-07-31T23:57:05.7683228Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-31T23:57:05.7747190Z ##[command]git config --get-all http.proxy
2019-07-31T23:57:05.7887749Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63177/merge:refs/remotes/pull/63177/merge
---
2019-07-31T23:57:41.0266929Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-31T23:57:41.0266953Z 
2019-07-31T23:57:41.0267120Z   git checkout -b <new-branch-name>
2019-07-31T23:57:41.0267156Z 
2019-07-31T23:57:41.0267213Z HEAD is now at 39ba8181c Merge 017f98768c66598901f9c95f24369f78185369e9 into 8a58268b5ad9c4a240be349a633069d48991eb0c
2019-07-31T23:57:41.0442922Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-31T23:57:41.0446911Z ==============================================================================
2019-07-31T23:57:41.0446960Z Task         : Bash
2019-07-31T23:57:41.0446996Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-01T00:02:46.8743375Z    |
2019-08-01T00:02:46.8743874Z 76 | #![feature(const_generics)]
2019-08-01T00:02:46.8744190Z    |            ^^^^^^^^^^^^^^
2019-08-01T00:02:46.8744487Z 
2019-08-01T00:02:49.7956176Z error[E0545]: incorrect 'issue'
2019-08-01T00:02:49.7956556Z     --> src/libcore/iter/traits/iterator.rs:1988:5
2019-08-01T00:02:49.7956756Z      |
2019-08-01T00:02:49.7957039Z 1988 |     #[unstable(feature = "find_result", reason = "new API", issue = "?")]
2019-08-01T00:02:49.7957344Z 
2019-08-01T00:02:54.7264226Z    Compiling libc v0.2.54
2019-08-01T00:02:55.5085223Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-08-01T00:02:56.0577475Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-08-01T00:02:56.0577475Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-08-01T00:02:56.9325848Z error: aborting due to previous error
2019-08-01T00:02:56.9326004Z 
2019-08-01T00:02:57.0306846Z error: Could not compile `core`.
2019-08-01T00:02:57.0318626Z warning: build failed, waiting for other jobs to finish...
2019-08-01T00:02:57.3330563Z error: build failed
2019-08-01T00:02:57.3351049Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-08-01T00:02:57.3364897Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-01T00:02:57.3365217Z Build completed unsuccessfully in 0:02:11
2019-08-01T00:02:57.3365217Z Build completed unsuccessfully in 0:02:11
2019-08-01T00:03:11.6957889Z ##[error]Bash exited with code '1'.
2019-08-01T00:03:11.6993422Z ##[section]Starting: Checkout
2019-08-01T00:03:11.6995163Z ==============================================================================
2019-08-01T00:03:11.6995212Z Task         : Get sources
2019-08-01T00:03:11.6995273Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
