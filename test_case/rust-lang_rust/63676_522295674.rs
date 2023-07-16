plain
2019-08-18T06:29:21.9637757Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-18T06:29:21.9797868Z ##[command]git config gc.auto 0
2019-08-18T06:29:21.9866701Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-18T06:29:21.9918446Z ##[command]git config --get-all http.proxy
2019-08-18T06:29:22.0057088Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63676/merge:refs/remotes/pull/63676/merge
---
2019-08-18T06:29:55.8442425Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-18T06:29:55.8442455Z 
2019-08-18T06:29:55.8442656Z   git checkout -b <new-branch-name>
2019-08-18T06:29:55.8442684Z 
2019-08-18T06:29:55.8442751Z HEAD is now at a4b25866b Merge 7bfb3d174bae45db93aae1cbdbfe6ce46a22b068 into fc8765d6d8623b2b5b4ca1d526ed1d7beb3fce18
2019-08-18T06:29:55.8603083Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-18T06:29:55.8605699Z ==============================================================================
2019-08-18T06:29:55.8605752Z Task         : Bash
2019-08-18T06:29:55.8605813Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-18T06:35:29.2459866Z     Checking backtrace v0.3.34
2019-08-18T06:35:30.9420821Z     Checking rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-08-18T06:35:31.0247171Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-08-18T06:35:31.0762614Z     Checking hashbrown v0.5.0
2019-08-18T06:35:32.3382587Z error: unexpected close delimiter: `)`
2019-08-18T06:35:32.3383106Z    --> src/libstd/sys/wasi/fd.rs:232:44
2019-08-18T06:35:32.3383834Z     |
2019-08-18T06:35:32.3384261Z 232 |         wasi::sock_shutdown(self.fd, how) }).map_err(From::from)
2019-08-18T06:35:32.3384728Z 
2019-08-18T06:35:32.3397522Z error: incorrect close delimiter: `)`
2019-08-18T06:35:32.3397522Z error: incorrect close delimiter: `)`
2019-08-18T06:35:32.3398310Z    --> src/libstd/sys/wasi/fd.rs:141:10
2019-08-18T06:35:32.3398568Z     |
2019-08-18T06:35:32.3398841Z 132 |     ) -> io::Result<WasiFd> {
2019-08-18T06:35:32.3399151Z     |                             - un-closed delimiter
2019-08-18T06:35:32.3399810Z 141 |         ))?;
2019-08-18T06:35:32.3400112Z     |          ^ incorrect close delimiter
2019-08-18T06:35:32.3400172Z 
2019-08-18T06:35:32.3741484Z error: aborting due to 2 previous errors
2019-08-18T06:35:32.3741484Z error: aborting due to 2 previous errors
2019-08-18T06:35:32.3741820Z 
2019-08-18T06:35:32.3846445Z error: Could not compile `std`.
2019-08-18T06:35:32.3846537Z 
2019-08-18T06:35:32.3846788Z To learn more, run the command again with --verbose.
2019-08-18T06:35:32.3889284Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-18T06:35:32.3899193Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-18T06:35:32.3899273Z Build completed unsuccessfully in 0:02:51
2019-08-18T06:35:32.3953373Z == clock drift check ==
2019-08-18T06:35:32.3969710Z   local time: Sun Aug 18 06:35:32 UTC 2019
2019-08-18T06:35:32.3969710Z   local time: Sun Aug 18 06:35:32 UTC 2019
2019-08-18T06:35:32.4871313Z   network time: Sun, 18 Aug 2019 06:35:32 GMT
2019-08-18T06:35:32.4872147Z == end clock drift check ==
2019-08-18T06:35:38.7802484Z ##[error]Bash exited with code '1'.
2019-08-18T06:35:38.7850677Z ##[section]Starting: Checkout
2019-08-18T06:35:38.7853471Z ==============================================================================
2019-08-18T06:35:38.7853529Z Task         : Get sources
2019-08-18T06:35:38.7853594Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
