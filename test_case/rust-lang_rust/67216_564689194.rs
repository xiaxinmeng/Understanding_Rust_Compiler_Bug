plain
2019-12-11T18:42:57.9089763Z ========================== Starting Command Output ===========================
2019-12-11T18:42:57.9095227Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4a3a19bd-72f1-48b3-bfb8-33d41b9f1ace.sh
2019-12-11T18:42:57.9095488Z 
2019-12-11T18:42:57.9099603Z ##[section]Finishing: Disable git automatic line ending conversion
2019-12-11T18:42:57.9106155Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67216/merge to s
2019-12-11T18:42:57.9108317Z Task         : Get sources
2019-12-11T18:42:57.9108352Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2019-12-11T18:42:57.9108385Z Version      : 1.0.0
2019-12-11T18:42:57.9108464Z Author       : Microsoft
---
2019-12-11T18:43:00.2441862Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-11T18:43:01.0241937Z ##[command]git config gc.auto 0
2019-12-11T18:43:01.0244568Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-11T18:43:01.0246695Z ##[command]git config --get-all http.proxy
2019-12-11T18:43:01.0254499Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67216/merge:refs/remotes/pull/67216/merge
---
2019-12-11T18:52:10.6069709Z 
2019-12-11T18:52:10.6116623Z error[E0425]: cannot find value `gates` in this scope
2019-12-11T18:52:10.6116921Z    --> src/librustc_passes/check_const.rs:169:46
2019-12-11T18:52:10.6117193Z     |
2019-12-11T18:52:10.6117513Z 169 |             &[gate @ sym::const_if_match] if gates.contains(&sym::const_loop) => {
2019-12-11T18:52:10.6118060Z     |                                              ^^^^^ help: a local variable with a similar name exists: `gate`
2019-12-11T18:52:11.1590589Z error: aborting due to 2 previous errors
2019-12-11T18:52:11.1590679Z 
2019-12-11T18:52:11.1594531Z For more information about this error, try `rustc --explain E0425`.
2019-12-11T18:52:11.1758195Z error: could not compile `rustc_passes`.
---
2019-12-11T18:52:29.6910304Z   local time: Wed Dec 11 18:52:29 UTC 2019
2019-12-11T18:52:29.9709226Z   network time: Wed, 11 Dec 2019 18:52:29 GMT
2019-12-11T18:52:29.9709494Z == end clock drift check ==
2019-12-11T18:52:31.4202197Z 
2019-12-11T18:52:31.4292907Z ##[error]Bash exited with code '1'.
2019-12-11T18:52:31.4307660Z ##[section]Finishing: Run build
2019-12-11T18:52:31.4361438Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67216/merge to s
2019-12-11T18:52:31.4363914Z Task         : Get sources
2019-12-11T18:52:31.4363962Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2019-12-11T18:52:31.4364010Z Version      : 1.0.0
2019-12-11T18:52:31.4364070Z Author       : Microsoft
2019-12-11T18:52:31.4364070Z Author       : Microsoft
2019-12-11T18:52:31.4364117Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2019-12-11T18:52:31.4364167Z ==============================================================================
2019-12-11T18:52:32.0053849Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2019-12-11T18:52:32.0103078Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67216/merge to s
2019-12-11T18:52:32.0234939Z Start cleaning up orphan processes.
2019-12-11T18:52:32.0366463Z Terminate orphan process: pid (3537) (python)
2019-12-11T18:52:32.2165299Z ##[section]Finishing: Finalize Job
2019-12-11T18:52:32.2237085Z ##[section]Finishing: Linux mingw-check
