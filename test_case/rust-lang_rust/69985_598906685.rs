plain
2020-03-13T20:47:59.7246822Z ========================== Starting Command Output ===========================
2020-03-13T20:47:59.7249619Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e2df2781-94cb-4ff6-a3ea-5fa1282c16fc.sh
2020-03-13T20:47:59.7249906Z 
2020-03-13T20:47:59.7254248Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-13T20:47:59.7274270Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-03-13T20:47:59.7277738Z Task         : Get sources
2020-03-13T20:47:59.7278081Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T20:47:59.7278391Z Version      : 1.0.0
2020-03-13T20:47:59.7278602Z Author       : Microsoft
---
2020-03-13T20:48:00.7203825Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-13T20:48:00.7209000Z ##[command]git config gc.auto 0
2020-03-13T20:48:00.7212943Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-13T20:48:00.7216381Z ##[command]git config --get-all http.proxy
2020-03-13T20:48:00.7222521Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69985/merge:refs/remotes/pull/69985/merge
---
2020-03-13T20:52:45.0272787Z 
2020-03-13T20:52:45.0323628Z error: variable does not need to be mutable
2020-03-13T20:52:45.0324621Z    --> src/libcore/array/mod.rs:271:26
2020-03-13T20:52:45.0325178Z     |
2020-03-13T20:52:45.0325986Z 271 |     default fn from_iter(mut iter: IntoIter<T, N>) -> Self {
2020-03-13T20:52:45.0327830Z     |                          |
2020-03-13T20:52:45.0328726Z     |                          help: remove this `mut`
2020-03-13T20:52:45.0334190Z 
2020-03-13T20:52:45.0359752Z error: variable does not need to be mutable
2020-03-13T20:52:45.0359752Z error: variable does not need to be mutable
2020-03-13T20:52:45.0360521Z    --> src/libcore/array/mod.rs:274:13
2020-03-13T20:52:45.0361089Z     |
2020-03-13T20:52:45.0361855Z 274 |         let mut array: [MaybeUninit<T>; N] = iter.data;
2020-03-13T20:52:45.0363507Z     |             |
2020-03-13T20:52:45.0364283Z     |             help: remove this `mut`
2020-03-13T20:52:45.0369221Z 
2020-03-13T20:52:45.0369221Z 
2020-03-13T20:52:45.0460260Z error[E0509]: cannot move out of type `array::iter::IntoIter<T, N>`, which implements the `Drop` trait
2020-03-13T20:52:45.0461173Z    --> src/libcore/array/mod.rs:274:46
2020-03-13T20:52:45.0461729Z     |
2020-03-13T20:52:45.0462487Z 274 |         let mut array: [MaybeUninit<T>; N] = iter.data;
2020-03-13T20:52:45.0464271Z     |                                              |
2020-03-13T20:52:45.0465154Z     |                                              cannot move out of here
2020-03-13T20:52:45.0465154Z     |                                              cannot move out of here
2020-03-13T20:52:45.0466501Z     |                                              move occurs because `iter.data` has type `[mem::maybe_uninit::MaybeUninit<T>; _]`, which does not implement the `Copy` trait
2020-03-13T20:52:45.0467852Z     |                                              help: consider borrowing here: `&iter.data`
2020-03-13T20:52:47.4948369Z    Compiling backtrace-sys v0.1.32
2020-03-13T20:52:47.5602604Z error: aborting due to 4 previous errors
2020-03-13T20:52:47.5635600Z 
2020-03-13T20:52:47.5696035Z For more information about this error, try `rustc --explain E0509`.
---
2020-03-13T20:52:48.3787510Z   local time: Fri Mar 13 20:52:48 UTC 2020
2020-03-13T20:52:48.6678775Z   network time: Fri, 13 Mar 2020 20:52:48 GMT
2020-03-13T20:52:48.6679132Z == end clock drift check ==
2020-03-13T20:52:51.1456906Z 
2020-03-13T20:52:51.1527825Z ##[error]Bash exited with code '1'.
2020-03-13T20:52:51.1573211Z ##[section]Finishing: Run build
2020-03-13T20:52:51.1627723Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-03-13T20:52:51.1633102Z Task         : Get sources
2020-03-13T20:52:51.1633483Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T20:52:51.1633870Z Version      : 1.0.0
2020-03-13T20:52:51.1634119Z Author       : Microsoft
2020-03-13T20:52:51.1634119Z Author       : Microsoft
2020-03-13T20:52:51.1634507Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-13T20:52:51.1634973Z ==============================================================================
2020-03-13T20:52:51.5122429Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-13T20:52:51.5170353Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-03-13T20:52:51.5264301Z Cleaning up task key
2020-03-13T20:52:51.5265691Z Start cleaning up orphan processes.
2020-03-13T20:52:51.5518405Z Terminate orphan process: pid (4043) (python)
2020-03-13T20:52:51.5684389Z ##[section]Finishing: Finalize Job
