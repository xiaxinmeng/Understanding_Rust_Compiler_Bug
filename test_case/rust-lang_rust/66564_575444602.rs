plain
2020-01-17T02:52:04.1781917Z ========================== Starting Command Output ===========================
2020-01-17T02:52:04.1790461Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5c3b1c35-2c72-4997-8c6d-880fe7db84e0.sh
2020-01-17T02:52:04.1790834Z 
2020-01-17T02:52:04.1795244Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-17T02:52:04.1802620Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/66564/merge to s
2020-01-17T02:52:04.1804645Z Task         : Get sources
2020-01-17T02:52:04.1804683Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-17T02:52:04.1804720Z Version      : 1.0.0
2020-01-17T02:52:04.1804821Z Author       : Microsoft
---
2020-01-17T02:52:05.1571720Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-17T02:52:05.1582993Z ##[command]git config gc.auto 0
2020-01-17T02:52:05.1585988Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-17T02:52:05.1588346Z ##[command]git config --get-all http.proxy
2020-01-17T02:52:05.1595646Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66564/merge:refs/remotes/pull/66564/merge
---
2020-01-17T02:58:03.1260176Z Done!
2020-01-17T02:58:03.1263375Z some tidy checks failed
2020-01-17T02:58:03.1267180Z 
2020-01-17T02:58:03.1267530Z 
2020-01-17T02:58:03.1268823Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-17T02:58:03.1269679Z 
2020-01-17T02:58:03.1269907Z 
2020-01-17T02:58:03.1273894Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-17T02:58:03.1274251Z Build completed unsuccessfully in 0:01:28
2020-01-17T02:58:03.1274251Z Build completed unsuccessfully in 0:01:28
2020-01-17T02:58:03.1329383Z == clock drift check ==
2020-01-17T02:58:03.1339273Z   local time: Fri Jan 17 02:58:03 UTC 2020
2020-01-17T02:58:03.6589962Z   network time: Fri, 17 Jan 2020 02:58:03 GMT
2020-01-17T02:58:03.6597734Z == end clock drift check ==
2020-01-17T02:58:04.4052504Z 
2020-01-17T02:58:04.4179189Z ##[error]Bash exited with code '1'.
2020-01-17T02:58:04.4194096Z ##[section]Finishing: Run build
2020-01-17T02:58:04.4210705Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/66564/merge to s
2020-01-17T02:58:04.4212833Z Task         : Get sources
2020-01-17T02:58:04.4212914Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-17T02:58:04.4212992Z Version      : 1.0.0
2020-01-17T02:58:04.4213038Z Author       : Microsoft
2020-01-17T02:58:04.4213038Z Author       : Microsoft
2020-01-17T02:58:04.4213117Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-17T02:58:04.4213176Z ==============================================================================
2020-01-17T02:58:04.8114761Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-17T02:58:04.8154757Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/66564/merge to s
2020-01-17T02:58:04.8252896Z Cleaning up task key
2020-01-17T02:58:04.8253637Z Start cleaning up orphan processes.
2020-01-17T02:58:04.8400722Z Terminate orphan process: pid (6512) (python)
2020-01-17T02:58:04.8566945Z ##[section]Finishing: Finalize Job
