plain
2020-02-04T19:39:39.8028009Z ========================== Starting Command Output ===========================
2020-02-04T19:39:39.8029411Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/dbbd57b7-dd81-4713-af8c-61a534573530.sh
2020-02-04T19:39:39.8029448Z 
2020-02-04T19:39:39.8032953Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-04T19:39:39.8038959Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68807/merge to s
2020-02-04T19:39:39.8040526Z Task         : Get sources
2020-02-04T19:39:39.8040558Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-04T19:39:39.8040606Z Version      : 1.0.0
2020-02-04T19:39:39.8040640Z Author       : Microsoft
---
2020-02-04T19:39:40.6506289Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-04T19:39:40.6602484Z ##[command]git config gc.auto 0
2020-02-04T19:39:40.6680018Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-04T19:39:40.6749232Z ##[command]git config --get-all http.proxy
2020-02-04T19:39:40.6900760Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68807/merge:refs/remotes/pull/68807/merge
---
2020-02-04T19:44:18.1712223Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-02-04T19:44:25.3585652Z error[E0599]: no method named `predeccessor_unchecked` found for type parameter `A` in the current scope
2020-02-04T19:44:25.3586244Z    --> src/libcore/iter/range.rs:760:39
2020-02-04T19:44:25.3586537Z     |
2020-02-04T19:44:25.3586848Z 760 |             let n = unsafe { self.end.predeccessor_unchecked() };
2020-02-04T19:44:25.3587239Z     |                                       ^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `predecessor_unchecked`
2020-02-04T19:44:28.0210052Z    Compiling libc v0.2.66
2020-02-04T19:44:28.2368416Z error: aborting due to previous error
2020-02-04T19:44:28.2369315Z 
2020-02-04T19:44:28.2372405Z For more information about this error, try `rustc --explain E0599`.
---
2020-02-04T19:44:29.8248173Z   local time: Tue Feb  4 19:44:29 UTC 2020
2020-02-04T19:44:29.9852723Z   network time: Tue, 04 Feb 2020 19:44:29 GMT
2020-02-04T19:44:29.9854935Z == end clock drift check ==
2020-02-04T19:44:32.8923253Z 
2020-02-04T19:44:32.9013532Z ##[error]Bash exited with code '1'.
2020-02-04T19:44:32.9025811Z ##[section]Finishing: Run build
2020-02-04T19:44:32.9044950Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68807/merge to s
2020-02-04T19:44:32.9046905Z Task         : Get sources
2020-02-04T19:44:32.9046950Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-04T19:44:32.9046994Z Version      : 1.0.0
2020-02-04T19:44:32.9047052Z Author       : Microsoft
2020-02-04T19:44:32.9047052Z Author       : Microsoft
2020-02-04T19:44:32.9047097Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-04T19:44:32.9047144Z ==============================================================================
2020-02-04T19:44:33.3294660Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-04T19:44:33.3334848Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68807/merge to s
2020-02-04T19:44:33.3452034Z Cleaning up task key
2020-02-04T19:44:33.3453043Z Start cleaning up orphan processes.
2020-02-04T19:44:33.3563658Z Terminate orphan process: pid (4309) (python)
2020-02-04T19:44:33.3754099Z ##[section]Finishing: Finalize Job
