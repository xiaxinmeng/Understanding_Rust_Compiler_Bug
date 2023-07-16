plain
2020-04-04T17:34:03.2561994Z ========================== Starting Command Output ===========================
2020-04-04T17:34:03.2576942Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/54bc4c34-1777-4482-9a80-ba2074fd651f.sh
2020-04-04T17:34:03.2768966Z 
2020-04-04T17:34:03.2821766Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-04T17:34:03.2842201Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70777/merge to s
2020-04-04T17:34:03.2845718Z Task         : Get sources
2020-04-04T17:34:03.2846046Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T17:34:03.2846351Z Version      : 1.0.0
2020-04-04T17:34:03.2846556Z Author       : Microsoft
---
2020-04-04T17:34:04.5135086Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-04T17:34:04.5141289Z ##[command]git config gc.auto 0
2020-04-04T17:34:04.5145050Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-04T17:34:04.5148543Z ##[command]git config --get-all http.proxy
2020-04-04T17:34:04.5155931Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70777/merge:refs/remotes/pull/70777/merge
---
2020-04-04T17:37:31.3755874Z  ---> 3fc1b512c57b
2020-04-04T17:37:31.3756245Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-04T17:37:31.3759665Z  ---> Using cache
2020-04-04T17:37:31.3760187Z  ---> 5ee4295733f4
2020-04-04T17:37:31.3761721Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-04T17:37:31.3764449Z  ---> 3d07a0fa42fe
2020-04-04T17:37:31.3799015Z Successfully built 3d07a0fa42fe
2020-04-04T17:37:31.3824638Z Successfully tagged rust-ci:latest
2020-04-04T17:37:31.4187591Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T17:37:31.4187591Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T17:37:31.4203259Z Looks like docker image is the same as before, not uploading
2020-04-04T17:37:38.2144583Z [CI_JOB_NAME=mingw-check]
2020-04-04T17:37:38.2377942Z [CI_JOB_NAME=mingw-check]
2020-04-04T17:37:38.2408257Z == clock drift check ==
2020-04-04T17:37:38.2414604Z   local time: Sat Apr  4 17:37:38 UTC 2020
2020-04-04T17:37:38.2580649Z   network time: Sat, 04 Apr 2020 17:37:38 GMT
2020-04-04T17:37:38.2599073Z Starting sccache server...
2020-04-04T17:37:38.3467341Z configure: processing command line
2020-04-04T17:37:38.3467957Z configure: 
2020-04-04T17:37:38.3468995Z configure: rust.parallel-compiler := True
---
2020-04-04T17:40:57.5977771Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T17:40:57.6643775Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T17:40:57.8538458Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-04T17:40:57.9890620Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T17:40:58.4490386Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T17:41:00.6593977Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T17:41:01.1208175Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-04T17:41:03.0687617Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-04T17:41:03.4776413Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-04T17:42:43.4142876Z configure: build.locked-deps    := True
2020-04-04T17:42:43.4143300Z configure: llvm.ccache          := sccache
2020-04-04T17:42:43.4143925Z configure: build.cargo-native-static := True
2020-04-04T17:42:43.4144528Z configure: dist.missing-tools   := True
2020-04-04T17:42:43.4145288Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-04T17:42:43.4146076Z configure: writing `config.toml` in current directory
2020-04-04T17:42:43.4146445Z configure: 
2020-04-04T17:42:43.4146971Z configure: run `python /checkout/x.py --help`
2020-04-04T17:42:43.4147318Z configure: 
---
2020-04-04T17:43:59.7051794Z Hugepagesize:       2048 kB
2020-04-04T17:43:59.7052017Z DirectMap4k:      143296 kB
2020-04-04T17:43:59.7052251Z DirectMap2M:     3002368 kB
2020-04-04T17:43:59.7052473Z DirectMap1G:     6291456 kB
2020-04-04T17:43:59.7053030Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-04T17:44:00.3243195Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T17:44:00.3243195Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T17:44:00.3249051Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-04T17:44:00.5572286Z    Compiling unicode-xid v0.2.0
2020-04-04T17:44:00.6849733Z    Compiling syn v1.0.11
2020-04-04T17:44:01.4782682Z    Compiling linked-hash-map v0.5.2
2020-04-04T17:44:01.5129194Z    Compiling lazy_static v1.4.0
2020-04-04T17:44:01.5129194Z    Compiling lazy_static v1.4.0
2020-04-04T17:44:01.6901947Z    Compiling yaml-rust v0.4.3
2020-04-04T17:44:05.4893775Z    Compiling quote v1.0.2
2020-04-04T17:44:18.6241019Z    Compiling thiserror-impl v1.0.5
2020-04-04T17:44:22.9132353Z    Compiling thiserror v1.0.5
2020-04-04T17:44:22.9708830Z    Compiling yaml-merge-keys v0.4.0
2020-04-04T17:44:24.0776831Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-04T17:44:25.6087501Z Build completed successfully in 0:00:26
2020-04-04T17:44:25.6099682Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-04T17:44:25.8471480Z     Finished dev [unoptimized] target(s) in 0.17s
2020-04-04T17:44:27.7060517Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-04T17:46:23.5889966Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T17:46:23.6164740Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T17:46:23.7958031Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-04T17:46:23.9670293Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T17:46:24.3649827Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T17:46:26.2956714Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T17:46:26.7549266Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-04T17:46:28.5765871Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-04T17:46:28.9840542Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-04T17:50:18.7962347Z Build completed successfully in 0:00:38
2020-04-04T17:50:18.7971771Z + /scripts/validate-toolstate.sh
2020-04-04T17:50:18.8021908Z Cloning into 'rust-toolstate'...
2020-04-04T17:50:19.4028223Z Traceback (most recent call last):
2020-04-04T17:50:19.4029350Z   File "../../src/tools/publish_toolstate.py", line 305, in <module>
2020-04-04T17:50:19.4030101Z     cur_datetime
2020-04-04T17:50:19.4030832Z   File "../../src/tools/publish_toolstate.py", line 205, in update_latest
2020-04-04T17:50:19.4031988Z     maintainers = ' '.join('@'+name for name in MAINTAINERS[tool])
2020-04-04T17:50:19.4032800Z KeyError: u'rustc-dev-guide'
2020-04-04T17:50:19.4080823Z   local time: Sat Apr  4 17:50:19 UTC 2020
2020-04-04T17:50:19.4080823Z   local time: Sat Apr  4 17:50:19 UTC 2020
2020-04-04T17:50:19.6715386Z   network time: Sat, 04 Apr 2020 17:50:19 GMT
2020-04-04T17:50:21.2374194Z 
2020-04-04T17:50:21.2374194Z 
2020-04-04T17:50:21.2440658Z ##[error]Bash exited with code '1'.
2020-04-04T17:50:21.2454109Z ##[section]Finishing: Run build
2020-04-04T17:50:21.2529171Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70777/merge to s
2020-04-04T17:50:21.2534005Z Task         : Get sources
2020-04-04T17:50:21.2534357Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T17:50:21.2534704Z Version      : 1.0.0
2020-04-04T17:50:21.2534929Z Author       : Microsoft
2020-04-04T17:50:21.2534929Z Author       : Microsoft
2020-04-04T17:50:21.2535292Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-04T17:50:21.2535724Z ==============================================================================
2020-04-04T17:50:21.5795463Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-04T17:50:21.5840386Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70777/merge to s
2020-04-04T17:50:21.5949982Z Cleaning up task key
2020-04-04T17:50:21.5951294Z Start cleaning up orphan processes.
2020-04-04T17:50:21.6142064Z Terminate orphan process: pid (4099) (python)
2020-04-04T17:50:21.6311192Z ##[section]Finishing: Finalize Job
