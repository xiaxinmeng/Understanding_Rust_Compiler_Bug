plain
2019-10-25T00:35:52.8274871Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-25T00:35:52.8495887Z ##[command]git config gc.auto 0
2019-10-25T00:35:52.8557800Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-25T00:35:52.8628558Z ##[command]git config --get-all http.proxy
2019-10-25T00:35:52.8759967Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65664/merge:refs/remotes/pull/65664/merge
---
2019-10-25T01:07:18.8916422Z    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
2019-10-25T01:07:19.4005213Z    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
2019-10-25T01:07:19.9183573Z    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
2019-10-25T01:07:20.4256900Z    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
2019-10-25T01:07:24.6612585Z error: internal compiler error: src/librustc_codegen_llvm/common.rs:295: missing allocation AllocId(9791)
2019-10-25T01:07:24.6613816Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:925:9
2019-10-25T01:07:24.6614115Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-25T01:07:24.6614360Z 
2019-10-25T01:07:24.6618397Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-25T01:07:24.6618397Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-25T01:07:24.6623052Z 
2019-10-25T01:07:24.6624357Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-25T01:07:24.6625486Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-25T01:07:24.6626034Z 
2019-10-25T01:07:24.6626034Z 
2019-10-25T01:07:24.6629434Z note: compiler flags: -Z external-macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=n --crate-type lib
2019-10-25T01:07:24.6630142Z note: some of the compiler flags provided by cargo are hidden
2019-10-25T01:07:24.6630690Z 
2019-10-25T01:07:24.9130397Z error: aborting due to previous error
2019-10-25T01:07:24.9134787Z 
---
2019-10-25T01:07:26.2773213Z   local time: Fri Oct 25 01:07:26 UTC 2019
2019-10-25T01:07:26.3646936Z   network time: Fri, 25 Oct 2019 01:07:26 GMT
2019-10-25T01:07:26.3647011Z == end clock drift check ==
2019-10-25T01:07:28.3984882Z 
2019-10-25T01:07:28.4090177Z ##[error]Bash exited with code '1'.
2019-10-25T01:07:28.4129772Z ##[section]Starting: Checkout
2019-10-25T01:07:28.4132880Z ==============================================================================
2019-10-25T01:07:28.4132964Z Task         : Get sources
2019-10-25T01:07:28.4133016Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
