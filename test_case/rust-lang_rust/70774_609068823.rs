plain
2020-04-04T16:37:03.5001179Z ========================== Starting Command Output ===========================
2020-04-04T16:37:03.5004059Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/01fa0999-8616-4865-87c8-14e31340e4ab.sh
2020-04-04T16:37:03.5004580Z 
2020-04-04T16:37:03.5007737Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-04T16:37:03.5021358Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70774/merge to s
2020-04-04T16:37:03.5023999Z Task         : Get sources
2020-04-04T16:37:03.5024264Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T16:37:03.5024506Z Version      : 1.0.0
2020-04-04T16:37:03.5024669Z Author       : Microsoft
---
2020-04-04T16:37:04.5013339Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-04T16:37:04.5022483Z ##[command]git config gc.auto 0
2020-04-04T16:37:04.5027966Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-04T16:37:04.5032386Z ##[command]git config --get-all http.proxy
2020-04-04T16:37:04.5039070Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70774/merge:refs/remotes/pull/70774/merge
---
2020-04-04T16:39:26.7197128Z  ---> 3fc1b512c57b
2020-04-04T16:39:26.7200966Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-04T16:39:26.7204153Z  ---> Using cache
2020-04-04T16:39:26.7205093Z  ---> 5ee4295733f4
2020-04-04T16:39:26.7213246Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-04T16:39:26.7233063Z  ---> 3d07a0fa42fe
2020-04-04T16:39:26.7239711Z Successfully built 3d07a0fa42fe
2020-04-04T16:39:26.7271932Z Successfully tagged rust-ci:latest
2020-04-04T16:39:26.7558806Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T16:39:26.7558806Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T16:39:26.7577199Z Looks like docker image is the same as before, not uploading
2020-04-04T16:39:34.9206187Z [CI_JOB_NAME=mingw-check]
2020-04-04T16:39:34.9384190Z [CI_JOB_NAME=mingw-check]
2020-04-04T16:39:34.9406983Z == clock drift check ==
2020-04-04T16:39:34.9414971Z   local time: Sat Apr  4 16:39:34 UTC 2020
2020-04-04T16:39:35.2192011Z   network time: Sat, 04 Apr 2020 16:39:35 GMT
2020-04-04T16:39:35.2223892Z Starting sccache server...
2020-04-04T16:39:35.3024084Z configure: processing command line
2020-04-04T16:39:35.3024745Z configure: 
2020-04-04T16:39:35.3025816Z configure: rust.parallel-compiler := True
---
2020-04-04T16:42:49.4309377Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T16:42:49.5686886Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T16:42:49.7552464Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-04T16:42:49.8084789Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T16:42:50.3187563Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T16:42:52.3969263Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T16:42:52.8383841Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-04T16:42:54.5869619Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-04T16:42:55.0306291Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-04T16:44:28.1012345Z configure: build.locked-deps    := True
2020-04-04T16:44:28.1012626Z configure: llvm.ccache          := sccache
2020-04-04T16:44:28.1013090Z configure: build.cargo-native-static := True
2020-04-04T16:44:28.1013526Z configure: dist.missing-tools   := True
2020-04-04T16:44:28.1014087Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-04T16:44:28.1014628Z configure: writing `config.toml` in current directory
2020-04-04T16:44:28.1014854Z configure: 
2020-04-04T16:44:28.1015246Z configure: run `python /checkout/x.py --help`
2020-04-04T16:44:28.1015462Z configure: 
---
2020-04-04T16:45:44.1874551Z Hugepagesize:       2048 kB
2020-04-04T16:45:44.1874895Z DirectMap4k:      135104 kB
2020-04-04T16:45:44.1875272Z DirectMap2M:     4059136 kB
2020-04-04T16:45:44.1875616Z DirectMap1G:     5242880 kB
2020-04-04T16:45:44.1876633Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-04T16:45:45.3755163Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T16:45:45.3755163Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T16:45:45.3763277Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-04T16:45:45.5875088Z    Compiling unicode-xid v0.2.0
2020-04-04T16:45:45.7051618Z    Compiling syn v1.0.11
2020-04-04T16:45:46.4760036Z    Compiling linked-hash-map v0.5.2
2020-04-04T16:45:46.5139978Z    Compiling lazy_static v1.4.0
2020-04-04T16:45:46.5139978Z    Compiling lazy_static v1.4.0
2020-04-04T16:45:46.7058586Z    Compiling yaml-rust v0.4.3
2020-04-04T16:45:50.4487281Z    Compiling quote v1.0.2
2020-04-04T16:46:02.4639841Z    Compiling thiserror-impl v1.0.5
2020-04-04T16:46:06.6709362Z    Compiling thiserror v1.0.5
2020-04-04T16:46:06.6715304Z    Compiling yaml-merge-keys v0.4.0
2020-04-04T16:46:07.4987021Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-04T16:46:08.8501955Z Build completed successfully in 0:00:24
2020-04-04T16:46:08.8510030Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-04T16:46:09.0454065Z     Finished dev [unoptimized] target(s) in 0.14s
2020-04-04T16:46:10.0140243Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-04T16:47:58.8025251Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T16:47:58.8944317Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T16:47:59.0677743Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-04T16:47:59.1649612Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T16:47:59.6128987Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T16:48:01.5447203Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T16:48:01.9945505Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-04T16:48:03.8153891Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-04T16:48:04.1852559Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-04T16:51:39.7642150Z Build completed successfully in 0:00:37
2020-04-04T16:51:39.7649550Z + /scripts/validate-toolstate.sh
2020-04-04T16:51:39.7695224Z Cloning into 'rust-toolstate'...
2020-04-04T16:51:40.4255987Z Traceback (most recent call last):
2020-04-04T16:51:40.4256607Z   File "../../src/tools/publish_toolstate.py", line 305, in <module>
2020-04-04T16:51:40.4257135Z     cur_datetime
2020-04-04T16:51:40.4257649Z   File "../../src/tools/publish_toolstate.py", line 205, in update_latest
2020-04-04T16:51:40.4259267Z     maintainers = ' '.join('@'+name for name in MAINTAINERS[tool])
2020-04-04T16:51:40.4260118Z KeyError: u'rustc-dev-guide'
2020-04-04T16:51:40.4311840Z   local time: Sat Apr  4 16:51:40 UTC 2020
2020-04-04T16:51:40.4311840Z   local time: Sat Apr  4 16:51:40 UTC 2020
2020-04-04T16:51:40.5917543Z   network time: Sat, 04 Apr 2020 16:51:40 GMT
2020-04-04T16:51:42.1996086Z 
2020-04-04T16:51:42.1996086Z 
2020-04-04T16:51:42.2059330Z ##[error]Bash exited with code '1'.
2020-04-04T16:51:42.2081858Z ##[section]Finishing: Run build
2020-04-04T16:51:42.2130272Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70774/merge to s
2020-04-04T16:51:42.2135128Z Task         : Get sources
2020-04-04T16:51:42.2135485Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T16:51:42.2135832Z Version      : 1.0.0
2020-04-04T16:51:42.2136206Z Author       : Microsoft
2020-04-04T16:51:42.2136206Z Author       : Microsoft
2020-04-04T16:51:42.2136569Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-04T16:51:42.2137011Z ==============================================================================
2020-04-04T16:51:42.5155771Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-04T16:51:42.5197980Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70774/merge to s
2020-04-04T16:51:42.5281193Z Cleaning up task key
2020-04-04T16:51:42.5282394Z Start cleaning up orphan processes.
2020-04-04T16:51:42.5458898Z Terminate orphan process: pid (3530) (python)
2020-04-04T16:51:42.5618729Z ##[section]Finishing: Finalize Job
