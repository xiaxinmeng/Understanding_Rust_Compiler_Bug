plain
2020-04-04T18:31:59.2431590Z ========================== Starting Command Output ===========================
2020-04-04T18:31:59.2434914Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1fe0b2f0-ad1f-43c7-bbb7-4808fb228e05.sh
2020-04-04T18:31:59.2435195Z 
2020-04-04T18:31:59.2439898Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-04T18:31:59.2459730Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70783/merge to s
2020-04-04T18:31:59.2463418Z Task         : Get sources
2020-04-04T18:31:59.2463704Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T18:31:59.2464006Z Version      : 1.0.0
2020-04-04T18:31:59.2464194Z Author       : Microsoft
---
2020-04-04T18:32:00.2705179Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-04T18:32:00.2711133Z ##[command]git config gc.auto 0
2020-04-04T18:32:00.2715306Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-04T18:32:00.2719037Z ##[command]git config --get-all http.proxy
2020-04-04T18:32:00.2726079Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70783/merge:refs/remotes/pull/70783/merge
---
2020-04-04T18:34:19.0250929Z  ---> 3fc1b512c57b
2020-04-04T18:34:19.0251139Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-04T18:34:19.0256765Z  ---> Using cache
2020-04-04T18:34:19.0257160Z  ---> 5ee4295733f4
2020-04-04T18:34:19.0258416Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-04T18:34:19.0266912Z  ---> 3d07a0fa42fe
2020-04-04T18:34:19.0342545Z Successfully built 3d07a0fa42fe
2020-04-04T18:34:19.0395750Z Successfully tagged rust-ci:latest
2020-04-04T18:34:19.1101606Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T18:34:19.1101606Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T18:34:19.1115325Z Looks like docker image is the same as before, not uploading
2020-04-04T18:34:25.7124789Z [CI_JOB_NAME=mingw-check]
2020-04-04T18:34:25.7383832Z [CI_JOB_NAME=mingw-check]
2020-04-04T18:34:25.7408953Z == clock drift check ==
2020-04-04T18:34:25.7418022Z   local time: Sat Apr  4 18:34:25 UTC 2020
2020-04-04T18:34:26.0299310Z   network time: Sat, 04 Apr 2020 18:34:26 GMT
2020-04-04T18:34:26.0345649Z Starting sccache server...
2020-04-04T18:34:26.1416394Z configure: processing command line
2020-04-04T18:34:26.1416634Z configure: 
2020-04-04T18:34:26.1417548Z configure: rust.parallel-compiler := True
---
2020-04-04T18:38:04.8511780Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T18:38:05.0836747Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T18:38:05.2396932Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-04T18:38:05.2859932Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T18:38:05.8802157Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T18:38:08.2107564Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T18:38:08.6927317Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-04T18:38:10.7213326Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-04T18:38:11.1999821Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-04T18:40:01.7483192Z configure: build.locked-deps    := True
2020-04-04T18:40:01.7483747Z configure: llvm.ccache          := sccache
2020-04-04T18:40:01.7484476Z configure: build.cargo-native-static := True
2020-04-04T18:40:01.7485205Z configure: dist.missing-tools   := True
2020-04-04T18:40:01.7486119Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-04T18:40:01.7487087Z configure: writing `config.toml` in current directory
2020-04-04T18:40:01.7487490Z configure: 
2020-04-04T18:40:01.7488838Z configure: run `python /checkout/x.py --help`
2020-04-04T18:40:01.7489345Z configure: 
---
2020-04-04T18:41:25.8215557Z Hugepagesize:       2048 kB
2020-04-04T18:41:25.8215825Z DirectMap4k:      124864 kB
2020-04-04T18:41:25.8216073Z DirectMap2M:     3020800 kB
2020-04-04T18:41:25.8216322Z DirectMap1G:     6291456 kB
2020-04-04T18:41:25.8343416Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-04T18:41:27.2462839Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T18:41:27.2462839Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T18:41:27.2472205Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-04T18:41:27.4643551Z    Compiling unicode-xid v0.2.0
2020-04-04T18:41:27.6162989Z    Compiling syn v1.0.11
2020-04-04T18:41:28.4182916Z    Compiling linked-hash-map v0.5.2
2020-04-04T18:41:28.5421862Z    Compiling lazy_static v1.4.0
2020-04-04T18:41:28.5421862Z    Compiling lazy_static v1.4.0
2020-04-04T18:41:28.6801388Z    Compiling yaml-rust v0.4.3
2020-04-04T18:41:32.9861144Z    Compiling quote v1.0.2
2020-04-04T18:41:47.8930650Z    Compiling thiserror-impl v1.0.5
2020-04-04T18:41:52.8442241Z    Compiling thiserror v1.0.5
2020-04-04T18:41:52.9083824Z    Compiling yaml-merge-keys v0.4.0
2020-04-04T18:41:54.1538383Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-04T18:41:56.2257157Z Build completed successfully in 0:00:30
2020-04-04T18:41:56.2270532Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-04T18:41:56.4633311Z     Finished dev [unoptimized] target(s) in 0.17s
2020-04-04T18:41:57.5817839Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-04T18:44:01.6438572Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-04T18:44:06.5846334Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-04T18:44:07.8546317Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T18:44:07.8972783Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T18:44:08.0969157Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T18:44:08.8518380Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T18:44:08.9802363Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-04T18:44:10.5002403Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T18:44:10.9993198Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-04T18:48:34.7939113Z Build completed successfully in 0:00:44
2020-04-04T18:48:34.7947885Z + /scripts/validate-toolstate.sh
2020-04-04T18:48:34.7999385Z Cloning into 'rust-toolstate'...
2020-04-04T18:48:35.4941363Z Traceback (most recent call last):
2020-04-04T18:48:35.4941799Z   File "../../src/tools/publish_toolstate.py", line 305, in <module>
2020-04-04T18:48:35.4942163Z     cur_datetime
2020-04-04T18:48:35.4942473Z   File "../../src/tools/publish_toolstate.py", line 205, in update_latest
2020-04-04T18:48:35.4943728Z     maintainers = ' '.join('@'+name for name in MAINTAINERS[tool])
2020-04-04T18:48:35.4944273Z KeyError: u'rustc-dev-guide'
2020-04-04T18:48:35.4989931Z   local time: Sat Apr  4 18:48:35 UTC 2020
2020-04-04T18:48:35.4989931Z   local time: Sat Apr  4 18:48:35 UTC 2020
2020-04-04T18:48:35.7759425Z   network time: Sat, 04 Apr 2020 18:48:35 GMT
2020-04-04T18:48:37.1020950Z 
2020-04-04T18:48:37.1020950Z 
2020-04-04T18:48:37.1107013Z ##[error]Bash exited with code '1'.
2020-04-04T18:48:37.1124441Z ##[section]Finishing: Run build
2020-04-04T18:48:37.1184478Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70783/merge to s
2020-04-04T18:48:37.1194819Z Task         : Get sources
2020-04-04T18:48:37.1195198Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T18:48:37.1195585Z Version      : 1.0.0
2020-04-04T18:48:37.1195849Z Author       : Microsoft
2020-04-04T18:48:37.1195849Z Author       : Microsoft
2020-04-04T18:48:37.1196252Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-04T18:48:37.1196753Z ==============================================================================
2020-04-04T18:48:37.4996167Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-04T18:48:37.5048999Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70783/merge to s
2020-04-04T18:48:37.5152059Z Cleaning up task key
2020-04-04T18:48:37.5153587Z Start cleaning up orphan processes.
2020-04-04T18:48:37.5393663Z Terminate orphan process: pid (3313) (python)
2020-04-04T18:48:37.5585717Z ##[section]Finishing: Finalize Job
