plain
2020-04-07T09:39:43.8975286Z ========================== Starting Command Output ===========================
2020-04-07T09:39:43.8977765Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3ca29b32-ce39-4dc4-adbe-9d8b9fcfb45c.sh
2020-04-07T09:39:43.8978073Z 
2020-04-07T09:39:43.8982311Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-07T09:39:43.9000299Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70042/merge to s
2020-04-07T09:39:43.9003408Z Task         : Get sources
2020-04-07T09:39:43.9003680Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T09:39:43.9003939Z Version      : 1.0.0
2020-04-07T09:39:43.9004114Z Author       : Microsoft
---
2020-04-07T09:39:44.8945164Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-07T09:39:44.8949480Z ##[command]git config gc.auto 0
2020-04-07T09:39:44.8952652Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-07T09:39:44.8955567Z ##[command]git config --get-all http.proxy
2020-04-07T09:39:44.8960766Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70042/merge:refs/remotes/pull/70042/merge
---
2020-04-07T09:41:57.8929478Z  ---> 3fc1b512c57b
2020-04-07T09:41:57.8929851Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-07T09:41:57.8930395Z  ---> Using cache
2020-04-07T09:41:57.8930858Z  ---> 5ee4295733f4
2020-04-07T09:41:57.8934198Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-07T09:41:57.8936070Z  ---> 3d07a0fa42fe
2020-04-07T09:41:57.8974776Z Successfully built 3d07a0fa42fe
2020-04-07T09:41:57.8997320Z Successfully tagged rust-ci:latest
2020-04-07T09:41:57.9234229Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-07T09:41:57.9234229Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-07T09:41:57.9249623Z Looks like docker image is the same as before, not uploading
2020-04-07T09:42:04.6256228Z [CI_JOB_NAME=mingw-check]
2020-04-07T09:42:04.6501445Z [CI_JOB_NAME=mingw-check]
2020-04-07T09:42:04.6523883Z == clock drift check ==
2020-04-07T09:42:04.6540292Z   local time: Tue Apr  7 09:42:04 UTC 2020
2020-04-07T09:42:04.9436606Z   network time: Tue, 07 Apr 2020 09:42:04 GMT
2020-04-07T09:42:04.9455547Z Starting sccache server...
2020-04-07T09:42:05.0260749Z configure: processing command line
2020-04-07T09:42:05.0261481Z configure: 
2020-04-07T09:42:05.0262500Z configure: rust.parallel-compiler := True
---
2020-04-07T09:45:29.2920185Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-07T09:45:29.3098185Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-07T09:45:29.4862705Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-07T09:45:29.6976979Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-07T09:45:30.0685043Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-07T09:45:32.4214585Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-07T09:45:32.8781488Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-07T09:45:34.7703113Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-07T09:45:36.0080327Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-07T09:47:21.7792095Z configure: build.locked-deps    := True
2020-04-07T09:47:21.7792332Z configure: llvm.ccache          := sccache
2020-04-07T09:47:21.7792717Z configure: build.cargo-native-static := True
2020-04-07T09:47:21.7793130Z configure: dist.missing-tools   := True
2020-04-07T09:47:21.7793603Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-07T09:47:21.7794110Z configure: writing `config.toml` in current directory
2020-04-07T09:47:21.7794300Z configure: 
2020-04-07T09:47:21.7794638Z configure: run `python /checkout/x.py --help`
2020-04-07T09:47:21.7794838Z configure: 
---
2020-04-07T09:48:38.2228865Z Hugepagesize:       2048 kB
2020-04-07T09:48:38.2229063Z DirectMap4k:      137152 kB
2020-04-07T09:48:38.2229398Z DirectMap2M:     4057088 kB
2020-04-07T09:48:38.2229573Z DirectMap1G:     5242880 kB
2020-04-07T09:48:38.2237422Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-07T09:48:39.3710167Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-07T09:48:39.3710167Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-07T09:48:39.3717831Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-07T09:48:39.5765811Z    Compiling unicode-xid v0.2.0
2020-04-07T09:48:39.6816434Z    Compiling syn v1.0.11
2020-04-07T09:48:40.4787961Z    Compiling linked-hash-map v0.5.2
2020-04-07T09:48:40.5070673Z    Compiling lazy_static v1.4.0
2020-04-07T09:48:40.5070673Z    Compiling lazy_static v1.4.0
2020-04-07T09:48:40.6961264Z    Compiling yaml-rust v0.4.3
2020-04-07T09:48:44.6790304Z    Compiling quote v1.0.2
2020-04-07T09:48:59.1064297Z    Compiling thiserror-impl v1.0.5
2020-04-07T09:49:03.8839517Z    Compiling thiserror v1.0.5
2020-04-07T09:49:03.9427309Z    Compiling yaml-merge-keys v0.4.0
2020-04-07T09:49:05.1231253Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-07T09:49:06.7769813Z Build completed successfully in 0:00:28
2020-04-07T09:49:06.7778340Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-07T09:49:07.0064963Z     Finished dev [unoptimized] target(s) in 0.16s
2020-04-07T09:49:08.0596182Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-07T09:51:07.1919218Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-07T09:51:07.1922135Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-07T09:51:07.3777241Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-07T09:51:07.5906374Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-07T09:51:07.9640287Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-07T09:51:10.1430912Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-07T09:51:10.6048567Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-07T09:51:12.5885680Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-07T09:51:12.9905158Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-07T09:54:59.9326067Z Found 0 error codes with no tests
2020-04-07T09:54:59.9326453Z Done!
2020-04-07T09:54:59.9327427Z 
2020-04-07T09:54:59.9327812Z 
2020-04-07T09:54:59.9329724Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-07T09:54:59.9330652Z 
2020-04-07T09:54:59.9330868Z 
2020-04-07T09:54:59.9337167Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-07T09:54:59.9337617Z Build completed unsuccessfully in 0:00:32
2020-04-07T09:54:59.9337617Z Build completed unsuccessfully in 0:00:32
2020-04-07T09:54:59.9393557Z == clock drift check ==
2020-04-07T09:54:59.9412847Z   local time: Tue Apr  7 09:54:59 UTC 2020
2020-04-07T09:55:00.2207596Z   network time: Tue, 07 Apr 2020 09:55:00 GMT
2020-04-07T09:55:01.8701322Z 
2020-04-07T09:55:01.8701322Z 
2020-04-07T09:55:01.8774133Z ##[error]Bash exited with code '1'.
2020-04-07T09:55:01.8791138Z ##[section]Finishing: Run build
2020-04-07T09:55:01.8837742Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70042/merge to s
2020-04-07T09:55:01.8842103Z Task         : Get sources
2020-04-07T09:55:01.8842419Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T09:55:01.8843283Z Version      : 1.0.0
2020-04-07T09:55:01.8843495Z Author       : Microsoft
2020-04-07T09:55:01.8843495Z Author       : Microsoft
2020-04-07T09:55:01.8843816Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-07T09:55:01.8844553Z ==============================================================================
2020-04-07T09:55:02.2143970Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-07T09:55:02.2192927Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70042/merge to s
2020-04-07T09:55:02.2287976Z Cleaning up task key
2020-04-07T09:55:02.2289475Z Start cleaning up orphan processes.
2020-04-07T09:55:02.2483586Z Terminate orphan process: pid (3923) (python)
2020-04-07T09:55:02.2653674Z ##[section]Finishing: Finalize Job
