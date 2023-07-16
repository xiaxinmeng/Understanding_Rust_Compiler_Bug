plain
2020-04-14T03:15:40.9292013Z ========================== Starting Command Output ===========================
2020-04-14T03:15:40.9294725Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/47b33584-14f8-4c97-a2f6-7a71feb1888f.sh
2020-04-14T03:15:40.9294967Z 
2020-04-14T03:15:40.9298800Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-14T03:15:40.9343529Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70998/merge to s
2020-04-14T03:15:40.9346939Z Task         : Get sources
2020-04-14T03:15:40.9347230Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T03:15:40.9347497Z Version      : 1.0.0
2020-04-14T03:15:40.9347679Z Author       : Microsoft
---
2020-04-14T03:15:41.9382617Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-14T03:15:41.9389969Z ##[command]git config gc.auto 0
2020-04-14T03:15:41.9394293Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-14T03:15:41.9399048Z ##[command]git config --get-all http.proxy
2020-04-14T03:15:41.9407123Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70998/merge:refs/remotes/pull/70998/merge
---
2020-04-14T03:19:03.9781730Z  ---> 78ad2f4d4aca
2020-04-14T03:19:03.9781986Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-14T03:19:03.9782400Z  ---> Using cache
2020-04-14T03:19:03.9782788Z  ---> 4d2dc61c4d00
2020-04-14T03:19:03.9784255Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-14T03:19:03.9785690Z  ---> 776b6266a8b7
2020-04-14T03:19:03.9786130Z Successfully built 776b6266a8b7
2020-04-14T03:19:03.9786647Z Successfully tagged rust-ci:latest
2020-04-14T03:19:03.9787030Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-14T03:19:03.9787030Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-14T03:19:03.9787438Z Looks like docker image is the same as before, not uploading
2020-04-14T03:19:11.4269297Z [CI_JOB_NAME=mingw-check]
2020-04-14T03:19:11.4487789Z [CI_JOB_NAME=mingw-check]
2020-04-14T03:19:11.4518999Z == clock drift check ==
2020-04-14T03:19:11.4531047Z   local time: Tue Apr 14 03:19:11 UTC 2020
2020-04-14T03:19:11.6461426Z   network time: Tue, 14 Apr 2020 03:19:11 GMT
2020-04-14T03:19:11.6483831Z Starting sccache server...
2020-04-14T03:19:11.7559481Z configure: processing command line
2020-04-14T03:19:11.7560870Z configure: 
2020-04-14T03:19:11.7561993Z configure: rust.parallel-compiler := True
---
2020-04-14T03:22:54.6814283Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-14T03:22:54.7876296Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-14T03:22:54.9799299Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-14T03:22:55.0780227Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-14T03:22:55.5850754Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-14T03:22:58.1144362Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-14T03:22:58.5882038Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-14T03:23:00.5777848Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-14T03:23:01.0046487Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-14T03:23:29.8055608Z 1282 | |     inherited: &'a Inherited<'a, 'tcx>,
2020-04-14T03:23:29.8056540Z 1283 | |     param_env: ty::ParamEnv<'tcx>,
2020-04-14T03:23:29.8057385Z 1284 | |     fn_sig: ty::FnSig<'tcx>,
2020-04-14T03:23:29.8058024Z ...    |
2020-04-14T03:23:29.8058869Z 1386 | |         self.tcx.sess.delay_span_bug(decl.output.span(), "`!Sized` return type");
2020-04-14T03:23:29.8059876Z      | |         ^^^^ `self` value is a keyword only available in methods with a `self` parameter
2020-04-14T03:23:29.8060625Z ...    |
2020-04-14T03:23:29.8061237Z 1552 | |     (fcx, gen_ty)
2020-04-14T03:23:29.8062665Z      | |_- this function doesn't have a `self` parameter
2020-04-14T03:23:29.8062968Z 
2020-04-14T03:23:30.3351787Z     Checking rustc_ty v0.0.0 (/checkout/src/librustc_ty)
2020-04-14T03:23:30.7025916Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
---
2020-04-14T03:23:33.3299527Z 
2020-04-14T03:23:33.3299921Z To learn more, run the command again with --verbose.
2020-04-14T03:23:33.3300501Z warning: build failed, waiting for other jobs to finish...
2020-04-14T03:23:43.8730416Z error: build failed
2020-04-14T03:23:43.8764911Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-14T03:23:43.8792373Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-14T03:23:43.8792785Z Build completed unsuccessfully in 0:04:32
2020-04-14T03:23:43.8900988Z == clock drift check ==
2020-04-14T03:23:43.8900988Z == clock drift check ==
2020-04-14T03:23:43.8917013Z   local time: Tue Apr 14 03:23:43 UTC 2020
2020-04-14T03:23:44.2125852Z   network time: Tue, 14 Apr 2020 03:23:44 GMT
2020-04-14T03:23:44.6809946Z 
2020-04-14T03:23:44.6809946Z 
2020-04-14T03:23:44.6870579Z ##[error]Bash exited with code '1'.
2020-04-14T03:23:44.6883418Z ##[section]Finishing: Run build
2020-04-14T03:23:44.6924208Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70998/merge to s
2020-04-14T03:23:44.6928695Z Task         : Get sources
2020-04-14T03:23:44.6929002Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T03:23:44.6929269Z Version      : 1.0.0
2020-04-14T03:23:44.6929459Z Author       : Microsoft
2020-04-14T03:23:44.6929459Z Author       : Microsoft
2020-04-14T03:23:44.6929776Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-14T03:23:44.6930111Z ==============================================================================
2020-04-14T03:23:45.0237338Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-14T03:23:45.0282624Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70998/merge to s
2020-04-14T03:23:45.0371178Z Cleaning up task key
2020-04-14T03:23:45.0372477Z Start cleaning up orphan processes.
2020-04-14T03:23:45.0552599Z Terminate orphan process: pid (3692) (python)
2020-04-14T03:23:45.0902157Z ##[section]Finishing: Finalize Job
