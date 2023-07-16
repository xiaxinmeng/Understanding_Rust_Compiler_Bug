plain
travis_time:end:0645fb2c:start=1544225124740862255,finish=1544225127469504875,duration=2728642620
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:05:17]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:35]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:12:09]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:12:09]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:12:09] error: missing `fn`, `type`, or `const` for impl-item declaration
[00:12:09]    --> src/librustc_typeck/check/method/suggest.rs:733:10
[00:12:09] 733 |           }
[00:12:09]     |  __________^
[00:12:09]     |  __________^
[00:12:09] 734 | |         tcx.hir().krate().visit_all_item_likes(&mut Visitor {
[00:12:09]     | |________^ missing `fn`, `type`, or `const`
[00:12:09] 
[00:12:09] error: expected one of `async`, `const`, `crate`, `default`, `existential`, `extern`, `fn`, `pub`, `type`, `unsafe`, or `}`, found `)`
[00:12:09]    --> src/librustc_typeck/check/method/suggest.rs:737:10
[00:12:09] 737 |         });
[00:12:09]     |          ^ expected one of 11 possible tokens here
[00:12:09] 
[00:12:09] 
[00:12:09] error: expected one of `async`, `const`, `crate`, `default`, `existential`, `extern`, `fn`, `pub`, `type`, `unsafe`, or `}`, found `let`
[00:12:09]    --> src/librustc_typeck/check/method/suggest.rs:740:9
[00:12:09] 737 |         });
[00:12:09] 737 |         });
[00:12:09]     |            - expected one of 11 possible tokens here
[00:12:09] ...
[00:12:09] 740 |         let mut external_mods = FxHashSet::default();
[00:12:09]     |         ^^^ unexpected token
[00:12:09] 
[00:12:09] error: expected pattern, found keyword `for`
[00:12:09]    --> src/librustc_typeck/check/method/suggest.rs:750:17
[00:12:09]     |
[00:12:09] 750 |                 for child in tcx.item_children(def_id).iter() {
[00:12:09] 
[00:12:09] error: expected expression, found reserved identifier `_`
[00:12:09]    --> src/librustc_typeck/check/method/suggest.rs:754:13
[00:12:09]     |
[00:12:09]     |
[00:12:09] 754 |             _ => {}
[00:12:09]     |             ^ expected expression
[00:12:09] 
[00:12:10] error[E0407]: method `handle_external_def` is not a member of trait `itemlikevisit::ItemLikeVisitor`
[00:12:10]    --> src/librustc_typeck/check/method/suggest.rs:741:9
[00:12:10] 741 | /         fn handle_external_def(tcx: TyCtxt,
[00:12:10] 742 | |                                traits: &mut Vec<DefId>,
[00:12:10] 742 | |                                traits: &mut Vec<DefId>,
[00:12:10] 743 | |                                external_mods: &mut FxHashSet<DefId>,
[00:12:10] 744 | |                                def: Def) {
[00:12:10] 754 | |             _ => {}
[00:12:10] 755 | |         }
[00:12:10] 755 | |         }
[00:12:10]     | |_________^ not a member of trait `itemlikevisit::ItemLikeVisitor`
[00:12:10] 
[00:12:10] error[E0425]: cannot find function `handle_external_def` in this scope
[00:12:10]    --> src/librustc_typeck/check/method/suggest.rs:762:9
[00:12:10]     |
[00:12:10] 762 |         handle_external_def(tcx, &mut traits, &mut external_mods, Def::Mod(def_id));
[00:12:10] 
[00:12:10] error[E0425]: cannot find value `external_mods` in this scope
[00:12:10]    --> src/librustc_typeck/check/method/suggest.rs:762:52
[00:12:10]     |
[00:12:10]     |
[00:12:10] 762 |         handle_external_def(tcx, &mut traits, &mut external_mods, Def::Mod(def_id));
[00:12:10] 
[00:12:10] 
[00:12:11] error[E0046]: not all trait items implemented, missing: `visit_trait_item`, `visit_impl_item`
[00:12:11]    --> src/librustc_typeck/check/method/suggest.rs:727:5
[00:12:11]     |
[00:12:11] 727 |     impl<'v, 'a, 'tcx> itemlikevisit::ItemLikeVisitor<'v> for Visitor<'a, 'tcx> {
[00:12:11]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `visit_trait_item`, `visit_impl_item` in implementation
[00:12:11]     |
[00:12:11]     = note: `visit_trait_item` from trait: `fn(&mut Self, &'hir rustc::hir::TraitItem)`
[00:12:11]     = note: `visit_impl_item` from trait: `fn(&mut Self, &'hir rustc::hir::ImplItem)`
[00:12:11] error: aborting due to 9 previous errors
[00:12:11] 
[00:12:11] Some errors occurred: E0046, E0407, E0425.
[00:12:11] For more information about an error, try `rustc --explain E0046`.
[00:12:11] For more information about an error, try `rustc --explain E0046`.
[00:12:11] error: Could not compile `rustc_typeck`.
[00:12:11] warning: build failed, waiting for other jobs to finish...
[00:15:06] error: build failed
[00:15:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:15:06] expected success, got: exit code: 101
[00:15:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:15:06] Build completed unsuccessfully in 0:11:49
[00:15:06] Makefile:28: recipe for target 'all' failed
[00:15:06] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0451cf4e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Dec  7 23:40:43 UTC 2018
