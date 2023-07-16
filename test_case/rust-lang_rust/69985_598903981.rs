plain
2020-03-13T20:38:20.4436954Z ========================== Starting Command Output ===========================
2020-03-13T20:38:20.4441081Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/10471c96-d939-41a2-b45d-4802a3213656.sh
2020-03-13T20:38:20.4441308Z 
2020-03-13T20:38:20.4447130Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-13T20:38:20.4467633Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-03-13T20:38:20.4470718Z Task         : Get sources
2020-03-13T20:38:20.4470954Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T20:38:20.4471182Z Version      : 1.0.0
2020-03-13T20:38:20.4471337Z Author       : Microsoft
---
2020-03-13T20:38:22.7925571Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-13T20:38:22.7993564Z ##[command]git config gc.auto 0
2020-03-13T20:38:22.8010058Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-13T20:38:22.8043197Z ##[command]git config --get-all http.proxy
2020-03-13T20:38:22.8137426Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69985/merge:refs/remotes/pull/69985/merge
---
2020-03-13T20:44:28.0729267Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-03-13T20:44:28.0730076Z error[E0061]: this function takes 1 parameter but 0 parameters were supplied
2020-03-13T20:44:28.0730821Z    --> src/libcore/array/mod.rs:253:40
2020-03-13T20:44:28.0731333Z     |
2020-03-13T20:44:28.0732048Z 253 |           for (p, v) in array.iter_mut().zip() {
2020-03-13T20:44:28.0733635Z     | 
2020-03-13T20:44:28.0734248Z    ::: src/libcore/iter/traits/iterator.rs:544:5
2020-03-13T20:44:28.0734782Z     |
2020-03-13T20:44:28.0734782Z     |
2020-03-13T20:44:28.0735562Z 544 | /     fn zip<U>(self, other: U) -> Zip<Self, U::IntoIter>
2020-03-13T20:44:28.0737283Z 546 | |         Self: Sized,
2020-03-13T20:44:28.0738213Z 547 | |         U: IntoIterator,
2020-03-13T20:44:28.0739042Z 548 | |     {
2020-03-13T20:44:28.0739925Z 549 | |         Zip::new(self, other.into_iter())
---
2020-03-13T20:44:32.4810391Z   local time: Fri Mar 13 20:44:32 UTC 2020
2020-03-13T20:44:32.6875781Z   network time: Fri, 13 Mar 2020 20:44:32 GMT
2020-03-13T20:44:32.6876149Z == end clock drift check ==
2020-03-13T20:44:34.7374134Z 
2020-03-13T20:44:34.7453782Z ##[error]Bash exited with code '1'.
2020-03-13T20:44:34.7466427Z ##[section]Finishing: Run build
2020-03-13T20:44:34.7521673Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-03-13T20:44:34.7526592Z Task         : Get sources
2020-03-13T20:44:34.7526924Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T20:44:34.7527207Z Version      : 1.0.0
2020-03-13T20:44:34.7527411Z Author       : Microsoft
2020-03-13T20:44:34.7527411Z Author       : Microsoft
2020-03-13T20:44:34.7527742Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-13T20:44:34.7528363Z ==============================================================================
2020-03-13T20:44:35.1087334Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-13T20:44:35.1131114Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-03-13T20:44:35.1223296Z Cleaning up task key
2020-03-13T20:44:35.1225174Z Start cleaning up orphan processes.
2020-03-13T20:44:35.1549910Z Terminate orphan process: pid (4343) (python)
2020-03-13T20:44:35.1579843Z ##[section]Finishing: Finalize Job
