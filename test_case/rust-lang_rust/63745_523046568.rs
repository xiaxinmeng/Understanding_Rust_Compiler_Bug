plain
2019-08-20T14:33:52.9389637Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-20T14:33:52.9613008Z ##[command]git config gc.auto 0
2019-08-20T14:33:52.9684315Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-20T14:33:52.9776861Z ##[command]git config --get-all http.proxy
2019-08-20T14:33:52.9891783Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63745/merge:refs/remotes/pull/63745/merge
---
2019-08-20T14:34:28.5215015Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-20T14:34:28.5215040Z 
2019-08-20T14:34:28.5215351Z   git checkout -b <new-branch-name>
2019-08-20T14:34:28.5215556Z 
2019-08-20T14:34:28.5215605Z HEAD is now at 11ff6f8c0 Merge 5e522ccfc828930a7b3f1a3c62e00ce5f30dac33 into 14890954ce17c44d944eda988c5a64bb4c5ec9eb
2019-08-20T14:34:28.5399492Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-20T14:34:28.5402089Z ==============================================================================
2019-08-20T14:34:28.5402133Z Task         : Bash
2019-08-20T14:34:28.5402167Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-20T14:40:05.8491237Z     Checking hashbrown v0.5.0
2019-08-20T14:40:06.9140118Z error: this file contains an un-closed delimiter
2019-08-20T14:40:06.9141972Z   --> src/libstd/os/linux/syscall/mod.rs:67:3
2019-08-20T14:40:06.9142253Z    |
2019-08-20T14:40:06.9142510Z 5  | #[cfg(target_arch = "x86")] {
2019-08-20T14:40:06.9143015Z    |                             - un-closed delimiter
2019-08-20T14:40:06.9143306Z ...
2019-08-20T14:40:06.9143881Z 8  | #[cfg(target_arch = "x86_64")] {
2019-08-20T14:40:06.9144157Z    |                                - un-closed delimiter
2019-08-20T14:40:06.9144358Z ...
2019-08-20T14:40:06.9144887Z 11 | #[cfg(target_arch = "aarch64")] {
2019-08-20T14:40:06.9145157Z    |                                 - un-closed delimiter
2019-08-20T14:40:06.9145546Z 67 | }
2019-08-20T14:40:06.9145743Z    |   ^
2019-08-20T14:40:06.9151733Z 
2019-08-20T14:40:06.9152371Z error: expected item after attributes
2019-08-20T14:40:06.9152371Z error: expected item after attributes
2019-08-20T14:40:06.9152784Z  --> src/libstd/os/linux/syscall/mod.rs:5:27
2019-08-20T14:40:06.9153113Z   |
2019-08-20T14:40:06.9153971Z 5 | #[cfg(target_arch = "x86")] {
2019-08-20T14:40:06.9154549Z 
2019-08-20T14:40:06.9269308Z error: aborting due to 2 previous errors
2019-08-20T14:40:06.9269395Z 
2019-08-20T14:40:06.9349721Z error: Could not compile `std`.
2019-08-20T14:40:06.9349721Z error: Could not compile `std`.
2019-08-20T14:40:06.9349822Z 
2019-08-20T14:40:06.9350106Z To learn more, run the command again with --verbose.
2019-08-20T14:40:06.9374607Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-20T14:40:06.9386852Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-20T14:40:06.9386980Z Build completed unsuccessfully in 0:02:48
2019-08-20T14:40:06.9433269Z == clock drift check ==
2019-08-20T14:40:06.9683233Z   local time: Tue Aug 20 14:40:06 UTC 2019
2019-08-20T14:40:06.9683233Z   local time: Tue Aug 20 14:40:06 UTC 2019
2019-08-20T14:40:07.1729241Z   network time: Tue, 20 Aug 2019 14:40:07 GMT
2019-08-20T14:40:07.1729528Z == end clock drift check ==
2019-08-20T14:40:15.9750749Z ##[error]Bash exited with code '1'.
2019-08-20T14:40:15.9783883Z ##[section]Starting: Checkout
2019-08-20T14:40:15.9785491Z ==============================================================================
2019-08-20T14:40:15.9785543Z Task         : Get sources
2019-08-20T14:40:15.9785587Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
