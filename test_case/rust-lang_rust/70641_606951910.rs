plain
2020-03-31T23:54:08.8232096Z ========================== Starting Command Output ===========================
2020-03-31T23:54:08.8236614Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0f7122ca-ac98-40f6-8bde-df02709a84b3.sh
2020-03-31T23:54:08.8237137Z 
2020-03-31T23:54:08.8243270Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-31T23:54:08.8272494Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70641/merge to s
2020-03-31T23:54:08.8275784Z Task         : Get sources
2020-03-31T23:54:08.8276156Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T23:54:08.8276460Z Version      : 1.0.0
2020-03-31T23:54:08.8276663Z Author       : Microsoft
---
2020-03-31T23:54:11.6625394Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-31T23:54:11.6633050Z ##[command]git config gc.auto 0
2020-03-31T23:54:11.6641210Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-31T23:54:11.6658015Z ##[command]git config --get-all http.proxy
2020-03-31T23:54:11.6665478Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70641/merge:refs/remotes/pull/70641/merge
---
2020-03-31T23:57:52.0789194Z  ---> 3fc1b512c57b
2020-03-31T23:57:52.0789605Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-03-31T23:57:52.0793548Z  ---> Using cache
2020-03-31T23:57:52.0794228Z  ---> 5ee4295733f4
2020-03-31T23:57:52.0795761Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-03-31T23:57:52.0801991Z  ---> 3d07a0fa42fe
2020-03-31T23:57:52.0834436Z Successfully built 3d07a0fa42fe
2020-03-31T23:57:52.0889205Z Successfully tagged rust-ci:latest
2020-03-31T23:57:52.1282245Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
---
2020-04-01T00:01:21.3749346Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-01T00:01:21.4415236Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-01T00:01:21.6247660Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-01T00:01:21.7530128Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-01T00:01:22.1925499Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-01T00:01:24.3718717Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-01T00:01:24.8106854Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-01T00:01:26.6997650Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-01T00:01:27.1198152Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-01T00:01:44.4072262Z 
2020-04-01T00:01:44.4251450Z error[E0433]: failed to resolve: use of undeclared type or module `rustc`
2020-04-01T00:01:44.4252200Z  --> src/librustc_infer/traits/util.rs:4:5
2020-04-01T00:01:44.4252737Z   |
2020-04-01T00:01:44.4253460Z 4 | use rustc::ty::{self, ToPolyTraitRef, ToPredicate, TyCtxt, WithConstness};
2020-04-01T00:01:44.4254735Z 
2020-04-01T00:01:44.4261056Z error[E0432]: unresolved import `rustc`
2020-04-01T00:01:44.4261724Z  --> src/librustc_infer/traits/util.rs:4:5
2020-04-01T00:01:44.4262215Z   |
2020-04-01T00:01:44.4262215Z   |
2020-04-01T00:01:44.4262888Z 4 | use rustc::ty::{self, ToPolyTraitRef, ToPredicate, TyCtxt, WithConstness};
2020-04-01T00:01:44.4264068Z 
2020-04-01T00:01:44.4264068Z 
2020-04-01T00:01:45.7958649Z error[E0599]: no method named `without_const` found for struct `rustc_middle::ty::Binder<rustc_middle::ty::TraitRef<'tcx>>` in the current scope
2020-04-01T00:01:45.7960031Z     |
2020-04-01T00:01:45.7960031Z     |
2020-04-01T00:01:45.7960704Z 100 |     elaborate_predicates(tcx, vec![trait_ref.without_const().to_predicate()])
2020-04-01T00:01:45.7961857Z     |                                              ^^^^^^^^^^^^^ method not found in `rustc_middle::ty::Binder<rustc_middle::ty::TraitRef<'tcx>>`
2020-04-01T00:01:45.7965572Z     = help: items from traits can only be used if the trait is in scope
2020-04-01T00:01:45.7967969Z     = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-04-01T00:01:45.7967969Z     = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-04-01T00:01:45.7969346Z             `use crate::rustc_middle::ty::WithConstness;`
2020-04-01T00:01:45.7969810Z 
2020-04-01T00:01:45.7990355Z error[E0599]: no method named `without_const` found for struct `rustc_middle::ty::Binder<rustc_middle::ty::TraitRef<'tcx>>` in the current scope
2020-04-01T00:01:45.7993396Z     |
2020-04-01T00:01:45.7993396Z     |
2020-04-01T00:01:45.7994751Z 107 |     let predicates = trait_refs.map(|trait_ref| trait_ref.without_const().to_predicate()).collect();
2020-04-01T00:01:45.7997086Z     |                                                           ^^^^^^^^^^^^^ method not found in `rustc_middle::ty::Binder<rustc_middle::ty::TraitRef<'tcx>>`
2020-04-01T00:01:45.7999846Z     = help: items from traits can only be used if the trait is in scope
2020-04-01T00:01:45.8001508Z     = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-04-01T00:01:45.8001508Z     = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-04-01T00:01:45.8002890Z             `use crate::rustc_middle::ty::WithConstness;`
2020-04-01T00:01:45.8187689Z error: aborting due to 5 previous errors
2020-04-01T00:01:45.8187975Z 
2020-04-01T00:01:45.8188683Z Some errors have detailed explanations: E0432, E0433, E0599.
2020-04-01T00:01:45.8191595Z For more information about an error, try `rustc --explain E0432`.
2020-04-01T00:01:45.8191595Z For more information about an error, try `rustc --explain E0432`.
2020-04-01T00:01:45.8269934Z error: could not compile `rustc_infer`.
2020-04-01T00:01:45.8270194Z 
2020-04-01T00:01:45.8270611Z To learn more, run the command again with --verbose.
2020-04-01T00:01:45.8288451Z warning: build failed, waiting for other jobs to finish...
2020-04-01T00:01:46.6867292Z error: build failed
2020-04-01T00:01:46.6922311Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-01T00:01:46.6954374Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-01T00:01:46.6955004Z Build completed unsuccessfully in 0:03:47
2020-04-01T00:01:46.6985278Z == clock drift check ==
2020-04-01T00:01:46.7004452Z   local time: Wed Apr  1 00:01:46 UTC 2020
2020-04-01T00:01:46.7004452Z   local time: Wed Apr  1 00:01:46 UTC 2020
2020-04-01T00:01:46.8560214Z   network time: Wed, 01 Apr 2020 00:01:46 GMT
2020-04-01T00:01:47.9232119Z 
2020-04-01T00:01:47.9232119Z 
2020-04-01T00:01:47.9295384Z ##[error]Bash exited with code '1'.
2020-04-01T00:01:47.9307957Z ##[section]Finishing: Run build
2020-04-01T00:01:47.9367630Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70641/merge to s
2020-04-01T00:01:47.9372144Z Task         : Get sources
2020-04-01T00:01:47.9372479Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-01T00:01:47.9372786Z Version      : 1.0.0
2020-04-01T00:01:47.9373000Z Author       : Microsoft
2020-04-01T00:01:47.9373000Z Author       : Microsoft
2020-04-01T00:01:47.9373330Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-01T00:01:47.9373728Z ==============================================================================
2020-04-01T00:01:48.2443814Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-01T00:01:48.2506813Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70641/merge to s
2020-04-01T00:01:48.2584316Z Cleaning up task key
2020-04-01T00:01:48.2585555Z Start cleaning up orphan processes.
2020-04-01T00:01:48.2778708Z Terminate orphan process: pid (3866) (python)
2020-04-01T00:01:48.2898525Z ##[section]Finishing: Finalize Job
