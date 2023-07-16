plain
2019-07-19T13:46:36.0979856Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-19T13:46:36.1194742Z ##[command]git config gc.auto 0
2019-07-19T13:46:36.1265229Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-19T13:46:36.1322647Z ##[command]git config --get-all http.proxy
2019-07-19T13:46:36.1466522Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62800/merge:refs/remotes/pull/62800/merge
---
2019-07-19T13:47:11.6163706Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-19T13:47:11.6163755Z 
2019-07-19T13:47:11.6163939Z   git checkout -b <new-branch-name>
2019-07-19T13:47:11.6163965Z 
2019-07-19T13:47:11.6164008Z HEAD is now at 122b30426 Merge ad169d2ff4a36b559fcf915d08c3160b9393d5dd into f9477a77c52af8d3dea361b3f4ac3e60653aa529
2019-07-19T13:47:11.6300635Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-19T13:47:11.6303862Z ==============================================================================
2019-07-19T13:47:11.6303930Z Task         : Bash
2019-07-19T13:47:11.6303969Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-19T13:50:37.9286375Z downloading https://static.rust-lang.org/dist/2019-07-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-07-19T13:50:38.0979933Z 
2019-07-19T13:50:38.0986081Z ######################################################################## 100.0%
2019-07-19T13:50:38.6386703Z extracting /checkout/obj/build/cache/2019-07-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-07-19T13:50:38.6685384Z error: failed to read `/polonius/polonius-engine/Cargo.toml`
2019-07-19T13:50:38.6685514Z Caused by:
2019-07-19T13:50:38.6706516Z   No such file or directory (os error 2)
2019-07-19T13:50:38.6706516Z   No such file or directory (os error 2)
2019-07-19T13:50:38.6707411Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2019-07-19T13:50:38.6707642Z Build completed unsuccessfully in 0:00:15
2019-07-19T13:50:55.8523168Z ##[error]Bash exited with code '1'.
2019-07-19T13:50:55.8579890Z ##[section]Starting: Checkout
2019-07-19T13:50:55.8581531Z ==============================================================================
2019-07-19T13:50:55.8581578Z Task         : Get sources
2019-07-19T13:50:55.8581638Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
