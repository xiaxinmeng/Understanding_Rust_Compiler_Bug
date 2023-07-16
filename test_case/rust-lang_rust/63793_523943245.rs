plain
2019-08-22T14:50:56.1683604Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-22T14:50:56.1868101Z ##[command]git config gc.auto 0
2019-08-22T14:50:56.1938279Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-22T14:50:56.1999724Z ##[command]git config --get-all http.proxy
2019-08-22T14:50:56.2143890Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63793/merge:refs/remotes/pull/63793/merge
2019-08-22T14:50:58.3345795Z remote:                                                                                         
---
2019-08-22T14:51:32.1438609Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-22T14:51:32.1438913Z 
2019-08-22T14:51:32.1439431Z   git checkout -b <new-branch-name>
2019-08-22T14:51:32.1439741Z 
2019-08-22T14:51:32.1440477Z HEAD is now at b3f7bc1b4 Merge c2d541f186a249260f09a32a3aef31f190804cd0 into 201e52e5fe73ccf3dd22946b1216ad8d64f8c2ba
2019-08-22T14:51:32.1605694Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-22T14:51:32.1609211Z ==============================================================================
2019-08-22T14:51:32.1609273Z Task         : Bash
2019-08-22T14:51:32.1609373Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-22T14:56:48.4599632Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-08-22T14:56:48.6125697Z error: attributes are not yet allowed on `if` expressions
2019-08-22T14:56:48.6126113Z    --> src/libcore/iter/adapters/mod.rs:527:13
2019-08-22T14:56:48.6126457Z     |
2019-08-22T14:56:48.6126798Z 527 |             #[cfg(boostrap_stdarch_ignore_this)]
2019-08-22T14:56:48.6127216Z 
2019-08-22T14:56:48.6225837Z error: aborting due to previous error
2019-08-22T14:56:48.6225948Z 
2019-08-22T14:56:48.6281427Z error: Could not compile `core`.
2019-08-22T14:56:48.6281427Z error: Could not compile `core`.
2019-08-22T14:56:48.6281804Z warning: build failed, waiting for other jobs to finish...
2019-08-22T14:56:53.4666642Z error: build failed
2019-08-22T14:56:53.4698646Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-22T14:56:53.4710292Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-22T14:56:53.4710554Z Build completed unsuccessfully in 0:02:31
2019-08-22T14:56:53.4763277Z == clock drift check ==
2019-08-22T14:56:53.4778337Z   local time: Thu Aug 22 14:56:53 UTC 2019
2019-08-22T14:56:53.4778337Z   local time: Thu Aug 22 14:56:53 UTC 2019
2019-08-22T14:56:53.6399129Z   network time: Thu, 22 Aug 2019 14:56:53 GMT
2019-08-22T14:56:53.6402864Z == end clock drift check ==
2019-08-22T14:57:06.7146623Z ##[error]Bash exited with code '1'.
2019-08-22T14:57:06.7195018Z ##[section]Starting: Checkout
2019-08-22T14:57:06.7196811Z ==============================================================================
2019-08-22T14:57:06.7196870Z Task         : Get sources
2019-08-22T14:57:06.7196922Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
