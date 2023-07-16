plain
2019-08-18T03:45:23.2093028Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-18T03:45:23.2276693Z ##[command]git config gc.auto 0
2019-08-18T03:45:23.2359758Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-18T03:45:23.2432225Z ##[command]git config --get-all http.proxy
2019-08-18T03:45:23.2581654Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63676/merge:refs/remotes/pull/63676/merge
---
2019-08-18T03:45:58.1236850Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-18T03:45:58.1238276Z 
2019-08-18T03:45:58.1239916Z   git checkout -b <new-branch-name>
2019-08-18T03:45:58.1241259Z 
2019-08-18T03:45:58.1242655Z HEAD is now at 98e920dc7 Merge 6c299244305cdf585e515d608cc2bafa31081475 into bd1da18b04afba5dfc09ad1b56df3285f1d039c3
2019-08-18T03:45:58.1403710Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-18T03:45:58.1407137Z ==============================================================================
2019-08-18T03:45:58.1407202Z Task         : Bash
2019-08-18T03:45:58.1407270Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-18T03:51:54.6947258Z     Checking rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-08-18T03:51:54.9078611Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-08-18T03:51:55.0511394Z     Checking hashbrown v0.5.0
2019-08-18T03:51:57.5337715Z     Checking backtrace v0.3.34
2019-08-18T03:51:58.1446193Z error: unexpected close delimiter: `)`
2019-08-18T03:51:58.1446595Z    --> src/libstd/sys/wasi/fd.rs:232:44
2019-08-18T03:51:58.1446901Z     |
2019-08-18T03:51:58.1447247Z 232 |         wasi::sock_shutdown(self.fd, how) }).map_err(From::from)
2019-08-18T03:51:58.1448057Z 
2019-08-18T03:51:58.1448387Z error: incorrect close delimiter: `)`
2019-08-18T03:51:58.1448387Z error: incorrect close delimiter: `)`
2019-08-18T03:51:58.1448679Z    --> src/libstd/sys/wasi/fd.rs:141:10
2019-08-18T03:51:58.1448947Z     |
2019-08-18T03:51:58.1449253Z 132 |     ) -> io::Result<WasiFd> {
2019-08-18T03:51:58.1449630Z     |                             - un-closed delimiter
2019-08-18T03:51:58.1450161Z 141 |         ))?;
2019-08-18T03:51:58.1450531Z     |          ^ incorrect close delimiter
2019-08-18T03:51:58.1450575Z 
2019-08-18T03:51:58.1685847Z error: aborting due to 2 previous errors
2019-08-18T03:51:58.1685847Z error: aborting due to 2 previous errors
2019-08-18T03:51:58.1685948Z 
2019-08-18T03:51:58.1776134Z error: Could not compile `std`.
2019-08-18T03:51:58.1776486Z 
2019-08-18T03:51:58.1776964Z To learn more, run the command again with --verbose.
2019-08-18T03:51:58.1833205Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-18T03:51:58.1835065Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-18T03:51:58.1835216Z Build completed unsuccessfully in 0:03:07
2019-08-18T03:51:58.1889349Z == clock drift check ==
2019-08-18T03:51:58.1906283Z   local time: Sun Aug 18 03:51:58 UTC 2019
2019-08-18T03:51:58.1906283Z   local time: Sun Aug 18 03:51:58 UTC 2019
2019-08-18T03:51:58.3399732Z   network time: Sun, 18 Aug 2019 03:51:58 GMT
2019-08-18T03:51:58.3401192Z == end clock drift check ==
2019-08-18T03:52:01.5379249Z ##[error]Bash exited with code '1'.
2019-08-18T03:52:01.5412911Z ##[section]Starting: Checkout
2019-08-18T03:52:01.5414349Z ==============================================================================
2019-08-18T03:52:01.5414413Z Task         : Get sources
2019-08-18T03:52:01.5414454Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
