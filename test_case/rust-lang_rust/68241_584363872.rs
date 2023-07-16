plain
2020-02-10T21:05:08.8468422Z ========================== Starting Command Output ===========================
2020-02-10T21:05:08.8469845Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/84e11f83-7cc3-47f9-82a2-6e4dcedcb3c5.sh
2020-02-10T21:05:08.8638797Z 
2020-02-10T21:05:08.8697760Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-10T21:05:08.8702654Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68241/merge to s
2020-02-10T21:05:08.8704106Z Task         : Get sources
2020-02-10T21:05:08.8704136Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-10T21:05:08.8704179Z Version      : 1.0.0
2020-02-10T21:05:08.8704208Z Author       : Microsoft
---
2020-02-10T21:05:09.7131322Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-10T21:05:09.7298693Z ##[command]git config gc.auto 0
2020-02-10T21:05:09.7370911Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-10T21:05:09.7424351Z ##[command]git config --get-all http.proxy
2020-02-10T21:05:09.7569477Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68241/merge:refs/remotes/pull/68241/merge
---
2020-02-10T21:13:04.1833855Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2020-02-10T21:13:05.2089812Z error[E0425]: cannot find value `resume_arg` in this scope
2020-02-10T21:13:05.2090196Z    --> src/librustc_mir/borrow_check/mod.rs:699:41
2020-02-10T21:13:05.2090430Z     |
2020-02-10T21:13:05.2090747Z 699 |                 self.mutate_place(loc, (resume_arg, span), Deep, JustWrite, flow_state);
2020-02-10T21:13:05.2091183Z 
2020-02-10T21:13:06.7384915Z error[E0027]: pattern does not mention field `resume_arg`
2020-02-10T21:13:06.7385231Z    --> src/librustc_mir/borrow_check/mod.rs:691:13
2020-02-10T21:13:06.7385480Z     |
2020-02-10T21:13:06.7385480Z     |
2020-02-10T21:13:06.7385776Z 691 |             TerminatorKind::Yield { value: _, resume: _, drop: _ } if self.movable_generator => {
2020-02-10T21:13:06.7386133Z     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing field `resume_arg`
2020-02-10T21:13:07.5826410Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2020-02-10T21:13:09.1638247Z     Checking rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
2020-02-10T21:13:09.1647199Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-02-10T21:13:09.3420478Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2020-02-10T21:13:09.3420478Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2020-02-10T21:13:12.0972665Z error: aborting due to 2 previous errors
2020-02-10T21:13:12.0972761Z 
2020-02-10T21:13:12.0981137Z Some errors have detailed explanations: E0027, E0425.
2020-02-10T21:13:12.0988810Z For more information about an error, try `rustc --explain E0027`.
2020-02-10T21:13:12.1063880Z error: could not compile `rustc_mir`.
2020-02-10T21:13:12.1064657Z warning: build failed, waiting for other jobs to finish...
2020-02-10T21:13:12.4082703Z error: build failed
2020-02-10T21:13:12.4151656Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-10T21:13:12.4157396Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-02-10T21:13:12.4158334Z Build completed unsuccessfully in 0:05:40
2020-02-10T21:13:12.4207603Z == clock drift check ==
2020-02-10T21:13:12.4221174Z   local time: Mon Feb 10 21:13:12 UTC 2020
2020-02-10T21:13:12.4221174Z   local time: Mon Feb 10 21:13:12 UTC 2020
2020-02-10T21:13:12.7116448Z   network time: Mon, 10 Feb 2020 21:13:12 GMT
2020-02-10T21:13:12.7121855Z == end clock drift check ==
2020-02-10T21:13:13.4239881Z 
2020-02-10T21:13:13.4330413Z ##[error]Bash exited with code '1'.
2020-02-10T21:13:13.4341438Z ##[section]Finishing: Run build
2020-02-10T21:13:13.4354746Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68241/merge to s
2020-02-10T21:13:13.4356624Z Task         : Get sources
2020-02-10T21:13:13.4356667Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-10T21:13:13.4356724Z Version      : 1.0.0
2020-02-10T21:13:13.4358336Z Author       : Microsoft
2020-02-10T21:13:13.4358336Z Author       : Microsoft
2020-02-10T21:13:13.4358409Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-10T21:13:13.4358474Z ==============================================================================
2020-02-10T21:13:13.8377427Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-10T21:13:13.8420047Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68241/merge to s
2020-02-10T21:13:13.8535247Z Cleaning up task key
2020-02-10T21:13:13.8535981Z Start cleaning up orphan processes.
2020-02-10T21:13:13.8822313Z Terminate orphan process: pid (3969) (python)
2020-02-10T21:13:13.8846825Z ##[section]Finishing: Finalize Job
