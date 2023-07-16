plain
2020-02-12T03:42:50.7331339Z ========================== Starting Command Output ===========================
2020-02-12T03:42:50.7333350Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c0481a41-7933-4ed4-8154-56d13d486a72.sh
2020-02-12T03:42:50.7333389Z 
2020-02-12T03:42:50.7336886Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-12T03:42:50.7342746Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69084/merge to s
2020-02-12T03:42:50.7344208Z Task         : Get sources
2020-02-12T03:42:50.7344254Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-12T03:42:50.7344287Z Version      : 1.0.0
2020-02-12T03:42:50.7344540Z Author       : Microsoft
---
2020-02-12T03:42:51.8331524Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-12T03:42:51.8345576Z ##[command]git config gc.auto 0
2020-02-12T03:42:51.8348006Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-12T03:42:51.8351250Z ##[command]git config --get-all http.proxy
2020-02-12T03:42:51.8358222Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69084/merge:refs/remotes/pull/69084/merge
---
2020-02-12T04:12:51.4997231Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2020-02-12T04:12:51.5016978Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-12T04:12:51.9532691Z    Compiling cc v1.0.50
2020-02-12T04:12:51.9533062Z    Compiling core v0.0.0 (/checkout/src/libcore)
2020-02-12T04:12:52.3733717Z thread 'rustc' panicked at 'src/librustc_lint/context.rs:179: duplicate specification of lint unused_doc_comments', src/librustc/util/bug.rs:37:26
2020-02-12T04:12:52.3741021Z 
2020-02-12T04:12:52.3745001Z error: internal compiler error: unexpected panic
2020-02-12T04:12:52.3747773Z 
2020-02-12T04:12:52.3751935Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-12T04:12:52.3751935Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-12T04:12:52.3753299Z 
2020-02-12T04:12:52.3759067Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-12T04:12:52.3763368Z note: rustc 1.43.0-nightly (932f50894 2020-02-12) running on x86_64-unknown-linux-gnu
2020-02-12T04:12:52.3764497Z 
2020-02-12T04:12:52.3764497Z 
2020-02-12T04:12:52.3770128Z note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib
2020-02-12T04:12:52.3774086Z note: some of the compiler flags provided by cargo are hidden
2020-02-12T04:12:52.3815228Z 
2020-02-12T04:12:52.4168956Z error: could not compile `core`.
2020-02-12T04:12:52.4179956Z warning: build failed, waiting for other jobs to finish...
---
2020-02-12T04:12:58.7923765Z   local time: Wed Feb 12 04:12:58 UTC 2020
2020-02-12T04:12:58.8851931Z   network time: Wed, 12 Feb 2020 04:12:58 GMT
2020-02-12T04:12:58.8854570Z == end clock drift check ==
2020-02-12T04:13:00.3423539Z 
2020-02-12T04:13:00.3518173Z ##[error]Bash exited with code '1'.
2020-02-12T04:13:00.3529979Z ##[section]Finishing: Run build
2020-02-12T04:13:00.3551814Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69084/merge to s
2020-02-12T04:13:00.3553398Z Task         : Get sources
2020-02-12T04:13:00.3553440Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-12T04:13:00.3553498Z Version      : 1.0.0
2020-02-12T04:13:00.3553533Z Author       : Microsoft
2020-02-12T04:13:00.3553533Z Author       : Microsoft
2020-02-12T04:13:00.3553575Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-12T04:13:00.3553648Z ==============================================================================
2020-02-12T04:13:00.7926765Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-12T04:13:00.7972887Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69084/merge to s
2020-02-12T04:13:00.8089911Z Cleaning up task key
2020-02-12T04:13:00.8090814Z Start cleaning up orphan processes.
2020-02-12T04:13:00.8235017Z Terminate orphan process: pid (3460) (python)
2020-02-12T04:13:00.8878082Z ##[section]Finishing: Finalize Job
