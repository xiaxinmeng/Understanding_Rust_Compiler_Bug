plain
2019-07-23T20:41:09.4738708Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-23T20:41:09.4926986Z ##[command]git config gc.auto 0
2019-07-23T20:41:09.4986482Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-23T20:41:09.5035194Z ##[command]git config --get-all http.proxy
2019-07-23T20:41:09.5167042Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62910/merge:refs/remotes/pull/62910/merge
---
2019-07-23T20:41:43.6810150Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-23T20:41:43.6812033Z 
2019-07-23T20:41:43.6812594Z   git checkout -b <new-branch-name>
2019-07-23T20:41:43.6812770Z 
2019-07-23T20:41:43.6812955Z HEAD is now at 5ba3d798d Merge 19d7b74797de3a5caf54880e0ada788258a31577 into 299ef86e1f8b3e53154f834115752c719b611fa1
2019-07-23T20:41:43.6946569Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-23T20:41:43.6950192Z ==============================================================================
2019-07-23T20:41:43.6950238Z Task         : Bash
2019-07-23T20:41:43.6950292Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-23T20:48:50.2981028Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-07-23T20:48:51.7014783Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-07-23T20:48:52.8952512Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-07-23T20:49:05.3699913Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-07-23T20:49:31.4611002Z error: unused variable: `syntax_globals`
2019-07-23T20:49:31.4611676Z    --> src/librustc/ty/query/job.rs:480:9
2019-07-23T20:49:31.4611885Z     |
2019-07-23T20:49:31.4612146Z 480 |     let syntax_globals = &*syntax_globals;
2019-07-23T20:49:31.4612658Z     |         ^^^^^^^^^^^^^^ help: consider prefixing with an underscore: `_syntax_globals`
2019-07-23T20:49:31.4613300Z     |
2019-07-23T20:49:31.4613781Z     = note: `-D unused-variables` implied by `-D warnings`
2019-07-23T20:49:47.0718618Z error: variable does not need to be mutable
2019-07-23T20:49:47.0718618Z error: variable does not need to be mutable
2019-07-23T20:49:47.0718932Z   --> src/librustc/ty/query/job.rs:84:17
2019-07-23T20:49:47.0719149Z    |
2019-07-23T20:49:47.0719651Z 84 |             let mut waiter = Lrc::new(QueryWaiter {
2019-07-23T20:49:47.0720202Z    |                 |
2019-07-23T20:49:47.0720592Z    |                 help: remove this `mut`
2019-07-23T20:49:47.0720625Z 
2019-07-23T20:49:47.1142355Z error: variable does not need to be mutable
2019-07-23T20:49:47.1142355Z error: variable does not need to be mutable
2019-07-23T20:49:47.1142643Z    --> src/librustc/ty/query/job.rs:435:13
2019-07-23T20:49:47.1142821Z     |
2019-07-23T20:49:47.1143045Z 435 |         let mut error = CycleError {
2019-07-23T20:49:47.1143964Z     |             |
2019-07-23T20:49:47.1144494Z     |             help: remove this `mut`
2019-07-23T20:49:47.1144537Z 
2019-07-23T20:49:49.5021674Z error: aborting due to 3 previous errors
2019-07-23T20:49:49.5021674Z error: aborting due to 3 previous errors
2019-07-23T20:49:49.5021770Z 
2019-07-23T20:49:49.8029963Z error: Could not compile `rustc`.
2019-07-23T20:49:49.8030437Z 
2019-07-23T20:49:49.8030727Z To learn more, run the command again with --verbose.
2019-07-23T20:49:49.8072247Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-23T20:49:49.8079710Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-23T20:49:49.8080466Z Build completed unsuccessfully in 0:05:11
2019-07-23T20:49:49.8080466Z Build completed unsuccessfully in 0:05:11
2019-07-23T20:49:50.2732674Z ##[error]Bash exited with code '1'.
2019-07-23T20:49:50.2763410Z ##[section]Starting: Checkout
2019-07-23T20:49:50.2765827Z ==============================================================================
2019-07-23T20:49:50.2765900Z Task         : Get sources
2019-07-23T20:49:50.2765950Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
