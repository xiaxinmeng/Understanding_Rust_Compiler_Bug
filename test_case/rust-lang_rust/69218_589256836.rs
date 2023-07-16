plain
2020-02-20T18:48:16.6491317Z ========================== Starting Command Output ===========================
2020-02-20T18:48:16.6493419Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/609c4a98-3cc6-42e9-be19-76b840d5196e.sh
2020-02-20T18:48:16.6493612Z 
2020-02-20T18:48:16.6496069Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-20T18:48:16.6511030Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69218/merge to s
2020-02-20T18:48:16.6513721Z Task         : Get sources
2020-02-20T18:48:16.6513984Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-20T18:48:16.6514192Z Version      : 1.0.0
2020-02-20T18:48:16.6514335Z Author       : Microsoft
---
2020-02-20T18:48:17.9854969Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-20T18:48:17.9860925Z ##[command]git config gc.auto 0
2020-02-20T18:48:17.9863931Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-20T18:48:17.9866773Z ##[command]git config --get-all http.proxy
2020-02-20T18:48:17.9873112Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69218/merge:refs/remotes/pull/69218/merge
---
2020-02-20T18:50:54.9237258Z #                                                                          2.1%
2020-02-20T18:50:54.9237656Z ######################################################################## 100.0%
2020-02-20T18:50:55.1277591Z extracting /checkout/obj/build/cache/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
2020-02-20T18:50:55.1913844Z     Updating crates.io index
2020-02-20T18:51:10.6792805Z     Updating git repository `https://github.com/Marwes/ena`
---
2020-02-20T18:54:05.3531544Z tidy check
2020-02-20T18:54:06.5060214Z * 589 error codes
2020-02-20T18:54:06.5060505Z * highest error code: E0746
2020-02-20T18:54:06.7889448Z * 281 features
2020-02-20T18:54:07.2947622Z invalid source: "git+https://github.com/Marwes/ena?branch=detach_undo_log#9b977ea7f209a35f46d65d33cdd74b8f4931fb8a"
2020-02-20T18:54:07.5196544Z some tidy checks failed
2020-02-20T18:54:07.5196773Z Found 487 error codes
2020-02-20T18:54:07.5197002Z Found 0 error codes with no tests
2020-02-20T18:54:07.5197153Z Done!
2020-02-20T18:54:07.5197153Z Done!
2020-02-20T18:54:07.5207436Z 
2020-02-20T18:54:07.5207569Z 
2020-02-20T18:54:07.5213755Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-02-20T18:54:07.5214827Z 
2020-02-20T18:54:07.5219995Z 
2020-02-20T18:54:07.5220420Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-02-20T18:54:07.5220933Z Build completed unsuccessfully in 0:01:24
2020-02-20T18:54:07.5220933Z Build completed unsuccessfully in 0:01:24
2020-02-20T18:54:07.5264933Z == clock drift check ==
2020-02-20T18:54:07.5278530Z   local time: Thu Feb 20 18:54:07 UTC 2020
2020-02-20T18:54:07.6924965Z   network time: Thu, 20 Feb 2020 18:54:07 GMT
2020-02-20T18:54:07.6930060Z == end clock drift check ==
2020-02-20T18:54:08.5451181Z 
2020-02-20T18:54:08.5523660Z ##[error]Bash exited with code '1'.
2020-02-20T18:54:08.5534885Z ##[section]Finishing: Run build
2020-02-20T18:54:08.5606936Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69218/merge to s
2020-02-20T18:54:08.5614599Z Task         : Get sources
2020-02-20T18:54:08.5614914Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-20T18:54:08.5615456Z Version      : 1.0.0
2020-02-20T18:54:08.5615813Z Author       : Microsoft
2020-02-20T18:54:08.5615813Z Author       : Microsoft
2020-02-20T18:54:08.5616481Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-20T18:54:08.5617159Z ==============================================================================
2020-02-20T18:54:08.8234109Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-20T18:54:08.8273480Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69218/merge to s
2020-02-20T18:54:08.8351439Z Cleaning up task key
2020-02-20T18:54:08.8352504Z Start cleaning up orphan processes.
2020-02-20T18:54:08.8497565Z Terminate orphan process: pid (4520) (python)
2020-02-20T18:54:08.8625191Z ##[section]Finishing: Finalize Job
