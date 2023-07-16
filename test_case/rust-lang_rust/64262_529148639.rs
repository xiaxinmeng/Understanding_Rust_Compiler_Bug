plain
2019-09-07T19:19:12.3228506Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-07T19:19:12.3381513Z ##[command]git config gc.auto 0
2019-09-07T19:19:12.3457064Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-07T19:19:12.3521805Z ##[command]git config --get-all http.proxy
2019-09-07T19:19:12.3686631Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64262/merge:refs/remotes/pull/64262/merge
---
2019-09-07T21:19:16.8714600Z    Compiling rustbook v0.1.0 (/checkout/src/tools/rustbook)
2019-09-07T21:19:21.8454776Z     Finished release [optimized] target(s) in 9m 09s
2019-09-07T21:21:13.7524783Z Error: there are broken links
2019-09-07T21:21:13.7526153Z  Caused By: There was an error while fetching "https://public.etherpad-mozilla.org/p/rust-compiler-meeting", https://public.etherpad-mozilla.org/p/rust-compiler-meeting: error trying to connect: Connection refused (os error 111)
2019-09-07T21:21:13.7526939Z  Caused By: There was an error while fetching "http://www.cs.ucla.edu/~palsberg/tba/papers/tofte-talpin-iandc97.pdf", http://www.cs.ucla.edu/~palsberg/tba/papers/tofte-talpin-iandc97.pdf: timed out
2019-09-07T21:21:13.7537536Z 
2019-09-07T21:21:13.7538411Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustbook" "linkcheck" "/checkout/src/doc/rustc-guide"
2019-09-07T21:21:13.7538512Z expected success, got: exit code: 101
2019-09-07T21:21:13.7538544Z 
---
2019-09-07T21:34:12.1856541Z Verifying status of rustfmt...
2019-09-07T21:34:12.1872506Z Verifying status of clippy-driver...
2019-09-07T21:34:12.1886576Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-09-07T21:34:12.1906906Z 
2019-09-07T21:34:12.1907465Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-09-07T21:34:12.1907528Z 
2019-09-07T21:34:12.1910713Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-09-07T21:34:12.1910793Z commit another update.
2019-09-07T21:34:12.1911642Z 
2019-09-07T21:34:12.1912028Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-09-07T21:34:12.1912305Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-09-07T21:34:12.1912356Z proper steps.
2019-09-07T21:34:12.1927985Z   local time: Sat Sep  7 21:34:12 UTC 2019
2019-09-07T21:34:12.2829109Z   network time: Sat, 07 Sep 2019 21:34:12 GMT
2019-09-07T21:34:12.2834891Z == end clock drift check ==
2019-09-07T21:34:12.2834891Z == end clock drift check ==
2019-09-07T21:34:13.8651715Z ##[error]Bash exited with code '3'.
2019-09-07T21:34:13.8694471Z ##[section]Starting: Checkout
2019-09-07T21:34:13.8696789Z ==============================================================================
2019-09-07T21:34:13.8696844Z Task         : Get sources
2019-09-07T21:34:13.8697102Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
