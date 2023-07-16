plain
2020-04-23T20:31:53.4975262Z ========================== Starting Command Output ===========================
2020-04-23T20:31:53.4980719Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/68b13ed4-e868-4420-ab72-3b4c450c276d.sh
2020-04-23T20:31:53.4981213Z 
2020-04-23T20:31:53.4986502Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-23T20:31:53.5006793Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71485/merge to s
2020-04-23T20:31:53.5010311Z Task         : Get sources
2020-04-23T20:31:53.5010617Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T20:31:53.5010911Z Version      : 1.0.0
2020-04-23T20:31:53.5011114Z Author       : Microsoft
---
2020-04-23T20:31:54.4928660Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-23T20:31:54.4935077Z ##[command]git config gc.auto 0
2020-04-23T20:31:54.4938810Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-23T20:31:54.4942158Z ##[command]git config --get-all http.proxy
2020-04-23T20:31:54.4948600Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71485/merge:refs/remotes/pull/71485/merge
---
2020-04-23T20:34:28.6240687Z  ---> 78ad2f4d4aca
2020-04-23T20:34:28.6241681Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-23T20:34:28.6242599Z  ---> Using cache
2020-04-23T20:34:28.6243116Z  ---> 4d2dc61c4d00
2020-04-23T20:34:28.6245350Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-23T20:34:28.6246874Z  ---> 776b6266a8b7
2020-04-23T20:34:28.6247100Z Successfully built 776b6266a8b7
2020-04-23T20:34:28.6282741Z Successfully tagged rust-ci:latest
2020-04-23T20:34:28.6627655Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-23T20:34:28.6627655Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-23T20:34:28.6645165Z Looks like docker image is the same as before, not uploading
2020-04-23T20:34:36.8136325Z [CI_JOB_NAME=mingw-check]
2020-04-23T20:34:36.8477964Z [CI_JOB_NAME=mingw-check]
2020-04-23T20:34:36.8512581Z == clock drift check ==
2020-04-23T20:34:36.8526267Z   local time: Thu Apr 23 20:34:36 UTC 2020
2020-04-23T20:34:36.9505766Z   network time: Thu, 23 Apr 2020 20:34:36 GMT
2020-04-23T20:34:36.9529889Z Starting sccache server...
2020-04-23T20:34:37.0741814Z configure: processing command line
2020-04-23T20:34:37.0742355Z configure: 
2020-04-23T20:34:37.0743327Z configure: rust.parallel-compiler := True
---
2020-04-23T20:38:47.2110994Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-23T20:38:47.3867839Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-23T20:38:47.5958106Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-23T20:38:47.7043529Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-23T20:38:48.2505222Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-23T20:38:50.8979212Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-23T20:38:51.4151807Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-23T20:38:53.7473858Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-23T20:38:54.2271449Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-23T20:40:56.0164448Z configure: build.locked-deps    := True
2020-04-23T20:40:56.0164899Z configure: build.cargo-native-static := True
2020-04-23T20:40:56.0165337Z configure: rust.debug-assertions := True
2020-04-23T20:40:56.0165592Z configure: build.submodules     := False
2020-04-23T20:40:56.0166157Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-23T20:40:56.0166809Z configure: writing `config.toml` in current directory
2020-04-23T20:40:56.0167042Z configure: 
2020-04-23T20:40:56.0167473Z configure: run `python /checkout/x.py --help`
2020-04-23T20:40:56.0167678Z configure: 
---
2020-04-23T20:42:38.4131307Z Hugepagesize:       2048 kB
2020-04-23T20:42:38.4131907Z DirectMap4k:      133056 kB
2020-04-23T20:42:38.4132138Z DirectMap2M:     4061184 kB
2020-04-23T20:42:38.4132383Z DirectMap1G:     5242880 kB
2020-04-23T20:42:38.4193374Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-23T20:42:39.9437110Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-23T20:42:39.9437110Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-23T20:42:39.9446626Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-23T20:42:40.2163014Z    Compiling unicode-xid v0.2.0
2020-04-23T20:42:40.3503710Z    Compiling syn v1.0.11
2020-04-23T20:42:41.2389366Z    Compiling linked-hash-map v0.5.2
2020-04-23T20:42:41.2753090Z    Compiling lazy_static v1.4.0
2020-04-23T20:42:41.2753090Z    Compiling lazy_static v1.4.0
2020-04-23T20:42:41.4830194Z    Compiling yaml-rust v0.4.3
2020-04-23T20:42:46.1295342Z    Compiling quote v1.0.2
2020-04-23T20:43:02.0431330Z    Compiling thiserror-impl v1.0.5
2020-04-23T20:43:07.2647868Z    Compiling thiserror v1.0.5
2020-04-23T20:43:07.3276866Z    Compiling yaml-merge-keys v0.4.0
2020-04-23T20:43:08.6132706Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-23T20:43:10.3758490Z Build completed successfully in 0:00:31
2020-04-23T20:43:10.3849417Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-23T20:43:10.6683476Z     Finished dev [unoptimized] target(s) in 0.18s
2020-04-23T20:43:11.8817121Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-23T20:45:24.0603968Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-23T20:45:29.2248588Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-23T20:45:30.6895576Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-23T20:45:30.7619468Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-23T20:45:31.0053871Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-23T20:45:31.8993920Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-23T20:45:31.9803304Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-23T20:45:33.7367671Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-23T20:45:34.2681947Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-23T20:50:07.9699485Z Found 0 error codes with no tests
2020-04-23T20:50:07.9699663Z Done!
2020-04-23T20:50:07.9713026Z 
2020-04-23T20:50:07.9713237Z 
2020-04-23T20:50:07.9714457Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-23T20:50:07.9715140Z 
2020-04-23T20:50:07.9715232Z 
2020-04-23T20:50:07.9715470Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-23T20:50:07.9715806Z Build completed unsuccessfully in 0:00:40
2020-04-23T20:50:07.9715806Z Build completed unsuccessfully in 0:00:40
2020-04-23T20:50:07.9716032Z some tidy checks failed
2020-04-23T20:50:07.9820583Z == clock drift check ==
2020-04-23T20:50:07.9841113Z   local time: Thu Apr 23 20:50:07 UTC 2020
2020-04-23T20:50:08.2719812Z   network time: Thu, 23 Apr 2020 20:50:08 GMT
2020-04-23T20:50:09.8230882Z 
2020-04-23T20:50:09.8230882Z 
2020-04-23T20:50:09.8304877Z ##[error]Bash exited with code '1'.
2020-04-23T20:50:09.8320908Z ##[section]Finishing: Run build
2020-04-23T20:50:09.8365181Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71485/merge to s
2020-04-23T20:50:09.8369834Z Task         : Get sources
2020-04-23T20:50:09.8370143Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T20:50:09.8370444Z Version      : 1.0.0
2020-04-23T20:50:09.8370648Z Author       : Microsoft
2020-04-23T20:50:09.8370648Z Author       : Microsoft
2020-04-23T20:50:09.8371711Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-23T20:50:09.8372106Z ==============================================================================
2020-04-23T20:50:10.2246031Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-23T20:50:10.2298552Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71485/merge to s
2020-04-23T20:50:10.2405801Z Cleaning up task key
2020-04-23T20:50:10.2407240Z Start cleaning up orphan processes.
2020-04-23T20:50:10.2627907Z Terminate orphan process: pid (3567) (python)
2020-04-23T20:50:10.2846918Z ##[section]Finishing: Finalize Job
