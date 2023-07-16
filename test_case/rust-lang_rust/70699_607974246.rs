plain
2020-04-02T15:23:30.4905589Z ========================== Starting Command Output ===========================
2020-04-02T15:23:30.4908012Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/78ec251d-104c-46fc-adb7-8523cbd97935.sh
2020-04-02T15:23:30.4908286Z 
2020-04-02T15:23:30.4911939Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-02T15:23:30.4931462Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70699/merge to s
2020-04-02T15:23:30.4934678Z Task         : Get sources
2020-04-02T15:23:30.4934987Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-02T15:23:30.4935282Z Version      : 1.0.0
2020-04-02T15:23:30.4935497Z Author       : Microsoft
---
2020-04-02T15:23:31.7682655Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-02T15:23:31.7693073Z ##[command]git config gc.auto 0
2020-04-02T15:23:31.7711991Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-02T15:23:31.7716757Z ##[command]git config --get-all http.proxy
2020-04-02T15:23:31.7726692Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70699/merge:refs/remotes/pull/70699/merge
---
2020-04-02T15:25:40.7310686Z  ---> 3fc1b512c57b
2020-04-02T15:25:40.7311130Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-02T15:25:40.7313463Z  ---> Using cache
2020-04-02T15:25:40.7314151Z  ---> 5ee4295733f4
2020-04-02T15:25:40.7315784Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-02T15:25:40.7319045Z  ---> 3d07a0fa42fe
2020-04-02T15:25:40.7351425Z Successfully built 3d07a0fa42fe
2020-04-02T15:25:40.7372858Z Successfully tagged rust-ci:latest
2020-04-02T15:25:40.7664701Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-02T15:25:40.7664701Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-02T15:25:40.7679236Z Looks like docker image is the same as before, not uploading
2020-04-02T15:25:47.5006885Z [CI_JOB_NAME=mingw-check]
2020-04-02T15:25:47.5207080Z [CI_JOB_NAME=mingw-check]
2020-04-02T15:25:47.5234752Z == clock drift check ==
2020-04-02T15:25:47.5245988Z   local time: Thu Apr  2 15:25:47 UTC 2020
2020-04-02T15:25:48.3014228Z   network time: Thu, 02 Apr 2020 15:25:47 GMT
2020-04-02T15:25:48.3016364Z Starting sccache server...
2020-04-02T15:25:48.3016997Z configure: processing command line
2020-04-02T15:25:48.3017565Z configure: 
2020-04-02T15:25:48.3019052Z configure: rust.parallel-compiler := True
---
2020-04-02T15:29:14.5333838Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-02T15:29:14.6494823Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-02T15:29:14.8311579Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-02T15:29:14.9288719Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-02T15:29:15.4098728Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-02T15:29:17.5824071Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-02T15:29:18.0346818Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-02T15:29:19.9336741Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-02T15:29:20.3390372Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-02T15:31:03.4659094Z configure: build.locked-deps    := True
2020-04-02T15:31:03.4659522Z configure: llvm.ccache          := sccache
2020-04-02T15:31:03.4660113Z configure: build.cargo-native-static := True
2020-04-02T15:31:03.4660724Z configure: dist.missing-tools   := True
2020-04-02T15:31:03.4661560Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-02T15:31:03.4662357Z configure: writing `config.toml` in current directory
2020-04-02T15:31:03.4663632Z configure: 
2020-04-02T15:31:03.4664242Z configure: run `python /checkout/x.py --help`
2020-04-02T15:31:03.4664624Z configure: 
---
2020-04-02T15:32:25.2615767Z Hugepagesize:       2048 kB
2020-04-02T15:32:25.2615996Z DirectMap4k:      157632 kB
2020-04-02T15:32:25.2616216Z DirectMap2M:     4036608 kB
2020-04-02T15:32:25.2616433Z DirectMap1G:     5242880 kB
2020-04-02T15:32:25.2633414Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-02T15:32:26.5735258Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-02T15:32:26.5735258Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-02T15:32:26.5743805Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-02T15:32:26.8014482Z    Compiling unicode-xid v0.2.0
2020-04-02T15:32:26.9227708Z    Compiling syn v1.0.11
2020-04-02T15:32:27.7238243Z    Compiling linked-hash-map v0.5.2
2020-04-02T15:32:27.7600643Z    Compiling lazy_static v1.4.0
2020-04-02T15:32:27.7600643Z    Compiling lazy_static v1.4.0
2020-04-02T15:32:27.9520272Z    Compiling yaml-rust v0.4.3
2020-04-02T15:32:32.0524005Z    Compiling quote v1.0.2
2020-04-02T15:32:45.6552753Z    Compiling thiserror-impl v1.0.5
2020-04-02T15:32:51.9392965Z    Compiling thiserror v1.0.5
2020-04-02T15:32:51.9983448Z    Compiling yaml-merge-keys v0.4.0
2020-04-02T15:32:53.1035489Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-02T15:32:56.2804533Z Build completed successfully in 0:00:30
2020-04-02T15:32:56.2812688Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-02T15:32:56.5154626Z     Finished dev [unoptimized] target(s) in 0.17s
2020-04-02T15:32:57.5754116Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-02T15:34:55.7540072Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-02T15:34:55.9551706Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-02T15:34:56.1399038Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-02T15:34:56.1525056Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-02T15:34:56.7158783Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-02T15:34:58.8135217Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-02T15:34:59.2710726Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-02T15:35:01.2093330Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-02T15:35:01.6767614Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-02T15:38:47.5343893Z Found 0 error codes with no tests
2020-04-02T15:38:47.5344252Z Done!
2020-04-02T15:38:47.5344386Z 
2020-04-02T15:38:47.5344693Z 
2020-04-02T15:38:47.5346298Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-02T15:38:47.5347182Z 
2020-04-02T15:38:47.5347300Z 
2020-04-02T15:38:47.5356636Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-02T15:38:47.5357025Z Build completed unsuccessfully in 0:00:32
2020-04-02T15:38:47.5357025Z Build completed unsuccessfully in 0:00:32
2020-04-02T15:38:47.5407143Z == clock drift check ==
2020-04-02T15:38:47.5421320Z   local time: Thu Apr  2 15:38:47 UTC 2020
2020-04-02T15:38:47.6093233Z   network time: Thu, 02 Apr 2020 15:38:47 GMT
2020-04-02T15:38:49.6430797Z 
2020-04-02T15:38:49.6430797Z 
2020-04-02T15:38:49.6506686Z ##[error]Bash exited with code '1'.
2020-04-02T15:38:49.6520730Z ##[section]Finishing: Run build
2020-04-02T15:38:49.6565511Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70699/merge to s
2020-04-02T15:38:49.6570246Z Task         : Get sources
2020-04-02T15:38:49.6570592Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-02T15:38:49.6570921Z Version      : 1.0.0
2020-04-02T15:38:49.6571141Z Author       : Microsoft
2020-04-02T15:38:49.6571141Z Author       : Microsoft
2020-04-02T15:38:49.6571485Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-02T15:38:49.6571899Z ==============================================================================
2020-04-02T15:38:49.9811650Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-02T15:38:49.9866577Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70699/merge to s
2020-04-02T15:38:49.9958650Z Cleaning up task key
2020-04-02T15:38:49.9959948Z Start cleaning up orphan processes.
2020-04-02T15:38:50.0152027Z Terminate orphan process: pid (3343) (python)
2020-04-02T15:38:50.0330381Z ##[section]Finishing: Finalize Job
