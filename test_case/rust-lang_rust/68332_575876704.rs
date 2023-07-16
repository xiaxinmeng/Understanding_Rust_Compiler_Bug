plain
2020-01-18T07:52:52.1011543Z ========================== Starting Command Output ===========================
2020-01-18T07:52:52.1030926Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/759cd4cd-75bc-4196-9316-362d6c7cd0de.sh
2020-01-18T07:52:53.0714126Z 
2020-01-18T07:52:53.0779472Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-18T07:52:53.0785481Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T07:52:53.0787340Z Task         : Get sources
2020-01-18T07:52:53.0787373Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T07:52:53.0787405Z Version      : 1.0.0
2020-01-18T07:52:53.0787483Z Author       : Microsoft
---
2020-01-18T07:52:58.4530060Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-18T07:52:58.4744068Z ##[command]git config gc.auto 0
2020-01-18T07:52:58.4799208Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-18T07:52:58.4851479Z ##[command]git config --get-all http.proxy
2020-01-18T07:52:58.5008406Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68332/merge:refs/remotes/pull/68332/merge
---
2020-01-18T07:57:34.4980854Z 290 |    macro_rules! gen_fn_struct_unopt {
2020-01-18T07:57:34.4981153Z     |   _-
2020-01-18T07:57:34.4981420Z     |  |_|
2020-01-18T07:57:34.4981676Z     |  |
2020-01-18T07:57:34.4982051Z 291 |  |     (impl $imp:ident, $method:ident in $m:path; with $z:ty) => {
2020-01-18T07:57:34.4982433Z 292 |  |         gen_fn_struct_unopt!(impl $imp, $method in $m; with $z,
2020-01-18T07:57:34.4982759Z     |  |_________-
2020-01-18T07:57:34.4983128Z 293 | ||             #[unstable(feature = stringify!($m))]);
2020-01-18T07:57:34.4983541Z     | ||___________________________________________________- in this macro invocation
2020-01-18T07:57:34.4984137Z 297 |  |         mod $m {
2020-01-18T07:57:34.4984786Z     |  |             ^^ expected identifier
2020-01-18T07:57:34.4985050Z ...    |
2020-01-18T07:57:34.4985714Z 376 |  |     };
2020-01-18T07:57:34.4985714Z 376 |  |     };
2020-01-18T07:57:34.4986155Z 377 |  | }
2020-01-18T07:57:34.4986455Z     |  | -
2020-01-18T07:57:34.4986717Z     |  |_|
2020-01-18T07:57:34.4987053Z     |  |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:57:34.4987377Z     |    in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:57:34.4987740Z 378 |  | gen_fn_struct_unopt!(impl Not, not in fn_not_impl; with FnNot);
2020-01-18T07:57:34.4988226Z 
2020-01-18T07:57:34.5379312Z error: aborting due to previous error
2020-01-18T07:57:34.5379420Z 
2020-01-18T07:57:34.5403995Z error: could not compile `core`.
---
2020-01-18T07:57:38.3403139Z   local time: Sat Jan 18 07:57:38 UTC 2020
2020-01-18T07:57:38.6650961Z   network time: Sat, 18 Jan 2020 07:57:38 GMT
2020-01-18T07:57:38.6654567Z == end clock drift check ==
2020-01-18T07:57:46.9909569Z 
2020-01-18T07:57:47.0006791Z ##[error]Bash exited with code '1'.
2020-01-18T07:57:47.0019438Z ##[section]Finishing: Run build
2020-01-18T07:57:47.0034782Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T07:57:47.0037068Z Task         : Get sources
2020-01-18T07:57:47.0037113Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T07:57:47.0037159Z Version      : 1.0.0
2020-01-18T07:57:47.0037216Z Author       : Microsoft
2020-01-18T07:57:47.0037216Z Author       : Microsoft
2020-01-18T07:57:47.0037262Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-18T07:57:47.0037313Z ==============================================================================
2020-01-18T07:57:47.4042257Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-18T07:57:47.4083002Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T07:57:47.4195631Z Cleaning up task key
2020-01-18T07:57:47.4196433Z Start cleaning up orphan processes.
2020-01-18T07:57:47.4308756Z Terminate orphan process: pid (3622) (python)
2020-01-18T07:57:47.4732594Z ##[section]Finishing: Finalize Job
