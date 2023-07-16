plain
2020-03-31T17:46:44.8324696Z ========================== Starting Command Output ===========================
2020-03-31T17:46:44.8327320Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/79502238-e9ce-4830-b84a-5e83e8bc24ae.sh
2020-03-31T17:46:44.8327631Z 
2020-03-31T17:46:44.8331582Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-31T17:46:44.8351899Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70627/merge to s
2020-03-31T17:46:44.8355327Z Task         : Get sources
2020-03-31T17:46:44.8355663Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T17:46:44.8355991Z Version      : 1.0.0
2020-03-31T17:46:44.8356225Z Author       : Microsoft
---
2020-03-31T17:46:46.1127334Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-31T17:46:47.1118614Z ##[command]git config gc.auto 0
2020-03-31T17:46:47.1122790Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-31T17:46:47.1127602Z ##[command]git config --get-all http.proxy
2020-03-31T17:46:47.1134909Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70627/merge:refs/remotes/pull/70627/merge
---
2020-03-31T17:50:58.6051899Z  ---> 3fc1b512c57b
2020-03-31T17:50:58.6052138Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-03-31T17:50:58.6052516Z  ---> Using cache
2020-03-31T17:50:58.6052862Z  ---> 5ee4295733f4
2020-03-31T17:50:58.6054227Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-03-31T17:50:58.6055594Z  ---> 3d07a0fa42fe
2020-03-31T17:50:58.6055800Z Successfully built 3d07a0fa42fe
2020-03-31T17:50:58.6076052Z Successfully tagged rust-ci:latest
2020-03-31T17:50:58.6328765Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
---
2020-03-31T17:54:04.5268054Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T17:54:04.6942928Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T17:54:04.8609082Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T17:54:04.8745737Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T17:54:05.3790622Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T17:54:07.1764611Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T17:54:07.5737076Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T17:54:09.2081943Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T17:54:09.5946234Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T17:54:35.6354506Z 
2020-03-31T17:54:35.9426004Z error[E0282]: type annotations needed
2020-03-31T17:54:35.9426692Z    --> src/librustc_mir_build/build/block.rs:151:35
2020-03-31T17:54:35.9427183Z     |
2020-03-31T17:54:35.9427801Z 151 | ...                   &mut |this, _, _, _, node, span, _, _| {
2020-03-31T17:54:35.9429332Z     |
2020-03-31T17:54:35.9429897Z     = note: type must be known at this point
2020-03-31T17:54:35.9430191Z 
2020-03-31T17:54:36.0096886Z error[E0282]: type annotations needed
2020-03-31T17:54:36.0096886Z error[E0282]: type annotations needed
2020-03-31T17:54:36.0097602Z    --> src/librustc_mir_build/build/matches/mod.rs:516:19
2020-03-31T17:54:36.0098182Z     |
2020-03-31T17:54:36.0098671Z 516 |             &mut |this, mutability, name, mode, var, span, ty, user_ty| {
2020-03-31T17:54:36.0099942Z     |
2020-03-31T17:54:36.0100355Z     = note: type must be known at this point
2020-03-31T17:54:36.0100559Z 
2020-03-31T17:54:36.6316356Z error: aborting due to 5 previous errors
---
2020-03-31T17:54:36.6358269Z 
2020-03-31T17:54:36.6358643Z To learn more, run the command again with --verbose.
2020-03-31T17:54:36.6359124Z warning: build failed, waiting for other jobs to finish...
2020-03-31T17:54:40.4518857Z error: build failed
2020-03-31T17:54:40.4546375Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-31T17:54:40.4553854Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-03-31T17:54:40.4554663Z Build completed unsuccessfully in 0:03:34
2020-03-31T17:54:40.4599639Z == clock drift check ==
2020-03-31T17:54:40.4615592Z   local time: Tue Mar 31 17:54:40 UTC 2020
2020-03-31T17:54:40.4615592Z   local time: Tue Mar 31 17:54:40 UTC 2020
2020-03-31T17:54:40.7442883Z   network time: Tue, 31 Mar 2020 17:54:40 GMT
2020-03-31T17:54:40.7443861Z == end clock drift check ==
2020-03-31T17:54:41.5138115Z 
2020-03-31T17:54:41.5187366Z ##[error]Bash exited with code '1'.
2020-03-31T17:54:41.5199589Z ##[section]Finishing: Run build
2020-03-31T17:54:41.5243690Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70627/merge to s
2020-03-31T17:54:41.5249311Z Task         : Get sources
2020-03-31T17:54:41.5249657Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T17:54:41.5249989Z Version      : 1.0.0
2020-03-31T17:54:41.5250218Z Author       : Microsoft
2020-03-31T17:54:41.5250218Z Author       : Microsoft
2020-03-31T17:54:41.5250568Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-31T17:54:41.5250986Z ==============================================================================
2020-03-31T17:54:41.8043277Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-31T17:54:41.8082045Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70627/merge to s
2020-03-31T17:54:41.8143940Z Cleaning up task key
2020-03-31T17:54:41.8144856Z Start cleaning up orphan processes.
2020-03-31T17:54:41.8283754Z Terminate orphan process: pid (3627) (python)
2020-03-31T17:54:41.8384122Z ##[section]Finishing: Finalize Job
