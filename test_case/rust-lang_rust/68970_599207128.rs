plain
2020-03-15T12:51:20.1305064Z ========================== Starting Command Output ===========================
2020-03-15T12:51:20.1310305Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d254c262-04cf-4cec-859e-3f8b708e054e.sh
2020-03-15T12:51:20.1310849Z 
2020-03-15T12:51:20.1315598Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-15T12:51:20.1336454Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68970/merge to s
2020-03-15T12:51:20.1339963Z Task         : Get sources
2020-03-15T12:51:20.1340290Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-15T12:51:20.1340623Z Version      : 1.0.0
2020-03-15T12:51:20.1340839Z Author       : Microsoft
---
2020-03-15T12:51:21.1215672Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-15T12:51:21.1222546Z ##[command]git config gc.auto 0
2020-03-15T12:51:21.1226606Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-15T12:51:21.1230271Z ##[command]git config --get-all http.proxy
2020-03-15T12:51:21.1237028Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68970/merge:refs/remotes/pull/68970/merge
---
2020-03-15T12:59:11.3696947Z     Checking rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-15T12:59:14.6927887Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-03-15T12:59:14.9017832Z     Checking rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
2020-03-15T12:59:15.4453051Z     Checking rustc_trait_selection v0.0.0 (/checkout/src/librustc_trait_selection)
2020-03-15T12:59:15.5082581Z error: expected `;`, found ``map_err``
2020-03-15T12:59:15.5083988Z      |
2020-03-15T12:59:15.5083988Z      |
2020-03-15T12:59:15.5084766Z 1014 |                     assoc_ty_def(selcx, impl_data.impl_def_id, obligation.predicate.item_def_id)
2020-03-15T12:59:15.5085965Z      |                                                                                                 ^ help: add `;` here
2020-03-15T12:59:15.5087199Z 1015 |                     map_err(|ErrorReported| ())?;
2020-03-15T12:59:15.5088104Z      |                     ------- unexpected token
2020-03-15T12:59:16.7471972Z error[E0425]: cannot find function `map_err` in this scope
2020-03-15T12:59:16.7478661Z     --> src/librustc_trait_selection/traits/project.rs:1015:21
2020-03-15T12:59:16.7479553Z      |
2020-03-15T12:59:16.7479553Z      |
2020-03-15T12:59:16.7480414Z 1015 |                     map_err(|ErrorReported| ())?;
2020-03-15T12:59:16.7482120Z 
2020-03-15T12:59:16.7482120Z 
2020-03-15T12:59:16.9119640Z error[E0609]: no field `node` on type `std::result::Result<rustc::traits::specialization_graph::NodeItem<rustc::ty::AssocItem>, rustc_errors::ErrorReported>`
2020-03-15T12:59:16.9121314Z      |
2020-03-15T12:59:16.9122026Z 1017 |                 let is_default = if node_item.node.is_from_trait() {
2020-03-15T12:59:16.9122892Z      |                                               ^^^^
2020-03-15T12:59:16.9127064Z 
2020-03-15T12:59:16.9127064Z 
2020-03-15T12:59:16.9160649Z error[E0609]: no field `item` on type `std::result::Result<rustc::traits::specialization_graph::NodeItem<rustc::ty::AssocItem>, rustc_errors::ErrorReported>`
2020-03-15T12:59:16.9162412Z      |
2020-03-15T12:59:16.9163087Z 1039 |                     node_item.item.defaultness.is_default()
2020-03-15T12:59:16.9163818Z      |                               ^^^^
2020-03-15T12:59:16.9167677Z 
2020-03-15T12:59:16.9167677Z 
2020-03-15T12:59:16.9199879Z error[E0609]: no field `node` on type `std::result::Result<rustc::traits::specialization_graph::NodeItem<rustc::ty::AssocItem>, rustc_errors::ErrorReported>`
2020-03-15T12:59:16.9201485Z      |
2020-03-15T12:59:16.9201485Z      |
2020-03-15T12:59:16.9202282Z 1040 |                         || super::util::impl_is_default(selcx.tcx(), node_item.node.def_id())
2020-03-15T12:59:16.9207234Z 
2020-03-15T12:59:16.9207234Z 
2020-03-15T12:59:16.9249263Z error[E0609]: no field `item` on type `std::result::Result<rustc::traits::specialization_graph::NodeItem<rustc::ty::AssocItem>, rustc_errors::ErrorReported>`
2020-03-15T12:59:16.9251017Z      |
2020-03-15T12:59:16.9251017Z      |
2020-03-15T12:59:16.9251703Z 1063 | ...                   selcx.tcx().def_path_str(node_item.item.def_id),
2020-03-15T12:59:16.9257216Z 
2020-03-15T12:59:17.4999867Z error: aborting due to 6 previous errors
2020-03-15T12:59:17.5004122Z 
2020-03-15T12:59:17.5013981Z Some errors have detailed explanations: E0425, E0609.
2020-03-15T12:59:17.5013981Z Some errors have detailed explanations: E0425, E0609.
2020-03-15T12:59:17.5024146Z For more information about an error, try `rustc --explain E0425`.
2020-03-15T12:59:17.5097433Z error: could not compile `rustc_trait_selection`.
2020-03-15T12:59:17.5118237Z warning: build failed, waiting for other jobs to finish...
2020-03-15T12:59:20.2139596Z error: build failed
2020-03-15T12:59:20.2162952Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-15T12:59:20.2178552Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-03-15T12:59:20.2179261Z Build completed unsuccessfully in 0:04:40
2020-03-15T12:59:20.2230639Z == clock drift check ==
2020-03-15T12:59:20.2246681Z   local time: Sun Mar 15 12:59:20 UTC 2020
2020-03-15T12:59:20.2246681Z   local time: Sun Mar 15 12:59:20 UTC 2020
2020-03-15T12:59:20.3862031Z   network time: Sun, 15 Mar 2020 12:59:20 GMT
2020-03-15T12:59:20.3863843Z == end clock drift check ==
2020-03-15T12:59:20.9782781Z 
2020-03-15T12:59:20.9862198Z ##[error]Bash exited with code '1'.
2020-03-15T12:59:20.9887197Z ##[section]Finishing: Run build
2020-03-15T12:59:20.9939464Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68970/merge to s
2020-03-15T12:59:20.9944654Z Task         : Get sources
2020-03-15T12:59:20.9945028Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-15T12:59:20.9945353Z Version      : 1.0.0
2020-03-15T12:59:20.9945788Z Author       : Microsoft
2020-03-15T12:59:20.9945788Z Author       : Microsoft
2020-03-15T12:59:20.9946199Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-15T12:59:20.9946613Z ==============================================================================
2020-03-15T12:59:21.3542718Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-15T12:59:21.3585475Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68970/merge to s
2020-03-15T12:59:21.3676665Z Cleaning up task key
2020-03-15T12:59:21.3677989Z Start cleaning up orphan processes.
2020-03-15T12:59:21.3868892Z Terminate orphan process: pid (3645) (python)
2020-03-15T12:59:21.4058787Z ##[section]Finishing: Finalize Job
