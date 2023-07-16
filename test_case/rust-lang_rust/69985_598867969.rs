plain
2020-03-13T18:51:13.0156401Z ========================== Starting Command Output ===========================
2020-03-13T18:51:13.0160399Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/343cc657-5ff9-484c-86c0-e50f8a997765.sh
2020-03-13T18:51:13.0160899Z 
2020-03-13T18:51:13.0165306Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-13T18:51:13.0187715Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-03-13T18:51:13.0191820Z Task         : Get sources
2020-03-13T18:51:13.0192210Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T18:51:13.0192570Z Version      : 1.0.0
2020-03-13T18:51:13.0192812Z Author       : Microsoft
---
2020-03-13T18:51:14.2927534Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-13T18:51:14.2935376Z ##[command]git config gc.auto 0
2020-03-13T18:51:14.2940142Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-13T18:51:14.2944787Z ##[command]git config --get-all http.proxy
2020-03-13T18:51:14.2969057Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69985/merge:refs/remotes/pull/69985/merge
---
2020-03-13T18:55:38.2762873Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-03-13T18:55:40.2636382Z error[E0433]: failed to resolve: use of undeclared type or module `std`
2020-03-13T18:55:40.2638012Z    --> src/libcore/array/mod.rs:238:13
2020-03-13T18:55:40.2638810Z     |
2020-03-13T18:55:40.2639583Z 238 |             std::ptr::read(&array as *const [MaybeUninit<T>; N] as *const [T; N])
2020-03-13T18:55:40.2640907Z 
2020-03-13T18:55:40.2657998Z error[E0433]: failed to resolve: use of undeclared type or module `std`
2020-03-13T18:55:40.2658968Z    --> src/libcore/array/mod.rs:261:13
2020-03-13T18:55:40.2660571Z     |
2020-03-13T18:55:40.2660571Z     |
2020-03-13T18:55:40.2661485Z 261 |             std::ptr::read(&array as *const [MaybeUninit<T>; N] as *const [T; N])
2020-03-13T18:55:40.2697453Z 
2020-03-13T18:55:40.2698754Z error[E0433]: failed to resolve: use of undeclared type or module `std`
2020-03-13T18:55:40.2699621Z    --> src/libcore/array/mod.rs:280:13
2020-03-13T18:55:40.2700244Z     |
2020-03-13T18:55:40.2700244Z     |
2020-03-13T18:55:40.2701141Z 280 |             std::ptr::read(&array as *const [MaybeUninit<T>; N] as *const [T; N])
2020-03-13T18:55:40.2702698Z 
2020-03-13T18:55:40.4772817Z error[E0412]: cannot find type `I` in this scope
2020-03-13T18:55:40.4773492Z    --> src/libcore/array/mod.rs:271:36
2020-03-13T18:55:40.4773950Z     |
2020-03-13T18:55:40.4773950Z     |
2020-03-13T18:55:40.4774631Z 268 | impl<T, const N: usize> ArrayFromIter<T, IntoIter<T, N>> for [T; N] 
2020-03-13T18:55:40.4775490Z     |      - similarly named type parameter `T` defined here
2020-03-13T18:55:40.4776619Z 271 |     default fn from_iter(mut iter: I) -> Self {
2020-03-13T18:55:40.4777616Z     |                                    ^ help: a type parameter with a similar name exists: `T`
2020-03-13T18:55:40.4778231Z 
2020-03-13T18:55:44.2472452Z error[E0061]: this function takes 1 parameter but 0 parameters were supplied
2020-03-13T18:55:44.2472452Z error[E0061]: this function takes 1 parameter but 0 parameters were supplied
2020-03-13T18:55:44.2474209Z    --> src/libcore/array/mod.rs:253:40
2020-03-13T18:55:44.2475581Z     |
2020-03-13T18:55:44.2477050Z 253 |           for (p, v) in array.iter_mut().zip() {
2020-03-13T18:55:44.2480384Z     | 
2020-03-13T18:55:44.2483787Z    ::: src/libcore/iter/traits/iterator.rs:544:5
2020-03-13T18:55:44.2484912Z     |
2020-03-13T18:55:44.2484912Z     |
2020-03-13T18:55:44.2486306Z 544 | /     fn zip<U>(self, other: U) -> Zip<Self, U::IntoIter>
2020-03-13T18:55:44.2489171Z 546 | |         Self: Sized,
2020-03-13T18:55:44.2490387Z 547 | |         U: IntoIterator,
2020-03-13T18:55:44.2492198Z 548 | |     {
2020-03-13T18:55:44.2493958Z 549 | |         Zip::new(self, other.into_iter())
---
2020-03-13T18:55:48.0165409Z   local time: Fri Mar 13 18:55:48 UTC 2020
2020-03-13T18:55:48.1788205Z   network time: Fri, 13 Mar 2020 18:55:48 GMT
2020-03-13T18:55:48.1789630Z == end clock drift check ==
2020-03-13T18:55:51.4470442Z 
2020-03-13T18:55:51.4514204Z ##[error]Bash exited with code '1'.
2020-03-13T18:55:51.4527680Z ##[section]Finishing: Run build
2020-03-13T18:55:51.4579906Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-03-13T18:55:51.4584842Z Task         : Get sources
2020-03-13T18:55:51.4585198Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T18:55:51.4585535Z Version      : 1.0.0
2020-03-13T18:55:51.4585782Z Author       : Microsoft
2020-03-13T18:55:51.4585782Z Author       : Microsoft
2020-03-13T18:55:51.4586142Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-13T18:55:51.4586714Z ==============================================================================
2020-03-13T18:55:51.7848093Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-13T18:55:51.7893152Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-03-13T18:55:51.7973568Z Cleaning up task key
2020-03-13T18:55:51.7974997Z Start cleaning up orphan processes.
2020-03-13T18:55:51.8273944Z Terminate orphan process: pid (4841) (python)
2020-03-13T18:55:51.8299550Z ##[section]Finishing: Finalize Job
