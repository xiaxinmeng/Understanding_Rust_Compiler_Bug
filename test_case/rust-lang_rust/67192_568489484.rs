plain
2019-12-23T14:02:35.2329348Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-23T14:02:35.2561564Z ##[command]git config gc.auto 0
2019-12-23T14:02:35.2678163Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-23T14:02:35.2744119Z ##[command]git config --get-all http.proxy
2019-12-23T14:02:35.2899866Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67192/merge:refs/remotes/pull/67192/merge
---
2019-12-23T14:11:19.3778931Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-12-23T14:11:25.1511299Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-12-23T14:11:26.1812266Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-12-23T14:11:27.2307591Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-12-23T14:11:28.4965629Z error[E0422]: cannot find struct, variant or union type `GlobalId` in this scope
2019-12-23T14:11:28.4966069Z    --> src/librustc_mir/interpret/intrinsics.rs:121:27
2019-12-23T14:11:28.4966391Z     |
2019-12-23T14:11:28.4967066Z 121 |                 let gid = GlobalId { instance, promoted: None };
2019-12-23T14:11:28.4967790Z     |
2019-12-23T14:11:28.4968280Z help: possible candidate is found in another module, you can import it into scope
2019-12-23T14:11:28.4968549Z     |
2019-12-23T14:11:28.4968874Z 5   | use rustc::mir::interpret::GlobalId;
2019-12-23T14:11:28.4968874Z 5   | use rustc::mir::interpret::GlobalId;
2019-12-23T14:11:28.4969108Z     |
2019-12-23T14:11:28.4969145Z 
2019-12-23T14:11:33.6602858Z error[E0599]: no method named `const_eval` found for type `rustc::ty::query::TyCtxtAt<'tcx>` in the current scope
2019-12-23T14:11:33.6603379Z    --> src/librustc_mir/interpret/eval_context.rs:771:28
2019-12-23T14:11:33.6604038Z 771 |         let val = self.tcx.const_eval(param_env.and(gid))?;
2019-12-23T14:11:33.6604038Z 771 |         let val = self.tcx.const_eval(param_env.and(gid))?;
2019-12-23T14:11:33.6604434Z     |                            ^^^^^^^^^^ method not found in `rustc::ty::query::TyCtxtAt<'tcx>`
2019-12-23T14:11:39.0852105Z error: aborting due to 2 previous errors
2019-12-23T14:11:39.0852241Z 
2019-12-23T14:11:39.0852557Z Some errors have detailed explanations: E0422, E0599.
2019-12-23T14:11:39.0852906Z For more information about an error, try `rustc --explain E0422`.
2019-12-23T14:11:39.0852906Z For more information about an error, try `rustc --explain E0422`.
2019-12-23T14:11:39.0951922Z error: could not compile `rustc_mir`.
2019-12-23T14:11:39.0955040Z warning: build failed, waiting for other jobs to finish...
2019-12-23T14:11:42.0383846Z error: build failed
2019-12-23T14:11:42.0411807Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-23T14:11:42.0430929Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-23T14:11:42.0431300Z Build completed unsuccessfully in 0:05:40
2019-12-23T14:11:42.0484356Z == clock drift check ==
2019-12-23T14:11:42.0503901Z   local time: Mon Dec 23 14:11:42 UTC 2019
2019-12-23T14:11:42.0503901Z   local time: Mon Dec 23 14:11:42 UTC 2019
2019-12-23T14:11:42.3278513Z   network time: Mon, 23 Dec 2019 14:11:42 GMT
2019-12-23T14:11:42.3279625Z == end clock drift check ==
2019-12-23T14:11:43.5025283Z 
2019-12-23T14:11:43.5142279Z ##[error]Bash exited with code '1'.
2019-12-23T14:11:43.5180229Z ##[section]Starting: Checkout
2019-12-23T14:11:43.5182535Z ==============================================================================
2019-12-23T14:11:43.5182619Z Task         : Get sources
2019-12-23T14:11:43.5182691Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
