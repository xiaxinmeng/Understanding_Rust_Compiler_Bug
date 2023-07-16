plain
2020-04-28T17:22:10.8014944Z ========================== Starting Command Output ===========================
2020-04-28T17:22:10.8017239Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0f273ff3-4df7-4639-8efb-0b310f538c5b.sh
2020-04-28T17:22:10.8017601Z 
2020-04-28T17:22:10.8021036Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-28T17:22:10.8039015Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71542/merge to s
2020-04-28T17:22:10.8042107Z Task         : Get sources
2020-04-28T17:22:10.8042385Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-28T17:22:10.8042656Z Version      : 1.0.0
2020-04-28T17:22:10.8042838Z Author       : Microsoft
---
2020-04-28T17:22:11.7968063Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-28T17:22:11.7974593Z ##[command]git config gc.auto 0
2020-04-28T17:22:11.7978057Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-28T17:22:11.7981229Z ##[command]git config --get-all http.proxy
2020-04-28T17:22:11.7987193Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71542/merge:refs/remotes/pull/71542/merge
---
2020-04-28T17:24:44.5086015Z  ---> cb2676f08729
2020-04-28T17:24:44.5086625Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-28T17:24:44.5091463Z  ---> Using cache
2020-04-28T17:24:44.5091781Z  ---> df25ce111862
2020-04-28T17:24:44.5092552Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-28T17:24:44.5097771Z  ---> 599b9ac96b27
2020-04-28T17:24:44.5098130Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-28T17:24:44.5102998Z  ---> Using cache
2020-04-28T17:24:44.5103314Z  ---> 091087e35a36
---
2020-04-28T17:24:44.5445441Z Looks like docker image is the same as before, not uploading
2020-04-28T17:24:51.9271472Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-28T17:24:51.9535750Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-28T17:24:51.9573826Z == clock drift check ==
2020-04-28T17:24:51.9574875Z   local time: Tue Apr 28 17:24:51 UTC 2020
2020-04-28T17:24:51.9993984Z   network time: Tue, 28 Apr 2020 17:24:51 GMT
2020-04-28T17:24:52.0040700Z Starting sccache server...
2020-04-28T17:24:52.0882951Z configure: processing command line
2020-04-28T17:24:52.0883323Z configure: 
2020-04-28T17:24:52.0884508Z configure: rust.dist-src        := False
---
2020-04-28T17:27:01.5546371Z    Compiling unicode-width v0.1.6
2020-04-28T17:27:01.6286945Z    Compiling getopts v0.2.21
2020-04-28T17:27:10.8516794Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-28T17:27:18.0391867Z     Finished release [optimized] target(s) in 53.92s
2020-04-28T17:27:18.0397174Z {"reason":"build-finished","success":true}
2020-04-28T17:27:18.0634936Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-28T17:27:18.5927812Z    Compiling cfg-if v0.1.10
2020-04-28T17:27:18.5928832Z    Compiling libc v0.2.69
2020-04-28T17:27:18.6333486Z    Compiling semver-parser v0.7.0
---
2020-04-28T17:29:33.4889798Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-28T17:29:34.7532656Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-28T17:29:36.1179312Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-28T17:29:36.3237549Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-28T17:29:44.7128308Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-28T17:29:46.0968483Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-28T17:29:49.7878009Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-28T17:29:53.3177029Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-28T17:30:02.1323149Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-28T17:42:50.3815995Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-28T17:43:16.7829086Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-28T17:45:52.2562833Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-28T17:45:52.8516020Z     Finished release [optimized] target(s) in 18m 34s
2020-04-28T17:45:52.8517224Z {"reason":"build-finished","success":true}
2020-04-28T17:45:52.9048288Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2020-04-28T17:45:52.9062142Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-28T17:45:53.1868918Z    Compiling cc v1.0.50
2020-04-28T17:45:53.1870415Z    Compiling core v0.0.0 (/checkout/src/libcore)
---
2020-04-28T17:46:34.8151215Z    Compiling unicode-width v0.1.6
2020-04-28T17:46:34.9073023Z    Compiling getopts v0.2.21
2020-04-28T17:46:44.6099798Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-28T17:46:52.4470546Z     Finished release [optimized] target(s) in 59.53s
2020-04-28T17:46:52.4471667Z {"reason":"build-finished","success":true}
2020-04-28T17:46:52.4609087Z Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-28T17:46:53.3483009Z    Compiling cfg-if v0.1.10
2020-04-28T17:46:53.3487174Z    Compiling libc v0.2.69
2020-04-28T17:46:53.3884435Z    Compiling semver-parser v0.7.0
---
2020-04-28T17:48:36.9006210Z    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2020-04-28T17:48:37.0973815Z error: identifier pair considered confusable between `I` and `l`
2020-04-28T17:48:37.0974625Z   --> src/librustc_data_structures/fingerprint.rs:64:14
2020-04-28T17:48:37.0975116Z    |
2020-04-28T17:48:37.0975780Z 64 |         let [l, r]: [u64; 2] = unsafe { mem::transmute(bytes) };
2020-04-28T17:48:37.0976962Z    | 
2020-04-28T17:48:37.0977512Z   ::: src/librustc_data_structures/box_region.rs:33:28
2020-04-28T17:48:37.0978020Z    |
2020-04-28T17:48:37.0978020Z    |
2020-04-28T17:48:37.0978558Z 33 | pub struct PinnedGenerator<I, A, R> {
2020-04-28T17:48:37.0979367Z    |                            - this is where the previous identifier occurred
2020-04-28T17:48:37.0980558Z    = note: `-D confusable-idents` implied by `-D warnings`
2020-04-28T17:48:37.0985537Z 
2020-04-28T17:48:37.9772782Z error: aborting due to previous error
2020-04-28T17:48:37.9773524Z 
2020-04-28T17:48:37.9773524Z 
2020-04-28T17:48:37.9843401Z error: could not compile `rustc_data_structures`.
2020-04-28T17:48:37.9843667Z 
2020-04-28T17:48:37.9844187Z To learn more, run the command again with --verbose.
2020-04-28T17:48:37.9875532Z warning: build failed, waiting for other jobs to finish...
2020-04-28T17:48:40.7511263Z {"reason":"build-finished","success":false}
2020-04-28T17:48:40.7666458Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-28T17:48:40.7668131Z expected success, got: exit code: 101
2020-04-28T17:48:40.7670417Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-28T17:48:40.7670979Z Build completed unsuccessfully in 0:22:18
2020-04-28T17:48:40.7670979Z Build completed unsuccessfully in 0:22:18
2020-04-28T17:48:40.7768327Z == clock drift check ==
2020-04-28T17:48:40.7792981Z   local time: Tue Apr 28 17:48:40 UTC 2020
2020-04-28T17:48:40.9410115Z   network time: Tue, 28 Apr 2020 17:48:40 GMT
2020-04-28T17:48:41.6478530Z 
2020-04-28T17:48:41.6478530Z 
2020-04-28T17:48:41.6543955Z ##[error]Bash exited with code '1'.
2020-04-28T17:48:41.6569446Z ##[section]Finishing: Run build
2020-04-28T17:48:41.6613463Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71542/merge to s
2020-04-28T17:48:41.6618156Z Task         : Get sources
2020-04-28T17:48:41.6618464Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-28T17:48:41.6618754Z Version      : 1.0.0
2020-04-28T17:48:41.6618952Z Author       : Microsoft
2020-04-28T17:48:41.6618952Z Author       : Microsoft
2020-04-28T17:48:41.6619275Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-28T17:48:41.6619638Z ==============================================================================
2020-04-28T17:48:41.9805866Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-28T17:48:41.9849393Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71542/merge to s
2020-04-28T17:48:41.9942107Z Cleaning up task key
2020-04-28T17:48:41.9943378Z Start cleaning up orphan processes.
2020-04-28T17:48:42.0146143Z Terminate orphan process: pid (3531) (python)
2020-04-28T17:48:42.0288609Z ##[section]Finishing: Finalize Job
