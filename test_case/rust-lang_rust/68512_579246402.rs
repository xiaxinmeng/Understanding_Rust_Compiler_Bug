plain
2020-01-28T13:16:16.9164021Z ========================== Starting Command Output ===========================
2020-01-28T13:16:16.9166334Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/952ef9e6-9add-4dc7-8d23-23d135b160db.sh
2020-01-28T13:16:16.9166488Z 
2020-01-28T13:16:16.9170234Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-28T13:16:16.9176249Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68512/merge to s
2020-01-28T13:16:16.9178670Z Task         : Get sources
2020-01-28T13:16:16.9178709Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-28T13:16:16.9178788Z Version      : 1.0.0
2020-01-28T13:16:16.9178827Z Author       : Microsoft
---
2020-01-28T13:16:17.7674119Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-28T13:16:17.7778954Z ##[command]git config gc.auto 0
2020-01-28T13:16:17.7859303Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-28T13:16:17.7935483Z ##[command]git config --get-all http.proxy
2020-01-28T13:16:17.8085998Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68512/merge:refs/remotes/pull/68512/merge
---
2020-01-28T13:24:58.1565412Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2020-01-28T13:24:58.6649485Z error[E0614]: type `rustc::mir::Local` cannot be dereferenced
2020-01-28T13:24:58.6649867Z     --> src/librustc_mir_build/build/matches/mod.rs:1261:28
2020-01-28T13:24:58.6650151Z      |
2020-01-28T13:24:58.6651023Z 1261 |                     local: *matched_place_ref.local,
2020-01-28T13:24:58.6658020Z 
2020-01-28T13:24:59.4380471Z error: aborting due to previous error
2020-01-28T13:24:59.4382139Z 
2020-01-28T13:24:59.4386233Z For more information about this error, try `rustc --explain E0614`.
2020-01-28T13:24:59.4386233Z For more information about this error, try `rustc --explain E0614`.
2020-01-28T13:24:59.4442302Z error: could not compile `rustc_mir_build`.
2020-01-28T13:24:59.4442667Z warning: build failed, waiting for other jobs to finish...
2020-01-28T13:25:00.9442849Z error[E0614]: type `rustc::mir::Local` cannot be dereferenced
2020-01-28T13:25:00.9444077Z    --> src/librustc_mir/borrow_check/diagnostics/conflict_errors.rs:702:28
2020-01-28T13:25:00.9444662Z     |
2020-01-28T13:25:00.9445334Z 702 |             Place { local: *root_place.local, projection: root_place_projection },
2020-01-28T13:25:00.9446497Z 
2020-01-28T13:25:00.9454943Z error[E0614]: type `rustc::mir::Local` cannot be dereferenced
2020-01-28T13:25:00.9455650Z    --> src/librustc_mir/borrow_check/diagnostics/conflict_errors.rs:713:28
2020-01-28T13:25:00.9456194Z     |
2020-01-28T13:25:00.9456194Z     |
2020-01-28T13:25:00.9456837Z 713 |             Place { local: *root_place.local, projection: root_place_projection },
2020-01-28T13:25:00.9457899Z 
2020-01-28T13:25:09.8487900Z error: aborting due to 2 previous errors
2020-01-28T13:25:09.8488607Z 
2020-01-28T13:25:09.8489160Z For more information about this error, try `rustc --explain E0614`.
2020-01-28T13:25:09.8489160Z For more information about this error, try `rustc --explain E0614`.
2020-01-28T13:25:09.8703771Z error: could not compile `rustc_mir`.
2020-01-28T13:25:09.8704497Z 
2020-01-28T13:25:09.8704959Z To learn more, run the command again with --verbose.
2020-01-28T13:25:09.8741551Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-28T13:25:09.8752773Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-28T13:25:09.8753025Z Build completed unsuccessfully in 0:05:32
2020-01-28T13:25:09.8813839Z == clock drift check ==
2020-01-28T13:25:09.8823627Z   local time: Tue Jan 28 13:25:09 UTC 2020
2020-01-28T13:25:09.8823627Z   local time: Tue Jan 28 13:25:09 UTC 2020
2020-01-28T13:25:09.9394879Z   network time: Tue, 28 Jan 2020 13:25:09 GMT
2020-01-28T13:25:09.9399757Z == end clock drift check ==
2020-01-28T13:25:10.3082824Z 
2020-01-28T13:25:10.3178682Z ##[error]Bash exited with code '1'.
2020-01-28T13:25:10.3191089Z ##[section]Finishing: Run build
2020-01-28T13:25:10.3231371Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68512/merge to s
2020-01-28T13:25:10.3233164Z Task         : Get sources
2020-01-28T13:25:10.3233232Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-28T13:25:10.3233303Z Version      : 1.0.0
2020-01-28T13:25:10.3233350Z Author       : Microsoft
2020-01-28T13:25:10.3233350Z Author       : Microsoft
2020-01-28T13:25:10.3233417Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-28T13:25:10.3233474Z ==============================================================================
2020-01-28T13:25:10.7479641Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-28T13:25:10.7518870Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68512/merge to s
2020-01-28T13:25:10.7631691Z Cleaning up task key
2020-01-28T13:25:10.7632489Z Start cleaning up orphan processes.
2020-01-28T13:25:10.7739086Z Terminate orphan process: pid (3648) (python)
2020-01-28T13:25:10.7927865Z ##[section]Finishing: Finalize Job
