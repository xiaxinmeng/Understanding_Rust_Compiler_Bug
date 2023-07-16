plain
2020-03-11T10:47:22.6579801Z ========================== Starting Command Output ===========================
2020-03-11T10:47:22.6584481Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3b423acd-605f-4e7c-ad94-869a2f0f3e60.sh
2020-03-11T10:47:22.6584783Z 
2020-03-11T10:47:22.6589821Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-11T10:47:22.6611557Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69915/merge to s
2020-03-11T10:47:22.6615118Z Task         : Get sources
2020-03-11T10:47:22.6615451Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-11T10:47:22.6615792Z Version      : 1.0.0
2020-03-11T10:47:22.6616011Z Author       : Microsoft
---
2020-03-11T10:47:23.6632743Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-11T10:47:23.6641850Z ##[command]git config gc.auto 0
2020-03-11T10:47:23.6650056Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-11T10:47:23.6660817Z ##[command]git config --get-all http.proxy
2020-03-11T10:47:23.6669193Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69915/merge:refs/remotes/pull/69915/merge
---
2020-03-11T10:53:11.9187617Z Found 0 error codes with no tests
2020-03-11T10:53:11.9187823Z Done!
2020-03-11T10:53:11.9210646Z 
2020-03-11T10:53:11.9218123Z 
2020-03-11T10:53:11.9219632Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-03-11T10:53:11.9220524Z 
2020-03-11T10:53:11.9220636Z 
2020-03-11T10:53:11.9220935Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-11T10:53:11.9221386Z Build completed unsuccessfully in 0:01:32
2020-03-11T10:53:11.9221386Z Build completed unsuccessfully in 0:01:32
2020-03-11T10:53:11.9268107Z == clock drift check ==
2020-03-11T10:53:11.9281402Z   local time: Wed Mar 11 10:53:11 UTC 2020
2020-03-11T10:53:12.0236385Z   network time: Wed, 11 Mar 2020 10:53:12 GMT
2020-03-11T10:53:12.0237042Z == end clock drift check ==
2020-03-11T10:53:12.7817815Z 
2020-03-11T10:53:12.7897267Z ##[error]Bash exited with code '1'.
2020-03-11T10:53:12.7919122Z ##[section]Finishing: Run build
2020-03-11T10:53:12.7982056Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69915/merge to s
2020-03-11T10:53:12.8011814Z Task         : Get sources
2020-03-11T10:53:12.8012310Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-11T10:53:12.8012765Z Version      : 1.0.0
2020-03-11T10:53:12.8013083Z Author       : Microsoft
2020-03-11T10:53:12.8013083Z Author       : Microsoft
2020-03-11T10:53:12.8013576Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-11T10:53:12.8014368Z ==============================================================================
2020-03-11T10:53:13.1850822Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-11T10:53:13.1915197Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69915/merge to s
2020-03-11T10:53:13.2009468Z Cleaning up task key
2020-03-11T10:53:13.2011146Z Start cleaning up orphan processes.
2020-03-11T10:53:13.2223900Z Terminate orphan process: pid (3632) (python)
2020-03-11T10:53:13.2451415Z ##[section]Finishing: Finalize Job
