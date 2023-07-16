plain
2020-04-05T15:48:00.4253718Z ========================== Starting Command Output ===========================
2020-04-05T15:48:00.4256145Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/968f4f99-0c1d-4105-88c9-b36e7759bbea.sh
2020-04-05T15:48:00.4256416Z 
2020-04-05T15:48:00.4260071Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-05T15:48:00.4280927Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-05T15:48:00.4284268Z Task         : Get sources
2020-04-05T15:48:00.4284551Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-05T15:48:00.4284833Z Version      : 1.0.0
2020-04-05T15:48:00.4285018Z Author       : Microsoft
---
2020-04-05T15:48:01.6606866Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-05T15:48:01.6613984Z ##[command]git config gc.auto 0
2020-04-05T15:48:01.6620939Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-05T15:48:01.6627424Z ##[command]git config --get-all http.proxy
2020-04-05T15:48:01.6638652Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70705/merge:refs/remotes/pull/70705/merge
---
2020-04-05T15:50:05.9595274Z  ---> 3fc1b512c57b
2020-04-05T15:50:05.9596774Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-05T15:50:05.9597322Z  ---> Using cache
2020-04-05T15:50:05.9597725Z  ---> 5ee4295733f4
2020-04-05T15:50:05.9599406Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-05T15:50:05.9603717Z  ---> 3d07a0fa42fe
2020-04-05T15:50:05.9639308Z Successfully built 3d07a0fa42fe
2020-04-05T15:50:05.9677177Z Successfully tagged rust-ci:latest
2020-04-05T15:50:05.9962266Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-05T15:50:05.9962266Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-05T15:50:05.9978721Z Looks like docker image is the same as before, not uploading
2020-04-05T15:50:12.7497572Z [CI_JOB_NAME=mingw-check]
2020-04-05T15:50:12.7798932Z [CI_JOB_NAME=mingw-check]
2020-04-05T15:50:12.7827842Z == clock drift check ==
2020-04-05T15:50:12.7839214Z   local time: Sun Apr  5 15:50:12 UTC 2020
2020-04-05T15:50:13.0483316Z   network time: Sun, 05 Apr 2020 15:50:13 GMT
2020-04-05T15:50:13.0510532Z Starting sccache server...
2020-04-05T15:50:13.1365378Z configure: processing command line
2020-04-05T15:50:13.1366002Z configure: 
2020-04-05T15:50:13.1367062Z configure: rust.parallel-compiler := True
---
2020-04-05T15:53:35.0915456Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-05T15:53:35.2084018Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-05T15:53:35.4126750Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-05T15:53:35.4836006Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-05T15:53:35.9948918Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-05T15:53:38.1461660Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-05T15:53:38.6033749Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-05T15:53:40.5056261Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-05T15:53:40.9296345Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-05T15:55:22.7871563Z configure: build.locked-deps    := True
2020-04-05T15:55:22.7872035Z configure: llvm.ccache          := sccache
2020-04-05T15:55:22.7872704Z configure: build.cargo-native-static := True
2020-04-05T15:55:22.7873368Z configure: dist.missing-tools   := True
2020-04-05T15:55:22.7874179Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-05T15:55:22.7874967Z configure: writing `config.toml` in current directory
2020-04-05T15:55:22.7875371Z configure: 
2020-04-05T15:55:22.7875918Z configure: run `python /checkout/x.py --help`
2020-04-05T15:55:22.7876199Z configure: 
---
2020-04-05T15:56:39.8265240Z Hugepagesize:       2048 kB
2020-04-05T15:56:39.8265462Z DirectMap4k:      145344 kB
2020-04-05T15:56:39.8265670Z DirectMap2M:     3000320 kB
2020-04-05T15:56:39.8265878Z DirectMap1G:     6291456 kB
2020-04-05T15:56:39.8292638Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-05T15:56:41.1342232Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-05T15:56:41.1342232Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-05T15:56:41.1350803Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-05T15:56:41.3666882Z    Compiling unicode-xid v0.2.0
2020-04-05T15:56:41.4945019Z    Compiling syn v1.0.11
2020-04-05T15:56:42.2715244Z    Compiling linked-hash-map v0.5.2
2020-04-05T15:56:42.3406813Z    Compiling lazy_static v1.4.0
2020-04-05T15:56:42.3406813Z    Compiling lazy_static v1.4.0
2020-04-05T15:56:42.4909698Z    Compiling yaml-rust v0.4.3
2020-04-05T15:56:46.6237704Z    Compiling quote v1.0.2
2020-04-05T15:57:00.3294536Z    Compiling thiserror-impl v1.0.5
2020-04-05T15:57:04.9454309Z    Compiling thiserror v1.0.5
2020-04-05T15:57:05.0050447Z    Compiling yaml-merge-keys v0.4.0
2020-04-05T15:57:06.1120809Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-05T15:57:07.6396951Z Build completed successfully in 0:00:27
2020-04-05T15:57:07.6403769Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-05T15:57:07.8760048Z     Finished dev [unoptimized] target(s) in 0.17s
2020-04-05T15:57:08.9956474Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-05T15:59:10.5019771Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-05T15:59:10.7484648Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-05T15:59:10.9308094Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-05T15:59:10.9477443Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-05T15:59:11.5240195Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-05T15:59:13.6497971Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-05T15:59:14.0979528Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-05T15:59:16.0664062Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-05T15:59:16.4858424Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-05T16:02:42.1954257Z    Compiling cargo_metadata v0.9.1
2020-04-05T16:02:45.7081460Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-04-05T16:02:55.9901604Z     Finished release [optimized] target(s) in 23.21s
2020-04-05T16:02:55.9996679Z tidy check
2020-04-05T16:02:59.9993161Z tidy error: /checkout/src/test/ui/generator/discriminant.rs:68: line longer than 100 chars
2020-04-05T16:03:05.3574333Z some tidy checks failed
2020-04-05T16:03:05.3577980Z Found 491 error codes
2020-04-05T16:03:05.3578648Z Found 0 error codes with no tests
2020-04-05T16:03:05.3579074Z Done!
2020-04-05T16:03:05.3579074Z Done!
2020-04-05T16:03:05.3581350Z 
2020-04-05T16:03:05.3581709Z 
2020-04-05T16:03:05.3583271Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-05T16:03:05.3584644Z 
2020-04-05T16:03:05.3584935Z 
2020-04-05T16:03:05.3589724Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-05T16:03:05.3590438Z Build completed unsuccessfully in 0:00:33
2020-04-05T16:03:05.3590438Z Build completed unsuccessfully in 0:00:33
2020-04-05T16:03:05.3637513Z == clock drift check ==
2020-04-05T16:03:05.3657771Z   local time: Sun Apr  5 16:03:05 UTC 2020
2020-04-05T16:03:05.8854445Z   network time: Sun, 05 Apr 2020 16:03:05 GMT
2020-04-05T16:03:07.4086996Z 
2020-04-05T16:03:07.4086996Z 
2020-04-05T16:03:07.4160755Z ##[error]Bash exited with code '1'.
2020-04-05T16:03:07.4180077Z ##[section]Finishing: Run build
2020-04-05T16:03:07.4227406Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-05T16:03:07.4231868Z Task         : Get sources
2020-04-05T16:03:07.4232195Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-05T16:03:07.4232496Z Version      : 1.0.0
2020-04-05T16:03:07.4232723Z Author       : Microsoft
2020-04-05T16:03:07.4232723Z Author       : Microsoft
2020-04-05T16:03:07.4233053Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-05T16:03:07.4233442Z ==============================================================================
2020-04-05T16:03:07.7361464Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-05T16:03:07.7409054Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-05T16:03:07.7494032Z Cleaning up task key
2020-04-05T16:03:07.7495249Z Start cleaning up orphan processes.
2020-04-05T16:03:07.7667954Z Terminate orphan process: pid (3616) (python)
2020-04-05T16:03:07.7856279Z ##[section]Finishing: Finalize Job
