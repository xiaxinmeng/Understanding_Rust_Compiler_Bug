plain
2020-04-04T18:46:14.9709518Z ========================== Starting Command Output ===========================
2020-04-04T18:46:14.9714344Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4da4977b-213b-4fd4-9ed0-f452e392345d.sh
2020-04-04T18:46:14.9714761Z 
2020-04-04T18:46:14.9718716Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-04T18:46:14.9734535Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70776/merge to s
2020-04-04T18:46:14.9737985Z Task         : Get sources
2020-04-04T18:46:14.9738237Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T18:46:14.9738718Z Version      : 1.0.0
2020-04-04T18:46:14.9738880Z Author       : Microsoft
---
2020-04-04T18:46:16.1663122Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-04T18:46:16.1671783Z ##[command]git config gc.auto 0
2020-04-04T18:46:16.1676183Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-04T18:46:16.1681637Z ##[command]git config --get-all http.proxy
2020-04-04T18:46:16.1691183Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70776/merge:refs/remotes/pull/70776/merge
---
2020-04-04T18:48:46.5463143Z  ---> 3fc1b512c57b
2020-04-04T18:48:46.5463398Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-04T18:48:46.5474987Z  ---> Using cache
2020-04-04T18:48:46.5477543Z  ---> 5ee4295733f4
2020-04-04T18:48:46.5479138Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-04T18:48:46.5480619Z  ---> 3d07a0fa42fe
2020-04-04T18:48:46.5589235Z Successfully built 3d07a0fa42fe
2020-04-04T18:48:46.5614430Z Successfully tagged rust-ci:latest
2020-04-04T18:48:46.5959356Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T18:48:46.5959356Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T18:48:46.5979136Z Looks like docker image is the same as before, not uploading
2020-04-04T18:48:52.3911717Z [CI_JOB_NAME=mingw-check]
2020-04-04T18:48:52.4149645Z [CI_JOB_NAME=mingw-check]
2020-04-04T18:48:52.4183218Z == clock drift check ==
2020-04-04T18:48:52.4193570Z   local time: Sat Apr  4 18:48:52 UTC 2020
2020-04-04T18:48:52.6867136Z   network time: Sat, 04 Apr 2020 18:48:52 GMT
2020-04-04T18:48:52.6923683Z Starting sccache server...
2020-04-04T18:48:52.7762159Z configure: processing command line
2020-04-04T18:48:52.7766695Z configure: 
2020-04-04T18:48:52.7768702Z configure: rust.parallel-compiler := True
---
2020-04-04T18:52:46.2458902Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T18:52:46.2610711Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T18:52:46.4699104Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-04T18:52:46.7057198Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T18:52:47.1628521Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T18:52:49.6909636Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T18:52:50.2083425Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-04T18:52:52.4202963Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-04T18:52:52.9219339Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-04T18:54:49.4273690Z configure: build.locked-deps    := True
2020-04-04T18:54:49.4274318Z configure: llvm.ccache          := sccache
2020-04-04T18:54:49.4275685Z configure: build.cargo-native-static := True
2020-04-04T18:54:49.4276700Z configure: dist.missing-tools   := True
2020-04-04T18:54:49.4277587Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-04T18:54:49.4278374Z configure: writing `config.toml` in current directory
2020-04-04T18:54:49.4278704Z configure: 
2020-04-04T18:54:49.4279245Z configure: run `python /checkout/x.py --help`
2020-04-04T18:54:49.4279593Z configure: 
---
2020-04-04T18:56:23.8390178Z Hugepagesize:       2048 kB
2020-04-04T18:56:23.8390425Z DirectMap4k:      116672 kB
2020-04-04T18:56:23.8390656Z DirectMap2M:     4077568 kB
2020-04-04T18:56:23.8390884Z DirectMap1G:     5242880 kB
2020-04-04T18:56:23.8391503Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-04T18:56:24.4580366Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T18:56:24.4580366Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T18:56:24.4588099Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-04T18:56:24.7165964Z    Compiling unicode-xid v0.2.0
2020-04-04T18:56:24.8554450Z    Compiling syn v1.0.11
2020-04-04T18:56:25.7593487Z    Compiling linked-hash-map v0.5.2
2020-04-04T18:56:25.8343910Z    Compiling lazy_static v1.4.0
2020-04-04T18:56:25.8343910Z    Compiling lazy_static v1.4.0
2020-04-04T18:56:26.0139574Z    Compiling yaml-rust v0.4.3
2020-04-04T18:56:31.0824299Z    Compiling quote v1.0.2
2020-04-04T18:56:47.3601906Z    Compiling thiserror-impl v1.0.5
2020-04-04T18:56:52.4809218Z    Compiling thiserror v1.0.5
2020-04-04T18:56:52.5395801Z    Compiling yaml-merge-keys v0.4.0
2020-04-04T18:56:53.8103933Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-04T18:56:55.5303079Z Build completed successfully in 0:00:32
2020-04-04T18:56:55.5308155Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-04T18:56:55.7817182Z     Finished dev [unoptimized] target(s) in 0.18s
2020-04-04T18:56:56.9427078Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-04T18:59:10.0393415Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-04T18:59:15.1597501Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-04T18:59:16.5642787Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T18:59:16.7079133Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T18:59:16.9754626Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T18:59:17.7904371Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T18:59:17.9150197Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-04T18:59:19.7008156Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T18:59:20.2618685Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-04T19:03:56.6300501Z Build completed successfully in 0:00:44
2020-04-04T19:03:56.6306932Z + /scripts/validate-toolstate.sh
2020-04-04T19:03:56.6356391Z Cloning into 'rust-toolstate'...
2020-04-04T19:03:57.3242892Z Traceback (most recent call last):
2020-04-04T19:03:57.3243649Z   File "../../src/tools/publish_toolstate.py", line 305, in <module>
2020-04-04T19:03:57.3243880Z     cur_datetime
2020-04-04T19:03:57.3244135Z   File "../../src/tools/publish_toolstate.py", line 205, in update_latest
2020-04-04T19:03:57.3245266Z     maintainers = ' '.join('@'+name for name in MAINTAINERS[tool])
2020-04-04T19:03:57.3245676Z KeyError: u'rustc-dev-guide'
2020-04-04T19:03:57.3289829Z   local time: Sat Apr  4 19:03:57 UTC 2020
2020-04-04T19:03:57.3289829Z   local time: Sat Apr  4 19:03:57 UTC 2020
2020-04-04T19:03:57.5952810Z   network time: Sat, 04 Apr 2020 19:03:57 GMT
2020-04-04T19:03:59.0376532Z 
2020-04-04T19:03:59.0376532Z 
2020-04-04T19:03:59.0454924Z ##[error]Bash exited with code '1'.
2020-04-04T19:03:59.0467644Z ##[section]Finishing: Run build
2020-04-04T19:03:59.0517188Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70776/merge to s
2020-04-04T19:03:59.0521769Z Task         : Get sources
2020-04-04T19:03:59.0522087Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T19:03:59.0522518Z Version      : 1.0.0
2020-04-04T19:03:59.0522720Z Author       : Microsoft
2020-04-04T19:03:59.0522720Z Author       : Microsoft
2020-04-04T19:03:59.0523044Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-04T19:03:59.0523398Z ==============================================================================
2020-04-04T19:03:59.4155251Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-04T19:03:59.4216238Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70776/merge to s
2020-04-04T19:03:59.4302100Z Cleaning up task key
2020-04-04T19:03:59.4303196Z Start cleaning up orphan processes.
2020-04-04T19:03:59.4558507Z Terminate orphan process: pid (3966) (python)
2020-04-04T19:03:59.4817185Z ##[section]Finishing: Finalize Job
