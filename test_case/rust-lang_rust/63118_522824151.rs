plain
2019-08-20T02:08:46.6864535Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-20T02:08:46.7496296Z ##[command]git config gc.auto 0
2019-08-20T02:08:46.7561978Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-20T02:08:46.7612370Z ##[command]git config --get-all http.proxy
2019-08-20T02:08:46.7796034Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63118/merge:refs/remotes/pull/63118/merge
---
2019-08-20T02:09:21.4216525Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-20T02:09:21.4216911Z 
2019-08-20T02:09:21.4217708Z   git checkout -b <new-branch-name>
2019-08-20T02:09:21.4218083Z 
2019-08-20T02:09:21.4219191Z HEAD is now at 1056d4a81 Merge 4d1197450880e428f0c23ce1a7fb48af1698c39e into c1b08dd26036e14f061b99b20cd6f169e29046f3
2019-08-20T02:09:21.4380266Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-20T02:09:21.4382703Z ==============================================================================
2019-08-20T02:09:21.4382751Z Task         : Bash
2019-08-20T02:09:21.4382805Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-20T02:14:27.1421151Z    Compiling cc v1.0.35
2019-08-20T02:14:27.1508401Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-08-20T02:14:35.3274108Z    Compiling libc v0.2.60
2019-08-20T02:14:36.2932528Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-08-20T02:14:37.1563179Z error[E0008]: cannot bind by-move into a pattern guard
2019-08-20T02:14:37.1563529Z     --> src/libcore/iter/adapters/mod.rs:1243:18
2019-08-20T02:14:37.1563790Z      |
2019-08-20T02:14:37.1564043Z 1243 |             Some(v @ Some(_)) if n == 0 => v,
2019-08-20T02:14:37.1564333Z      |                  ^^^^^^^^^^^ moves value into pattern guard
2019-08-20T02:14:37.1564971Z      = help: add `#![feature(bind_by_move_pattern_guards)]` to the crate attributes to enable
2019-08-20T02:14:37.1565007Z 
2019-08-20T02:14:37.8766949Z    Compiling cmake v0.1.38
2019-08-20T02:14:40.9757867Z    Compiling compiler_builtins v0.1.18
2019-08-20T02:14:40.9757867Z    Compiling compiler_builtins v0.1.18
2019-08-20T02:14:41.7983939Z error: aborting due to previous error
2019-08-20T02:14:41.7984852Z 
2019-08-20T02:14:41.7992260Z For more information about this error, try `rustc --explain E0008`.
2019-08-20T02:14:41.9280407Z error: Could not compile `core`.
2019-08-20T02:14:41.9281562Z warning: build failed, waiting for other jobs to finish...
2019-08-20T02:14:42.7242388Z error: build failed
2019-08-20T02:14:42.7266007Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-20T02:14:42.7277210Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-20T02:14:42.7277537Z Build completed unsuccessfully in 0:02:37
2019-08-20T02:14:42.7328377Z == clock drift check ==
2019-08-20T02:14:42.7345358Z   local time: Tue Aug 20 02:14:42 UTC 2019
2019-08-20T02:14:42.7345358Z   local time: Tue Aug 20 02:14:42 UTC 2019
2019-08-20T02:14:42.8839863Z   network time: Tue, 20 Aug 2019 02:14:42 GMT
2019-08-20T02:14:42.8844460Z == end clock drift check ==
2019-08-20T02:14:55.6734467Z ##[error]Bash exited with code '1'.
2019-08-20T02:14:55.6771321Z ##[section]Starting: Checkout
2019-08-20T02:14:55.6772970Z ==============================================================================
2019-08-20T02:14:55.6773044Z Task         : Get sources
2019-08-20T02:14:55.6773111Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
