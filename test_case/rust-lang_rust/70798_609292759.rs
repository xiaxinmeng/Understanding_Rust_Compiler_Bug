plain
2020-04-05T02:48:55.2806615Z ========================== Starting Command Output ===========================
2020-04-05T02:48:55.2810599Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/70fde522-2437-4334-8f3d-e137791d805e.sh
2020-04-05T02:48:55.2810959Z 
2020-04-05T02:48:55.2815577Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-05T02:48:55.2831215Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70798/merge to s
2020-04-05T02:48:55.2834204Z Task         : Get sources
2020-04-05T02:48:55.2834486Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-05T02:48:55.2834780Z Version      : 1.0.0
2020-04-05T02:48:55.2834966Z Author       : Microsoft
---
2020-04-05T02:48:56.2790884Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-05T02:48:56.2795247Z ##[command]git config gc.auto 0
2020-04-05T02:48:56.2798164Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-05T02:48:56.2800870Z ##[command]git config --get-all http.proxy
2020-04-05T02:48:56.2806545Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70798/merge:refs/remotes/pull/70798/merge
---
2020-04-05T02:51:17.1779893Z  ---> 3fc1b512c57b
2020-04-05T02:51:17.1780080Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-05T02:51:17.1786280Z  ---> Using cache
2020-04-05T02:51:17.1786550Z  ---> 5ee4295733f4
2020-04-05T02:51:17.1787548Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-05T02:51:17.1788706Z  ---> 3d07a0fa42fe
2020-04-05T02:51:17.1803840Z Successfully built 3d07a0fa42fe
2020-04-05T02:51:17.1840331Z Successfully tagged rust-ci:latest
2020-04-05T02:51:17.8194448Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-05T02:51:17.8194448Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-05T02:51:17.8196199Z Looks like docker image is the same as before, not uploading
2020-04-05T02:51:23.9277570Z [CI_JOB_NAME=mingw-check]
2020-04-05T02:51:23.9471131Z [CI_JOB_NAME=mingw-check]
2020-04-05T02:51:23.9495248Z == clock drift check ==
2020-04-05T02:51:23.9501122Z   local time: Sun Apr  5 02:51:23 UTC 2020
2020-04-05T02:51:24.1352571Z   network time: Sun, 05 Apr 2020 02:51:24 GMT
2020-04-05T02:51:24.1374930Z Starting sccache server...
2020-04-05T02:51:24.2086806Z configure: processing command line
2020-04-05T02:51:24.2087364Z configure: 
2020-04-05T02:51:24.2088320Z configure: rust.parallel-compiler := True
---
2020-04-05T02:54:23.8490472Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-05T02:54:24.0848526Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-05T02:54:24.2178941Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-05T02:54:24.2437772Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-05T02:54:24.7608689Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-05T02:54:26.6166196Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-05T02:54:26.9976927Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-05T02:54:28.6432019Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-05T02:54:28.9906417Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-05T02:55:58.0219763Z configure: build.locked-deps    := True
2020-04-05T02:55:58.0220018Z configure: llvm.ccache          := sccache
2020-04-05T02:55:58.0220401Z configure: build.cargo-native-static := True
2020-04-05T02:55:58.0220777Z configure: dist.missing-tools   := True
2020-04-05T02:55:58.0221263Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-05T02:55:58.0221717Z configure: writing `config.toml` in current directory
2020-04-05T02:55:58.0221924Z configure: 
2020-04-05T02:55:58.0222246Z configure: run `python /checkout/x.py --help`
2020-04-05T02:55:58.0222433Z configure: 
---
2020-04-05T02:57:07.0278594Z Hugepagesize:       2048 kB
2020-04-05T02:57:07.0278880Z DirectMap4k:      143296 kB
2020-04-05T02:57:07.0279170Z DirectMap2M:     4050944 kB
2020-04-05T02:57:07.0279443Z DirectMap1G:     5242880 kB
2020-04-05T02:57:07.0335947Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-05T02:57:08.0764557Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-05T02:57:08.0764557Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-05T02:57:08.0769814Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-05T02:57:08.2677992Z    Compiling unicode-xid v0.2.0
2020-04-05T02:57:08.3735906Z    Compiling syn v1.0.11
2020-04-05T02:57:09.0891396Z    Compiling linked-hash-map v0.5.2
2020-04-05T02:57:09.1116770Z    Compiling lazy_static v1.4.0
2020-04-05T02:57:09.1116770Z    Compiling lazy_static v1.4.0
2020-04-05T02:57:09.2789342Z    Compiling yaml-rust v0.4.3
2020-04-05T02:57:12.9072403Z    Compiling quote v1.0.2
2020-04-05T02:57:24.7190407Z    Compiling thiserror-impl v1.0.5
2020-04-05T02:57:28.4488392Z    Compiling thiserror v1.0.5
2020-04-05T02:57:28.5021216Z    Compiling yaml-merge-keys v0.4.0
2020-04-05T02:57:29.4650263Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-05T02:57:30.8061499Z Build completed successfully in 0:00:23
2020-04-05T02:57:30.8070622Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-05T02:57:31.0125612Z     Finished dev [unoptimized] target(s) in 0.14s
2020-04-05T02:57:31.8953967Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-05T02:59:16.6324018Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-05T02:59:16.6406629Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-05T02:59:16.7978843Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-05T02:59:17.1921009Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-05T02:59:18.5164891Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-05T02:59:19.5584466Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-05T02:59:21.3622061Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-05T02:59:21.7432304Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-05T02:59:22.7952043Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
2020-04-05T03:02:36.2652495Z Done!
2020-04-05T03:02:36.2656079Z some tidy checks failed
2020-04-05T03:02:36.2659067Z 
2020-04-05T03:02:36.2659501Z 
2020-04-05T03:02:36.2661163Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-05T03:02:36.2662503Z 
2020-04-05T03:02:36.2662826Z 
2020-04-05T03:02:36.2665965Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-05T03:02:36.2666680Z Build completed unsuccessfully in 0:00:27
2020-04-05T03:02:36.2666680Z Build completed unsuccessfully in 0:00:27
2020-04-05T03:02:36.2701703Z == clock drift check ==
2020-04-05T03:02:36.2722193Z   local time: Sun Apr  5 03:02:36 UTC 2020
2020-04-05T03:02:36.6272592Z   network time: Sun, 05 Apr 2020 03:02:36 GMT
2020-04-05T03:02:38.3906044Z 
2020-04-05T03:02:38.3906044Z 
2020-04-05T03:02:38.3963597Z ##[error]Bash exited with code '1'.
2020-04-05T03:02:38.3975297Z ##[section]Finishing: Run build
2020-04-05T03:02:38.4015255Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70798/merge to s
2020-04-05T03:02:38.4021719Z Task         : Get sources
2020-04-05T03:02:38.4022077Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-05T03:02:38.4022419Z Version      : 1.0.0
2020-04-05T03:02:38.4022656Z Author       : Microsoft
2020-04-05T03:02:38.4022656Z Author       : Microsoft
2020-04-05T03:02:38.4023019Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-05T03:02:38.4023456Z ==============================================================================
2020-04-05T03:02:38.6896296Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-05T03:02:38.6942123Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70798/merge to s
2020-04-05T03:02:38.7023140Z Cleaning up task key
2020-04-05T03:02:38.7024148Z Start cleaning up orphan processes.
2020-04-05T03:02:38.7175497Z Terminate orphan process: pid (3549) (python)
2020-04-05T03:02:38.7347705Z ##[section]Finishing: Finalize Job
