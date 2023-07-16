plain
2020-01-30T20:32:06.7748153Z ========================== Starting Command Output ===========================
2020-01-30T20:32:06.7749635Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/48bba453-eb8c-40db-aba1-8b1493e06fab.sh
2020-01-30T20:32:06.7749671Z 
2020-01-30T20:32:06.7752282Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-30T20:32:06.7758275Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68679/merge to s
2020-01-30T20:32:06.7759852Z Task         : Get sources
2020-01-30T20:32:06.7759888Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-30T20:32:06.7759924Z Version      : 1.0.0
2020-01-30T20:32:06.7760005Z Author       : Microsoft
---
2020-01-30T20:32:07.5430654Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-30T20:32:07.5580010Z ##[command]git config gc.auto 0
2020-01-30T20:32:07.5671595Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-30T20:32:07.5731918Z ##[command]git config --get-all http.proxy
2020-01-30T20:32:07.5871233Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68679/merge:refs/remotes/pull/68679/merge
---
2020-01-30T20:56:43.1323160Z    Compiling byteorder v1.3.2
2020-01-30T20:56:43.3771219Z    Compiling log v0.4.8
2020-01-30T20:56:44.6206649Z    Compiling nodrop v0.1.12
2020-01-30T20:56:45.4234105Z    Compiling smallvec v1.0.0
2020-01-30T20:56:45.6702108Z error: internal compiler error: src/librustc_traits/normalize_erasing_regions.rs:34: could not fully normalize `<A as Array>::Item`
2020-01-30T20:56:45.6702598Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:883:9
2020-01-30T20:56:45.6702693Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-30T20:56:45.6702768Z 
2020-01-30T20:56:46.3748928Z note: the compiler unexpectedly panicked. this is a bug.
2020-01-30T20:56:46.3748928Z note: the compiler unexpectedly panicked. this is a bug.
2020-01-30T20:56:46.3749013Z 
2020-01-30T20:56:46.3752238Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-01-30T20:56:46.3753388Z note: rustc 1.42.0-nightly (45a7b1537 2020-01-30) running on x86_64-unknown-linux-gnu
2020-01-30T20:56:46.3753764Z 
2020-01-30T20:56:46.3753764Z 
2020-01-30T20:56:46.3754390Z note: compiler flags: -Z external-macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-01-30T20:56:46.3754540Z note: some of the compiler flags provided by cargo are hidden
2020-01-30T20:56:46.3754580Z 
2020-01-30T20:56:46.3754896Z error: aborting due to previous error
2020-01-30T20:56:46.3754939Z 
---
2020-01-30T20:56:46.3757930Z   local time: Thu Jan 30 20:56:45 UTC 2020
2020-01-30T20:56:46.3757991Z   network time: Thu, 30 Jan 2020 20:56:45 GMT
2020-01-30T20:56:46.3758049Z == end clock drift check ==
2020-01-30T20:56:46.5462772Z 
2020-01-30T20:56:46.5520560Z ##[error]Bash exited with code '1'.
2020-01-30T20:56:46.5532491Z ##[section]Finishing: Run build
2020-01-30T20:56:46.5555108Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68679/merge to s
2020-01-30T20:56:46.5557269Z Task         : Get sources
2020-01-30T20:56:46.5557319Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-30T20:56:46.5557368Z Version      : 1.0.0
2020-01-30T20:56:46.5557430Z Author       : Microsoft
2020-01-30T20:56:46.5557430Z Author       : Microsoft
2020-01-30T20:56:46.5557479Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-30T20:56:46.5557533Z ==============================================================================
2020-01-30T20:56:46.9391020Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-30T20:56:46.9431647Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68679/merge to s
2020-01-30T20:56:46.9538573Z Cleaning up task key
2020-01-30T20:56:46.9539592Z Start cleaning up orphan processes.
2020-01-30T20:56:46.9639229Z Terminate orphan process: pid (3624) (python)
2020-01-30T20:56:46.9837376Z ##[section]Finishing: Finalize Job
