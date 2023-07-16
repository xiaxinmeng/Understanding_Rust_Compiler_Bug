plain
2020-01-23T20:45:12.2083218Z ========================== Starting Command Output ===========================
2020-01-23T20:45:12.2086120Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/781ffe22-ceb2-42e3-99e4-4fab464016e0.sh
2020-01-23T20:45:12.2086272Z 
2020-01-23T20:45:12.2089425Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-23T20:45:12.2095352Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68494/merge to s
2020-01-23T20:45:12.2096987Z Task         : Get sources
2020-01-23T20:45:12.2097022Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-23T20:45:12.2097056Z Version      : 1.0.0
2020-01-23T20:45:12.2097137Z Author       : Microsoft
---
2020-01-23T20:45:12.9863677Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-23T20:45:12.9960923Z ##[command]git config gc.auto 0
2020-01-23T20:45:13.0041373Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-23T20:45:13.0096247Z ##[command]git config --get-all http.proxy
2020-01-23T20:45:13.0239869Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68494/merge:refs/remotes/pull/68494/merge
---
2020-01-23T21:09:24.1619245Z    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-01-23T21:09:29.7306621Z error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
2020-01-23T21:09:29.7308608Z     --> src/libstd/thread/mod.rs:1065:16
2020-01-23T21:09:29.7308935Z      |
2020-01-23T21:09:29.7309301Z 1065 |             if COUNTER == crate::u64::MAX {
2020-01-23T21:09:29.7309716Z      |                ^^^^^^^ dereference of raw pointer
2020-01-23T21:09:29.7309964Z      |
2020-01-23T21:09:29.7310348Z      = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
2020-01-23T21:09:29.7310716Z error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
2020-01-23T21:09:29.7310987Z     --> src/libstd/thread/mod.rs:1069:22
2020-01-23T21:09:29.7311205Z      |
2020-01-23T21:09:29.7311518Z 1069 |             let id = COUNTER;
2020-01-23T21:09:29.7311518Z 1069 |             let id = COUNTER;
2020-01-23T21:09:29.7312119Z      |                      ^^^^^^^ dereference of raw pointer
2020-01-23T21:09:29.7312380Z      |
2020-01-23T21:09:29.7312763Z      = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
2020-01-23T21:09:29.7313122Z error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
2020-01-23T21:09:29.7313392Z     --> src/libstd/thread/mod.rs:1070:13
2020-01-23T21:09:29.7313609Z      |
2020-01-23T21:09:29.7313900Z 1070 |             COUNTER += 1;
2020-01-23T21:09:29.7313900Z 1070 |             COUNTER += 1;
2020-01-23T21:09:29.7314341Z      |             ^^^^^^^ dereference of raw pointer
2020-01-23T21:09:29.7314566Z      |
2020-01-23T21:09:29.7314957Z      = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
2020-01-23T21:09:29.7344016Z error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
2020-01-23T21:09:29.7344365Z    --> src/libstd/time.rs:238:32
2020-01-23T21:09:29.7344590Z     |
2020-01-23T21:09:29.7344590Z     |
2020-01-23T21:09:29.7344907Z 238 |             let now = cmp::max(LAST_NOW, os_now);
2020-01-23T21:09:29.7345289Z     |                                ^^^^^^^^ dereference of raw pointer
2020-01-23T21:09:29.7345532Z     |
2020-01-23T21:09:29.7345914Z     = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
2020-01-23T21:09:29.7346352Z error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
2020-01-23T21:09:29.7346613Z    --> src/libstd/time.rs:239:13
2020-01-23T21:09:29.7346830Z     |
2020-01-23T21:09:29.7347182Z 239 |             LAST_NOW = now;
2020-01-23T21:09:29.7347182Z 239 |             LAST_NOW = now;
2020-01-23T21:09:29.7347503Z     |             ^^^^^^^^ dereference of raw pointer
2020-01-23T21:09:29.7347732Z     |
2020-01-23T21:09:29.7410230Z     = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
2020-01-23T21:09:29.7450199Z error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
2020-01-23T21:09:29.7450621Z   --> src/libstd/sys_common/at_exit_imp.rs:45:35
2020-01-23T21:09:29.7450849Z    |
2020-01-23T21:09:29.7450849Z    |
2020-01-23T21:09:29.7451198Z 45 |                 mem::replace(&mut QUEUE, if i == ITERS { DONE } else { ptr::null_mut() })
2020-01-23T21:09:29.7451545Z    |                                   ^^^^^ dereference of raw pointer
2020-01-23T21:09:29.7451794Z    |
2020-01-23T21:09:29.7452333Z    = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
2020-01-23T21:09:29.7452695Z error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
2020-01-23T21:09:29.7452978Z   --> src/libstd/sys_common/at_exit_imp.rs:69:15
2020-01-23T21:09:29.7453197Z    |
2020-01-23T21:09:29.7453197Z    |
2020-01-23T21:09:29.7453485Z 69 |             (*QUEUE).push(f);
2020-01-23T21:09:29.7453811Z    |               ^^^^^ dereference of raw pointer
2020-01-23T21:09:29.7454124Z    |
2020-01-23T21:09:29.7454521Z    = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
2020-01-23T21:09:29.7454862Z error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
2020-01-23T21:09:29.7455141Z    --> src/libstd/panicking.rs:117:24
2020-01-23T21:09:29.7455360Z     |
2020-01-23T21:09:29.7455360Z     |
2020-01-23T21:09:29.7455633Z 117 |         let old_hook = HOOK;
2020-01-23T21:09:29.7456280Z     |                        ^^^^ dereference of raw pointer
2020-01-23T21:09:29.7456545Z     |
2020-01-23T21:09:29.7456950Z     = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
2020-01-23T21:09:29.7457294Z error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
2020-01-23T21:09:29.7457569Z    --> src/libstd/panicking.rs:118:9
2020-01-23T21:09:29.7457786Z     |
2020-01-23T21:09:29.7457786Z     |
2020-01-23T21:09:29.7458077Z 118 |         HOOK = Hook::Custom(Box::into_raw(hook));
2020-01-23T21:09:29.7458407Z     |         ^^^^ dereference of raw pointer
2020-01-23T21:09:29.7458629Z     |
2020-01-23T21:09:29.7459017Z     = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
2020-01-23T21:09:29.7459373Z error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
2020-01-23T21:09:29.7459647Z    --> src/libstd/panicking.rs:165:20
2020-01-23T21:09:29.7459865Z     |
2020-01-23T21:09:29.7459865Z     |
2020-01-23T21:09:29.7460132Z 165 |         let hook = HOOK;
2020-01-23T21:09:29.7460468Z     |                    ^^^^ dereference of raw pointer
2020-01-23T21:09:29.7465541Z     |
2020-01-23T21:09:29.7465958Z     = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
2020-01-23T21:09:29.7466472Z error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
2020-01-23T21:09:29.7466752Z    --> src/libstd/panicking.rs:166:9
2020-01-23T21:09:29.7466990Z     |
2020-01-23T21:09:29.7466990Z     |
2020-01-23T21:09:29.7467266Z 166 |         HOOK = Hook::Default;
2020-01-23T21:09:29.7467594Z     |         ^^^^ dereference of raw pointer
2020-01-23T21:09:29.7467816Z     |
2020-01-23T21:09:29.7468193Z     = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
2020-01-23T21:09:29.7468671Z error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
2020-01-23T21:09:29.7468935Z    --> src/libstd/panicking.rs:462:15
2020-01-23T21:09:29.7469166Z     |
2020-01-23T21:09:29.7469432Z 462 |         match HOOK {
2020-01-23T21:09:29.7469432Z 462 |         match HOOK {
2020-01-23T21:09:29.7469752Z     |               ^^^^ dereference of raw pointer
2020-01-23T21:09:29.7469992Z     |
2020-01-23T21:09:29.7470367Z     = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
2020-01-23T21:09:29.7470720Z error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
2020-01-23T21:09:29.7471003Z    --> src/libstd/sys/unix/args.rs:139:17
2020-01-23T21:09:29.7471288Z     |
2020-01-23T21:09:29.7471556Z 139 |             (0..ARGC)
2020-01-23T21:09:29.7471556Z 139 |             (0..ARGC)
2020-01-23T21:09:29.7471878Z     |                 ^^^^ dereference of raw pointer
2020-01-23T21:09:29.7472112Z     |
2020-01-23T21:09:29.7472486Z     = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
2020-01-23T21:09:29.7747015Z error: aborting due to 13 previous errors
2020-01-23T21:09:29.7747142Z 
2020-01-23T21:09:29.7747434Z For more information about this error, try `rustc --explain E0133`.
2020-01-23T21:09:29.7921854Z error: could not compile `std`.
---
2020-01-23T21:09:29.8040940Z   local time: Thu Jan 23 21:09:29 UTC 2020
2020-01-23T21:09:30.0989660Z   network time: Thu, 23 Jan 2020 21:09:30 GMT
2020-01-23T21:09:30.0994915Z == end clock drift check ==
2020-01-23T21:09:31.5044751Z 
2020-01-23T21:09:31.5143265Z ##[error]Bash exited with code '1'.
2020-01-23T21:09:31.5156261Z ##[section]Finishing: Run build
2020-01-23T21:09:31.5179999Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68494/merge to s
2020-01-23T21:09:31.5182466Z Task         : Get sources
2020-01-23T21:09:31.5182517Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-23T21:09:31.5182566Z Version      : 1.0.0
2020-01-23T21:09:31.5182627Z Author       : Microsoft
2020-01-23T21:09:31.5182627Z Author       : Microsoft
2020-01-23T21:09:31.5182676Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-23T21:09:31.5182853Z ==============================================================================
2020-01-23T21:09:31.9439133Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-23T21:09:31.9476882Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68494/merge to s
2020-01-23T21:09:31.9589717Z Cleaning up task key
2020-01-23T21:09:31.9590577Z Start cleaning up orphan processes.
2020-01-23T21:09:31.9692970Z Terminate orphan process: pid (3498) (python)
2020-01-23T21:09:31.9889527Z ##[section]Finishing: Finalize Job
