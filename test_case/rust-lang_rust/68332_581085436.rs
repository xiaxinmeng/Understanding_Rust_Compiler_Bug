plain
2020-02-02T00:47:19.4068147Z ========================== Starting Command Output ===========================
2020-02-02T00:47:19.4069615Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/532d4663-2e6c-465c-b048-7a8ade39fac0.sh
2020-02-02T00:47:19.4069653Z 
2020-02-02T00:47:19.4075629Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-02T00:47:19.4084914Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-02-02T00:47:19.4086899Z Task         : Get sources
2020-02-02T00:47:19.4086978Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-02T00:47:19.4087013Z Version      : 1.0.0
2020-02-02T00:47:19.4087046Z Author       : Microsoft
---
2020-02-02T00:47:20.4052892Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-02T00:47:20.4070519Z ##[command]git config gc.auto 0
2020-02-02T00:47:20.4072841Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-02T00:47:20.4075891Z ##[command]git config --get-all http.proxy
2020-02-02T00:47:20.4109579Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68332/merge:refs/remotes/pull/68332/merge
---
2020-02-02T00:52:35.7950744Z 290 |    macro_rules! gen_fn_struct_unopt {
2020-02-02T00:52:35.7950990Z     |   _-
2020-02-02T00:52:35.7951252Z     |  |_|
2020-02-02T00:52:35.7951495Z     |  |
2020-02-02T00:52:35.7951831Z 291 |  |     (impl $imp:ident, $method:ident in $m:ident; with $z:ty) => {
2020-02-02T00:52:35.7952189Z 292 |  |         gen_fn_struct_unopt!(impl $imp, $method in $m; with $z,
2020-02-02T00:52:35.7952480Z     |  |_________-
2020-02-02T00:52:35.7952833Z 293 | ||             #[unstable(feature = stringify!($m))]);
2020-02-02T00:52:35.7953213Z     | ||___________________________________________________- in this macro invocation
2020-02-02T00:52:35.7953473Z ...    |
2020-02-02T00:52:35.7953770Z 300 |  |             struct $z<F> {
2020-02-02T00:52:35.7954965Z ...    |
2020-02-02T00:52:35.7955263Z 376 |  |     };
2020-02-02T00:52:35.7955554Z 377 |  | }
2020-02-02T00:52:35.7956091Z     |  | -
2020-02-02T00:52:35.7956091Z     |  | -
2020-02-02T00:52:35.7956507Z     |  |_|
2020-02-02T00:52:35.7956869Z     |  |_in this expansion of `gen_fn_struct_unopt!`
2020-02-02T00:52:35.7957197Z     |    in this expansion of `gen_fn_struct_unopt!`
2020-02-02T00:52:35.7957404Z ...
2020-02-02T00:52:35.7957750Z 459 |  | gen_fn_struct_unopt!(impl Not, not in fn_not_impl; with FnNot);
2020-02-02T00:52:35.7958251Z 
2020-02-02T00:52:35.8383471Z error: aborting due to previous error
2020-02-02T00:52:35.8383628Z 
2020-02-02T00:52:35.8418624Z error: could not compile `core`.
---
2020-02-02T00:52:41.1787505Z   local time: Sun Feb  2 00:52:41 UTC 2020
2020-02-02T00:52:41.2706016Z   network time: Sun, 02 Feb 2020 00:52:41 GMT
2020-02-02T00:52:41.2708842Z == end clock drift check ==
2020-02-02T00:52:45.5557219Z 
2020-02-02T00:52:45.5653807Z ##[error]Bash exited with code '1'.
2020-02-02T00:52:45.5680825Z ##[section]Finishing: Run build
2020-02-02T00:52:45.5697706Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-02-02T00:52:45.5699860Z Task         : Get sources
2020-02-02T00:52:45.5699907Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-02T00:52:45.5699954Z Version      : 1.0.0
2020-02-02T00:52:45.5700015Z Author       : Microsoft
2020-02-02T00:52:45.5700015Z Author       : Microsoft
2020-02-02T00:52:45.5700065Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-02T00:52:45.5700115Z ==============================================================================
2020-02-02T00:52:46.0174972Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-02T00:52:46.0218848Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-02-02T00:52:46.0344902Z Cleaning up task key
2020-02-02T00:52:46.0345874Z Start cleaning up orphan processes.
2020-02-02T00:52:46.0519093Z Terminate orphan process: pid (5511) (python)
2020-02-02T00:52:46.0732648Z ##[section]Finishing: Finalize Job
