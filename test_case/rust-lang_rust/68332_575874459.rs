plain
2020-01-18T07:14:31.1669856Z ========================== Starting Command Output ===========================
2020-01-18T07:14:31.1671174Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/46fb606f-1ad6-4b9c-88fa-668154f9a324.sh
2020-01-18T07:14:31.1671216Z 
2020-01-18T07:14:31.1673492Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-18T07:14:31.1679123Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T07:14:31.1680726Z Task         : Get sources
2020-01-18T07:14:31.1680758Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T07:14:31.1680789Z Version      : 1.0.0
2020-01-18T07:14:31.1680819Z Author       : Microsoft
---
2020-01-18T07:14:31.9818098Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-18T07:14:31.9890233Z ##[command]git config gc.auto 0
2020-01-18T07:14:31.9965981Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-18T07:14:32.0024822Z ##[command]git config --get-all http.proxy
2020-01-18T07:14:32.0158026Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68332/merge:refs/remotes/pull/68332/merge
---
2020-01-18T07:18:29.8055515Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-01-18T07:18:31.2899137Z error: expected unsuffixed literal or identifier, found `fn_not_impl`
2020-01-18T07:18:31.2899528Z    --> src/libcore/ops/function.rs:376:41
2020-01-18T07:18:31.2899812Z     |
2020-01-18T07:18:31.2900088Z 376 | gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T07:18:31.2900639Z 
2020-01-18T07:18:31.2914568Z error: expected unsuffixed literal or identifier, found `fn_neg_impl`
2020-01-18T07:18:31.2915264Z    --> src/libcore/ops/function.rs:377:41
2020-01-18T07:18:31.2915463Z     |
2020-01-18T07:18:31.2915463Z     |
2020-01-18T07:18:31.2915886Z 377 | gen_fn_struct_unopt!(impl Neg, neg with fn_neg_impl in FnNeg);
2020-01-18T07:18:31.2916222Z 
2020-01-18T07:18:33.7122668Z error[E0207]: the type parameter `A` is not constrained by the impl trait, self type, or predicates
2020-01-18T07:18:33.7123304Z    --> src/libcore/ops/function.rs:303:18
2020-01-18T07:18:33.7123522Z     |
2020-01-18T07:18:33.7123522Z     |
2020-01-18T07:18:33.7123780Z 290 |    macro_rules! gen_fn_struct_unopt {
2020-01-18T07:18:33.7123996Z     |   _-
2020-01-18T07:18:33.7124218Z     |  |_|
2020-01-18T07:18:33.7124423Z     |  |
2020-01-18T07:18:33.7124715Z 291 |  |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T07:18:33.7125385Z 292 |  |         gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T07:18:33.7125655Z     |  |_________-
2020-01-18T07:18:33.7126202Z 293 | ||             #[unstable(feature = $m)]);
2020-01-18T07:18:33.7126535Z     | ||_______________________________________- in this macro invocation
2020-01-18T07:18:33.7126816Z ...    |
2020-01-18T07:18:33.7127265Z 303 |  |             impl<A, F: ?Sized> $imp for F
2020-01-18T07:18:33.7127566Z     |  |                  ^ unconstrained type parameter
2020-01-18T07:18:33.7128066Z 374 |  |     };
2020-01-18T07:18:33.7128329Z 375 |  | }
2020-01-18T07:18:33.7128569Z     |  | -
2020-01-18T07:18:33.7128778Z     |  |_|
2020-01-18T07:18:33.7128778Z     |  |_|
2020-01-18T07:18:33.7129059Z     |  |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:18:33.7129303Z     |    in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:18:33.7129591Z 376 |  | gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T07:18:33.7130104Z 
2020-01-18T07:18:33.7130406Z error[E0207]: the type parameter `A` is not constrained by the impl trait, self type, or predicates
2020-01-18T07:18:33.7130627Z    --> src/libcore/ops/function.rs:312:18
2020-01-18T07:18:33.7130808Z     |
2020-01-18T07:18:33.7130808Z     |
2020-01-18T07:18:33.7131068Z 290 |    macro_rules! gen_fn_struct_unopt {
2020-01-18T07:18:33.7131276Z     |   _-
2020-01-18T07:18:33.7131481Z     |  |_|
2020-01-18T07:18:33.7131978Z     |  |
2020-01-18T07:18:33.7132285Z 291 |  |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T07:18:33.7132605Z 292 |  |         gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T07:18:33.7132858Z     |  |_________-
2020-01-18T07:18:33.7133148Z 293 | ||             #[unstable(feature = $m)]);
2020-01-18T07:18:33.7133492Z     | ||_______________________________________- in this macro invocation
2020-01-18T07:18:33.7133712Z ...    |
2020-01-18T07:18:33.7134014Z 312 |  |             impl<A, F: ?Sized> $imp for $z<F>
2020-01-18T07:18:33.7134327Z     |  |                  ^ unconstrained type parameter
2020-01-18T07:18:33.7134981Z 374 |  |     };
2020-01-18T07:18:33.7135235Z 375 |  | }
2020-01-18T07:18:33.7135631Z     |  | -
2020-01-18T07:18:33.7136026Z     |  |_|
2020-01-18T07:18:33.7136026Z     |  |_|
2020-01-18T07:18:33.7136464Z     |  |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:18:33.7136724Z     |    in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:18:33.7137050Z 376 |  | gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T07:18:33.7137459Z 
2020-01-18T07:18:33.7276319Z error[E0207]: the type parameter `A` is not constrained by the impl trait, self type, or predicates
2020-01-18T07:18:33.7276610Z    --> src/libcore/ops/function.rs:303:18
2020-01-18T07:18:33.7276840Z     |
2020-01-18T07:18:33.7276840Z     |
2020-01-18T07:18:33.7277225Z 290 |    macro_rules! gen_fn_struct_unopt {
2020-01-18T07:18:33.7277524Z     |   _-
2020-01-18T07:18:33.7277749Z     |  |_|
2020-01-18T07:18:33.7277965Z     |  |
2020-01-18T07:18:33.7278293Z 291 |  |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T07:18:33.7278611Z 292 |  |         gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T07:18:33.7278978Z     |  |_________-
2020-01-18T07:18:33.7279308Z 293 | ||             #[unstable(feature = $m)]);
2020-01-18T07:18:33.7279645Z     | ||_______________________________________- in this macro invocation
2020-01-18T07:18:33.7279881Z ...    |
2020-01-18T07:18:33.7280168Z 303 |  |             impl<A, F: ?Sized> $imp for F
2020-01-18T07:18:33.7280704Z     |  |                  ^ unconstrained type parameter
2020-01-18T07:18:33.7281230Z 374 |  |     };
2020-01-18T07:18:33.7281675Z 375 |  | }
2020-01-18T07:18:33.7282183Z     |  | -
2020-01-18T07:18:33.7282409Z     |  |_|
2020-01-18T07:18:33.7282409Z     |  |_|
2020-01-18T07:18:33.7282692Z     |  |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:18:33.7282981Z     |    in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:18:33.7283432Z 376 |    gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T07:18:33.7283774Z 377 |  | gen_fn_struct_unopt!(impl Neg, neg with fn_neg_impl in FnNeg);
2020-01-18T07:18:33.7284174Z 
2020-01-18T07:18:33.7289506Z error[E0207]: the type parameter `A` is not constrained by the impl trait, self type, or predicates
2020-01-18T07:18:33.7289767Z    --> src/libcore/ops/function.rs:312:18
2020-01-18T07:18:33.7289985Z     |
2020-01-18T07:18:33.7289985Z     |
2020-01-18T07:18:33.7290244Z 290 |    macro_rules! gen_fn_struct_unopt {
2020-01-18T07:18:33.7290483Z     |   _-
2020-01-18T07:18:33.7290720Z     |  |_|
2020-01-18T07:18:33.7291221Z     |  |
2020-01-18T07:18:33.7291789Z 291 |  |     (impl $imp:ident, $method:ident with $m:ident in $z:ident) => {
2020-01-18T07:18:33.7292129Z 292 |  |         gen_fn_struct_unopt!(impl $imp, $method with $m in $z,
2020-01-18T07:18:33.7292385Z     |  |_________-
2020-01-18T07:18:33.7292936Z 293 | ||             #[unstable(feature = $m)]);
2020-01-18T07:18:33.7293395Z     | ||_______________________________________- in this macro invocation
2020-01-18T07:18:33.7293632Z ...    |
2020-01-18T07:18:33.7293956Z 312 |  |             impl<A, F: ?Sized> $imp for $z<F>
2020-01-18T07:18:33.7294437Z     |  |                  ^ unconstrained type parameter
2020-01-18T07:18:33.7295078Z 374 |  |     };
2020-01-18T07:18:33.7295324Z 375 |  | }
2020-01-18T07:18:33.7295589Z     |  | -
2020-01-18T07:18:33.7295813Z     |  |_|
2020-01-18T07:18:33.7295813Z     |  |_|
2020-01-18T07:18:33.7296080Z     |  |_in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:18:33.7296348Z     |    in this expansion of `gen_fn_struct_unopt!`
2020-01-18T07:18:33.7296623Z 376 |    gen_fn_struct_unopt!(impl Not, not with fn_not_impl in FnNot);
2020-01-18T07:18:33.7296921Z 377 |  | gen_fn_struct_unopt!(impl Neg, neg with fn_neg_impl in FnNeg);
2020-01-18T07:18:33.7297327Z 
2020-01-18T07:18:33.7962042Z error: aborting due to 6 previous errors
2020-01-18T07:18:33.7962179Z 
2020-01-18T07:18:33.7962490Z For more information about this error, try `rustc --explain E0207`.
---
2020-01-18T07:18:35.6587981Z   local time: Sat Jan 18 07:18:35 UTC 2020
2020-01-18T07:18:35.9431050Z   network time: Sat, 18 Jan 2020 07:18:35 GMT
2020-01-18T07:18:35.9435856Z == end clock drift check ==
2020-01-18T07:18:44.3502441Z 
2020-01-18T07:18:44.3605845Z ##[error]Bash exited with code '1'.
2020-01-18T07:18:44.3616736Z ##[section]Finishing: Run build
2020-01-18T07:18:44.3630765Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T07:18:44.3632766Z Task         : Get sources
2020-01-18T07:18:44.3632825Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T07:18:44.3632865Z Version      : 1.0.0
2020-01-18T07:18:44.3632900Z Author       : Microsoft
2020-01-18T07:18:44.3632900Z Author       : Microsoft
2020-01-18T07:18:44.3632958Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-18T07:18:44.3633001Z ==============================================================================
2020-01-18T07:18:44.7544938Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-18T07:18:44.7585413Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T07:18:44.7698412Z Cleaning up task key
2020-01-18T07:18:44.7699371Z Start cleaning up orphan processes.
2020-01-18T07:18:44.7802940Z Terminate orphan process: pid (4106) (python)
2020-01-18T07:18:44.7989119Z ##[section]Finishing: Finalize Job
