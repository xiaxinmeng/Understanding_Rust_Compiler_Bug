plain
2020-04-08T17:53:44.9097063Z ========================== Starting Command Output ===========================
2020-04-08T17:53:44.9100227Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d1e47edf-f71a-4ba3-8190-482fa6ef0b85.sh
2020-04-08T17:53:44.9100760Z 
2020-04-08T17:53:44.9105741Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-08T17:53:44.9126707Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70855/merge to s
2020-04-08T17:53:44.9130079Z Task         : Get sources
2020-04-08T17:53:44.9130380Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-08T17:53:44.9130649Z Version      : 1.0.0
2020-04-08T17:53:44.9130829Z Author       : Microsoft
---
2020-04-08T17:53:45.8962714Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-08T17:53:45.8967305Z ##[command]git config gc.auto 0
2020-04-08T17:53:45.8970534Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-08T17:53:45.8973473Z ##[command]git config --get-all http.proxy
2020-04-08T17:53:45.8978800Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70855/merge:refs/remotes/pull/70855/merge
---
2020-04-08T17:58:47.5979810Z  ---> 3fc1b512c57b
2020-04-08T17:58:47.6026408Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-08T17:58:47.6027331Z  ---> Using cache
2020-04-08T17:58:47.6027680Z  ---> 5ee4295733f4
2020-04-08T17:58:47.6029175Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-08T17:58:47.6031579Z  ---> 3d07a0fa42fe
2020-04-08T17:58:47.6031781Z Successfully built 3d07a0fa42fe
2020-04-08T17:58:47.6060124Z Successfully tagged rust-ci:latest
2020-04-08T17:58:47.6538288Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-08T17:58:47.6538288Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-08T17:58:47.6554511Z Looks like docker image is the same as before, not uploading
2020-04-08T17:58:57.3214633Z [CI_JOB_NAME=mingw-check]
2020-04-08T17:58:57.3477189Z [CI_JOB_NAME=mingw-check]
2020-04-08T17:58:57.3506262Z == clock drift check ==
2020-04-08T17:58:57.3510792Z   local time: Wed Apr  8 17:58:57 UTC 2020
2020-04-08T17:58:57.6340905Z   network time: Wed, 08 Apr 2020 17:58:57 GMT
2020-04-08T17:58:57.6355387Z Starting sccache server...
2020-04-08T17:58:57.7131934Z configure: processing command line
2020-04-08T17:58:57.7132161Z configure: 
2020-04-08T17:58:57.7132955Z configure: rust.parallel-compiler := True
---
2020-04-08T18:02:37.0431069Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-08T18:02:37.2491420Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-08T18:02:37.4698084Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-08T18:02:37.4825754Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-08T18:02:38.1429585Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-08T18:02:40.5614901Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-08T18:02:41.0918835Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-08T18:02:43.5661899Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-08T18:02:43.7695422Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-08T18:04:33.6798580Z configure: build.locked-deps    := True
2020-04-08T18:04:33.6798877Z configure: llvm.ccache          := sccache
2020-04-08T18:04:33.6799387Z configure: build.cargo-native-static := True
2020-04-08T18:04:33.6799866Z configure: dist.missing-tools   := True
2020-04-08T18:04:33.6800458Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-08T18:04:33.6801210Z configure: writing `config.toml` in current directory
2020-04-08T18:04:33.6801452Z configure: 
2020-04-08T18:04:33.6801929Z configure: run `python /checkout/x.py --help`
2020-04-08T18:04:33.6802158Z configure: 
---
2020-04-08T18:05:58.2840531Z Hugepagesize:       2048 kB
2020-04-08T18:05:58.2840771Z DirectMap4k:      135104 kB
2020-04-08T18:05:58.2840997Z DirectMap2M:     4059136 kB
2020-04-08T18:05:58.2841221Z DirectMap1G:     5242880 kB
2020-04-08T18:05:58.2841929Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-08T18:05:59.6721136Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-08T18:05:59.6721136Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-08T18:05:59.6728425Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-08T18:05:59.9151250Z    Compiling unicode-xid v0.2.0
2020-04-08T18:06:00.0343399Z    Compiling syn v1.0.11
2020-04-08T18:06:00.9218249Z    Compiling linked-hash-map v0.5.2
2020-04-08T18:06:00.9493967Z    Compiling lazy_static v1.4.0
2020-04-08T18:06:00.9493967Z    Compiling lazy_static v1.4.0
2020-04-08T18:06:01.1589221Z    Compiling yaml-rust v0.4.3
2020-04-08T18:06:05.6580901Z    Compiling quote v1.0.2
2020-04-08T18:06:21.0801671Z    Compiling thiserror-impl v1.0.5
2020-04-08T18:06:25.9804564Z    Compiling thiserror v1.0.5
2020-04-08T18:06:26.0401189Z    Compiling yaml-merge-keys v0.4.0
2020-04-08T18:06:27.2767273Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-08T18:06:31.7435286Z Build completed successfully in 0:00:33
2020-04-08T18:06:31.7443915Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-08T18:06:31.9922111Z     Finished dev [unoptimized] target(s) in 0.18s
2020-04-08T18:06:33.0844761Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-08T18:08:43.5832519Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-08T18:08:43.6761873Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-08T18:08:43.8769048Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-08T18:08:44.0022675Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-08T18:08:44.5047729Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-08T18:08:46.7948132Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-08T18:08:47.2774211Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-08T18:08:49.3627660Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-08T18:08:49.8220666Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-08T18:12:56.5298590Z Done!
2020-04-08T18:12:56.5299445Z some tidy checks failed
2020-04-08T18:12:56.5299649Z 
2020-04-08T18:12:56.5299753Z 
2020-04-08T18:12:56.5325008Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-08T18:12:56.5325805Z 
2020-04-08T18:12:56.5325900Z 
2020-04-08T18:12:56.5326151Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-08T18:12:56.5326502Z Build completed unsuccessfully in 0:00:36
2020-04-08T18:12:56.5326502Z Build completed unsuccessfully in 0:00:36
2020-04-08T18:12:56.5373772Z == clock drift check ==
2020-04-08T18:12:56.5400532Z   local time: Wed Apr  8 18:12:56 UTC 2020
2020-04-08T18:12:56.8319109Z   network time: Wed, 08 Apr 2020 18:12:56 GMT
2020-04-08T18:12:59.2309603Z 
2020-04-08T18:12:59.2309603Z 
2020-04-08T18:12:59.2385197Z ##[error]Bash exited with code '1'.
2020-04-08T18:12:59.2415461Z ##[section]Finishing: Run build
2020-04-08T18:12:59.2464951Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70855/merge to s
2020-04-08T18:12:59.2469673Z Task         : Get sources
2020-04-08T18:12:59.2469979Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-08T18:12:59.2470261Z Version      : 1.0.0
2020-04-08T18:12:59.2470474Z Author       : Microsoft
2020-04-08T18:12:59.2470474Z Author       : Microsoft
2020-04-08T18:12:59.2470784Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-08T18:12:59.2471293Z ==============================================================================
2020-04-08T18:12:59.5856586Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-08T18:12:59.5898910Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70855/merge to s
2020-04-08T18:12:59.5990810Z Cleaning up task key
2020-04-08T18:12:59.5992065Z Start cleaning up orphan processes.
2020-04-08T18:12:59.6188095Z Terminate orphan process: pid (3734) (python)
2020-04-08T18:12:59.6416657Z ##[section]Finishing: Finalize Job
