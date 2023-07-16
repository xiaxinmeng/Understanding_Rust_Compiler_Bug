plain
2019-11-27T11:40:40.7363390Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-27T11:40:40.7416272Z ##[command]git config gc.auto 0
2019-11-27T11:40:40.7419936Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-27T11:40:40.7423840Z ##[command]git config --get-all http.proxy
2019-11-27T11:40:40.7427960Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66807/merge:refs/remotes/pull/66807/merge
---
2019-11-27T11:51:20.2832742Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2019-11-27T11:52:10.4667714Z    Compiling syntax_expand v0.0.0 (/checkout/src/libsyntax_expand)
2019-11-27T11:53:18.9252053Z    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-11-27T11:57:10.9657418Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-11-27T11:57:20.3542602Z error[E0271]: type mismatch resolving `<interpret::eval_context::InterpCx<'mir, 'tcx, M> as rustc_target::abi::LayoutOf>::TyLayout == rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>`
2019-11-27T11:57:20.3544239Z   --> src/librustc_mir/interpret/terminator.rs:71:46
2019-11-27T11:57:20.3545118Z    |
2019-11-27T11:57:20.3550815Z 71 |                         (fn_val, caller_abi, FnAbi::of_fn_ptr(self, sig, &[]))
2019-11-27T11:57:20.3551777Z    |                                              ^^^^^^^^^^^^^^^^ expected struct `rustc_target::abi::TyLayout`, found enum `std::result::Result`
2019-11-27T11:57:20.3555820Z    |
2019-11-27T11:57:20.3557007Z    = note: expected type `rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>`
2019-11-27T11:57:20.3557760Z               found type `std::result::Result<rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>, rustc::mir::interpret::InterpErrorInfo<'_>>`
2019-11-27T11:57:20.3558438Z    = note: required by `rustc::ty::layout::FnAbiExt::of_fn_ptr`
2019-11-27T11:57:20.3561697Z 
2019-11-27T11:57:20.3610246Z error[E0277]: the trait bound `interpret::eval_context::InterpCx<'mir, 'tcx, M>: rustc_target::spec::HasTargetSpec` is not satisfied
2019-11-27T11:57:20.3610982Z   --> src/librustc_mir/interpret/terminator.rs:71:46
2019-11-27T11:57:20.3611584Z    |
2019-11-27T11:57:20.3612254Z 71 |                         (fn_val, caller_abi, FnAbi::of_fn_ptr(self, sig, &[]))
2019-11-27T11:57:20.3613279Z    |                                              ^^^^^^^^^^^^^^^^ the trait `rustc_target::spec::HasTargetSpec` is not implemented for `interpret::eval_context::InterpCx<'mir, 'tcx, M>`
2019-11-27T11:57:20.3613886Z    |
2019-11-27T11:57:20.3614465Z    = note: required by `rustc::ty::layout::FnAbiExt::of_fn_ptr`
2019-11-27T11:57:20.3618215Z 
2019-11-27T11:57:20.3732792Z error[E0271]: type mismatch resolving `<interpret::eval_context::InterpCx<'mir, 'tcx, M> as rustc_target::abi::LayoutOf>::TyLayout == rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>`
2019-11-27T11:57:20.3734701Z   --> src/librustc_mir/interpret/terminator.rs:78:38
2019-11-27T11:57:20.3736792Z    |
2019-11-27T11:57:20.3738576Z 78 |                         let fn_abi = FnAbi::of_instance(self, instance, &[]);
2019-11-27T11:57:20.3740856Z    |                                      ^^^^^^^^^^^^^^^^^^ expected struct `rustc_target::abi::TyLayout`, found enum `std::result::Result`
2019-11-27T11:57:20.3741194Z    |
2019-11-27T11:57:20.3741514Z    = note: expected type `rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>`
2019-11-27T11:57:20.3741875Z               found type `std::result::Result<rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>, rustc::mir::interpret::InterpErrorInfo<'_>>`
2019-11-27T11:57:20.3742301Z    = note: required by `rustc::ty::layout::FnAbiExt::of_instance`
2019-11-27T11:57:20.3747882Z 
2019-11-27T11:57:20.3754888Z error[E0277]: the trait bound `interpret::eval_context::InterpCx<'mir, 'tcx, M>: rustc_target::spec::HasTargetSpec` is not satisfied
2019-11-27T11:57:20.3755220Z   --> src/librustc_mir/interpret/terminator.rs:78:38
2019-11-27T11:57:20.3755492Z    |
2019-11-27T11:57:20.3755826Z 78 |                         let fn_abi = FnAbi::of_instance(self, instance, &[]);
2019-11-27T11:57:20.3756721Z    |                                      ^^^^^^^^^^^^^^^^^^ the trait `rustc_target::spec::HasTargetSpec` is not implemented for `interpret::eval_context::InterpCx<'mir, 'tcx, M>`
2019-11-27T11:57:20.3757185Z    |
2019-11-27T11:57:20.3757501Z    = note: required by `rustc::ty::layout::FnAbiExt::of_instance`
2019-11-27T11:57:20.3762125Z 
2019-11-27T11:57:20.4363286Z error[E0271]: type mismatch resolving `<interpret::eval_context::InterpCx<'mir, 'tcx, M> as rustc_target::abi::LayoutOf>::TyLayout == rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>`
2019-11-27T11:57:20.4363713Z    --> src/librustc_mir/interpret/terminator.rs:268:33
2019-11-27T11:57:20.4364035Z     |
2019-11-27T11:57:20.4364409Z 268 |             let callee_fn_abi = FnAbi::of_instance(self, instance, &[]);
2019-11-27T11:57:20.4364885Z     |                                 ^^^^^^^^^^^^^^^^^^ expected struct `rustc_target::abi::TyLayout`, found enum `std::result::Result`
2019-11-27T11:57:20.4365216Z     |
2019-11-27T11:57:20.4365606Z     = note: expected type `rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>`
2019-11-27T11:57:20.4366430Z                found type `std::result::Result<rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>, rustc::mir::interpret::InterpErrorInfo<'_>>`
2019-11-27T11:57:20.4366810Z     = note: required by `rustc::ty::layout::FnAbiExt::of_instance`
2019-11-27T11:57:20.4373082Z 
2019-11-27T11:57:20.4418874Z error[E0277]: the trait bound `interpret::eval_context::InterpCx<'mir, 'tcx, M>: rustc_target::spec::HasTargetSpec` is not satisfied
2019-11-27T11:57:20.4419259Z    --> src/librustc_mir/interpret/terminator.rs:268:33
2019-11-27T11:57:20.4419551Z     |
2019-11-27T11:57:20.4419958Z 268 |             let callee_fn_abi = FnAbi::of_instance(self, instance, &[]);
2019-11-27T11:57:20.4420488Z     |                                 ^^^^^^^^^^^^^^^^^^ the trait `rustc_target::spec::HasTargetSpec` is not implemented for `interpret::eval_context::InterpCx<'mir, 'tcx, M>`
2019-11-27T11:57:20.4420782Z     |
2019-11-27T11:57:20.4421125Z     = note: required by `rustc::ty::layout::FnAbiExt::of_instance`
2019-11-27T11:57:20.4421170Z 
2019-11-27T11:57:20.5180988Z error[E0271]: type mismatch resolving `<interpret::eval_context::InterpCx<'mir, 'tcx, M> as rustc_target::abi::LayoutOf>::TyLayout == rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>`
2019-11-27T11:57:20.5181613Z    --> src/librustc_mir/interpret/terminator.rs:483:22
2019-11-27T11:57:20.5181981Z     |
2019-11-27T11:57:20.5182348Z 483 |         let fn_abi = FnAbi::of_instance(self, instance, &[]);
2019-11-27T11:57:20.5182814Z     |                      ^^^^^^^^^^^^^^^^^^ expected struct `rustc_target::abi::TyLayout`, found enum `std::result::Result`
2019-11-27T11:57:20.5183115Z     |
2019-11-27T11:57:20.5183506Z     = note: expected type `rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>`
2019-11-27T11:57:20.5183979Z                found type `std::result::Result<rustc_target::abi::TyLayout<'_, &rustc::ty::TyS<'_>>, rustc::mir::interpret::InterpErrorInfo<'_>>`
2019-11-27T11:57:20.5184496Z     = note: required by `rustc::ty::layout::FnAbiExt::of_instance`
2019-11-27T11:57:20.5199990Z 
2019-11-27T11:57:20.5300130Z error[E0277]: the trait bound `interpret::eval_context::InterpCx<'mir, 'tcx, M>: rustc_target::spec::HasTargetSpec` is not satisfied
2019-11-27T11:57:20.5300517Z    --> src/librustc_mir/interpret/terminator.rs:483:22
2019-11-27T11:57:20.5300786Z     |
2019-11-27T11:57:20.5301167Z 483 |         let fn_abi = FnAbi::of_instance(self, instance, &[]);
2019-11-27T11:57:20.5301675Z     |                      ^^^^^^^^^^^^^^^^^^ the trait `rustc_target::spec::HasTargetSpec` is not implemented for `interpret::eval_context::InterpCx<'mir, 'tcx, M>`
2019-11-27T11:57:20.5301996Z     |
2019-11-27T11:57:20.5302353Z     = note: required by `rustc::ty::layout::FnAbiExt::of_instance`
2019-11-27T11:57:21.6766087Z error: aborting due to 8 previous errors
2019-11-27T11:57:21.6771108Z 
2019-11-27T11:57:21.6781589Z Some errors have detailed explanations: E0271, E0277.
2019-11-27T11:57:21.6792420Z For more information about an error, try `rustc --explain E0271`.
---
2019-11-27T11:59:46.5580380Z   local time: Wed Nov 27 11:59:46 UTC 2019
2019-11-27T11:59:46.7126253Z   network time: Wed, 27 Nov 2019 11:59:46 GMT
2019-11-27T11:59:46.7128480Z == end clock drift check ==
2019-11-27T11:59:49.6933836Z 
2019-11-27T11:59:49.7011119Z ##[error]Bash exited with code '1'.
2019-11-27T11:59:49.7061138Z ##[section]Starting: Checkout
2019-11-27T11:59:49.7062980Z ==============================================================================
2019-11-27T11:59:49.7063041Z Task         : Get sources
2019-11-27T11:59:49.7063094Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
