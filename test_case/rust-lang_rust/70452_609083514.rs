plain
2020-04-04T18:40:29.5658614Z ========================== Starting Command Output ===========================
2020-04-04T18:40:29.5662157Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6417ccf7-2427-420c-899f-5a756bdff14a.sh
2020-04-04T18:40:29.5662590Z 
2020-04-04T18:40:29.5667671Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-04T18:40:29.5687624Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70452/merge to s
2020-04-04T18:40:29.5691692Z Task         : Get sources
2020-04-04T18:40:29.5692090Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T18:40:29.5692399Z Version      : 1.0.0
2020-04-04T18:40:29.5692609Z Author       : Microsoft
---
2020-04-04T18:40:30.5589746Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-04T18:40:30.5595590Z ##[command]git config gc.auto 0
2020-04-04T18:40:30.5599162Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-04T18:40:30.5602459Z ##[command]git config --get-all http.proxy
2020-04-04T18:40:30.5608632Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70452/merge:refs/remotes/pull/70452/merge
---
2020-04-04T18:42:50.4286635Z  ---> 3fc1b512c57b
2020-04-04T18:42:50.4290045Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-04T18:42:50.4292867Z  ---> Using cache
2020-04-04T18:42:50.4296004Z  ---> 5ee4295733f4
2020-04-04T18:42:50.4300574Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-04T18:42:50.4328391Z  ---> 3d07a0fa42fe
2020-04-04T18:42:50.4371358Z Successfully built 3d07a0fa42fe
2020-04-04T18:42:50.4372834Z Successfully tagged rust-ci:latest
2020-04-04T18:42:50.4790013Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T18:42:50.4790013Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T18:42:50.4808769Z Looks like docker image is the same as before, not uploading
2020-04-04T18:42:55.9578888Z [CI_JOB_NAME=mingw-check]
2020-04-04T18:42:55.9792070Z [CI_JOB_NAME=mingw-check]
2020-04-04T18:42:55.9813485Z == clock drift check ==
2020-04-04T18:42:55.9823130Z   local time: Sat Apr  4 18:42:55 UTC 2020
2020-04-04T18:42:56.1067127Z   network time: Sat, 04 Apr 2020 18:42:56 GMT
2020-04-04T18:42:56.1105081Z Starting sccache server...
2020-04-04T18:42:56.2064398Z configure: processing command line
2020-04-04T18:42:56.2065991Z configure: 
2020-04-04T18:42:56.2068201Z configure: rust.parallel-compiler := True
---
2020-04-04T18:46:38.7970215Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T18:46:38.9767732Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T18:46:39.2080384Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-04T18:46:39.2601987Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T18:46:39.8710711Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T18:46:42.3393756Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T18:46:42.8598307Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-04T18:46:45.0867908Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-04T18:46:45.5595472Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-04T18:48:40.9096325Z configure: build.locked-deps    := True
2020-04-04T18:48:40.9096644Z configure: llvm.ccache          := sccache
2020-04-04T18:48:40.9097112Z configure: build.cargo-native-static := True
2020-04-04T18:48:40.9097569Z configure: dist.missing-tools   := True
2020-04-04T18:48:40.9098168Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-04T18:48:40.9098945Z configure: writing `config.toml` in current directory
2020-04-04T18:48:40.9099206Z configure: 
2020-04-04T18:48:40.9099609Z configure: run `python /checkout/x.py --help`
2020-04-04T18:48:40.9099829Z configure: 
---
2020-04-04T18:50:07.8103767Z Hugepagesize:       2048 kB
2020-04-04T18:50:07.8103983Z DirectMap4k:      143296 kB
2020-04-04T18:50:07.8104199Z DirectMap2M:     4050944 kB
2020-04-04T18:50:07.8104412Z DirectMap1G:     5242880 kB
2020-04-04T18:50:07.8105660Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-04T18:50:09.2717354Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T18:50:09.2717354Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T18:50:09.2727184Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-04T18:50:09.5485398Z    Compiling unicode-xid v0.2.0
2020-04-04T18:50:09.6849266Z    Compiling syn v1.0.11
2020-04-04T18:50:10.5321373Z    Compiling linked-hash-map v0.5.2
2020-04-04T18:50:10.5739039Z    Compiling lazy_static v1.4.0
2020-04-04T18:50:10.5739039Z    Compiling lazy_static v1.4.0
2020-04-04T18:50:10.7845470Z    Compiling yaml-rust v0.4.3
2020-04-04T18:50:15.0962705Z    Compiling quote v1.0.2
2020-04-04T18:50:29.5426221Z    Compiling thiserror-impl v1.0.5
2020-04-04T18:50:34.1712467Z    Compiling thiserror v1.0.5
2020-04-04T18:50:34.2417398Z    Compiling yaml-merge-keys v0.4.0
2020-04-04T18:50:35.4571291Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-04T18:50:37.1595953Z Build completed successfully in 0:00:29
2020-04-04T18:50:37.1604023Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-04T18:50:37.4197347Z     Finished dev [unoptimized] target(s) in 0.19s
2020-04-04T18:50:38.6586377Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-04T18:52:48.0831262Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T18:52:48.2866619Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T18:52:48.5073411Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-04T18:52:48.5312023Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T18:52:49.1818939Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T18:52:51.6293325Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T18:52:52.1674289Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-04T18:52:54.4187799Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-04T18:52:54.8982899Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-04T18:57:09.7947242Z Build completed successfully in 0:00:41
2020-04-04T18:57:09.7955314Z + /scripts/validate-toolstate.sh
2020-04-04T18:57:09.8007724Z Cloning into 'rust-toolstate'...
2020-04-04T18:57:10.6123634Z Traceback (most recent call last):
2020-04-04T18:57:10.6124078Z   File "../../src/tools/publish_toolstate.py", line 305, in <module>
2020-04-04T18:57:10.6124491Z     cur_datetime
2020-04-04T18:57:10.6124816Z   File "../../src/tools/publish_toolstate.py", line 205, in update_latest
2020-04-04T18:57:10.6126194Z     maintainers = ' '.join('@'+name for name in MAINTAINERS[tool])
2020-04-04T18:57:10.6126772Z KeyError: u'rustc-dev-guide'
2020-04-04T18:57:10.6203229Z   local time: Sat Apr  4 18:57:10 UTC 2020
2020-04-04T18:57:10.6203229Z   local time: Sat Apr  4 18:57:10 UTC 2020
2020-04-04T18:57:10.7800468Z   network time: Sat, 04 Apr 2020 18:57:10 GMT
2020-04-04T18:57:12.1565666Z 
2020-04-04T18:57:12.1565666Z 
2020-04-04T18:57:12.1641114Z ##[error]Bash exited with code '1'.
2020-04-04T18:57:12.1656184Z ##[section]Finishing: Run build
2020-04-04T18:57:12.1702398Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70452/merge to s
2020-04-04T18:57:12.1707875Z Task         : Get sources
2020-04-04T18:57:12.1708243Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T18:57:12.1708591Z Version      : 1.0.0
2020-04-04T18:57:12.1708825Z Author       : Microsoft
2020-04-04T18:57:12.1708825Z Author       : Microsoft
2020-04-04T18:57:12.1709199Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-04T18:57:12.1709638Z ==============================================================================
2020-04-04T18:57:12.5070245Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-04T18:57:12.5121705Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70452/merge to s
2020-04-04T18:57:12.5219924Z Cleaning up task key
2020-04-04T18:57:12.5221448Z Start cleaning up orphan processes.
2020-04-04T18:57:12.5411438Z Terminate orphan process: pid (3965) (python)
2020-04-04T18:57:12.5623262Z ##[section]Finishing: Finalize Job
