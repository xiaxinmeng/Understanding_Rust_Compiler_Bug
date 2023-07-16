plain
2020-04-02T21:08:00.7376621Z ========================== Starting Command Output ===========================
2020-04-02T21:08:00.7378964Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/453a0567-d692-46b7-9e7d-9527d54e4f25.sh
2020-04-02T21:08:00.7379375Z 
2020-04-02T21:08:00.7384329Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-02T21:08:00.7401823Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-02T21:08:00.7404992Z Task         : Get sources
2020-04-02T21:08:00.7405243Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-02T21:08:00.7405484Z Version      : 1.0.0
2020-04-02T21:08:00.7405695Z Author       : Microsoft
---
2020-04-02T21:08:01.9880680Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-02T21:08:01.9888965Z ##[command]git config gc.auto 0
2020-04-02T21:08:01.9894925Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-02T21:08:01.9900999Z ##[command]git config --get-all http.proxy
2020-04-02T21:08:01.9911754Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70705/merge:refs/remotes/pull/70705/merge
---
2020-04-02T21:10:03.6475072Z Looks like docker image is the same as before, not uploading
2020-04-02T21:10:11.7464224Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-02T21:10:11.7709774Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-02T21:10:11.7735568Z == clock drift check ==
2020-04-02T21:10:11.7743266Z   local time: Thu Apr  2 21:10:11 UTC 2020
2020-04-02T21:10:11.8497827Z   network time: Thu, 02 Apr 2020 21:10:11 GMT
2020-04-02T21:10:11.8525225Z Starting sccache server...
2020-04-02T21:10:11.9327217Z configure: processing command line
2020-04-02T21:10:11.9330160Z configure: 
2020-04-02T21:10:11.9330958Z configure: rust.dist-src        := False
---
2020-04-02T21:15:07.5779777Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-02T21:15:09.0677972Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-02T21:15:10.6207405Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-02T21:15:12.0073135Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-02T21:15:20.6858340Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-02T21:15:23.2411084Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-02T21:15:27.6894434Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-02T21:15:31.8216927Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-02T21:15:41.1652616Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-02T21:37:12.0043195Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-02T21:37:12.0047780Z 
2020-04-02T21:37:12.0055750Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-02T21:37:12.0060673Z 
2020-04-02T21:37:12.0067433Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-02T21:37:12.0078416Z note: rustc 1.44.0-nightly (965aba1d0 2020-04-02) running on x86_64-unknown-linux-gnu
2020-04-02T21:37:12.0082141Z 
2020-04-02T21:37:12.0082141Z 
2020-04-02T21:37:12.0091815Z note: compiler flags: -Z macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-04-02T21:37:12.0100968Z note: some of the compiler flags provided by cargo are hidden
2020-04-02T21:37:12.0111812Z 
2020-04-02T21:37:12.0491517Z error: aborting due to previous error
2020-04-02T21:37:12.0495501Z 
---
2020-04-02T21:37:24.7515716Z expected success, got: exit code: 101
2020-04-02T21:37:24.7522972Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-02T21:37:24.7523697Z Build completed unsuccessfully in 0:25:45
2020-04-02T21:37:24.7569796Z == clock drift check ==
2020-04-02T21:37:24.7589290Z   local time: Thu Apr  2 21:37:24 UTC 2020
2020-04-02T21:37:25.3992616Z   network time: Thu, 02 Apr 2020 21:37:24 GMT
2020-04-02T21:37:25.5216060Z 
2020-04-02T21:37:25.5216060Z 
2020-04-02T21:37:25.5285238Z ##[error]Bash exited with code '1'.
2020-04-02T21:37:25.5297215Z ##[section]Finishing: Run build
2020-04-02T21:37:25.5343694Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-02T21:37:25.5348494Z Task         : Get sources
2020-04-02T21:37:25.5348796Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-02T21:37:25.5349071Z Version      : 1.0.0
2020-04-02T21:37:25.5349280Z Author       : Microsoft
2020-04-02T21:37:25.5349280Z Author       : Microsoft
2020-04-02T21:37:25.5349589Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-02T21:37:25.5350152Z ==============================================================================
2020-04-02T21:37:25.8504012Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-02T21:37:25.8558605Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-02T21:37:25.8646988Z Cleaning up task key
2020-04-02T21:37:25.8648093Z Start cleaning up orphan processes.
2020-04-02T21:37:25.8826412Z Terminate orphan process: pid (8881) (python)
2020-04-02T21:37:25.9089173Z ##[section]Finishing: Finalize Job
