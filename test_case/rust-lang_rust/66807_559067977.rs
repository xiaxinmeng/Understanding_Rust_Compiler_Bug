plain
2019-11-27T12:10:01.6064391Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-27T12:10:01.6227981Z ##[command]git config gc.auto 0
2019-11-27T12:10:01.6304367Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-27T12:10:01.6367978Z ##[command]git config --get-all http.proxy
2019-11-27T12:10:01.6533390Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66807/merge:refs/remotes/pull/66807/merge
---
2019-11-27T12:21:45.6626584Z    Compiling syntax_expand v0.0.0 (/checkout/src/libsyntax_expand)
2019-11-27T12:22:53.5432687Z    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-11-27T12:26:47.2271710Z    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-11-27T12:27:28.4973176Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-11-27T12:27:38.5632591Z error[E0271]: type mismatch resolving `<interpret::eval_context::InterpCx<'mir, 'tcx, M> as rustc_target::abi::LayoutOf>::TyLayout == rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>`
2019-11-27T12:27:38.5634267Z   --> src/librustc_mir/interpret/terminator.rs:71:46
2019-11-27T12:27:38.5634867Z    |
2019-11-27T12:27:38.5635538Z 71 |                         (fn_val, caller_abi, FnAbi::of_fn_ptr(self, sig, &[]))
2019-11-27T12:27:38.5636353Z    |                                              ^^^^^^^^^^^^^^^^ expected struct `rustc_target::abi::TyLayout`, found enum `std::result::Result`
2019-11-27T12:27:38.5636909Z    |
2019-11-27T12:27:38.5637585Z    = note: expected type `rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>`
2019-11-27T12:27:38.5638816Z               found type `std::result::Result<rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>, rustc::mir::interpret::InterpErrorInfo<'_>>`
2019-11-27T12:27:38.5639620Z    = note: required by `rustc::ty::layout::FnAbiExt::of_fn_ptr`
2019-11-27T12:27:38.5644489Z 
2019-11-27T12:27:38.5775262Z error[E0277]: the trait bound `interpret::eval_context::InterpCx<'mir, 'tcx, M>: rustc_target::spec::HasTargetSpec` is not satisfied
2019-11-27T12:27:38.5775670Z   --> src/librustc_mir/interpret/terminator.rs:71:46
2019-11-27T12:27:38.5775916Z    |
2019-11-27T12:27:38.5776266Z 71 |                         (fn_val, caller_abi, FnAbi::of_fn_ptr(self, sig, &[]))
2019-11-27T12:27:38.5777015Z    |                                              ^^^^^^^^^^^^^^^^ the trait `rustc_target::spec::HasTargetSpec` is not implemented for `interpret::eval_context::InterpCx<'mir, 'tcx, M>`
2019-11-27T12:27:38.5777342Z    |
2019-11-27T12:27:38.5777685Z    = note: required by `rustc::ty::layout::FnAbiExt::of_fn_ptr`
2019-11-27T12:27:38.5847063Z 
2019-11-27T12:27:38.5849385Z error[E0271]: type mismatch resolving `<interpret::eval_context::InterpCx<'mir, 'tcx, M> as rustc_target::abi::LayoutOf>::TyLayout == rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>`
2019-11-27T12:27:38.5849800Z   --> src/librustc_mir/interpret/terminator.rs:78:38
2019-11-27T12:27:38.5850090Z    |
2019-11-27T12:27:38.5850477Z 78 |                         let fn_abi = FnAbi::of_instance(self, instance, &[]);
2019-11-27T12:27:38.5850965Z    |                                      ^^^^^^^^^^^^^^^^^^ expected struct `rustc_target::abi::TyLayout`, found enum `std::result::Result`
2019-11-27T12:27:38.5851240Z    |
2019-11-27T12:27:38.5851876Z    = note: expected type `rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>`
2019-11-27T12:27:38.5852334Z               found type `std::result::Result<rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>, rustc::mir::interpret::InterpErrorInfo<'_>>`
2019-11-27T12:27:38.5852696Z    = note: required by `rustc::ty::layout::FnAbiExt::of_instance`
2019-11-27T12:27:38.5852761Z 
2019-11-27T12:27:38.5853155Z error[E0277]: the trait bound `interpret::eval_context::InterpCx<'mir, 'tcx, M>: rustc_target::spec::HasTargetSpec` is not satisfied
2019-11-27T12:27:38.5853492Z   --> src/librustc_mir/interpret/terminator.rs:78:38
2019-11-27T12:27:38.5853745Z    |
2019-11-27T12:27:38.5854098Z 78 |                         let fn_abi = FnAbi::of_instance(self, instance, &[]);
2019-11-27T12:27:38.5854619Z    |                                      ^^^^^^^^^^^^^^^^^^ the trait `rustc_target::spec::HasTargetSpec` is not implemented for `interpret::eval_context::InterpCx<'mir, 'tcx, M>`
2019-11-27T12:27:38.5854879Z    |
2019-11-27T12:27:38.5855226Z    = note: required by `rustc::ty::layout::FnAbiExt::of_instance`
2019-11-27T12:27:38.5855269Z 
2019-11-27T12:27:38.6310092Z error[E0271]: type mismatch resolving `<interpret::eval_context::InterpCx<'mir, 'tcx, M> as rustc_target::abi::LayoutOf>::TyLayout == rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>`
2019-11-27T12:27:38.6310562Z    --> src/librustc_mir/interpret/terminator.rs:268:33
2019-11-27T12:27:38.6310854Z     |
2019-11-27T12:27:38.6311219Z 268 |             let callee_fn_abi = FnAbi::of_instance(self, instance, &[]);
2019-11-27T12:27:38.6311698Z     |                                 ^^^^^^^^^^^^^^^^^^ expected struct `rustc_target::abi::TyLayout`, found enum `std::result::Result`
2019-11-27T12:27:38.6311972Z     |
2019-11-27T12:27:38.6312577Z     = note: expected type `rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>`
2019-11-27T12:27:38.6313075Z                found type `std::result::Result<rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>, rustc::mir::interpret::InterpErrorInfo<'_>>`
2019-11-27T12:27:38.6313435Z     = note: required by `rustc::ty::layout::FnAbiExt::of_instance`
2019-11-27T12:27:38.6317623Z 
2019-11-27T12:27:38.6367399Z error[E0277]: the trait bound `interpret::eval_context::InterpCx<'mir, 'tcx, M>: rustc_target::spec::HasTargetSpec` is not satisfied
2019-11-27T12:27:38.6367766Z    --> src/librustc_mir/interpret/terminator.rs:268:33
2019-11-27T12:27:38.6368337Z     |
2019-11-27T12:27:38.6368726Z 268 |             let callee_fn_abi = FnAbi::of_instance(self, instance, &[]);
2019-11-27T12:27:38.6369244Z     |                                 ^^^^^^^^^^^^^^^^^^ the trait `rustc_target::spec::HasTargetSpec` is not implemented for `interpret::eval_context::InterpCx<'mir, 'tcx, M>`
2019-11-27T12:27:38.6369511Z     |
2019-11-27T12:27:38.6370060Z     = note: required by `rustc::ty::layout::FnAbiExt::of_instance`
2019-11-27T12:27:38.6373999Z 
2019-11-27T12:27:38.7279268Z error[E0271]: type mismatch resolving `<interpret::eval_context::InterpCx<'mir, 'tcx, M> as rustc_target::abi::LayoutOf>::TyLayout == rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>`
2019-11-27T12:27:38.7279738Z    --> src/librustc_mir/interpret/terminator.rs:483:22
2019-11-27T12:27:38.7280046Z     |
2019-11-27T12:27:38.7295462Z 483 |         let fn_abi = FnAbi::of_instance(self, instance, &[]);
2019-11-27T12:27:38.7295979Z     |                      ^^^^^^^^^^^^^^^^^^ expected struct `rustc_target::abi::TyLayout`, found enum `std::result::Result`
2019-11-27T12:27:38.7296308Z     |
2019-11-27T12:27:38.7296684Z     = note: expected type `rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>`
2019-11-27T12:27:38.7297139Z                found type `std::result::Result<rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>, rustc::mir::interpret::InterpErrorInfo<'_>>`
2019-11-27T12:27:38.7297495Z     = note: required by `rustc::ty::layout::FnAbiExt::of_instance`
2019-11-27T12:27:38.7297603Z 
2019-11-27T12:27:38.7410532Z error[E0277]: the trait bound `interpret::eval_context::InterpCx<'mir, 'tcx, M>: rustc_target::spec::HasTargetSpec` is not satisfied
2019-11-27T12:27:38.7410962Z    --> src/librustc_mir/interpret/terminator.rs:483:22
2019-11-27T12:27:38.7411235Z     |
2019-11-27T12:27:38.7411663Z 483 |         let fn_abi = FnAbi::of_instance(self, instance, &[]);
2019-11-27T12:27:38.7412143Z     |                      ^^^^^^^^^^^^^^^^^^ the trait `rustc_target::spec::HasTargetSpec` is not implemented for `interpret::eval_context::InterpCx<'mir, 'tcx, M>`
2019-11-27T12:27:38.7412439Z     |
2019-11-27T12:27:38.7413007Z     = note: required by `rustc::ty::layout::FnAbiExt::of_instance`
2019-11-27T12:27:39.9758984Z error: aborting due to 8 previous errors
2019-11-27T12:27:39.9763207Z 
2019-11-27T12:27:39.9774910Z Some errors have detailed explanations: E0271, E0277.
2019-11-27T12:27:39.9785581Z For more information about an error, try `rustc --explain E0271`.
---
2019-11-27T12:29:48.8671152Z   local time: Wed Nov 27 12:29:48 UTC 2019
2019-11-27T12:29:49.1493446Z   network time: Wed, 27 Nov 2019 12:29:49 GMT
2019-11-27T12:29:49.1494491Z == end clock drift check ==
2019-11-27T12:29:52.0065713Z 
2019-11-27T12:29:52.0177927Z ##[error]Bash exited with code '1'.
2019-11-27T12:29:52.0212993Z ##[section]Starting: Checkout
2019-11-27T12:29:52.0215681Z ==============================================================================
2019-11-27T12:29:52.0215757Z Task         : Get sources
2019-11-27T12:29:52.0215805Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
