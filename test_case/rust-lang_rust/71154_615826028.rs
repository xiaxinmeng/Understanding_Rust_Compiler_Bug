plain
2020-04-18T08:51:08.9311146Z ========================== Starting Command Output ===========================
2020-04-18T08:51:08.9313956Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3abf9820-710c-4cd5-853a-6e52a5f89eea.sh
2020-04-18T08:51:08.9314215Z 
2020-04-18T08:51:08.9324865Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-18T08:51:08.9348871Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-18T08:51:08.9352569Z Task         : Get sources
2020-04-18T08:51:08.9352867Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T08:51:08.9353155Z Version      : 1.0.0
2020-04-18T08:51:08.9353352Z Author       : Microsoft
---
2020-04-18T08:51:10.1184325Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-18T08:51:10.1193103Z ##[command]git config gc.auto 0
2020-04-18T08:51:10.1200230Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-18T08:51:10.1206000Z ##[command]git config --get-all http.proxy
2020-04-18T08:51:10.1217131Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71154/merge:refs/remotes/pull/71154/merge
---
2020-04-18T08:54:20.8942961Z  ---> 318032b5f0e2
2020-04-18T08:54:20.8943803Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-18T08:54:20.8944471Z  ---> Using cache
2020-04-18T08:54:20.8944823Z  ---> d44a858fd1ce
2020-04-18T08:54:20.8945858Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-18T08:54:20.8983982Z  ---> 58b910f50f5a
2020-04-18T08:54:20.8984222Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-18T08:54:20.8992390Z  ---> Using cache
2020-04-18T08:54:20.8996165Z  ---> ee7702aadba1
---
2020-04-18T08:54:20.9529457Z Looks like docker image is the same as before, not uploading
2020-04-18T08:54:28.4215730Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-18T08:54:28.4643517Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-18T08:54:28.4682755Z == clock drift check ==
2020-04-18T08:54:28.4692802Z   local time: Sat Apr 18 08:54:28 UTC 2020
2020-04-18T08:54:28.6633633Z   network time: Sat, 18 Apr 2020 08:54:28 GMT
2020-04-18T08:54:28.6677927Z Starting sccache server...
2020-04-18T08:54:28.7561354Z configure: processing command line
2020-04-18T08:54:28.7562081Z configure: 
2020-04-18T08:54:28.7568064Z configure: rust.dist-src        := False
---
2020-04-18T09:00:19.9663715Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T09:00:21.6450397Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T09:00:23.4104914Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T09:00:25.1621696Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T09:00:34.7309778Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T09:00:38.2730621Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T09:00:43.3541224Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T09:00:48.1240339Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T09:00:58.1206141Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T09:22:05.5995879Z error: internal compiler error: unexpected panic
2020-04-18T09:22:05.6000173Z 
2020-04-18T09:22:05.6009273Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-18T09:22:05.6013590Z 
2020-04-18T09:22:05.6022115Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-18T09:22:05.6037520Z note: rustc 1.44.0-nightly (c624c3858 2020-04-18) running on x86_64-unknown-linux-gnu
2020-04-18T09:22:05.6037800Z 
2020-04-18T09:22:05.6037800Z 
2020-04-18T09:22:05.6046940Z note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C panic=abort -C debug-assertions=no --crate-type lib
2020-04-18T09:22:05.6058459Z note: some of the compiler flags provided by cargo are hidden
2020-04-18T09:22:05.6058693Z 
2020-04-18T09:22:05.6163139Z error: could not compile `compiler_builtins`.
2020-04-18T09:22:06.2936207Z 
---
2020-04-18T09:22:10.9145986Z expected success, got: exit code: 101
2020-04-18T09:22:10.9155700Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-18T09:22:10.9156444Z Build completed unsuccessfully in 0:25:54
2020-04-18T09:22:10.9206813Z == clock drift check ==
2020-04-18T09:22:11.4310322Z   local time: Sat Apr 18 09:22:11 UTC 2020
2020-04-18T09:22:11.7503597Z   network time: Sat, 18 Apr 2020 09:22:11 GMT
2020-04-18T09:22:13.1327967Z 
2020-04-18T09:22:13.1327967Z 
2020-04-18T09:22:13.1410339Z ##[error]Bash exited with code '1'.
2020-04-18T09:22:13.1426840Z ##[section]Finishing: Run build
2020-04-18T09:22:13.1488515Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-18T09:22:13.1494023Z Task         : Get sources
2020-04-18T09:22:13.1494397Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T09:22:13.1494725Z Version      : 1.0.0
2020-04-18T09:22:13.1494955Z Author       : Microsoft
2020-04-18T09:22:13.1494955Z Author       : Microsoft
2020-04-18T09:22:13.1495335Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-18T09:22:13.1495751Z ==============================================================================
2020-04-18T09:22:13.5305435Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-18T09:22:13.5358911Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-18T09:22:13.5454424Z Cleaning up task key
2020-04-18T09:22:13.5455717Z Start cleaning up orphan processes.
2020-04-18T09:22:13.5692267Z Terminate orphan process: pid (5002) (python)
2020-04-18T09:22:13.5866887Z ##[section]Finishing: Finalize Job
