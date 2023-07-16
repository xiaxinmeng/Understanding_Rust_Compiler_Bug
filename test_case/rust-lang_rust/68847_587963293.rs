plain
2020-02-18T23:48:37.4540883Z ========================== Starting Command Output ===========================
2020-02-18T23:48:37.4542718Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e4b4bba3-5650-4953-b8aa-89b99984d0f5.sh
2020-02-18T23:48:37.4542760Z 
2020-02-18T23:48:37.4546093Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-18T23:48:37.4555315Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68847/merge to s
2020-02-18T23:48:37.4557377Z Task         : Get sources
2020-02-18T23:48:37.4557414Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-18T23:48:37.4557450Z Version      : 1.0.0
2020-02-18T23:48:37.4557546Z Author       : Microsoft
---
2020-02-18T23:48:38.4294324Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-18T23:48:38.4305883Z ##[command]git config gc.auto 0
2020-02-18T23:48:38.4308826Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-18T23:48:38.4311006Z ##[command]git config --get-all http.proxy
2020-02-18T23:48:38.4317879Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68847/merge:refs/remotes/pull/68847/merge
---
2020-02-18T23:52:51.3785679Z extracting /checkout/obj/build/cache/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
2020-02-18T23:52:51.4510270Z error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
2020-02-18T23:52:51.4510424Z 
2020-02-18T23:52:51.4510518Z Caused by:
2020-02-18T23:52:51.4510977Z   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2020-02-18T23:52:51.4521301Z Build completed unsuccessfully in 0:00:23
2020-02-18T23:52:51.4563651Z == clock drift check ==
2020-02-18T23:52:51.4570851Z   local time: Tue Feb 18 23:52:51 UTC 2020
2020-02-18T23:52:51.7443716Z   network time: Tue, 18 Feb 2020 23:52:51 GMT
2020-02-18T23:52:51.7443716Z   network time: Tue, 18 Feb 2020 23:52:51 GMT
2020-02-18T23:52:51.7448434Z == end clock drift check ==
2020-02-18T23:52:59.2789069Z 
2020-02-18T23:52:59.2904004Z ##[error]Bash exited with code '1'.
2020-02-18T23:52:59.2919995Z ##[section]Finishing: Run build
2020-02-18T23:52:59.2937350Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68847/merge to s
2020-02-18T23:52:59.2939638Z Task         : Get sources
2020-02-18T23:52:59.2939744Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-18T23:52:59.2939801Z Version      : 1.0.0
2020-02-18T23:52:59.2939847Z Author       : Microsoft
2020-02-18T23:52:59.2939847Z Author       : Microsoft
2020-02-18T23:52:59.2939935Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-18T23:52:59.2939992Z ==============================================================================
2020-02-18T23:52:59.6898863Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-18T23:52:59.6936762Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68847/merge to s
2020-02-18T23:52:59.7040852Z Cleaning up task key
2020-02-18T23:52:59.7041643Z Start cleaning up orphan processes.
2020-02-18T23:52:59.7153561Z Terminate orphan process: pid (3682) (python)
2020-02-18T23:52:59.7348342Z ##[section]Finishing: Finalize Job
