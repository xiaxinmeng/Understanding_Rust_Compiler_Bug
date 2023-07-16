plain
2020-02-09T13:52:59.7803816Z ========================== Starting Command Output ===========================
2020-02-09T13:52:59.7805099Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b9372ccd-9648-4997-8d67-c20615ae50b6.sh
2020-02-09T13:52:59.7805130Z 
2020-02-09T13:52:59.7808041Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-09T13:52:59.7814641Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68996/merge to s
2020-02-09T13:52:59.7816261Z Task         : Get sources
2020-02-09T13:52:59.7816288Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-09T13:52:59.7816314Z Version      : 1.0.0
2020-02-09T13:52:59.7816340Z Author       : Microsoft
---
2020-02-09T13:53:04.5280557Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-09T13:53:04.5354728Z ##[command]git config gc.auto 0
2020-02-09T13:53:04.5398346Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-09T13:53:04.5410058Z ##[command]git config --get-all http.proxy
2020-02-09T13:53:04.5415548Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68996/merge:refs/remotes/pull/68996/merge
---
2020-02-09T14:00:19.9665031Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-02-09T14:00:22.3649103Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-02-09T14:00:23.0999799Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-02-09T14:00:24.6234371Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-02-09T14:00:29.4804142Z error[E0063]: missing field `skip_leak_check` in initializer of `infer::InferCtxtInner<'_>`
2020-02-09T14:00:29.4806191Z     |
2020-02-09T14:00:29.4806720Z 180 |         InferCtxtInner {
2020-02-09T14:00:29.4807294Z     |         ^^^^^^^^^^^^^^ missing `skip_leak_check`
2020-02-09T14:00:29.4807549Z 
2020-02-09T14:00:29.4807549Z 
2020-02-09T14:00:29.5251872Z error[E0560]: struct `infer::InferCtxt<'_, '_>` has no field named `skip_leak_check`
2020-02-09T14:00:29.5253627Z    --> src/librustc/infer/mod.rs:578:17
2020-02-09T14:00:29.5254237Z     |
2020-02-09T14:00:29.5254795Z 578 |                 skip_leak_check: Cell::new(false),
2020-02-09T14:00:29.5255458Z     |                 ^^^^^^^^^^^^^^^ `infer::InferCtxt<'_, '_>` does not have this field
2020-02-09T14:00:29.5255954Z     |
2020-02-09T14:00:29.5256580Z     = note: available fields are: `tcx`, `in_progress_tables`, `inner`, `lexical_region_resolutions`, `selection_cache` ... and 7 others
2020-02-09T14:00:29.5257494Z error[E0560]: struct `infer::InferCtxt<'_, '_>` has no field named `region_obligations`
2020-02-09T14:00:29.5258059Z    --> src/librustc/infer/mod.rs:579:17
2020-02-09T14:00:29.5258711Z     |
2020-02-09T14:00:29.5258711Z     |
2020-02-09T14:00:29.5259266Z 579 |                 region_obligations: RefCell::new(vec![]),
2020-02-09T14:00:29.5259921Z     |                 ^^^^^^^^^^^^^^^^^^ `infer::InferCtxt<'_, '_>` does not have this field
2020-02-09T14:00:29.5260555Z     |
2020-02-09T14:00:29.5261192Z     = note: available fields are: `tcx`, `in_progress_tables`, `inner`, `lexical_region_resolutions`, `selection_cache` ... and 7 others
2020-02-09T14:00:29.5262462Z 
2020-02-09T14:00:29.5681267Z error[E0609]: no field `skip_leak_check` on type `&infer::InferCtxt<'a, 'tcx>`
2020-02-09T14:00:29.5684334Z     |
2020-02-09T14:00:29.5684334Z     |
2020-02-09T14:00:29.5684603Z 751 |             was_skip_leak_check: self.skip_leak_check.get(),
2020-02-09T14:00:29.5692476Z     |
2020-02-09T14:00:29.5692476Z     |
2020-02-09T14:00:29.5693310Z     = note: available fields are: `tcx`, `in_progress_tables`, `inner`, `lexical_region_resolutions`, `selection_cache` ... and 7 others
2020-02-09T14:00:29.5693501Z 
2020-02-09T14:00:29.5711656Z error[E0609]: no field `skip_leak_check` on type `&infer::InferCtxt<'a, 'tcx>`
2020-02-09T14:00:29.5715881Z     |
2020-02-09T14:00:29.5715881Z     |
2020-02-09T14:00:29.5716168Z 776 |         self.skip_leak_check.set(was_skip_leak_check);
2020-02-09T14:00:29.5716868Z     |
2020-02-09T14:00:29.5716868Z     |
2020-02-09T14:00:29.5717214Z     = note: available fields are: `tcx`, `in_progress_tables`, `inner`, `lexical_region_resolutions`, `selection_cache` ... and 7 others
2020-02-09T14:00:29.5717260Z 
2020-02-09T14:00:29.5748704Z error[E0609]: no field `skip_leak_check` on type `&infer::InferCtxt<'a, 'tcx>`
2020-02-09T14:00:29.5749259Z     |
2020-02-09T14:00:29.5749259Z     |
2020-02-09T14:00:29.5749726Z 805 |         self.skip_leak_check.set(was_skip_leak_check);
2020-02-09T14:00:29.5750402Z     |
2020-02-09T14:00:29.5750402Z     |
2020-02-09T14:00:29.5750726Z     = note: available fields are: `tcx`, `in_progress_tables`, `inner`, `lexical_region_resolutions`, `selection_cache` ... and 7 others
2020-02-09T14:00:29.5750767Z 
2020-02-09T14:00:29.5841553Z error[E0609]: no field `skip_leak_check` on type `&infer::InferCtxt<'a, 'tcx>`
2020-02-09T14:00:29.5842263Z     |
2020-02-09T14:00:29.5842263Z     |
2020-02-09T14:00:29.5842608Z 867 |         let skip_leak_check = should_skip || self.skip_leak_check.get();
2020-02-09T14:00:29.5843219Z     |
2020-02-09T14:00:29.5843219Z     |
2020-02-09T14:00:29.5843577Z     = note: available fields are: `tcx`, `in_progress_tables`, `inner`, `lexical_region_resolutions`, `selection_cache` ... and 7 others
2020-02-09T14:00:29.5843624Z 
2020-02-09T14:00:29.5859094Z error[E0609]: no field `skip_leak_check` on type `&infer::InferCtxt<'a, 'tcx>`
2020-02-09T14:00:29.5859631Z     |
2020-02-09T14:00:29.5859631Z     |
2020-02-09T14:00:29.5859922Z 868 |         self.skip_leak_check.set(skip_leak_check);
2020-02-09T14:00:29.5860478Z     |
2020-02-09T14:00:29.5860478Z     |
2020-02-09T14:00:29.5860826Z     = note: available fields are: `tcx`, `in_progress_tables`, `inner`, `lexical_region_resolutions`, `selection_cache` ... and 7 others
2020-02-09T14:00:29.5861006Z 
2020-02-09T14:00:30.2414916Z error[E0609]: no field `skip_leak_check` on type `&infer::InferCtxt<'a, 'tcx>`
2020-02-09T14:00:30.2416459Z    --> src/librustc/infer/higher_ranked/mod.rs:137:68
2020-02-09T14:00:30.2417033Z     |
2020-02-09T14:00:30.2417599Z 137 |         if self.tcx.sess.opts.debugging_opts.no_leak_check || self.skip_leak_check.get() {
2020-02-09T14:00:30.2419155Z     |
2020-02-09T14:00:30.2419155Z     |
2020-02-09T14:00:30.2419798Z     = note: available fields are: `tcx`, `in_progress_tables`, `inner`, `lexical_region_resolutions`, `selection_cache` ... and 7 others
2020-02-09T14:00:39.0270952Z error: aborting due to 9 previous errors
2020-02-09T14:00:39.0271696Z 
2020-02-09T14:00:39.0272232Z Some errors have detailed explanations: E0063, E0560, E0609.
2020-02-09T14:00:39.0272693Z For more information about an error, try `rustc --explain E0063`.
2020-02-09T14:00:39.0272693Z For more information about an error, try `rustc --explain E0063`.
2020-02-09T14:00:39.0542085Z error: could not compile `rustc`.
2020-02-09T14:00:39.0542769Z 
2020-02-09T14:00:39.0543251Z To learn more, run the command again with --verbose.
2020-02-09T14:00:39.0580213Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-09T14:00:39.0594008Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-02-09T14:00:39.0594365Z Build completed unsuccessfully in 0:04:47
2020-02-09T14:00:39.0643018Z == clock drift check ==
2020-02-09T14:00:39.0655168Z   local time: Sun Feb  9 14:00:39 UTC 2020
2020-02-09T14:00:39.0655168Z   local time: Sun Feb  9 14:00:39 UTC 2020
2020-02-09T14:00:39.3434251Z   network time: Sun, 09 Feb 2020 14:00:39 GMT
2020-02-09T14:00:39.3437614Z == end clock drift check ==
2020-02-09T14:00:39.9829388Z 
2020-02-09T14:00:39.9923880Z ##[error]Bash exited with code '1'.
2020-02-09T14:00:39.9938017Z ##[section]Finishing: Run build
2020-02-09T14:00:39.9985490Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68996/merge to s
2020-02-09T14:00:39.9987227Z Task         : Get sources
2020-02-09T14:00:39.9987265Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-09T14:00:39.9987319Z Version      : 1.0.0
2020-02-09T14:00:39.9987352Z Author       : Microsoft
2020-02-09T14:00:39.9987352Z Author       : Microsoft
2020-02-09T14:00:39.9987390Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-09T14:00:39.9987444Z ==============================================================================
2020-02-09T14:00:40.3817700Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-09T14:00:40.3859940Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68996/merge to s
2020-02-09T14:00:40.3965408Z Cleaning up task key
2020-02-09T14:00:40.3966200Z Start cleaning up orphan processes.
2020-02-09T14:00:40.4067665Z Terminate orphan process: pid (3570) (python)
2020-02-09T14:00:40.4714576Z ##[section]Finishing: Finalize Job
