plain
2020-04-04T18:26:07.7611716Z ========================== Starting Command Output ===========================
2020-04-04T18:26:07.7614751Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/151bfbbf-9a32-4e82-93e8-719f5001fae8.sh
2020-04-04T18:26:07.7614985Z 
2020-04-04T18:26:07.7619382Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-04T18:26:07.7637053Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70772/merge to s
2020-04-04T18:26:07.7640824Z Task         : Get sources
2020-04-04T18:26:07.7641117Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T18:26:07.7641355Z Version      : 1.0.0
2020-04-04T18:26:07.7641515Z Author       : Microsoft
---
2020-04-04T18:26:08.7545008Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-04T18:26:08.7554948Z ##[command]git config gc.auto 0
2020-04-04T18:26:08.7561724Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-04T18:26:08.7568351Z ##[command]git config --get-all http.proxy
2020-04-04T18:26:08.7574774Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70772/merge:refs/remotes/pull/70772/merge
---
2020-04-04T18:28:28.1049533Z  ---> 3fc1b512c57b
2020-04-04T18:28:28.1049798Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-04T18:28:28.1050235Z  ---> Using cache
2020-04-04T18:28:28.1050602Z  ---> 5ee4295733f4
2020-04-04T18:28:28.1054934Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-04T18:28:28.1056387Z  ---> 3d07a0fa42fe
2020-04-04T18:28:28.1090797Z Successfully built 3d07a0fa42fe
2020-04-04T18:28:28.1116134Z Successfully tagged rust-ci:latest
2020-04-04T18:28:28.2007217Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T18:28:28.2007217Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T18:28:28.2023200Z Looks like docker image is the same as before, not uploading
2020-04-04T18:28:34.9671849Z [CI_JOB_NAME=mingw-check]
2020-04-04T18:28:34.9938593Z [CI_JOB_NAME=mingw-check]
2020-04-04T18:28:34.9967080Z == clock drift check ==
2020-04-04T18:28:34.9975948Z   local time: Sat Apr  4 18:28:34 UTC 2020
2020-04-04T18:28:35.1594782Z   network time: Sat, 04 Apr 2020 18:28:35 GMT
2020-04-04T18:28:35.1602418Z Starting sccache server...
2020-04-04T18:28:35.2493290Z configure: processing command line
2020-04-04T18:28:35.2494266Z configure: 
2020-04-04T18:28:35.2495401Z configure: rust.parallel-compiler := True
---
2020-04-04T18:32:15.9232221Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-04T18:32:20.8367462Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-04T18:32:22.1418738Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T18:32:22.1842065Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T18:32:22.4025033Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T18:32:23.2200901Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T18:32:23.2920413Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-04T18:32:24.9003501Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T18:32:25.4204724Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-04T18:34:24.4649198Z configure: build.locked-deps    := True
2020-04-04T18:34:24.4649680Z configure: llvm.ccache          := sccache
2020-04-04T18:34:24.4650632Z configure: build.cargo-native-static := True
2020-04-04T18:34:24.4651815Z configure: dist.missing-tools   := True
2020-04-04T18:34:24.4652945Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-04T18:34:24.4653782Z configure: writing `config.toml` in current directory
2020-04-04T18:34:24.4655147Z configure: 
2020-04-04T18:34:24.4655929Z configure: run `python /checkout/x.py --help`
2020-04-04T18:34:24.4656288Z configure: 
---
2020-04-04T18:35:53.4296837Z Hugepagesize:       2048 kB
2020-04-04T18:35:53.4297060Z DirectMap4k:      145344 kB
2020-04-04T18:35:53.4297282Z DirectMap2M:     4048896 kB
2020-04-04T18:35:53.4297523Z DirectMap1G:     5242880 kB
2020-04-04T18:35:53.4298085Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-04T18:35:54.1152910Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T18:35:54.1152910Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T18:35:54.1160512Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-04T18:35:54.3484992Z    Compiling unicode-xid v0.2.0
2020-04-04T18:35:54.4932279Z    Compiling syn v1.0.11
2020-04-04T18:35:55.3939043Z    Compiling linked-hash-map v0.5.2
2020-04-04T18:35:55.4021922Z    Compiling lazy_static v1.4.0
2020-04-04T18:35:55.4021922Z    Compiling lazy_static v1.4.0
2020-04-04T18:35:55.6243833Z    Compiling yaml-rust v0.4.3
2020-04-04T18:36:00.1773036Z    Compiling quote v1.0.2
2020-04-04T18:36:15.8685730Z    Compiling thiserror-impl v1.0.5
2020-04-04T18:36:21.0825124Z    Compiling thiserror v1.0.5
2020-04-04T18:36:21.1500457Z    Compiling yaml-merge-keys v0.4.0
2020-04-04T18:36:22.4575408Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-04T18:36:25.1194550Z Build completed successfully in 0:00:32
2020-04-04T18:36:25.1206417Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-04T18:36:25.3792110Z     Finished dev [unoptimized] target(s) in 0.18s
2020-04-04T18:36:26.5581578Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-04T18:38:42.5641760Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T18:38:42.6605878Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T18:38:42.8993750Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-04T18:38:43.0146227Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T18:38:43.5623842Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T18:38:46.0785145Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T18:38:46.6118499Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-04T18:38:48.8861553Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-04T18:38:49.3793209Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-04T18:43:13.8462319Z Build completed successfully in 0:00:44
2020-04-04T18:43:13.8466196Z + /scripts/validate-toolstate.sh
2020-04-04T18:43:13.8537273Z Cloning into 'rust-toolstate'...
2020-04-04T18:43:14.5164782Z Traceback (most recent call last):
2020-04-04T18:43:14.5166190Z   File "../../src/tools/publish_toolstate.py", line 305, in <module>
2020-04-04T18:43:14.5166717Z     cur_datetime
2020-04-04T18:43:14.5167052Z   File "../../src/tools/publish_toolstate.py", line 205, in update_latest
2020-04-04T18:43:14.5168026Z     maintainers = ' '.join('@'+name for name in MAINTAINERS[tool])
2020-04-04T18:43:14.5168557Z KeyError: u'rustc-dev-guide'
2020-04-04T18:43:14.5218106Z   local time: Sat Apr  4 18:43:14 UTC 2020
2020-04-04T18:43:14.5218106Z   local time: Sat Apr  4 18:43:14 UTC 2020
2020-04-04T18:43:14.7286553Z   network time: Sat, 04 Apr 2020 18:43:14 GMT
2020-04-04T18:43:16.0732029Z 
2020-04-04T18:43:16.0732029Z 
2020-04-04T18:43:16.0809417Z ##[error]Bash exited with code '1'.
2020-04-04T18:43:16.0825597Z ##[section]Finishing: Run build
2020-04-04T18:43:16.0900131Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70772/merge to s
2020-04-04T18:43:16.0907132Z Task         : Get sources
2020-04-04T18:43:16.0907511Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T18:43:16.0907988Z Version      : 1.0.0
2020-04-04T18:43:16.0908208Z Author       : Microsoft
2020-04-04T18:43:16.0908208Z Author       : Microsoft
2020-04-04T18:43:16.0908750Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-04T18:43:16.0909499Z ==============================================================================
2020-04-04T18:43:16.4435462Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-04T18:43:16.4479035Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70772/merge to s
2020-04-04T18:43:16.4577072Z Cleaning up task key
2020-04-04T18:43:16.4578460Z Start cleaning up orphan processes.
2020-04-04T18:43:16.4770036Z Terminate orphan process: pid (3930) (python)
2020-04-04T18:43:16.4944850Z ##[section]Finishing: Finalize Job
