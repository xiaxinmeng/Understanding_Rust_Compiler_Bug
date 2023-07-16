plain
2020-04-21T09:20:29.3786434Z ========================== Starting Command Output ===========================
2020-04-21T09:20:29.3789761Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b200b51d-74b9-45c3-aee3-f90d4cba68c6.sh
2020-04-21T09:20:29.3790143Z 
2020-04-21T09:20:29.3793687Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-21T09:20:29.3811925Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71370/merge to s
2020-04-21T09:20:29.3815118Z Task         : Get sources
2020-04-21T09:20:29.3815398Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-21T09:20:29.3815669Z Version      : 1.0.0
2020-04-21T09:20:29.3815875Z Author       : Microsoft
---
2020-04-21T09:20:30.6208446Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-21T09:20:30.6217504Z ##[command]git config gc.auto 0
2020-04-21T09:20:30.6223399Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-21T09:20:30.6229239Z ##[command]git config --get-all http.proxy
2020-04-21T09:20:30.6239406Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71370/merge:refs/remotes/pull/71370/merge
---
2020-04-21T09:23:00.3679292Z  ---> 78ad2f4d4aca
2020-04-21T09:23:00.3679540Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-21T09:23:00.3684870Z  ---> Using cache
2020-04-21T09:23:00.3685246Z  ---> 4d2dc61c4d00
2020-04-21T09:23:00.3686527Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-21T09:23:00.3693197Z  ---> 776b6266a8b7
2020-04-21T09:23:00.3724675Z Successfully built 776b6266a8b7
2020-04-21T09:23:00.3760376Z Successfully tagged rust-ci:latest
2020-04-21T09:23:00.4042602Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-21T09:23:00.4042602Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-21T09:23:00.4057672Z Looks like docker image is the same as before, not uploading
2020-04-21T09:23:08.4358407Z [CI_JOB_NAME=mingw-check]
2020-04-21T09:23:08.4730093Z [CI_JOB_NAME=mingw-check]
2020-04-21T09:23:08.4765431Z == clock drift check ==
2020-04-21T09:23:08.4772045Z   local time: Tue Apr 21 09:23:08 UTC 2020
2020-04-21T09:23:08.6341321Z   network time: Tue, 21 Apr 2020 09:23:08 GMT
2020-04-21T09:23:08.6367034Z Starting sccache server...
2020-04-21T09:23:08.7461539Z configure: processing command line
2020-04-21T09:23:08.7461981Z configure: 
2020-04-21T09:23:08.7463050Z configure: rust.parallel-compiler := True
---
2020-04-21T09:26:43.1639741Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-21T09:26:47.3295331Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-21T09:26:48.4502807Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-21T09:26:48.5185384Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-21T09:26:48.7111166Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-21T09:26:49.4060308Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-21T09:26:49.5066948Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-21T09:26:50.8519912Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-21T09:26:51.3293206Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-21T09:28:37.9181994Z configure: llvm.assertions      := True
2020-04-21T09:28:37.9182378Z configure: rust.channel         := nightly
2020-04-21T09:28:37.9182925Z configure: dist.missing-tools   := True
2020-04-21T09:28:37.9183482Z configure: rust.dist-src        := False
2020-04-21T09:28:37.9184172Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-21T09:28:37.9184892Z configure: writing `config.toml` in current directory
2020-04-21T09:28:37.9185232Z configure: 
2020-04-21T09:28:37.9185729Z configure: run `python /checkout/x.py --help`
2020-04-21T09:28:37.9186064Z configure: 
---
2020-04-21T09:30:07.6266671Z Hugepagesize:       2048 kB
2020-04-21T09:30:07.6266869Z DirectMap4k:      124864 kB
2020-04-21T09:30:07.6267078Z DirectMap2M:     4069376 kB
2020-04-21T09:30:07.6267281Z DirectMap1G:     5242880 kB
2020-04-21T09:30:07.6341790Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-21T09:30:08.9456801Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-21T09:30:08.9456801Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-21T09:30:08.9471189Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-21T09:30:09.1730947Z    Compiling unicode-xid v0.2.0
2020-04-21T09:30:09.2967038Z    Compiling syn v1.0.11
2020-04-21T09:30:10.0805617Z    Compiling linked-hash-map v0.5.2
2020-04-21T09:30:10.1712698Z    Compiling lazy_static v1.4.0
2020-04-21T09:30:10.1712698Z    Compiling lazy_static v1.4.0
2020-04-21T09:30:10.2959735Z    Compiling yaml-rust v0.4.3
2020-04-21T09:30:14.6620596Z    Compiling quote v1.0.2
2020-04-21T09:30:28.2699447Z    Compiling thiserror-impl v1.0.5
2020-04-21T09:30:33.0945955Z    Compiling thiserror v1.0.5
2020-04-21T09:30:33.1538786Z    Compiling yaml-merge-keys v0.4.0
2020-04-21T09:30:34.3175724Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-21T09:30:36.4882618Z Build completed successfully in 0:00:28
2020-04-21T09:30:36.4983733Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-21T09:30:36.7865921Z     Finished dev [unoptimized] target(s) in 0.19s
2020-04-21T09:30:38.0476253Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-21T09:32:38.6501201Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-21T09:32:38.9733406Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-21T09:32:39.0468173Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-21T09:32:39.1829936Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-21T09:32:39.6187907Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-21T09:32:41.6786257Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-21T09:32:42.1454390Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-21T09:32:44.0671132Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-21T09:32:44.4908765Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-21T09:36:16.2057043Z    Compiling cargo_metadata v0.9.1
2020-04-21T09:36:19.6115301Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-04-21T09:36:30.2769390Z     Finished release [optimized] target(s) in 24.10s
2020-04-21T09:36:30.2872400Z tidy check
2020-04-21T09:36:31.6405060Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0696.md:12: line longer than 80 chars
2020-04-21T09:36:31.6405912Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0696.md:29: trailing whitespace
2020-04-21T09:36:40.9059844Z Found 491 error codes
2020-04-21T09:36:40.9061275Z Found 0 error codes with no tests
2020-04-21T09:36:40.9061526Z Done!
2020-04-21T09:36:40.9061709Z some tidy checks failed
2020-04-21T09:36:40.9061709Z some tidy checks failed
2020-04-21T09:36:40.9061861Z 
2020-04-21T09:36:40.9061970Z 
2020-04-21T09:36:40.9104155Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-21T09:36:40.9108739Z 
2020-04-21T09:36:40.9108835Z 
2020-04-21T09:36:40.9109091Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-21T09:36:40.9109406Z Build completed unsuccessfully in 0:00:36
2020-04-21T09:36:40.9109406Z Build completed unsuccessfully in 0:00:36
2020-04-21T09:36:40.9188144Z == clock drift check ==
2020-04-21T09:36:40.9201409Z   local time: Tue Apr 21 09:36:40 UTC 2020
2020-04-21T09:36:41.2285338Z   network time: Tue, 21 Apr 2020 09:36:41 GMT
2020-04-21T09:36:42.8176746Z 
2020-04-21T09:36:42.8176746Z 
2020-04-21T09:36:42.8245324Z ##[error]Bash exited with code '1'.
2020-04-21T09:36:42.8258556Z ##[section]Finishing: Run build
2020-04-21T09:36:42.8309815Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71370/merge to s
2020-04-21T09:36:42.8314604Z Task         : Get sources
2020-04-21T09:36:42.8314937Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-21T09:36:42.8315218Z Version      : 1.0.0
2020-04-21T09:36:42.8315419Z Author       : Microsoft
2020-04-21T09:36:42.8315419Z Author       : Microsoft
2020-04-21T09:36:42.8315754Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-21T09:36:42.8316113Z ==============================================================================
2020-04-21T09:36:43.1812562Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-21T09:36:43.1864938Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71370/merge to s
2020-04-21T09:36:43.1958624Z Cleaning up task key
2020-04-21T09:36:43.1959867Z Start cleaning up orphan processes.
2020-04-21T09:36:43.2192634Z Terminate orphan process: pid (3619) (python)
2020-04-21T09:36:43.2439096Z ##[section]Finishing: Finalize Job
