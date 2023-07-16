plain
2019-08-21T18:12:19.8607065Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-21T18:12:19.8805627Z ##[command]git config gc.auto 0
2019-08-21T18:12:19.8889655Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-21T18:12:19.8936506Z ##[command]git config --get-all http.proxy
2019-08-21T18:12:19.9079955Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63793/merge:refs/remotes/pull/63793/merge
---
2019-08-21T18:12:56.4261839Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-21T18:12:56.4264361Z 
2019-08-21T18:12:56.4264801Z   git checkout -b <new-branch-name>
2019-08-21T18:12:56.4264836Z 
2019-08-21T18:12:56.4264915Z HEAD is now at 2dca2ba86 Merge 6fb59fa17c87baab781792ee5db4a9db834cb92f into 7b0085a613e69cb69fc9e4eb5d422fa4a39d5de1
2019-08-21T18:12:56.4450033Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-21T18:12:56.4453709Z ==============================================================================
2019-08-21T18:12:56.4453796Z Task         : Bash
2019-08-21T18:12:56.4453848Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-21T18:18:21.6178904Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-08-21T18:18:21.7454577Z error: attributes are not yet allowed on `if` expressions
2019-08-21T18:18:21.7455006Z    --> src/libcore/iter/adapters/mod.rs:527:13
2019-08-21T18:18:21.7455359Z     |
2019-08-21T18:18:21.7455774Z 527 |             #[cfg(boostrap_stdarch_ignore_this)]
2019-08-21T18:18:21.7456216Z 
2019-08-21T18:18:21.7532903Z error: aborting due to previous error
2019-08-21T18:18:21.7532989Z 
2019-08-21T18:18:21.7553992Z error: Could not compile `core`.
2019-08-21T18:18:21.7553992Z error: Could not compile `core`.
2019-08-21T18:18:21.7554389Z warning: build failed, waiting for other jobs to finish...
2019-08-21T18:18:26.8122596Z error: build failed
2019-08-21T18:18:26.8178221Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-21T18:18:26.8185797Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-21T18:18:26.8185904Z Build completed unsuccessfully in 0:02:34
2019-08-21T18:18:26.8235919Z == clock drift check ==
2019-08-21T18:18:26.8253643Z   local time: Wed Aug 21 18:18:26 UTC 2019
2019-08-21T18:18:26.8253643Z   local time: Wed Aug 21 18:18:26 UTC 2019
2019-08-21T18:18:26.9816313Z   network time: Wed, 21 Aug 2019 18:18:26 GMT
2019-08-21T18:18:26.9819441Z == end clock drift check ==
2019-08-21T18:18:40.0060021Z ##[error]Bash exited with code '1'.
2019-08-21T18:18:40.0097444Z ##[section]Starting: Checkout
2019-08-21T18:18:40.0099262Z ==============================================================================
2019-08-21T18:18:40.0099348Z Task         : Get sources
2019-08-21T18:18:40.0099401Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
