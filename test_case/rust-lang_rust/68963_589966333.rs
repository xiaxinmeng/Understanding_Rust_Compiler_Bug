plain
2020-02-22T15:20:57.3471605Z ========================== Starting Command Output ===========================
2020-02-22T15:20:57.3474604Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d88d471e-6ff7-4f82-88f7-7b27cefde95a.sh
2020-02-22T15:20:57.3474828Z 
2020-02-22T15:20:57.3478170Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-22T15:20:57.3494046Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68963/merge to s
2020-02-22T15:20:57.3496671Z Task         : Get sources
2020-02-22T15:20:57.3496936Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T15:20:57.3497145Z Version      : 1.0.0
2020-02-22T15:20:57.3497284Z Author       : Microsoft
---
2020-02-22T15:20:58.6782225Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-22T15:20:58.6788391Z ##[command]git config gc.auto 0
2020-02-22T15:20:58.6793834Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-22T15:20:58.6799137Z ##[command]git config --get-all http.proxy
2020-02-22T15:20:58.6807239Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68963/merge:refs/remotes/pull/68963/merge
---
2020-02-22T15:23:34.6604762Z extracting /checkout/obj/build/cache/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
2020-02-22T15:23:34.7170013Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2020-02-22T15:23:34.7170773Z 
2020-02-22T15:23:34.7170948Z Caused by:
2020-02-22T15:23:34.7171683Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2020-02-22T15:23:34.7178311Z Build completed unsuccessfully in 0:00:11
2020-02-22T15:23:34.7212194Z == clock drift check ==
2020-02-22T15:23:34.7221728Z   local time: Sat Feb 22 15:23:34 UTC 2020
2020-02-22T15:23:34.8791052Z   network time: Sat, 22 Feb 2020 15:23:34 GMT
2020-02-22T15:23:34.8791052Z   network time: Sat, 22 Feb 2020 15:23:34 GMT
2020-02-22T15:23:34.8794918Z == end clock drift check ==
2020-02-22T15:23:42.3849801Z 
2020-02-22T15:23:42.3880980Z ##[error]Bash exited with code '1'.
2020-02-22T15:23:42.3892254Z ##[section]Finishing: Run build
2020-02-22T15:23:42.3925323Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68963/merge to s
2020-02-22T15:23:42.3931825Z Task         : Get sources
2020-02-22T15:23:42.3932274Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T15:23:42.3932511Z Version      : 1.0.0
2020-02-22T15:23:42.3932675Z Author       : Microsoft
2020-02-22T15:23:42.3932675Z Author       : Microsoft
2020-02-22T15:23:42.3932956Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-22T15:23:42.3933259Z ==============================================================================
2020-02-22T15:23:42.6608364Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-22T15:23:42.6639905Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68963/merge to s
2020-02-22T15:23:42.6706216Z Cleaning up task key
2020-02-22T15:23:42.6707208Z Start cleaning up orphan processes.
2020-02-22T15:23:42.6939766Z Terminate orphan process: pid (4174) (python)
2020-02-22T15:23:42.6968067Z ##[section]Finishing: Finalize Job
