plain
2020-02-02T00:55:40.4585324Z ========================== Starting Command Output ===========================
2020-02-02T00:55:40.4587273Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4055d56d-d4e7-4aed-9998-18cf3f57c7dc.sh
2020-02-02T00:55:40.4587432Z 
2020-02-02T00:55:40.4590335Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-02T00:55:40.4595372Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-02-02T00:55:40.4597318Z Task         : Get sources
2020-02-02T00:55:40.4597351Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-02T00:55:40.4597383Z Version      : 1.0.0
2020-02-02T00:55:40.4597424Z Author       : Microsoft
---
2020-02-02T00:55:41.3486720Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-02T00:55:41.3596171Z ##[command]git config gc.auto 0
2020-02-02T00:55:41.3675534Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-02T00:55:41.3730425Z ##[command]git config --get-all http.proxy
2020-02-02T00:55:41.3887352Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68332/merge:refs/remotes/pull/68332/merge
---
2020-02-02T01:00:23.2172602Z 290 |    macro_rules! gen_fn_struct_unopt {
2020-02-02T01:00:23.2173084Z     |   _-
2020-02-02T01:00:23.2173591Z     |  |_|
2020-02-02T01:00:23.2174439Z     |  |
2020-02-02T01:00:23.2175291Z 291 |  |     (impl $imp:ident, $method:ident in $m:ident; with $z:ty) => {
2020-02-02T01:00:23.2175915Z 292 |  |         gen_fn_struct_unopt!(impl $imp, $method in $m; with $z,
2020-02-02T01:00:23.2176449Z     |  |_________-
2020-02-02T01:00:23.2177348Z 293 | ||             #[unstable(feature = stringify!($m))]);
2020-02-02T01:00:23.2178978Z     | ||___________________________________________________- in this macro invocation
2020-02-02T01:00:23.2179736Z ...    |
2020-02-02T01:00:23.2182273Z 300 |  |             struct $z<F> {
2020-02-02T01:00:23.2184004Z ...    |
2020-02-02T01:00:23.2184506Z 376 |  |     };
2020-02-02T01:00:23.2185030Z 377 |  | }
2020-02-02T01:00:23.2185536Z     |  | -
2020-02-02T01:00:23.2185536Z     |  | -
2020-02-02T01:00:23.2186019Z     |  |_|
2020-02-02T01:00:23.2186692Z     |  |_in this expansion of `gen_fn_struct_unopt!`
2020-02-02T01:00:23.2187186Z     |    in this expansion of `gen_fn_struct_unopt!`
2020-02-02T01:00:23.2187625Z ...
2020-02-02T01:00:23.2188158Z 465 |  | gen_fn_struct_unopt!(impl Not, not in fn_not_impl; with FnNot);
2020-02-02T01:00:23.2188980Z 
2020-02-02T01:00:23.2604556Z error: aborting due to previous error
2020-02-02T01:00:23.2605553Z 
2020-02-02T01:00:23.2646541Z error: could not compile `core`.
---
2020-02-02T01:00:27.7380845Z   local time: Sun Feb  2 01:00:27 UTC 2020
2020-02-02T01:00:27.8884168Z   network time: Sun, 02 Feb 2020 01:00:27 GMT
2020-02-02T01:00:27.8887307Z == end clock drift check ==
2020-02-02T01:00:35.9557043Z 
2020-02-02T01:00:35.9670046Z ##[error]Bash exited with code '1'.
2020-02-02T01:00:35.9683438Z ##[section]Finishing: Run build
2020-02-02T01:00:35.9698333Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-02-02T01:00:35.9699928Z Task         : Get sources
2020-02-02T01:00:35.9699991Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-02T01:00:35.9700031Z Version      : 1.0.0
2020-02-02T01:00:35.9700067Z Author       : Microsoft
2020-02-02T01:00:35.9700067Z Author       : Microsoft
2020-02-02T01:00:35.9700125Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-02T01:00:35.9700169Z ==============================================================================
2020-02-02T01:00:36.3803590Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-02T01:00:36.3845082Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-02-02T01:00:36.3966256Z Cleaning up task key
2020-02-02T01:00:36.3967403Z Start cleaning up orphan processes.
2020-02-02T01:00:36.4077956Z Terminate orphan process: pid (5645) (python)
2020-02-02T01:00:36.4325397Z ##[section]Finishing: Finalize Job
