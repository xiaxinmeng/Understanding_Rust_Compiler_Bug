plain
2020-04-04T20:27:27.5960487Z ========================== Starting Command Output ===========================
2020-04-04T20:27:27.5964869Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fd50a65b-84b2-4509-8862-aaefa9012264.sh
2020-04-04T20:27:27.5965357Z 
2020-04-04T20:27:27.5971818Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-04T20:27:27.6005929Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70721/merge to s
2020-04-04T20:27:27.6012092Z Task         : Get sources
2020-04-04T20:27:27.6012669Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T20:27:27.6013202Z Version      : 1.0.0
2020-04-04T20:27:27.6013567Z Author       : Microsoft
---
2020-04-04T20:27:28.6095929Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-04T20:27:28.6101829Z ##[command]git config gc.auto 0
2020-04-04T20:27:28.6106218Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-04T20:27:28.6110595Z ##[command]git config --get-all http.proxy
2020-04-04T20:27:28.6117769Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70721/merge:refs/remotes/pull/70721/merge
---
2020-04-04T20:30:03.5831743Z  ---> 3fc1b512c57b
2020-04-04T20:30:03.5832006Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-04T20:30:03.5832394Z  ---> Using cache
2020-04-04T20:30:03.5832731Z  ---> 5ee4295733f4
2020-04-04T20:30:03.5834163Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-04T20:30:03.5835544Z  ---> 3d07a0fa42fe
2020-04-04T20:30:03.5835776Z Successfully built 3d07a0fa42fe
2020-04-04T20:30:03.5860548Z Successfully tagged rust-ci:latest
2020-04-04T20:30:03.6175714Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T20:30:03.6175714Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T20:30:03.6191805Z Looks like docker image is the same as before, not uploading
2020-04-04T20:30:09.9345604Z [CI_JOB_NAME=mingw-check]
2020-04-04T20:30:09.9564481Z [CI_JOB_NAME=mingw-check]
2020-04-04T20:30:09.9590621Z == clock drift check ==
2020-04-04T20:30:09.9600902Z   local time: Sat Apr  4 20:30:09 UTC 2020
2020-04-04T20:30:10.0569468Z   network time: Sat, 04 Apr 2020 20:30:10 GMT
2020-04-04T20:30:10.0604331Z Starting sccache server...
2020-04-04T20:30:10.1456363Z configure: processing command line
2020-04-04T20:30:10.1456701Z configure: 
2020-04-04T20:30:10.1457708Z configure: rust.parallel-compiler := True
---
2020-04-04T20:33:24.8370667Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T20:33:25.0274047Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T20:33:25.2263470Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-04T20:33:25.2371186Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T20:33:25.8609463Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T20:33:28.1368968Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T20:33:28.6180638Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-04T20:33:30.7306912Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-04T20:33:31.5192752Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-04T20:35:19.2290358Z configure: build.locked-deps    := True
2020-04-04T20:35:19.2290665Z configure: llvm.ccache          := sccache
2020-04-04T20:35:19.2291289Z configure: build.cargo-native-static := True
2020-04-04T20:35:19.2291911Z configure: dist.missing-tools   := True
2020-04-04T20:35:19.2292912Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-04T20:35:19.2293630Z configure: writing `config.toml` in current directory
2020-04-04T20:35:19.2293897Z configure: 
2020-04-04T20:35:19.2294402Z configure: run `python /checkout/x.py --help`
2020-04-04T20:35:19.2294748Z configure: 
---
2020-04-04T20:36:40.6617477Z Hugepagesize:       2048 kB
2020-04-04T20:36:40.6617724Z DirectMap4k:      133056 kB
2020-04-04T20:36:40.6617952Z DirectMap2M:     4061184 kB
2020-04-04T20:36:40.6618178Z DirectMap1G:     5242880 kB
2020-04-04T20:36:40.6641151Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-04T20:36:41.9285648Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T20:36:41.9285648Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T20:36:41.9289926Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-04T20:36:42.1520019Z    Compiling unicode-xid v0.2.0
2020-04-04T20:36:42.2726400Z    Compiling syn v1.0.11
2020-04-04T20:36:43.0489888Z    Compiling linked-hash-map v0.5.2
2020-04-04T20:36:43.0960177Z    Compiling lazy_static v1.4.0
2020-04-04T20:36:43.0960177Z    Compiling lazy_static v1.4.0
2020-04-04T20:36:43.2642848Z    Compiling yaml-rust v0.4.3
2020-04-04T20:36:47.2927818Z    Compiling quote v1.0.2
2020-04-04T20:37:00.6010652Z    Compiling thiserror-impl v1.0.5
2020-04-04T20:37:04.9646118Z    Compiling thiserror v1.0.5
2020-04-04T20:37:05.0284340Z    Compiling yaml-merge-keys v0.4.0
2020-04-04T20:37:06.1792836Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-04T20:37:09.6864482Z Build completed successfully in 0:00:28
2020-04-04T20:37:09.6871126Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-04T20:37:09.9164767Z     Finished dev [unoptimized] target(s) in 0.17s
2020-04-04T20:37:10.9959702Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-04T20:39:05.9167354Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T20:39:06.0071689Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T20:39:06.2041483Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-04T20:39:06.3173938Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T20:39:06.7845516Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T20:39:09.4510867Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T20:39:09.9780167Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-04T20:39:11.9742074Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-04T20:39:12.3892069Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-04T20:43:00.2717427Z Done!
2020-04-04T20:43:00.2718143Z some tidy checks failed
2020-04-04T20:43:00.2722973Z 
2020-04-04T20:43:00.2723297Z 
2020-04-04T20:43:00.2724718Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-04T20:43:00.2726191Z 
2020-04-04T20:43:00.2726392Z 
2020-04-04T20:43:00.2741064Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-04T20:43:00.2741395Z Build completed unsuccessfully in 0:00:31
2020-04-04T20:43:00.2741395Z Build completed unsuccessfully in 0:00:31
2020-04-04T20:43:00.2817733Z == clock drift check ==
2020-04-04T20:43:00.2831052Z   local time: Sat Apr  4 20:43:00 UTC 2020
2020-04-04T20:43:00.4058584Z   network time: Sat, 04 Apr 2020 20:43:00 GMT
2020-04-04T20:43:02.0714785Z 
2020-04-04T20:43:02.0714785Z 
2020-04-04T20:43:02.0772394Z ##[error]Bash exited with code '1'.
2020-04-04T20:43:02.0786352Z ##[section]Finishing: Run build
2020-04-04T20:43:02.0831154Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70721/merge to s
2020-04-04T20:43:02.0835021Z Task         : Get sources
2020-04-04T20:43:02.0835272Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T20:43:02.0835501Z Version      : 1.0.0
2020-04-04T20:43:02.0835675Z Author       : Microsoft
2020-04-04T20:43:02.0835675Z Author       : Microsoft
2020-04-04T20:43:02.0835928Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-04T20:43:02.0836220Z ==============================================================================
2020-04-04T20:43:02.4011312Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-04T20:43:02.4056177Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70721/merge to s
2020-04-04T20:43:02.4141531Z Cleaning up task key
2020-04-04T20:43:02.4142868Z Start cleaning up orphan processes.
2020-04-04T20:43:02.4323140Z Terminate orphan process: pid (3984) (python)
2020-04-04T20:43:02.4455992Z ##[section]Finishing: Finalize Job
