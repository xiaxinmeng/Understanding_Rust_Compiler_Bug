plain
2020-03-13T19:16:52.7389291Z ========================== Starting Command Output ===========================
2020-03-13T19:16:52.7394740Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/76d219df-cda1-4c39-bc29-c880b762c5c4.sh
2020-03-13T19:16:52.7395270Z 
2020-03-13T19:16:52.7416286Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-13T19:16:52.7437258Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-03-13T19:16:52.7441006Z Task         : Get sources
2020-03-13T19:16:52.7441331Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T19:16:52.7441663Z Version      : 1.0.0
2020-03-13T19:16:52.7441879Z Author       : Microsoft
---
2020-03-13T19:16:55.0262326Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-13T19:16:55.0409420Z ##[command]git config gc.auto 0
2020-03-13T19:16:55.0461521Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-13T19:16:55.0500491Z ##[command]git config --get-all http.proxy
2020-03-13T19:16:55.0628388Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69985/merge:refs/remotes/pull/69985/merge
---
2020-03-13T19:22:25.0755812Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-03-13T19:22:27.4513405Z error[E0433]: failed to resolve: use of undeclared type or module `core`
2020-03-13T19:22:27.4514209Z    --> src/libcore/array/mod.rs:238:13
2020-03-13T19:22:27.4514771Z     |
2020-03-13T19:22:27.4515545Z 238 |             core::ptr::read(&array as *const [MaybeUninit<T>; N] as *const [T; N])
2020-03-13T19:22:27.4516939Z 
2020-03-13T19:22:27.4533299Z error[E0433]: failed to resolve: use of undeclared type or module `core`
2020-03-13T19:22:27.4534044Z    --> src/libcore/array/mod.rs:261:13
2020-03-13T19:22:27.4534861Z     |
2020-03-13T19:22:27.4534861Z     |
2020-03-13T19:22:27.4535632Z 261 |             core::ptr::read(&array as *const [MaybeUninit<T>; N] as *const [T; N])
2020-03-13T19:22:27.4536990Z 
2020-03-13T19:22:27.4571993Z error[E0433]: failed to resolve: use of undeclared type or module `core`
2020-03-13T19:22:27.4572951Z    --> src/libcore/array/mod.rs:280:13
2020-03-13T19:22:27.4573519Z     |
2020-03-13T19:22:27.4573519Z     |
2020-03-13T19:22:27.4574290Z 280 |             core::ptr::read(&array as *const [MaybeUninit<T>; N] as *const [T; N])
2020-03-13T19:22:27.4575656Z 
2020-03-13T19:22:27.6746862Z error[E0412]: cannot find type `I` in this scope
2020-03-13T19:22:27.6748103Z    --> src/libcore/array/mod.rs:271:36
2020-03-13T19:22:27.6748875Z     |
2020-03-13T19:22:27.6748875Z     |
2020-03-13T19:22:27.6749785Z 268 | impl<T, const N: usize> ArrayFromIter<T, IntoIter<T, N>> for [T; N] 
2020-03-13T19:22:27.6750963Z     |      - similarly named type parameter `T` defined here
2020-03-13T19:22:27.6752576Z 271 |     default fn from_iter(mut iter: I) -> Self {
2020-03-13T19:22:27.6755522Z     |                                    ^ help: a type parameter with a similar name exists: `T`
2020-03-13T19:22:27.6756274Z 
2020-03-13T19:22:32.2021122Z error[E0061]: this function takes 1 parameter but 0 parameters were supplied
2020-03-13T19:22:32.2021122Z error[E0061]: this function takes 1 parameter but 0 parameters were supplied
2020-03-13T19:22:32.2022700Z    --> src/libcore/array/mod.rs:253:40
2020-03-13T19:22:32.2023678Z     |
2020-03-13T19:22:32.2024893Z 253 |           for (p, v) in array.iter_mut().zip() {
2020-03-13T19:22:32.2028615Z     | 
2020-03-13T19:22:32.2030561Z    ::: src/libcore/iter/traits/iterator.rs:544:5
2020-03-13T19:22:32.2031537Z     |
2020-03-13T19:22:32.2031537Z     |
2020-03-13T19:22:32.2034862Z 544 | /     fn zip<U>(self, other: U) -> Zip<Self, U::IntoIter>
2020-03-13T19:22:32.2038295Z 546 | |         Self: Sized,
2020-03-13T19:22:32.2039368Z 547 | |         U: IntoIterator,
2020-03-13T19:22:32.2040564Z 548 | |     {
2020-03-13T19:22:32.2041740Z 549 | |         Zip::new(self, other.into_iter())
---
2020-03-13T19:22:35.9064628Z   local time: Fri Mar 13 19:22:35 UTC 2020
2020-03-13T19:22:36.4249705Z   network time: Fri, 13 Mar 2020 19:22:36 GMT
2020-03-13T19:22:36.4253377Z == end clock drift check ==
2020-03-13T19:22:40.0698702Z 
2020-03-13T19:22:40.0815663Z ##[error]Bash exited with code '1'.
2020-03-13T19:22:40.0830281Z ##[section]Finishing: Run build
2020-03-13T19:22:40.0888004Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-03-13T19:22:40.0893353Z Task         : Get sources
2020-03-13T19:22:40.0893768Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T19:22:40.0894155Z Version      : 1.0.0
2020-03-13T19:22:40.0894410Z Author       : Microsoft
2020-03-13T19:22:40.0894410Z Author       : Microsoft
2020-03-13T19:22:40.0894831Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-13T19:22:40.0895334Z ==============================================================================
2020-03-13T19:22:40.4406405Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-13T19:22:40.4453688Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-03-13T19:22:40.4541545Z Cleaning up task key
2020-03-13T19:22:40.4542809Z Start cleaning up orphan processes.
2020-03-13T19:22:40.4758269Z Terminate orphan process: pid (4692) (python)
2020-03-13T19:22:40.4901436Z ##[section]Finishing: Finalize Job
