plain
2020-04-28T14:00:40.4220226Z ========================== Starting Command Output ===========================
2020-04-28T14:00:40.4225286Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/49863417-6614-4100-abee-f0e63fbc80e0.sh
2020-04-28T14:00:40.4225768Z 
2020-04-28T14:00:40.4230938Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-28T14:00:40.4250784Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71542/merge to s
2020-04-28T14:00:40.4254826Z Task         : Get sources
2020-04-28T14:00:40.4255130Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-28T14:00:40.4255424Z Version      : 1.0.0
2020-04-28T14:00:40.4255644Z Author       : Microsoft
---
2020-04-28T14:00:41.4073747Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-28T14:00:41.4080187Z ##[command]git config gc.auto 0
2020-04-28T14:00:41.4084739Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-28T14:00:41.4088916Z ##[command]git config --get-all http.proxy
2020-04-28T14:00:41.4097429Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71542/merge:refs/remotes/pull/71542/merge
---
2020-04-28T14:02:50.3910110Z  ---> cb2676f08729
2020-04-28T14:02:50.3910937Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-28T14:02:50.3911590Z  ---> Using cache
2020-04-28T14:02:50.3911945Z  ---> df25ce111862
2020-04-28T14:02:50.3912946Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-28T14:02:50.3914064Z  ---> 599b9ac96b27
2020-04-28T14:02:50.3914279Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-28T14:02:50.3914670Z  ---> Using cache
2020-04-28T14:02:50.3915016Z  ---> 091087e35a36
---
2020-04-28T14:02:50.4315092Z Looks like docker image is the same as before, not uploading
2020-04-28T14:02:56.8328922Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-28T14:02:56.8645766Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-28T14:02:56.8680190Z == clock drift check ==
2020-04-28T14:02:56.8703232Z   local time: Tue Apr 28 14:02:56 UTC 2020
2020-04-28T14:02:56.9712067Z   network time: Tue, 28 Apr 2020 14:02:56 GMT
2020-04-28T14:02:56.9739434Z Starting sccache server...
2020-04-28T14:02:57.0633894Z configure: processing command line
2020-04-28T14:02:57.0637071Z configure: 
2020-04-28T14:02:57.0638182Z configure: rust.dist-src        := False
---
2020-04-28T14:05:27.1675916Z    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
2020-04-28T14:05:31.7351883Z    Compiling unicode-width v0.1.6
2020-04-28T14:05:31.8288589Z    Compiling getopts v0.2.21
2020-04-28T14:05:43.3596064Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-28T14:05:52.8669480Z {"reason":"build-finished","success":true}
2020-04-28T14:05:52.8803476Z Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-28T14:05:52.8996807Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-28T14:05:53.5587641Z    Compiling cfg-if v0.1.10
2020-04-28T14:05:53.5591617Z    Compiling libc v0.2.69
---
2020-04-28T14:08:40.7576655Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-28T14:08:42.3101892Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-28T14:08:43.9735907Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-28T14:08:45.2834628Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-28T14:08:54.8085554Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-28T14:08:57.3979539Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-28T14:09:02.4921964Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-28T14:09:07.2118950Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-28T14:09:18.0288357Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-28T14:26:22.6368111Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-28T14:26:23.4073488Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-28T14:29:13.0858747Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-28T14:29:13.7542091Z     Finished release [optimized] target(s) in 23m 20s
2020-04-28T14:29:13.7542748Z {"reason":"build-finished","success":true}
2020-04-28T14:29:13.8097110Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2020-04-28T14:29:13.8116512Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-28T14:29:14.1239514Z    Compiling cc v1.0.50
2020-04-28T14:29:14.1243777Z    Compiling core v0.0.0 (/checkout/src/libcore)
---
2020-04-28T14:30:03.3155312Z    Compiling term v0.0.0 (/checkout/src/libterm)
2020-04-28T14:30:08.0609833Z    Compiling unicode-width v0.1.6
2020-04-28T14:30:08.1639475Z    Compiling getopts v0.2.21
2020-04-28T14:30:21.2285941Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-28T14:30:31.7065336Z     Finished {"reason":"build-finished","success":true}
2020-04-28T14:30:31.7199969Z Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-28T14:30:31.7217571Z Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-28T14:30:32.4147538Z    Compiling cfg-if v0.1.10
2020-04-28T14:30:32.4148077Z    Compiling libc v0.2.69
---
2020-04-28T14:31:00.0306238Z    Compiling serialize v0.0.0 (/checkout/src/libserialize)
2020-04-28T14:31:00.1874635Z error: identifier pair considered confusable between `I` and `l`
2020-04-28T14:31:00.1876190Z     --> src/libserialize/json.rs:2393:38
2020-04-28T14:31:00.1877002Z      |
2020-04-28T14:31:00.1878579Z 2393 |                     Some(Json::Array(l)) => {
2020-04-28T14:31:00.1880676Z ...
2020-04-28T14:31:00.1880676Z ...
2020-04-28T14:31:00.1881453Z 2668 | tuple_impl! {A, B, C, D, E, F, G, H, I}
2020-04-28T14:31:00.1882483Z      |                                      - this is where the previous identifier occurred
2020-04-28T14:31:00.1884078Z      = note: `-D confusable-idents` implied by `-D warnings`
2020-04-28T14:31:00.1884534Z 
2020-04-28T14:31:00.2191685Z    Compiling block-padding v0.1.5
2020-04-28T14:31:00.4206465Z    Compiling itertools v0.8.0
2020-04-28T14:31:00.4206465Z    Compiling itertools v0.8.0
2020-04-28T14:31:01.3034940Z error: aborting due to previous error
2020-04-28T14:31:01.3035796Z 
2020-04-28T14:31:01.3082120Z error: could not compile `serialize`.
2020-04-28T14:31:01.3082606Z 
2020-04-28T14:31:01.3083371Z To learn more, run the command again with --verbose.
2020-04-28T14:31:01.3084159Z warning: build failed, waiting for other jobs to finish...
2020-04-28T14:31:02.2296419Z {"reason":"build-finished","success":false}
2020-04-28T14:31:02.2431572Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-28T14:31:02.2432864Z expected success, got: exit code: 101
2020-04-28T14:31:02.2449672Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-28T14:31:02.2450346Z Build completed unsuccessfully in 0:26:19
2020-04-28T14:31:02.2450346Z Build completed unsuccessfully in 0:26:19
2020-04-28T14:31:02.2506212Z == clock drift check ==
2020-04-28T14:31:02.2525151Z   local time: Tue Apr 28 14:31:02 UTC 2020
2020-04-28T14:31:02.4203673Z   network time: Tue, 28 Apr 2020 14:31:02 GMT
2020-04-28T14:31:03.2682365Z 
2020-04-28T14:31:03.2682365Z 
2020-04-28T14:31:03.2772371Z ##[error]Bash exited with code '1'.
2020-04-28T14:31:03.2787988Z ##[section]Finishing: Run build
2020-04-28T14:31:03.2836559Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71542/merge to s
2020-04-28T14:31:03.2842099Z Task         : Get sources
2020-04-28T14:31:03.2842468Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-28T14:31:03.2842788Z Version      : 1.0.0
2020-04-28T14:31:03.2843018Z Author       : Microsoft
2020-04-28T14:31:03.2843018Z Author       : Microsoft
2020-04-28T14:31:03.2843413Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-28T14:31:03.2843824Z ==============================================================================
2020-04-28T14:31:03.6720968Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-28T14:31:03.6775991Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71542/merge to s
2020-04-28T14:31:03.6885957Z Cleaning up task key
2020-04-28T14:31:03.6887429Z Start cleaning up orphan processes.
2020-04-28T14:31:03.7122919Z Terminate orphan process: pid (3803) (python)
2020-04-28T14:31:03.7411535Z ##[section]Finishing: Finalize Job
