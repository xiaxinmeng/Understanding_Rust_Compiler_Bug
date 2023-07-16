plain
2020-04-05T16:48:34.2669605Z ========================== Starting Command Output ===========================
2020-04-05T16:48:34.2673755Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/88112c51-70b0-437b-b765-9c3144babf82.sh
2020-04-05T16:48:34.2674065Z 
2020-04-05T16:48:34.2677837Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-05T16:48:34.2698996Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70817/merge to s
2020-04-05T16:48:34.2702810Z Task         : Get sources
2020-04-05T16:48:34.2703134Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-05T16:48:34.2703471Z Version      : 1.0.0
2020-04-05T16:48:34.2703688Z Author       : Microsoft
---
2020-04-05T16:48:35.2690965Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-05T16:48:35.2696716Z ##[command]git config gc.auto 0
2020-04-05T16:48:35.2701150Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-05T16:48:35.2704952Z ##[command]git config --get-all http.proxy
2020-04-05T16:48:35.2712487Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70817/merge:refs/remotes/pull/70817/merge
---
2020-04-05T16:51:45.3205556Z  ---> 3fc1b512c57b
2020-04-05T16:51:45.3205937Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-05T16:51:45.3206491Z  ---> Using cache
2020-04-05T16:51:45.3207001Z  ---> 5ee4295733f4
2020-04-05T16:51:45.3208546Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-05T16:51:45.3210332Z  ---> 3d07a0fa42fe
2020-04-05T16:51:45.3211331Z Successfully built 3d07a0fa42fe
2020-04-05T16:51:45.3212891Z Successfully tagged rust-ci:latest
2020-04-05T16:51:45.3213276Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-05T16:51:45.3213276Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-05T16:51:45.3213687Z Looks like docker image is the same as before, not uploading
2020-04-05T16:51:54.0952534Z [CI_JOB_NAME=mingw-check]
2020-04-05T16:51:54.1270719Z [CI_JOB_NAME=mingw-check]
2020-04-05T16:51:54.1306359Z == clock drift check ==
2020-04-05T16:51:54.1318354Z   local time: Sun Apr  5 16:51:54 UTC 2020
2020-04-05T16:51:54.2296372Z   network time: Sun, 05 Apr 2020 16:51:54 GMT
2020-04-05T16:51:54.2321858Z Starting sccache server...
2020-04-05T16:51:54.3212977Z configure: processing command line
2020-04-05T16:51:54.3213508Z configure: 
2020-04-05T16:51:54.3214516Z configure: rust.parallel-compiler := True
---
2020-04-05T16:55:54.5284961Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-05T16:55:54.5490756Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-05T16:55:54.7633837Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-05T16:55:54.9669964Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-05T16:55:55.4892544Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-05T16:55:58.1470152Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-05T16:55:58.7666974Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-05T16:56:01.0816472Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-05T16:56:01.5810980Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-05T16:57:59.8054061Z configure: build.locked-deps    := True
2020-04-05T16:57:59.8055097Z configure: llvm.ccache          := sccache
2020-04-05T16:57:59.8055757Z configure: build.cargo-native-static := True
2020-04-05T16:57:59.8056290Z configure: dist.missing-tools   := True
2020-04-05T16:57:59.8056964Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-05T16:57:59.8057781Z configure: writing `config.toml` in current directory
2020-04-05T16:57:59.8058060Z configure: 
2020-04-05T16:57:59.8058563Z configure: run `python /checkout/x.py --help`
2020-04-05T16:57:59.8058816Z configure: 
---
2020-04-05T16:59:31.2930543Z Hugepagesize:       2048 kB
2020-04-05T16:59:31.2930809Z DirectMap4k:      141248 kB
2020-04-05T16:59:31.2931053Z DirectMap2M:     4052992 kB
2020-04-05T16:59:31.2931297Z DirectMap1G:     5242880 kB
2020-04-05T16:59:31.2947362Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-05T16:59:32.7735964Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-05T16:59:32.7735964Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-05T16:59:32.7743186Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-05T16:59:33.0341293Z    Compiling unicode-xid v0.2.0
2020-04-05T16:59:33.1839445Z    Compiling syn v1.0.11
2020-04-05T16:59:34.1290675Z    Compiling linked-hash-map v0.5.2
2020-04-05T16:59:34.1731099Z    Compiling lazy_static v1.4.0
2020-04-05T16:59:34.1731099Z    Compiling lazy_static v1.4.0
2020-04-05T16:59:34.3782207Z    Compiling yaml-rust v0.4.3
2020-04-05T16:59:39.2532639Z    Compiling quote v1.0.2
2020-04-05T16:59:55.9196304Z    Compiling thiserror-impl v1.0.5
2020-04-05T17:00:01.3400637Z    Compiling thiserror v1.0.5
2020-04-05T17:00:01.4107101Z    Compiling yaml-merge-keys v0.4.0
2020-04-05T17:00:02.7553518Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-05T17:00:04.5723209Z Build completed successfully in 0:00:33
2020-04-05T17:00:04.5734250Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-05T17:00:04.8365250Z     Finished dev [unoptimized] target(s) in 0.19s
2020-04-05T17:00:06.0547399Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-05T17:02:28.8410494Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-05T17:02:28.8432771Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-05T17:02:29.0736855Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-05T17:02:29.3442237Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-05T17:02:29.7716141Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-05T17:02:32.5379639Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-05T17:02:33.1285849Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-05T17:02:35.4867688Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-05T17:02:35.9702557Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-05T17:07:06.5585832Z Found 0 error codes with no tests
2020-04-05T17:07:06.5586013Z Done!
2020-04-05T17:07:06.5590959Z 
2020-04-05T17:07:06.5591143Z 
2020-04-05T17:07:06.5592566Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-05T17:07:06.5593212Z 
2020-04-05T17:07:06.5593294Z 
2020-04-05T17:07:06.5604528Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-05T17:07:06.5604845Z Build completed unsuccessfully in 0:00:38
2020-04-05T17:07:06.5604845Z Build completed unsuccessfully in 0:00:38
2020-04-05T17:07:06.5653387Z == clock drift check ==
2020-04-05T17:07:06.5667237Z   local time: Sun Apr  5 17:07:06 UTC 2020
2020-04-05T17:07:06.6618501Z   network time: Sun, 05 Apr 2020 17:07:06 GMT
2020-04-05T17:07:08.2391600Z 
2020-04-05T17:07:08.2391600Z 
2020-04-05T17:07:08.2468577Z ##[error]Bash exited with code '1'.
2020-04-05T17:07:08.2485790Z ##[section]Finishing: Run build
2020-04-05T17:07:08.2542344Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70817/merge to s
2020-04-05T17:07:08.2547647Z Task         : Get sources
2020-04-05T17:07:08.2548056Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-05T17:07:08.2548432Z Version      : 1.0.0
2020-04-05T17:07:08.2548682Z Author       : Microsoft
2020-04-05T17:07:08.2548682Z Author       : Microsoft
2020-04-05T17:07:08.2549075Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-05T17:07:08.2549552Z ==============================================================================
2020-04-05T17:07:08.6061580Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-05T17:07:08.6104686Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70817/merge to s
2020-04-05T17:07:08.6216338Z Cleaning up task key
2020-04-05T17:07:08.6217888Z Start cleaning up orphan processes.
2020-04-05T17:07:08.6463005Z Terminate orphan process: pid (3576) (python)
2020-04-05T17:07:08.6693985Z ##[section]Finishing: Finalize Job
