plain
2019-07-31T19:36:16.6033173Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-31T19:36:16.6239099Z ##[command]git config gc.auto 0
2019-07-31T19:36:16.6309983Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-31T19:36:16.6366412Z ##[command]git config --get-all http.proxy
2019-07-31T19:36:16.6499532Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62987/merge:refs/remotes/pull/62987/merge
---
2019-07-31T19:36:52.9366708Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-31T19:36:52.9366793Z 
2019-07-31T19:36:52.9367063Z   git checkout -b <new-branch-name>
2019-07-31T19:36:52.9367130Z 
2019-07-31T19:36:52.9367208Z HEAD is now at 2a7d79bad Merge 11ea929ea6c0e241253839e6a61dbdf3cbae4953 into e3976fff44e6ce14c2f92252e6a807800b9aa7c0
2019-07-31T19:36:52.9532522Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-31T19:36:52.9535492Z ==============================================================================
2019-07-31T19:36:52.9535553Z Task         : Bash
2019-07-31T19:36:52.9535621Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-31T19:46:45.5487659Z configure: build.locked-deps    := True
2019-07-31T19:46:45.5487710Z configure: llvm.ccache          := sccache
2019-07-31T19:46:45.5487948Z configure: build.cargo-native-static := True
2019-07-31T19:46:45.5488164Z configure: dist.missing-tools   := True
2019-07-31T19:46:45.5488423Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-07-31T19:46:45.5488729Z configure: writing `config.toml` in current directory
2019-07-31T19:46:45.5488774Z configure: 
2019-07-31T19:46:45.5489041Z configure: run `python /checkout/x.py --help`
2019-07-31T19:46:45.5489113Z configure: 
---
2019-07-31T19:48:44.8301446Z     Checking hashbrown v0.4.0
2019-07-31T19:48:48.4768338Z error[E0308]: mismatched types
2019-07-31T19:48:48.4773383Z   --> src/libstd/sys/windows/io.rs:26:27
2019-07-31T19:48:48.4773600Z    |
2019-07-31T19:48:48.4774564Z 26 |         if self.vec.len < n {
2019-07-31T19:48:48.4774852Z    |                           ^ expected u32, found usize
2019-07-31T19:48:48.4775298Z help: you can convert an `usize` to `u32` and panic if the converted value wouldn't fit
2019-07-31T19:48:48.4775509Z    |
2019-07-31T19:48:48.4775750Z 26 |         if self.vec.len < n.try_into().unwrap() {
2019-07-31T19:48:48.4776042Z 
2019-07-31T19:48:48.4783063Z error[E0308]: mismatched types
2019-07-31T19:48:48.4783345Z   --> src/libstd/sys/windows/io.rs:65:27
2019-07-31T19:48:48.4783820Z    |
2019-07-31T19:48:48.4783820Z    |
2019-07-31T19:48:48.4784090Z 65 |         if self.vec.len < n {
2019-07-31T19:48:48.4784562Z    |                           ^ expected u32, found usize
2019-07-31T19:48:48.4784887Z help: you can convert an `usize` to `u32` and panic if the converted value wouldn't fit
2019-07-31T19:48:48.4785103Z    |
2019-07-31T19:48:48.4785534Z 65 |         if self.vec.len < n.try_into().unwrap() {
2019-07-31T19:48:48.4785862Z 
2019-07-31T19:48:48.6382406Z error: aborting due to 2 previous errors
2019-07-31T19:48:48.6382613Z 
2019-07-31T19:48:48.6382959Z For more information about this error, try `rustc --explain E0308`.
2019-07-31T19:48:48.6382959Z For more information about this error, try `rustc --explain E0308`.
2019-07-31T19:48:48.6755472Z error: Could not compile `std`.
2019-07-31T19:48:48.6755572Z 
2019-07-31T19:48:48.6755845Z To learn more, run the command again with --verbose.
2019-07-31T19:48:48.6774793Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-31T19:48:48.6784717Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2019-07-31T19:48:48.6785007Z Build completed unsuccessfully in 0:00:30
2019-07-31T19:48:48.6785007Z Build completed unsuccessfully in 0:00:30
2019-07-31T19:48:54.7230236Z ##[error]Bash exited with code '1'.
2019-07-31T19:48:54.7263221Z ##[section]Starting: Checkout
2019-07-31T19:48:54.7264815Z ==============================================================================
2019-07-31T19:48:54.7264866Z Task         : Get sources
2019-07-31T19:48:54.7264910Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
