plain
2020-01-18T06:34:53.4847767Z ========================== Starting Command Output ===========================
2020-01-18T06:34:53.6908312Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d35f9428-a750-4eee-a381-b22eaa80e15a.sh
2020-01-18T06:34:53.6908509Z 
2020-01-18T06:34:53.6962313Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-18T06:34:53.6968239Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T06:34:53.6969998Z Task         : Get sources
2020-01-18T06:34:53.6970029Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T06:34:53.6970059Z Version      : 1.0.0
2020-01-18T06:34:53.6970107Z Author       : Microsoft
---
2020-01-18T06:34:54.6448517Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-18T06:34:54.6528620Z ##[command]git config gc.auto 0
2020-01-18T06:34:54.6595702Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-18T06:34:54.6654096Z ##[command]git config --get-all http.proxy
2020-01-18T06:34:54.6809013Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68332/merge:refs/remotes/pull/68332/merge
---
2020-01-18T06:39:43.1216129Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-01-18T06:39:45.2537206Z error: cannot find macro `gen_fn_struct` in this scope
2020-01-18T06:39:45.2537630Z    --> src/libcore/ops/function.rs:292:9
2020-01-18T06:39:45.2537892Z     |
2020-01-18T06:39:45.2538219Z 290 | / macro_rules! gen_fn_struct_unopt {
2020-01-18T06:39:45.2538608Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T06:39:45.2538962Z 292 | |         gen_fn_struct!(impl $imp, $method with $m in $z,
2020-01-18T06:39:45.2539290Z     | |         ^^^^^^^^^^^^^
2020-01-18T06:39:45.2539632Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T06:39:45.2540214Z 356 | |     };
2020-01-18T06:39:45.2540502Z 357 | | }
2020-01-18T06:39:45.2540502Z 357 | | }
2020-01-18T06:39:45.2541404Z     | |_- in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:39:45.2541759Z 358 |   gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T06:39:45.2542214Z 
2020-01-18T06:39:45.2548561Z error: cannot find macro `gen_fn_struct` in this scope
2020-01-18T06:39:45.2549006Z    --> src/libcore/ops/function.rs:292:9
2020-01-18T06:39:45.2549268Z     |
2020-01-18T06:39:45.2549268Z     |
2020-01-18T06:39:45.2549587Z 290 | / macro_rules! gen_fn_struct_unopt {
2020-01-18T06:39:45.2549950Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T06:39:45.2550328Z 292 | |         gen_fn_struct!(impl $imp, $method with $m in $z,
2020-01-18T06:39:45.2550912Z     | |         ^^^^^^^^^^^^^
2020-01-18T06:39:45.2551327Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T06:39:45.2551905Z 356 | |     };
2020-01-18T06:39:45.2552195Z 357 | | }
2020-01-18T06:39:45.2552195Z 357 | | }
2020-01-18T06:39:45.2552518Z     | |_- in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:39:45.2552862Z 358 |   gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T06:39:45.2553192Z 359 |   gen_fn_struct_unopt!(impl Neg, neg with fn_neg_impl in FnNeg);
2020-01-18T06:39:45.2553657Z 
2020-01-18T06:39:52.0253226Z    Compiling libc v0.2.66
2020-01-18T06:39:53.0620033Z    Compiling autocfg v0.1.6
2020-01-18T06:39:54.2675028Z error: aborting due to 2 previous errors
---
2020-01-18T06:39:54.5910953Z   local time: Sat Jan 18 06:39:54 UTC 2020
2020-01-18T06:39:54.7474755Z   network time: Sat, 18 Jan 2020 06:39:54 GMT
2020-01-18T06:39:54.7479371Z == end clock drift check ==
2020-01-18T06:40:02.8542928Z 
2020-01-18T06:40:02.8719738Z ##[error]Bash exited with code '1'.
2020-01-18T06:40:02.8736079Z ##[section]Finishing: Run build
2020-01-18T06:40:02.8753226Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T06:40:02.8755279Z Task         : Get sources
2020-01-18T06:40:02.8755345Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T06:40:02.8755538Z Version      : 1.0.0
2020-01-18T06:40:02.8755579Z Author       : Microsoft
2020-01-18T06:40:02.8755579Z Author       : Microsoft
2020-01-18T06:40:02.8755644Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-18T06:40:02.8755692Z ==============================================================================
2020-01-18T06:40:03.3121217Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-18T06:40:03.3162337Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T06:40:03.3279892Z Cleaning up task key
2020-01-18T06:40:03.3281191Z Start cleaning up orphan processes.
2020-01-18T06:40:03.3399479Z Terminate orphan process: pid (3429) (python)
2020-01-18T06:40:03.3603013Z ##[section]Finishing: Finalize Job
