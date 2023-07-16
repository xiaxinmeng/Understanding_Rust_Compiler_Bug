plain
2020-04-19T10:19:40.2724893Z ========================== Starting Command Output ===========================
2020-04-19T10:19:40.2740157Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9c0511a5-a0e8-4175-b529-8269d6ecf114.sh
2020-04-19T10:19:40.5201554Z 
2020-04-19T10:19:40.5273777Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-19T10:19:40.5302943Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-19T10:19:40.5313180Z Task         : Get sources
2020-04-19T10:19:40.5313630Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T10:19:40.5314082Z Version      : 1.0.0
2020-04-19T10:19:40.5314406Z Author       : Microsoft
---
2020-04-19T10:19:43.1010127Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-19T10:19:43.1210247Z ##[command]git config gc.auto 0
2020-04-19T10:19:43.1234742Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-19T10:19:43.1263865Z ##[command]git config --get-all http.proxy
2020-04-19T10:19:43.1339451Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71154/merge:refs/remotes/pull/71154/merge
---
2020-04-19T10:23:15.1690825Z  ---> 318032b5f0e2
2020-04-19T10:23:15.1692178Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-19T10:23:15.1693203Z  ---> Using cache
2020-04-19T10:23:15.1693771Z  ---> d44a858fd1ce
2020-04-19T10:23:15.1695416Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-19T10:23:15.1698524Z  ---> 58b910f50f5a
2020-04-19T10:23:15.1698905Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-19T10:23:15.1699567Z  ---> Using cache
2020-04-19T10:23:15.1700134Z  ---> ee7702aadba1
---
2020-04-19T10:23:15.2382496Z Looks like docker image is the same as before, not uploading
2020-04-19T10:23:23.6125865Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-19T10:23:23.6439822Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-19T10:23:23.6456117Z == clock drift check ==
2020-04-19T10:23:23.6474429Z   local time: Sun Apr 19 10:23:23 UTC 2020
2020-04-19T10:23:23.6737151Z   network time: Sun, 19 Apr 2020 10:23:23 GMT
2020-04-19T10:23:23.6768583Z Starting sccache server...
2020-04-19T10:23:23.7546764Z configure: processing command line
2020-04-19T10:23:23.7547161Z configure: 
2020-04-19T10:23:23.7548415Z configure: rust.dist-src        := False
---
2020-04-19T10:28:06.8087299Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T10:28:08.1217793Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T10:28:09.5937870Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T10:28:10.4159685Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T10:28:18.7782829Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T10:28:20.8806096Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T10:28:24.7944169Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T10:28:28.4649758Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T10:28:37.4675907Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T10:45:32.2179426Z error: internal compiler error: unexpected panic
2020-04-19T10:45:32.2180090Z 
2020-04-19T10:45:32.2180486Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-19T10:45:32.2180802Z 
2020-04-19T10:45:32.2181738Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-19T10:45:32.2182952Z note: rustc 1.44.0-nightly (59a754e6c 2020-04-19) running on x86_64-unknown-linux-gnu
2020-04-19T10:45:32.2183305Z 
2020-04-19T10:45:32.2183305Z 
2020-04-19T10:45:32.2184418Z note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C panic=abort -C debug-assertions=no --crate-type lib
2020-04-19T10:45:32.2185419Z note: some of the compiler flags provided by cargo are hidden
2020-04-19T10:45:32.2185737Z 
2020-04-19T10:45:32.2186323Z error: could not compile `compiler_builtins`.
2020-04-19T10:45:32.2186674Z 
---
2020-04-19T10:45:35.7325016Z expected success, got: exit code: 101
2020-04-19T10:45:35.7339921Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-19T10:45:35.7340323Z Build completed unsuccessfully in 0:20:46
2020-04-19T10:45:35.7391790Z == clock drift check ==
2020-04-19T10:45:35.7411142Z   local time: Sun Apr 19 10:45:35 UTC 2020
2020-04-19T10:45:35.8556380Z   network time: Sun, 19 Apr 2020 10:45:35 GMT
2020-04-19T10:45:38.3439126Z 
2020-04-19T10:45:38.3439126Z 
2020-04-19T10:45:38.3509646Z ##[error]Bash exited with code '1'.
2020-04-19T10:45:38.3543863Z ##[section]Finishing: Run build
2020-04-19T10:45:38.3590333Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-19T10:45:38.3595447Z Task         : Get sources
2020-04-19T10:45:38.3595764Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T10:45:38.3596069Z Version      : 1.0.0
2020-04-19T10:45:38.3596283Z Author       : Microsoft
2020-04-19T10:45:38.3596283Z Author       : Microsoft
2020-04-19T10:45:38.3596609Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-19T10:45:38.3596995Z ==============================================================================
2020-04-19T10:45:38.6969066Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-19T10:45:38.7019628Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-19T10:45:38.7107433Z Cleaning up task key
2020-04-19T10:45:38.7108693Z Start cleaning up orphan processes.
2020-04-19T10:45:38.7434911Z Terminate orphan process: pid (5089) (python)
2020-04-19T10:45:38.7478744Z ##[section]Finishing: Finalize Job
