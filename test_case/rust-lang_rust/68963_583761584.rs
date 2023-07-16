plain
2020-02-08T17:47:02.3386084Z ========================== Starting Command Output ===========================
2020-02-08T17:47:02.3390153Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3c6e4edb-2b35-4f14-a0d3-c2209bc112f3.sh
2020-02-08T17:47:02.3390480Z 
2020-02-08T17:47:02.3394271Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-08T17:47:02.3401439Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68963/merge to s
2020-02-08T17:47:02.3403448Z Task         : Get sources
2020-02-08T17:47:02.3403480Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-08T17:47:02.3403508Z Version      : 1.0.0
2020-02-08T17:47:02.3403597Z Author       : Microsoft
---
2020-02-08T17:47:03.3402667Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-08T17:47:03.3416371Z ##[command]git config gc.auto 0
2020-02-08T17:47:03.3419942Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-08T17:47:03.3422256Z ##[command]git config --get-all http.proxy
2020-02-08T17:47:03.3476612Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68963/merge:refs/remotes/pull/68963/merge
---
2020-02-08T17:53:07.9160588Z Found 0 error codes with no tests
2020-02-08T17:53:07.9160848Z Done!
2020-02-08T17:53:07.9165988Z 
2020-02-08T17:53:07.9166068Z 
2020-02-08T17:53:07.9167757Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-02-08T17:53:07.9170300Z 
2020-02-08T17:53:07.9170505Z 
2020-02-08T17:53:07.9181746Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-02-08T17:53:07.9181834Z Build completed unsuccessfully in 0:01:41
2020-02-08T17:53:07.9181834Z Build completed unsuccessfully in 0:01:41
2020-02-08T17:53:07.9237840Z == clock drift check ==
2020-02-08T17:53:07.9249408Z   local time: Sat Feb  8 17:53:07 UTC 2020
2020-02-08T17:53:08.0852530Z   network time: Sat, 08 Feb 2020 17:53:08 GMT
2020-02-08T17:53:08.0857059Z == end clock drift check ==
2020-02-08T17:53:08.9485658Z 
2020-02-08T17:53:08.9572435Z ##[error]Bash exited with code '1'.
2020-02-08T17:53:08.9585456Z ##[section]Finishing: Run build
2020-02-08T17:53:08.9602056Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68963/merge to s
2020-02-08T17:53:08.9604115Z Task         : Get sources
2020-02-08T17:53:08.9604186Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-08T17:53:08.9604258Z Version      : 1.0.0
2020-02-08T17:53:08.9604306Z Author       : Microsoft
2020-02-08T17:53:08.9604306Z Author       : Microsoft
2020-02-08T17:53:08.9604374Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-08T17:53:08.9604430Z ==============================================================================
2020-02-08T17:53:09.3670950Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-08T17:53:09.3712776Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68963/merge to s
2020-02-08T17:53:09.3837637Z Cleaning up task key
2020-02-08T17:53:09.3838543Z Start cleaning up orphan processes.
2020-02-08T17:53:09.4000169Z Terminate orphan process: pid (4713) (python)
2020-02-08T17:53:09.4222585Z ##[section]Finishing: Finalize Job
