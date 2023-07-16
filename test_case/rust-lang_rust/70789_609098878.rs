plain
2020-04-04T20:43:37.0993791Z ========================== Starting Command Output ===========================
2020-04-04T20:43:37.1001188Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c0233313-ccbe-4974-83bb-0298ba4f450f.sh
2020-04-04T20:43:37.1001968Z 
2020-04-04T20:43:37.1013962Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-04T20:43:37.1035870Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70789/merge to s
2020-04-04T20:43:37.1038352Z Task         : Get sources
2020-04-04T20:43:37.1038568Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T20:43:37.1038794Z Version      : 1.0.0
2020-04-04T20:43:37.1038937Z Author       : Microsoft
---
2020-04-04T20:43:38.0983655Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-04T20:43:38.0989848Z ##[command]git config gc.auto 0
2020-04-04T20:43:38.0994147Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-04T20:43:38.0998158Z ##[command]git config --get-all http.proxy
2020-04-04T20:43:38.1005794Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70789/merge:refs/remotes/pull/70789/merge
---
2020-04-04T20:45:47.0892858Z  ---> 3fc1b512c57b
2020-04-04T20:45:47.0893095Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-04T20:45:47.0893450Z  ---> Using cache
2020-04-04T20:45:47.0893777Z  ---> 5ee4295733f4
2020-04-04T20:45:47.0895287Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-04T20:45:47.0896771Z  ---> 3d07a0fa42fe
2020-04-04T20:45:47.0896972Z Successfully built 3d07a0fa42fe
2020-04-04T20:45:47.0912962Z Successfully tagged rust-ci:latest
2020-04-04T20:45:47.1137583Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T20:45:47.1137583Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T20:45:47.1149470Z Looks like docker image is the same as before, not uploading
2020-04-04T20:45:53.7915139Z [CI_JOB_NAME=mingw-check]
2020-04-04T20:45:53.8118439Z [CI_JOB_NAME=mingw-check]
2020-04-04T20:45:53.8135192Z == clock drift check ==
2020-04-04T20:45:53.8142689Z   local time: Sat Apr  4 20:45:53 UTC 2020
2020-04-04T20:45:53.8509780Z   network time: Sat, 04 Apr 2020 20:45:53 GMT
2020-04-04T20:45:53.8552999Z Starting sccache server...
2020-04-04T20:45:53.9298892Z configure: processing command line
2020-04-04T20:45:53.9299120Z configure: 
2020-04-04T20:45:53.9299829Z configure: rust.parallel-compiler := True
---
2020-04-04T20:48:46.8895813Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T20:48:46.9847706Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T20:48:47.1352841Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-04T20:48:47.2055883Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T20:48:47.5936593Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T20:48:49.2994602Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T20:48:49.6605328Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-04T20:48:51.2110123Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-04T20:48:51.5410070Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-04T20:50:12.4401884Z configure: build.locked-deps    := True
2020-04-04T20:50:12.4402227Z configure: llvm.ccache          := sccache
2020-04-04T20:50:12.4404015Z configure: build.cargo-native-static := True
2020-04-04T20:50:12.4404501Z configure: dist.missing-tools   := True
2020-04-04T20:50:12.4405168Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-04T20:50:12.4405933Z configure: writing `config.toml` in current directory
2020-04-04T20:50:12.4406218Z configure: 
2020-04-04T20:50:12.4406616Z configure: run `python /checkout/x.py --help`
2020-04-04T20:50:12.4407402Z configure: 
---
2020-04-04T20:51:21.6489132Z Hugepagesize:       2048 kB
2020-04-04T20:51:21.6489354Z DirectMap4k:      131008 kB
2020-04-04T20:51:21.6489559Z DirectMap2M:     3014656 kB
2020-04-04T20:51:21.6489764Z DirectMap1G:     6291456 kB
2020-04-04T20:51:21.6521220Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-04T20:51:22.7029469Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T20:51:22.7029469Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T20:51:22.7031477Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-04T20:51:22.8849336Z    Compiling unicode-xid v0.2.0
2020-04-04T20:51:22.9842557Z    Compiling syn v1.0.11
2020-04-04T20:51:23.6427846Z    Compiling linked-hash-map v0.5.2
2020-04-04T20:51:23.6676860Z    Compiling lazy_static v1.4.0
2020-04-04T20:51:23.6676860Z    Compiling lazy_static v1.4.0
2020-04-04T20:51:23.8127909Z    Compiling yaml-rust v0.4.3
2020-04-04T20:51:27.0474522Z    Compiling quote v1.0.2
2020-04-04T20:51:37.7109345Z    Compiling thiserror-impl v1.0.5
2020-04-04T20:51:41.1553522Z    Compiling thiserror v1.0.5
2020-04-04T20:51:41.2065361Z    Compiling yaml-merge-keys v0.4.0
2020-04-04T20:51:42.0730694Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-04T20:51:43.2922363Z Build completed successfully in 0:00:21
2020-04-04T20:51:43.2927729Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-04T20:51:43.4858948Z     Finished dev [unoptimized] target(s) in 0.14s
2020-04-04T20:51:44.3472759Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-04T20:53:21.2898652Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T20:53:21.3795384Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T20:53:21.5349229Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-04T20:53:21.6202240Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T20:53:22.0348384Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T20:53:23.7030670Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T20:53:24.0632716Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-04T20:53:25.6092095Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-04T20:53:25.9306458Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-04T20:56:39.3417620Z Build completed successfully in 0:00:40
2020-04-04T20:56:39.3426311Z + /scripts/validate-toolstate.sh
2020-04-04T20:56:39.3468468Z Cloning into 'rust-toolstate'...
2020-04-04T20:56:39.9543392Z Traceback (most recent call last):
2020-04-04T20:56:39.9543868Z   File "../../src/tools/publish_toolstate.py", line 305, in <module>
2020-04-04T20:56:39.9544233Z     cur_datetime
2020-04-04T20:56:39.9544513Z   File "../../src/tools/publish_toolstate.py", line 205, in update_latest
2020-04-04T20:56:39.9545540Z     maintainers = ' '.join('@'+name for name in MAINTAINERS[tool])
2020-04-04T20:56:39.9546005Z KeyError: u'rustc-dev-guide'
2020-04-04T20:56:39.9588356Z   local time: Sat Apr  4 20:56:39 UTC 2020
2020-04-04T20:56:39.9588356Z   local time: Sat Apr  4 20:56:39 UTC 2020
2020-04-04T20:56:40.0578082Z   network time: Sat, 04 Apr 2020 20:56:40 GMT
2020-04-04T20:56:41.3774752Z 
2020-04-04T20:56:41.3774752Z 
2020-04-04T20:56:41.3825390Z ##[error]Bash exited with code '1'.
2020-04-04T20:56:41.3836369Z ##[section]Finishing: Run build
2020-04-04T20:56:41.3869696Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70789/merge to s
2020-04-04T20:56:41.3873410Z Task         : Get sources
2020-04-04T20:56:41.3873656Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T20:56:41.3873893Z Version      : 1.0.0
2020-04-04T20:56:41.3874070Z Author       : Microsoft
2020-04-04T20:56:41.3874070Z Author       : Microsoft
2020-04-04T20:56:41.3874318Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-04T20:56:41.3874605Z ==============================================================================
2020-04-04T20:56:41.6461894Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-04T20:56:41.6509447Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70789/merge to s
2020-04-04T20:56:41.6571151Z Cleaning up task key
2020-04-04T20:56:41.6573134Z Start cleaning up orphan processes.
2020-04-04T20:56:41.6703857Z Terminate orphan process: pid (3583) (python)
2020-04-04T20:56:41.6866705Z ##[section]Finishing: Finalize Job
