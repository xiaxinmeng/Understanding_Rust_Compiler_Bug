plain
2020-03-17T18:01:41.0902806Z ========================== Starting Command Output ===========================
2020-03-17T18:01:41.0907109Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9f8b829b-8892-4b2a-a7eb-8fec941a9eb3.sh
2020-03-17T18:01:41.0907414Z 
2020-03-17T18:01:41.0912000Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-17T18:01:41.0932456Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70071/merge to s
2020-03-17T18:01:41.0935944Z Task         : Get sources
2020-03-17T18:01:41.0936268Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T18:01:41.0936583Z Version      : 1.0.0
2020-03-17T18:01:41.0936816Z Author       : Microsoft
---
2020-03-17T18:01:42.0800298Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-17T18:01:42.0806146Z ##[command]git config gc.auto 0
2020-03-17T18:01:42.0810339Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-17T18:01:42.0814187Z ##[command]git config --get-all http.proxy
2020-03-17T18:01:42.0821446Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70071/merge:refs/remotes/pull/70071/merge
---
2020-03-17T18:07:36.6438311Z Done!
2020-03-17T18:07:36.6438610Z some tidy checks failed
2020-03-17T18:07:36.6442344Z 
2020-03-17T18:07:36.6442646Z 
2020-03-17T18:07:36.6444070Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-03-17T18:07:36.6445137Z 
2020-03-17T18:07:36.6445340Z 
2020-03-17T18:07:36.6451025Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-17T18:07:36.6451622Z Build completed unsuccessfully in 0:01:30
2020-03-17T18:07:36.6451622Z Build completed unsuccessfully in 0:01:30
2020-03-17T18:07:36.6498094Z == clock drift check ==
2020-03-17T18:07:36.6518590Z   local time: Tue Mar 17 18:07:36 UTC 2020
2020-03-17T18:07:36.8133754Z   network time: Tue, 17 Mar 2020 18:07:36 GMT
2020-03-17T18:07:36.8134886Z == end clock drift check ==
2020-03-17T18:07:37.6848373Z 
2020-03-17T18:07:37.6939497Z ##[error]Bash exited with code '1'.
2020-03-17T18:07:37.6957135Z ##[section]Finishing: Run build
2020-03-17T18:07:37.7004351Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70071/merge to s
2020-03-17T18:07:37.7009635Z Task         : Get sources
2020-03-17T18:07:37.7010041Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T18:07:37.7010403Z Version      : 1.0.0
2020-03-17T18:07:37.7010874Z Author       : Microsoft
2020-03-17T18:07:37.7010874Z Author       : Microsoft
2020-03-17T18:07:37.7012603Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-17T18:07:37.7013088Z ==============================================================================
2020-03-17T18:07:38.0724391Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-17T18:07:38.0769306Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70071/merge to s
2020-03-17T18:07:38.0875340Z Cleaning up task key
2020-03-17T18:07:38.0876688Z Start cleaning up orphan processes.
2020-03-17T18:07:38.1095370Z Terminate orphan process: pid (4076) (python)
2020-03-17T18:07:38.1240145Z ##[section]Finishing: Finalize Job
