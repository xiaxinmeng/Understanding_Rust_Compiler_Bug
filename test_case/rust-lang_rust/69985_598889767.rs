plain
2020-03-13T19:55:48.6860092Z ========================== Starting Command Output ===========================
2020-03-13T19:55:48.6862838Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ed693292-3c9d-4fc0-b28e-b0a10ac5bd8f.sh
2020-03-13T19:55:48.6863121Z 
2020-03-13T19:55:48.6866605Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-13T19:55:48.6887779Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-03-13T19:55:48.6891412Z Task         : Get sources
2020-03-13T19:55:48.6891754Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T19:55:48.6892065Z Version      : 1.0.0
2020-03-13T19:55:48.6892276Z Author       : Microsoft
---
2020-03-13T19:55:49.7049536Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-13T19:55:49.7058969Z ##[command]git config gc.auto 0
2020-03-13T19:55:49.7066125Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-13T19:55:49.7075547Z ##[command]git config --get-all http.proxy
2020-03-13T19:55:49.7085485Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69985/merge:refs/remotes/pull/69985/merge
---
2020-03-13T20:01:03.7882711Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-03-13T20:01:06.5047337Z error[E0412]: cannot find type `I` in this scope
2020-03-13T20:01:06.5048575Z    --> src/libcore/array/mod.rs:271:36
2020-03-13T20:01:06.5049766Z     |
2020-03-13T20:01:06.5050686Z 268 | impl<T, const N: usize> ArrayFromIter<T, IntoIter<T, N>> for [T; N] 
2020-03-13T20:01:06.5051807Z     |      - similarly named type parameter `T` defined here
2020-03-13T20:01:06.5053446Z 271 |     default fn from_iter(mut iter: I) -> Self {
2020-03-13T20:01:06.5054731Z     |                                    ^ help: a type parameter with a similar name exists: `T`
2020-03-13T20:01:06.5055386Z 
2020-03-13T20:01:11.0524147Z error[E0061]: this function takes 1 parameter but 0 parameters were supplied
2020-03-13T20:01:11.0524147Z error[E0061]: this function takes 1 parameter but 0 parameters were supplied
2020-03-13T20:01:11.0533310Z    --> src/libcore/array/mod.rs:253:40
2020-03-13T20:01:11.0552769Z     |
2020-03-13T20:01:11.0554006Z 253 |           for (p, v) in array.iter_mut().zip() {
2020-03-13T20:01:11.0556480Z     | 
2020-03-13T20:01:11.0557451Z    ::: src/libcore/iter/traits/iterator.rs:544:5
2020-03-13T20:01:11.0558232Z     |
2020-03-13T20:01:11.0558232Z     |
2020-03-13T20:01:11.0559262Z 544 | /     fn zip<U>(self, other: U) -> Zip<Self, U::IntoIter>
2020-03-13T20:01:11.0561584Z 546 | |         Self: Sized,
2020-03-13T20:01:11.0562783Z 547 | |         U: IntoIterator,
2020-03-13T20:01:11.0563861Z 548 | |     {
2020-03-13T20:01:11.0565020Z 549 | |         Zip::new(self, other.into_iter())
---
2020-03-13T20:01:14.8832508Z   local time: Fri Mar 13 20:01:14 UTC 2020
2020-03-13T20:01:15.5770287Z   network time: Fri, 13 Mar 2020 20:01:15 GMT
2020-03-13T20:01:15.5775647Z == end clock drift check ==
2020-03-13T20:01:21.2926693Z 
2020-03-13T20:01:21.3013528Z ##[error]Bash exited with code '1'.
2020-03-13T20:01:21.3029253Z ##[section]Finishing: Run build
2020-03-13T20:01:21.3085478Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-03-13T20:01:21.3090858Z Task         : Get sources
2020-03-13T20:01:21.3091308Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T20:01:21.3091674Z Version      : 1.0.0
2020-03-13T20:01:21.3091928Z Author       : Microsoft
2020-03-13T20:01:21.3091928Z Author       : Microsoft
2020-03-13T20:01:21.3092361Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-13T20:01:21.3092840Z ==============================================================================
2020-03-13T20:01:21.6805779Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-13T20:01:21.6853597Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-03-13T20:01:21.6947780Z Cleaning up task key
2020-03-13T20:01:21.6949035Z Start cleaning up orphan processes.
2020-03-13T20:01:21.7271326Z Terminate orphan process: pid (4185) (python)
2020-03-13T20:01:21.7298303Z ##[section]Finishing: Finalize Job
