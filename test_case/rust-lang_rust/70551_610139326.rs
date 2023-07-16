plain
2020-04-07T02:08:17.3814201Z ========================== Starting Command Output ===========================
2020-04-07T02:08:17.3817605Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b01bae61-efba-450b-9ba4-c58da1fd576b.sh
2020-04-07T02:08:17.3817944Z 
2020-04-07T02:08:17.3823361Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-07T02:08:17.3871424Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-04-07T02:08:17.3874612Z Task         : Get sources
2020-04-07T02:08:17.3874838Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T02:08:17.3875056Z Version      : 1.0.0
2020-04-07T02:08:17.3875266Z Author       : Microsoft
---
2020-04-07T02:08:18.3776441Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-07T02:08:18.3782908Z ##[command]git config gc.auto 0
2020-04-07T02:08:18.3789114Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-07T02:08:18.3792635Z ##[command]git config --get-all http.proxy
2020-04-07T02:08:18.3798218Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70551/merge:refs/remotes/pull/70551/merge
---
2020-04-07T02:10:35.2171782Z Looks like docker image is the same as before, not uploading
2020-04-07T02:10:42.5815738Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-07T02:10:42.6121580Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-07T02:10:42.6149071Z == clock drift check ==
2020-04-07T02:10:42.6164286Z   local time: Tue Apr  7 02:10:42 UTC 2020
2020-04-07T02:10:42.9091457Z   network time: Tue, 07 Apr 2020 02:10:42 GMT
2020-04-07T02:10:42.9123674Z Starting sccache server...
2020-04-07T02:10:42.9955083Z configure: processing command line
2020-04-07T02:10:42.9955968Z configure: 
2020-04-07T02:10:42.9957149Z configure: rust.dist-src        := False
---
2020-04-07T02:16:00.0874270Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-07T02:16:01.5366089Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-07T02:16:03.0619408Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-07T02:16:04.5679226Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-07T02:16:12.7804326Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-07T02:16:15.6280362Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-07T02:16:19.9992130Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-07T02:16:24.0328479Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-07T02:16:33.0500249Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-07T02:34:07.6175209Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2020-04-07T02:34:07.6202567Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-07T02:34:07.8762017Z    Compiling cc v1.0.50
2020-04-07T02:34:07.8811423Z    Compiling core v0.0.0 (/checkout/src/libcore)
2020-04-07T02:34:12.9173840Z error: internal compiler error: src/librustc_typeck/variance/constraints.rs:354: unexpected type encountered in variance inference: FreshTy(0)
2020-04-07T02:34:12.9178883Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:880:9
2020-04-07T02:34:12.9179527Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-07T02:34:12.9179912Z 
2020-04-07T02:34:12.9180354Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-07T02:34:12.9180354Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-07T02:34:12.9180729Z 
2020-04-07T02:34:12.9181624Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-07T02:34:12.9182819Z note: rustc 1.44.0-nightly (42f8a18b9 2020-04-07) running on x86_64-unknown-linux-gnu
2020-04-07T02:34:12.9183261Z 
2020-04-07T02:34:12.9183261Z 
2020-04-07T02:34:12.9184371Z note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-04-07T02:34:12.9186320Z note: some of the compiler flags provided by cargo are hidden
2020-04-07T02:34:12.9187649Z 
2020-04-07T02:34:13.4098568Z    Compiling libc v0.2.66
2020-04-07T02:34:13.9890846Z    Compiling autocfg v0.1.7
---
2020-04-07T02:34:15.1884579Z expected success, got: exit code: 101
2020-04-07T02:34:15.1893410Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-07T02:34:15.1894253Z Build completed unsuccessfully in 0:21:52
2020-04-07T02:34:15.1953428Z == clock drift check ==
2020-04-07T02:34:15.1976568Z   local time: Tue Apr  7 02:34:15 UTC 2020
2020-04-07T02:34:15.4932745Z   network time: Tue, 07 Apr 2020 02:34:15 GMT
2020-04-07T02:34:16.9647508Z 
2020-04-07T02:34:16.9647508Z 
2020-04-07T02:34:16.9720202Z ##[error]Bash exited with code '1'.
2020-04-07T02:34:16.9734688Z ##[section]Finishing: Run build
2020-04-07T02:34:16.9781796Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-04-07T02:34:16.9786111Z Task         : Get sources
2020-04-07T02:34:16.9786368Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T02:34:16.9786620Z Version      : 1.0.0
2020-04-07T02:34:16.9786786Z Author       : Microsoft
2020-04-07T02:34:16.9786786Z Author       : Microsoft
2020-04-07T02:34:16.9787361Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-07T02:34:16.9787685Z ==============================================================================
2020-04-07T02:34:17.3158721Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-07T02:34:17.3199821Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70551/merge to s
2020-04-07T02:34:17.3282164Z Cleaning up task key
2020-04-07T02:34:17.3283653Z Start cleaning up orphan processes.
2020-04-07T02:34:17.3479746Z Terminate orphan process: pid (3959) (python)
2020-04-07T02:34:17.3719101Z ##[section]Finishing: Finalize Job
