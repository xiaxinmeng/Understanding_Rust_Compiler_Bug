plain
2020-03-03T16:54:57.1807301Z ========================== Starting Command Output ===========================
2020-03-03T16:54:57.1812628Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/857bed0e-4b6d-4d42-80d8-6c89105df056.sh
2020-03-03T16:54:57.1813165Z 
2020-03-03T16:54:57.1817843Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-03T16:54:57.1839228Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-03-03T16:54:57.1843088Z Task         : Get sources
2020-03-03T16:54:57.1843417Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T16:54:57.1843734Z Version      : 1.0.0
2020-03-03T16:54:57.1843949Z Author       : Microsoft
---
2020-03-03T16:54:58.3704950Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-03T16:54:58.3715991Z ##[command]git config gc.auto 0
2020-03-03T16:54:58.3720290Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-03T16:54:58.3727736Z ##[command]git config --get-all http.proxy
2020-03-03T16:54:58.3739872Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69406/merge:refs/remotes/pull/69406/merge
---
2020-03-03T16:58:43.4774390Z 
2020-03-03T16:58:43.4805076Z ########################################################                  78.8%
2020-03-03T16:58:43.4809437Z ######################################################################## 100.0%
2020-03-03T16:58:43.7705137Z extracting /checkout/obj/build/cache/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
2020-03-03T16:58:43.8061028Z error: failed to read `/chalk/chalk-ir/Cargo.toml`
2020-03-03T16:58:43.8061463Z Caused by:
2020-03-03T16:58:43.8061696Z   No such file or directory (os error 2)
2020-03-03T16:58:43.8069166Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2020-03-03T16:58:43.8070090Z Build completed unsuccessfully in 0:00:10
2020-03-03T16:58:43.8070090Z Build completed unsuccessfully in 0:00:10
2020-03-03T16:58:43.8121787Z == clock drift check ==
2020-03-03T16:58:43.8131655Z   local time: Tue Mar  3 16:58:43 UTC 2020
2020-03-03T16:58:44.1030746Z   network time: Tue, 03 Mar 2020 16:58:44 GMT
2020-03-03T16:58:44.1032019Z == end clock drift check ==
2020-03-03T16:58:51.7954541Z 
2020-03-03T16:58:51.8042963Z ##[error]Bash exited with code '1'.
2020-03-03T16:58:51.8085663Z ##[section]Finishing: Run build
2020-03-03T16:58:51.8134999Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-03-03T16:58:51.8140646Z Task         : Get sources
2020-03-03T16:58:51.8141055Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T16:58:51.8141413Z Version      : 1.0.0
2020-03-03T16:58:51.8141665Z Author       : Microsoft
2020-03-03T16:58:51.8141665Z Author       : Microsoft
2020-03-03T16:58:51.8142084Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-03T16:58:51.8142544Z ==============================================================================
2020-03-03T16:58:52.1841292Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-03T16:58:52.1886689Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-03-03T16:58:52.1983669Z Cleaning up task key
2020-03-03T16:58:52.1985065Z Start cleaning up orphan processes.
2020-03-03T16:58:52.2185252Z Terminate orphan process: pid (3917) (python)
2020-03-03T16:58:52.2341980Z ##[section]Finishing: Finalize Job
