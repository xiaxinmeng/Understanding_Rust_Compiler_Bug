plain
2019-08-18T06:41:41.0670806Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-18T06:41:41.0876630Z ##[command]git config gc.auto 0
2019-08-18T06:41:41.0965462Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-18T06:41:41.1029046Z ##[command]git config --get-all http.proxy
2019-08-18T06:41:41.1171820Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63676/merge:refs/remotes/pull/63676/merge
---
2019-08-18T06:42:16.3196721Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-18T06:42:16.3196964Z 
2019-08-18T06:42:16.3197418Z   git checkout -b <new-branch-name>
2019-08-18T06:42:16.3197655Z 
2019-08-18T06:42:16.3197871Z HEAD is now at 8326e8d07 Merge b8472cea1e8f40fe89ed673bd8bcd37a106c2e0d into fc8765d6d8623b2b5b4ca1d526ed1d7beb3fce18
2019-08-18T06:42:16.3352678Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-18T06:42:16.3355453Z ==============================================================================
2019-08-18T06:42:16.3355499Z Task         : Bash
2019-08-18T06:42:16.3355534Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-18T06:48:01.4101509Z 
2019-08-18T06:48:01.4101870Z error: incorrect close delimiter: `)`
2019-08-18T06:48:01.4102509Z   --> src/libstd/sys/wasi/time.rs:17:6
2019-08-18T06:48:01.4102841Z    |
2019-08-18T06:48:01.4104032Z 13 | fn current_time(clock: u32) -> Duration {
2019-08-18T06:48:01.4104595Z    |                                         - un-closed delimiter
2019-08-18T06:48:01.4105438Z 17 |     )).unwrap();
2019-08-18T06:48:01.4105907Z    |      ^ incorrect close delimiter
2019-08-18T06:48:01.4106064Z 
2019-08-18T06:48:01.4350086Z error: aborting due to 2 previous errors
2019-08-18T06:48:01.4350086Z error: aborting due to 2 previous errors
2019-08-18T06:48:01.4350207Z 
2019-08-18T06:48:02.4009850Z error: Could not compile `std`.
2019-08-18T06:48:02.4012067Z 
2019-08-18T06:48:02.4012724Z To learn more, run the command again with --verbose.
2019-08-18T06:48:02.4013897Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-18T06:48:02.4014372Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-18T06:48:02.4014531Z Build completed unsuccessfully in 0:02:47
2019-08-18T06:48:02.4014694Z == clock drift check ==
2019-08-18T06:48:02.4014854Z   local time: Sun Aug 18 06:48:01 UTC 2019
2019-08-18T06:48:02.4014854Z   local time: Sun Aug 18 06:48:01 UTC 2019
2019-08-18T06:48:02.4014997Z   network time: Sun, 18 Aug 2019 06:48:01 GMT
2019-08-18T06:48:02.4015138Z == end clock drift check ==
2019-08-18T06:48:08.0989245Z ##[error]Bash exited with code '1'.
2019-08-18T06:48:08.1042627Z ##[section]Starting: Checkout
2019-08-18T06:48:08.1044764Z ==============================================================================
2019-08-18T06:48:08.1044838Z Task         : Get sources
2019-08-18T06:48:08.1044887Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
