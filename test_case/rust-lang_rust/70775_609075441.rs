plain
2020-04-04T17:23:31.6609079Z ========================== Starting Command Output ===========================
2020-04-04T17:23:31.6612327Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ed0c409f-4b59-4d09-ac19-823abc4637e3.sh
2020-04-04T17:23:31.6612623Z 
2020-04-04T17:23:31.6616519Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-04T17:23:31.6638060Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70775/merge to s
2020-04-04T17:23:31.6642239Z Task         : Get sources
2020-04-04T17:23:31.6642579Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T17:23:31.6642925Z Version      : 1.0.0
2020-04-04T17:23:31.6643144Z Author       : Microsoft
---
2020-04-04T17:23:34.2805112Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-04T17:23:34.3014755Z ##[command]git config gc.auto 0
2020-04-04T17:23:34.3059339Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-04T17:23:34.3091707Z ##[command]git config --get-all http.proxy
2020-04-04T17:23:34.9283700Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70775/merge:refs/remotes/pull/70775/merge
---
2020-04-04T17:27:21.8070845Z  ---> 3fc1b512c57b
2020-04-04T17:27:21.8071119Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-04T17:27:21.8075452Z  ---> Using cache
2020-04-04T17:27:21.8075873Z  ---> 5ee4295733f4
2020-04-04T17:27:21.8077833Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-04T17:27:21.8079223Z  ---> 3d07a0fa42fe
2020-04-04T17:27:21.8082335Z Successfully built 3d07a0fa42fe
2020-04-04T17:27:21.8111829Z Successfully tagged rust-ci:latest
2020-04-04T17:27:21.9158393Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T17:27:21.9158393Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T17:27:21.9172330Z Looks like docker image is the same as before, not uploading
2020-04-04T17:27:28.4917540Z [CI_JOB_NAME=mingw-check]
2020-04-04T17:27:28.5218141Z [CI_JOB_NAME=mingw-check]
2020-04-04T17:27:28.5247188Z == clock drift check ==
2020-04-04T17:27:28.5254977Z   local time: Sat Apr  4 17:27:28 UTC 2020
2020-04-04T17:27:28.5390497Z   network time: Sat, 04 Apr 2020 17:27:28 GMT
2020-04-04T17:27:28.5412087Z Starting sccache server...
2020-04-04T17:27:28.6273149Z configure: processing command line
2020-04-04T17:27:28.6274016Z configure: 
2020-04-04T17:27:28.6275010Z configure: rust.parallel-compiler := True
---
2020-04-04T17:30:48.6088015Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T17:30:48.7233707Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T17:30:48.9203619Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-04T17:30:49.0096237Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T17:30:49.4898728Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T17:30:51.6298262Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T17:30:52.0843713Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-04T17:30:53.9660198Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-04T17:30:54.3813788Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-04T17:32:34.2633838Z configure: build.locked-deps    := True
2020-04-04T17:32:34.2634134Z configure: llvm.ccache          := sccache
2020-04-04T17:32:34.2634626Z configure: build.cargo-native-static := True
2020-04-04T17:32:34.2635091Z configure: dist.missing-tools   := True
2020-04-04T17:32:34.2635689Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-04T17:32:34.2636255Z configure: writing `config.toml` in current directory
2020-04-04T17:32:34.2636492Z configure: 
2020-04-04T17:32:34.2636907Z configure: run `python /checkout/x.py --help`
2020-04-04T17:32:34.2637134Z configure: 
---
2020-04-04T17:33:51.8287034Z Hugepagesize:       2048 kB
2020-04-04T17:33:51.8287269Z DirectMap4k:      153536 kB
2020-04-04T17:33:51.8287489Z DirectMap2M:     4040704 kB
2020-04-04T17:33:51.8287709Z DirectMap1G:     5242880 kB
2020-04-04T17:33:51.8309846Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-04T17:33:53.1593476Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T17:33:53.1593476Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T17:33:53.1599999Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-04T17:33:53.3946922Z    Compiling unicode-xid v0.2.0
2020-04-04T17:33:53.5228194Z    Compiling syn v1.0.11
2020-04-04T17:33:54.3129034Z    Compiling linked-hash-map v0.5.2
2020-04-04T17:33:54.3690380Z    Compiling lazy_static v1.4.0
2020-04-04T17:33:54.3690380Z    Compiling lazy_static v1.4.0
2020-04-04T17:33:54.5318978Z    Compiling yaml-rust v0.4.3
2020-04-04T17:33:58.6834877Z    Compiling quote v1.0.2
2020-04-04T17:34:12.4995298Z    Compiling thiserror-impl v1.0.5
2020-04-04T17:34:16.9901614Z    Compiling thiserror v1.0.5
2020-04-04T17:34:17.0517724Z    Compiling yaml-merge-keys v0.4.0
2020-04-04T17:34:18.1650503Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-04T17:34:21.4481370Z Build completed successfully in 0:00:29
2020-04-04T17:34:21.4502450Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-04T17:34:21.6869982Z     Finished dev [unoptimized] target(s) in 0.17s
2020-04-04T17:34:22.7782571Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-04T17:36:24.9779454Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T17:36:25.1926896Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T17:36:25.3891341Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-04T17:36:25.3930668Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T17:36:26.0024153Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T17:36:28.2035108Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T17:36:28.6767551Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-04T17:36:30.7175278Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-04T17:36:31.1627025Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-04T17:40:32.0040174Z Build completed successfully in 0:00:41
2020-04-04T17:40:32.0047253Z + /scripts/validate-toolstate.sh
2020-04-04T17:40:32.0099201Z Cloning into 'rust-toolstate'...
2020-04-04T17:40:32.6665592Z Traceback (most recent call last):
2020-04-04T17:40:32.6666254Z   File "../../src/tools/publish_toolstate.py", line 305, in <module>
2020-04-04T17:40:32.6666530Z     cur_datetime
2020-04-04T17:40:32.6666808Z   File "../../src/tools/publish_toolstate.py", line 205, in update_latest
2020-04-04T17:40:32.6667898Z     maintainers = ' '.join('@'+name for name in MAINTAINERS[tool])
2020-04-04T17:40:32.6668378Z KeyError: u'rustc-dev-guide'
2020-04-04T17:40:32.6730281Z   local time: Sat Apr  4 17:40:32 UTC 2020
2020-04-04T17:40:32.6730281Z   local time: Sat Apr  4 17:40:32 UTC 2020
2020-04-04T17:40:32.6966892Z   network time: Sat, 04 Apr 2020 17:40:32 GMT
2020-04-04T17:40:34.2165701Z 
2020-04-04T17:40:34.2165701Z 
2020-04-04T17:40:34.2236608Z ##[error]Bash exited with code '1'.
2020-04-04T17:40:34.2250295Z ##[section]Finishing: Run build
2020-04-04T17:40:34.2299572Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70775/merge to s
2020-04-04T17:40:34.2305199Z Task         : Get sources
2020-04-04T17:40:34.2305569Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T17:40:34.2305905Z Version      : 1.0.0
2020-04-04T17:40:34.2306132Z Author       : Microsoft
2020-04-04T17:40:34.2306132Z Author       : Microsoft
2020-04-04T17:40:34.2306490Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-04T17:40:34.2306915Z ==============================================================================
2020-04-04T17:40:34.5698865Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-04T17:40:34.5750234Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70775/merge to s
2020-04-04T17:40:34.5837969Z Cleaning up task key
2020-04-04T17:40:34.5839215Z Start cleaning up orphan processes.
2020-04-04T17:40:34.6011213Z Terminate orphan process: pid (5418) (python)
2020-04-04T17:40:34.6338519Z ##[section]Finishing: Finalize Job
