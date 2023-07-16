plain
2020-03-10T09:41:58.2595571Z ========================== Starting Command Output ===========================
2020-03-10T09:41:58.2599532Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c8ee23d7-703b-406f-a59f-6f80ab156acf.sh
2020-03-10T09:41:58.2599800Z 
2020-03-10T09:41:58.2603016Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-10T09:41:58.2620695Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69883/merge to s
2020-03-10T09:41:58.2624147Z Task         : Get sources
2020-03-10T09:41:58.2624440Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-10T09:41:58.2624669Z Version      : 1.0.0
2020-03-10T09:41:58.2624824Z Author       : Microsoft
---
2020-03-10T09:42:01.2777350Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-10T09:42:01.2784267Z ##[command]git config gc.auto 0
2020-03-10T09:42:01.2788897Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-10T09:42:01.2793476Z ##[command]git config --get-all http.proxy
2020-03-10T09:42:01.2801635Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69883/merge:refs/remotes/pull/69883/merge
---
2020-03-10T09:49:49.8475172Z     Checking rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2020-03-10T09:49:51.6855381Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2020-03-10T09:49:52.1514045Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2020-03-10T09:49:52.7025293Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2020-03-10T09:49:52.9274775Z error[E0433]: failed to resolve: use of undeclared type or module `ImportDirectiveSubclass`
2020-03-10T09:49:52.9275922Z     |
2020-03-10T09:49:52.9275922Z     |
2020-03-10T09:49:52.9276619Z 968 |                     ImportDirectiveSubclass::SingleImport { source, .. } => {
2020-03-10T09:49:52.9277515Z     |                     ^^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type or module `ImportDirectiveSubclass`
2020-03-10T09:49:52.9284066Z 
2020-03-10T09:49:52.9309417Z error[E0433]: failed to resolve: use of undeclared type or module `ImportDirectiveSubclass`
2020-03-10T09:49:52.9310762Z     |
2020-03-10T09:49:52.9310762Z     |
2020-03-10T09:49:52.9311339Z 972 |                     ImportDirectiveSubclass::GlobImport { .. }
2020-03-10T09:49:52.9312212Z     |                     ^^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type or module `ImportDirectiveSubclass`
2020-03-10T09:49:52.9316348Z 
2020-03-10T09:49:52.9341747Z error[E0433]: failed to resolve: use of undeclared type or module `ImportDirectiveSubclass`
2020-03-10T09:49:52.9342846Z     |
2020-03-10T09:49:52.9342846Z     |
2020-03-10T09:49:52.9343472Z 973 |                     | ImportDirectiveSubclass::MacroUse => Some(binding),
2020-03-10T09:49:52.9344361Z     |                       ^^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type or module `ImportDirectiveSubclass`
2020-03-10T09:49:52.9348377Z 
2020-03-10T09:49:52.9371302Z error[E0433]: failed to resolve: use of undeclared type or module `ImportDirectiveSubclass`
2020-03-10T09:49:52.9372417Z     |
2020-03-10T09:49:52.9373018Z 974 |                     ImportDirectiveSubclass::ExternCrate { .. } => None,
2020-03-10T09:49:52.9373018Z 974 |                     ImportDirectiveSubclass::ExternCrate { .. } => None,
2020-03-10T09:49:52.9373890Z     |                     ^^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type or module `ImportDirectiveSubclass`
2020-03-10T09:49:52.9377912Z 
2020-03-10T09:49:53.6028728Z error[E0026]: variant `NameBindingKind::Import` does not have a field named `directive`
2020-03-10T09:49:53.6029951Z     |
2020-03-10T09:49:53.6029951Z     |
2020-03-10T09:49:53.6030633Z 966 |                 NameBindingKind::Import { binding, directive, .. } => match directive.subclass {
2020-03-10T09:49:53.6031718Z     |                                                    ^^^^^^^^^ variant `NameBindingKind::Import` does not have this field
2020-03-10T09:49:54.2531871Z error: aborting due to 5 previous errors
2020-03-10T09:49:54.2536198Z 
2020-03-10T09:49:54.2545298Z Some errors have detailed explanations: E0026, E0433.
2020-03-10T09:49:54.2554966Z For more information about an error, try `rustc --explain E0026`.
2020-03-10T09:49:54.2554966Z For more information about an error, try `rustc --explain E0026`.
2020-03-10T09:49:54.2617005Z error: could not compile `rustc_resolve`.
2020-03-10T09:49:54.2644576Z warning: build failed, waiting for other jobs to finish...
2020-03-10T09:49:58.7988555Z error: build failed
2020-03-10T09:49:58.8014070Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-10T09:49:58.8024425Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-03-10T09:49:58.8024776Z Build completed unsuccessfully in 0:04:07
2020-03-10T09:49:58.8071924Z == clock drift check ==
2020-03-10T09:49:58.8086129Z   local time: Tue Mar 10 09:49:58 UTC 2020
2020-03-10T09:49:58.8086129Z   local time: Tue Mar 10 09:49:58 UTC 2020
2020-03-10T09:49:59.0912336Z   network time: Tue, 10 Mar 2020 09:49:59 GMT
2020-03-10T09:49:59.0912665Z == end clock drift check ==
2020-03-10T09:49:59.7639357Z 
2020-03-10T09:49:59.7708743Z ##[error]Bash exited with code '1'.
2020-03-10T09:49:59.7737921Z ##[section]Finishing: Run build
2020-03-10T09:49:59.7780671Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69883/merge to s
2020-03-10T09:49:59.7785100Z Task         : Get sources
2020-03-10T09:49:59.7785388Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-10T09:49:59.7785673Z Version      : 1.0.0
2020-03-10T09:49:59.7785856Z Author       : Microsoft
2020-03-10T09:49:59.7785856Z Author       : Microsoft
2020-03-10T09:49:59.7786146Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-10T09:49:59.7786501Z ==============================================================================
2020-03-10T09:50:00.0531867Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-10T09:50:00.0581826Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69883/merge to s
2020-03-10T09:50:00.0658415Z Cleaning up task key
2020-03-10T09:50:00.0659463Z Start cleaning up orphan processes.
2020-03-10T09:50:00.0812588Z Terminate orphan process: pid (4910) (python)
2020-03-10T09:50:00.0947223Z ##[section]Finishing: Finalize Job
