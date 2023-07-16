plain
2020-02-09T02:23:01.6105934Z ========================== Starting Command Output ===========================
2020-02-09T02:23:01.6107466Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6638de6c-46d5-4e11-9183-c40a69ac503a.sh
2020-02-09T02:23:01.6107506Z 
2020-02-09T02:23:01.6110266Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-09T02:23:01.6116219Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68694/merge to s
2020-02-09T02:23:01.6117851Z Task         : Get sources
2020-02-09T02:23:01.6117900Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-09T02:23:01.6117934Z Version      : 1.0.0
2020-02-09T02:23:01.6117967Z Author       : Microsoft
---
2020-02-09T02:23:02.4718464Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-09T02:23:02.4809101Z ##[command]git config gc.auto 0
2020-02-09T02:23:02.4892275Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-09T02:23:02.4961247Z ##[command]git config --get-all http.proxy
2020-02-09T02:23:02.5117122Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68694/merge:refs/remotes/pull/68694/merge
---
2020-02-09T02:30:00.5943522Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-02-09T02:30:03.0304294Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-02-09T02:30:03.8329294Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-02-09T02:30:05.4007644Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-02-09T02:30:10.3007868Z error[E0063]: missing field `skip_leak_check` in initializer of `infer::InferCtxtInner<'_>`
2020-02-09T02:30:10.3010017Z     |
2020-02-09T02:30:10.3011132Z 180 |         InferCtxtInner {
2020-02-09T02:30:10.3013020Z     |         ^^^^^^^^^^^^^^ missing `skip_leak_check`
2020-02-09T02:30:10.3013478Z 
2020-02-09T02:30:10.3013478Z 
2020-02-09T02:30:10.3407283Z error[E0560]: struct `infer::InferCtxt<'_, '_>` has no field named `skip_leak_check`
2020-02-09T02:30:10.3408450Z    --> src/librustc/infer/mod.rs:578:17
2020-02-09T02:30:10.3408933Z     |
2020-02-09T02:30:10.3409424Z 578 |                 skip_leak_check: Cell::new(false),
2020-02-09T02:30:10.3410063Z     |                 ^^^^^^^^^^^^^^^ `infer::InferCtxt<'_, '_>` does not have this field
2020-02-09T02:30:10.3410481Z     |
2020-02-09T02:30:10.3411068Z     = note: available fields are: `tcx`, `in_progress_tables`, `inner`, `lexical_region_resolutions`, `selection_cache` ... and 7 others
2020-02-09T02:30:10.3411807Z error[E0560]: struct `infer::InferCtxt<'_, '_>` has no field named `region_obligations`
2020-02-09T02:30:10.3412275Z    --> src/librustc/infer/mod.rs:579:17
2020-02-09T02:30:10.3412673Z     |
2020-02-09T02:30:10.3412673Z     |
2020-02-09T02:30:10.3413172Z 579 |                 region_obligations: RefCell::new(vec![]),
2020-02-09T02:30:10.3413749Z     |                 ^^^^^^^^^^^^^^^^^^ `infer::InferCtxt<'_, '_>` does not have this field
2020-02-09T02:30:10.3414154Z     |
2020-02-09T02:30:10.3414719Z     = note: available fields are: `tcx`, `in_progress_tables`, `inner`, `lexical_region_resolutions`, `selection_cache` ... and 7 others
2020-02-09T02:30:10.3414897Z 
2020-02-09T02:30:10.3820852Z error[E0609]: no field `skip_leak_check` on type `&infer::InferCtxt<'a, 'tcx>`
2020-02-09T02:30:10.3822049Z     |
2020-02-09T02:30:10.3822049Z     |
2020-02-09T02:30:10.3822362Z 751 |             was_skip_leak_check: self.skip_leak_check.get(),
2020-02-09T02:30:10.3825141Z     |
2020-02-09T02:30:10.3825141Z     |
2020-02-09T02:30:10.3825579Z     = note: available fields are: `tcx`, `in_progress_tables`, `inner`, `lexical_region_resolutions`, `selection_cache` ... and 7 others
2020-02-09T02:30:10.3825634Z 
2020-02-09T02:30:10.3849210Z error[E0609]: no field `skip_leak_check` on type `&infer::InferCtxt<'a, 'tcx>`
2020-02-09T02:30:10.3849882Z     |
2020-02-09T02:30:10.3849882Z     |
2020-02-09T02:30:10.3850224Z 776 |         self.skip_leak_check.set(was_skip_leak_check);
2020-02-09T02:30:10.3851091Z     |
2020-02-09T02:30:10.3851091Z     |
2020-02-09T02:30:10.3851639Z     = note: available fields are: `tcx`, `in_progress_tables`, `inner`, `lexical_region_resolutions`, `selection_cache` ... and 7 others
2020-02-09T02:30:10.3851699Z 
2020-02-09T02:30:10.3878741Z error[E0609]: no field `skip_leak_check` on type `&infer::InferCtxt<'a, 'tcx>`
2020-02-09T02:30:10.3879547Z     |
2020-02-09T02:30:10.3879547Z     |
2020-02-09T02:30:10.3879925Z 805 |         self.skip_leak_check.set(was_skip_leak_check);
2020-02-09T02:30:10.3880558Z     |
2020-02-09T02:30:10.3880558Z     |
2020-02-09T02:30:10.3880944Z     = note: available fields are: `tcx`, `in_progress_tables`, `inner`, `lexical_region_resolutions`, `selection_cache` ... and 7 others
2020-02-09T02:30:10.3880993Z 
2020-02-09T02:30:10.3973087Z error[E0609]: no field `skip_leak_check` on type `&infer::InferCtxt<'a, 'tcx>`
2020-02-09T02:30:10.3973685Z     |
2020-02-09T02:30:10.3973685Z     |
2020-02-09T02:30:10.3974011Z 867 |         let skip_leak_check = should_skip || self.skip_leak_check.get();
2020-02-09T02:30:10.3974657Z     |
2020-02-09T02:30:10.3974657Z     |
2020-02-09T02:30:10.3975039Z     = note: available fields are: `tcx`, `in_progress_tables`, `inner`, `lexical_region_resolutions`, `selection_cache` ... and 7 others
2020-02-09T02:30:10.3975103Z 
2020-02-09T02:30:10.3986155Z error[E0609]: no field `skip_leak_check` on type `&infer::InferCtxt<'a, 'tcx>`
2020-02-09T02:30:10.3986708Z     |
2020-02-09T02:30:10.3986708Z     |
2020-02-09T02:30:10.3987019Z 868 |         self.skip_leak_check.set(skip_leak_check);
2020-02-09T02:30:10.3987590Z     |
2020-02-09T02:30:10.3987590Z     |
2020-02-09T02:30:10.3987984Z     = note: available fields are: `tcx`, `in_progress_tables`, `inner`, `lexical_region_resolutions`, `selection_cache` ... and 7 others
2020-02-09T02:30:10.3988028Z 
2020-02-09T02:30:11.0206345Z error[E0609]: no field `skip_leak_check` on type `&infer::InferCtxt<'a, 'tcx>`
2020-02-09T02:30:11.0206767Z    --> src/librustc/infer/higher_ranked/mod.rs:137:68
2020-02-09T02:30:11.0207052Z     |
2020-02-09T02:30:11.0207466Z 137 |         if self.tcx.sess.opts.debugging_opts.no_leak_check || self.skip_leak_check.get() {
2020-02-09T02:30:11.0208149Z     |
2020-02-09T02:30:11.0208149Z     |
2020-02-09T02:30:11.0208778Z     = note: available fields are: `tcx`, `in_progress_tables`, `inner`, `lexical_region_resolutions`, `selection_cache` ... and 7 others
2020-02-09T02:30:20.2725696Z error: aborting due to 9 previous errors
2020-02-09T02:30:20.2726505Z 
2020-02-09T02:30:20.2727104Z Some errors have detailed explanations: E0063, E0560, E0609.
2020-02-09T02:30:20.2727873Z For more information about an error, try `rustc --explain E0063`.
2020-02-09T02:30:20.2727873Z For more information about an error, try `rustc --explain E0063`.
2020-02-09T02:30:20.3005883Z error: could not compile `rustc`.
2020-02-09T02:30:20.3006632Z 
2020-02-09T02:30:20.3007634Z To learn more, run the command again with --verbose.
2020-02-09T02:30:20.3053193Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-09T02:30:20.3057062Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-02-09T02:30:20.3057422Z Build completed unsuccessfully in 0:05:12
2020-02-09T02:30:20.3114155Z == clock drift check ==
2020-02-09T02:30:20.3146073Z   local time: Sun Feb  9 02:30:20 UTC 2020
2020-02-09T02:30:20.3146073Z   local time: Sun Feb  9 02:30:20 UTC 2020
2020-02-09T02:30:20.4680992Z   network time: Sun, 09 Feb 2020 02:30:20 GMT
2020-02-09T02:30:20.4685655Z == end clock drift check ==
2020-02-09T02:30:21.1172139Z 
2020-02-09T02:30:21.1284204Z ##[error]Bash exited with code '1'.
2020-02-09T02:30:21.1296085Z ##[section]Finishing: Run build
2020-02-09T02:30:21.1310763Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68694/merge to s
2020-02-09T02:30:21.1312575Z Task         : Get sources
2020-02-09T02:30:21.1312623Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-09T02:30:21.1312688Z Version      : 1.0.0
2020-02-09T02:30:21.1312730Z Author       : Microsoft
2020-02-09T02:30:21.1312730Z Author       : Microsoft
2020-02-09T02:30:21.1312776Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-09T02:30:21.1312845Z ==============================================================================
2020-02-09T02:30:21.5531193Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-09T02:30:21.5533589Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68694/merge to s
2020-02-09T02:30:21.5650593Z Cleaning up task key
2020-02-09T02:30:21.5651361Z Start cleaning up orphan processes.
2020-02-09T02:30:21.5762672Z Terminate orphan process: pid (3685) (python)
2020-02-09T02:30:21.5956988Z ##[section]Finishing: Finalize Job
