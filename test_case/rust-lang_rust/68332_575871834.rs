plain
2020-01-18T06:26:57.7704595Z ========================== Starting Command Output ===========================
2020-01-18T06:26:57.7707163Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/859c9f0a-58c1-46b2-ae4f-4069ac497374.sh
2020-01-18T06:26:57.7707260Z 
2020-01-18T06:26:57.7711176Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-18T06:26:57.7717771Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T06:26:57.7719456Z Task         : Get sources
2020-01-18T06:26:57.7719489Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T06:26:57.7719522Z Version      : 1.0.0
2020-01-18T06:26:57.7719594Z Author       : Microsoft
---
2020-01-18T06:26:58.4905551Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-18T06:26:58.4919891Z ##[command]git config gc.auto 0
2020-01-18T06:26:58.4980731Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-18T06:26:58.5031095Z ##[command]git config --get-all http.proxy
2020-01-18T06:26:58.5166880Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68332/merge:refs/remotes/pull/68332/merge
---
2020-01-18T06:31:01.8963523Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-18T06:31:01.8981204Z Checking std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-18T06:31:02.1504286Z    Compiling cc v1.0.49
2020-01-18T06:31:02.1510386Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-01-18T06:31:03.9910328Z error: cannot find macro `gen_bit_fn_struct` in this scope
2020-01-18T06:31:03.9911763Z    --> src/libcore/ops/function.rs:292:9
2020-01-18T06:31:03.9913559Z 290 | / macro_rules! gen_fn_struct_unopt {
2020-01-18T06:31:03.9913559Z 290 | / macro_rules! gen_fn_struct_unopt {
2020-01-18T06:31:03.9913995Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T06:31:03.9914358Z 292 | |         gen_bit_fn_struct!(impl $imp, $method with $m in $z,
2020-01-18T06:31:03.9914719Z     | |         ^^^^^^^^^^^^^^^^^
2020-01-18T06:31:03.9915054Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T06:31:03.9915618Z 356 | |     };
2020-01-18T06:31:03.9915903Z 357 | | }
2020-01-18T06:31:03.9915903Z 357 | | }
2020-01-18T06:31:03.9916440Z     | |_- in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:31:03.9916838Z 358 |   gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T06:31:03.9917458Z 
2020-01-18T06:31:03.9917458Z 
2020-01-18T06:31:03.9917813Z error: cannot find macro `gen_bit_fn_struct` in this scope
2020-01-18T06:31:03.9918103Z    --> src/libcore/ops/function.rs:292:9
2020-01-18T06:31:03.9918647Z 290 | / macro_rules! gen_fn_struct_unopt {
2020-01-18T06:31:03.9918647Z 290 | / macro_rules! gen_fn_struct_unopt {
2020-01-18T06:31:03.9919032Z 291 | |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T06:31:03.9919389Z 292 | |         gen_bit_fn_struct!(impl $imp, $method with $m in $z,
2020-01-18T06:31:03.9919726Z     | |         ^^^^^^^^^^^^^^^^^
2020-01-18T06:31:03.9920057Z 293 | |             #[unstable(feature = $m)]);
2020-01-18T06:31:03.9920611Z 356 | |     };
2020-01-18T06:31:03.9920899Z 357 | | }
2020-01-18T06:31:03.9920899Z 357 | | }
2020-01-18T06:31:03.9921227Z     | |_- in this expansion of `gen_fn_struct_unopt!`
2020-01-18T06:31:03.9921568Z 358 |   gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T06:31:03.9944432Z 359 |   gen_fn_struct_unopt!(impl Neg, neg with fn_neg_impl in FnNeg);
2020-01-18T06:31:03.9945140Z 
2020-01-18T06:31:09.8441137Z    Compiling libc v0.2.66
2020-01-18T06:31:10.7646568Z    Compiling autocfg v0.1.6
2020-01-18T06:31:11.9983022Z error: aborting due to 2 previous errors
---
2020-01-18T06:31:12.2002578Z   local time: Sat Jan 18 06:31:12 UTC 2020
2020-01-18T06:31:12.4840521Z   network time: Sat, 18 Jan 2020 06:31:12 GMT
2020-01-18T06:31:12.4840642Z == end clock drift check ==
2020-01-18T06:31:20.2361073Z 
2020-01-18T06:31:20.2460925Z ##[error]Bash exited with code '1'.
2020-01-18T06:31:20.2474163Z ##[section]Finishing: Run build
2020-01-18T06:31:20.2488967Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T06:31:20.2490814Z Task         : Get sources
2020-01-18T06:31:20.2490862Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T06:31:20.2490908Z Version      : 1.0.0
2020-01-18T06:31:20.2490967Z Author       : Microsoft
2020-01-18T06:31:20.2490967Z Author       : Microsoft
2020-01-18T06:31:20.2491014Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-18T06:31:20.2491066Z ==============================================================================
2020-01-18T06:31:20.6853144Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-18T06:31:20.6893138Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T06:31:20.7009364Z Cleaning up task key
2020-01-18T06:31:20.7010182Z Start cleaning up orphan processes.
2020-01-18T06:31:20.7131124Z Terminate orphan process: pid (3882) (python)
2020-01-18T06:31:20.7416167Z ##[section]Finishing: Finalize Job
