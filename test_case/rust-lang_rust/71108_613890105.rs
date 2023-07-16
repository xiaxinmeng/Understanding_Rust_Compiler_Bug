plain
2020-04-15T06:10:23.9390222Z ========================== Starting Command Output ===========================
2020-04-15T06:10:23.9396311Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5143e1db-5ab0-4ff3-be60-f4718193e833.sh
2020-04-15T06:10:23.9396585Z 
2020-04-15T06:10:23.9400946Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-15T06:10:23.9425199Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71108/merge to s
2020-04-15T06:10:23.9434845Z Task         : Get sources
2020-04-15T06:10:23.9435710Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-15T06:10:23.9436025Z Version      : 1.0.0
2020-04-15T06:10:23.9436223Z Author       : Microsoft
---
2020-04-15T06:10:24.9293265Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-15T06:10:24.9299057Z ##[command]git config gc.auto 0
2020-04-15T06:10:24.9303005Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-15T06:10:24.9306640Z ##[command]git config --get-all http.proxy
2020-04-15T06:10:24.9313818Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71108/merge:refs/remotes/pull/71108/merge
---
2020-04-15T06:12:44.0881811Z  ---> 78ad2f4d4aca
2020-04-15T06:12:44.0882075Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-15T06:12:44.0882475Z  ---> Using cache
2020-04-15T06:12:44.0882825Z  ---> 4d2dc61c4d00
2020-04-15T06:12:44.0884242Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-15T06:12:44.0885601Z  ---> 776b6266a8b7
2020-04-15T06:12:44.0885833Z Successfully built 776b6266a8b7
2020-04-15T06:12:44.0886286Z Successfully tagged rust-ci:latest
2020-04-15T06:12:44.1203244Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-15T06:12:44.1203244Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-15T06:12:44.1219217Z Looks like docker image is the same as before, not uploading
2020-04-15T06:12:51.7224027Z [CI_JOB_NAME=mingw-check]
2020-04-15T06:12:51.7460481Z [CI_JOB_NAME=mingw-check]
2020-04-15T06:12:51.7509646Z == clock drift check ==
2020-04-15T06:12:51.7514737Z   local time: Wed Apr 15 06:12:51 UTC 2020
2020-04-15T06:12:52.0807487Z   network time: Wed, 15 Apr 2020 06:12:52 GMT
2020-04-15T06:12:52.0829178Z Starting sccache server...
2020-04-15T06:12:52.2067478Z configure: processing command line
2020-04-15T06:12:52.2068539Z configure: 
2020-04-15T06:12:52.2069818Z configure: rust.parallel-compiler := True
---
2020-04-15T06:16:49.2314505Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-15T06:16:54.1911726Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-15T06:16:55.4817867Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-15T06:16:55.6262855Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-15T06:16:55.8520953Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-15T06:16:56.5997911Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-15T06:16:56.8387618Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-15T06:16:58.4341044Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-15T06:16:58.9348625Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-15T06:19:00.3290144Z configure: build.submodules     := False
2020-04-15T06:19:00.3291123Z configure: rust.dist-src        := False
2020-04-15T06:19:00.3291848Z configure: build.locked-deps    := True
2020-04-15T06:19:00.3292303Z configure: llvm.assertions      := True
2020-04-15T06:19:00.3293184Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-15T06:19:00.3294052Z configure: writing `config.toml` in current directory
2020-04-15T06:19:00.3294432Z configure: 
2020-04-15T06:19:00.3295034Z configure: run `python /checkout/x.py --help`
2020-04-15T06:19:00.3295424Z configure: 
---
2020-04-15T06:20:44.7185567Z Hugepagesize:       2048 kB
2020-04-15T06:20:44.7185772Z DirectMap4k:      131008 kB
2020-04-15T06:20:44.7185993Z DirectMap2M:     4063232 kB
2020-04-15T06:20:44.7186195Z DirectMap1G:     5242880 kB
2020-04-15T06:20:44.7234173Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-15T06:20:46.1853761Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-15T06:20:46.1853761Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-15T06:20:46.1861774Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-15T06:20:46.4257539Z    Compiling unicode-xid v0.2.0
2020-04-15T06:20:46.5499692Z    Compiling syn v1.0.11
2020-04-15T06:20:47.4961681Z    Compiling linked-hash-map v0.5.2
2020-04-15T06:20:47.5258676Z    Compiling lazy_static v1.4.0
2020-04-15T06:20:47.5258676Z    Compiling lazy_static v1.4.0
2020-04-15T06:20:47.7332086Z    Compiling yaml-rust v0.4.3
2020-04-15T06:20:52.4426769Z    Compiling quote v1.0.2
2020-04-15T06:21:08.0926445Z    Compiling thiserror-impl v1.0.5
2020-04-15T06:21:13.2754903Z    Compiling thiserror v1.0.5
2020-04-15T06:21:13.3338584Z    Compiling yaml-merge-keys v0.4.0
2020-04-15T06:21:15.0981595Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-15T06:21:17.8629293Z Build completed successfully in 0:00:33
2020-04-15T06:21:17.8726315Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-15T06:21:18.1574516Z     Finished dev [unoptimized] target(s) in 0.19s
2020-04-15T06:21:19.3128584Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-15T06:23:35.3620352Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-15T06:23:35.3911196Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-15T06:23:35.5978535Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-15T06:23:35.8087954Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-15T06:23:36.2753741Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-15T06:23:38.7435985Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-15T06:23:39.2539768Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-15T06:23:41.4218195Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-15T06:23:41.8906171Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-15T06:28:14.2239526Z Done!
2020-04-15T06:28:14.2239697Z some tidy checks failed
2020-04-15T06:28:14.2239865Z 
2020-04-15T06:28:14.2239968Z 
2020-04-15T06:28:14.2241270Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-15T06:28:14.2242033Z 
2020-04-15T06:28:14.2242135Z 
2020-04-15T06:28:14.2251036Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-15T06:28:14.2251455Z Build completed unsuccessfully in 0:00:39
2020-04-15T06:28:14.2251455Z Build completed unsuccessfully in 0:00:39
2020-04-15T06:28:14.2358307Z == clock drift check ==
2020-04-15T06:28:14.2374086Z   local time: Wed Apr 15 06:28:14 UTC 2020
2020-04-15T06:28:14.5434236Z   network time: Wed, 15 Apr 2020 06:28:14 GMT
2020-04-15T06:28:16.0315095Z 
2020-04-15T06:28:16.0315095Z 
2020-04-15T06:28:16.0373338Z ##[error]Bash exited with code '1'.
2020-04-15T06:28:16.0391913Z ##[section]Finishing: Run build
2020-04-15T06:28:16.0441622Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71108/merge to s
2020-04-15T06:28:16.0447322Z Task         : Get sources
2020-04-15T06:28:16.0447695Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-15T06:28:16.0448042Z Version      : 1.0.0
2020-04-15T06:28:16.0448279Z Author       : Microsoft
2020-04-15T06:28:16.0448279Z Author       : Microsoft
2020-04-15T06:28:16.0448647Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-15T06:28:16.0449080Z ==============================================================================
2020-04-15T06:28:16.4202617Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-15T06:28:16.4247276Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71108/merge to s
2020-04-15T06:28:16.4349824Z Cleaning up task key
2020-04-15T06:28:16.4351036Z Start cleaning up orphan processes.
2020-04-15T06:28:16.4568733Z Terminate orphan process: pid (3866) (python)
2020-04-15T06:28:16.4845557Z ##[section]Finishing: Finalize Job
