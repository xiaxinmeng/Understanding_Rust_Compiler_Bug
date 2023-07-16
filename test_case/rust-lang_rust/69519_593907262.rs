plain
2020-03-03T11:18:17.9296438Z ========================== Starting Command Output ===========================
2020-03-03T11:18:17.9300049Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/cda4cc99-c271-4acb-b211-7e531fe54078.sh
2020-03-03T11:18:17.9301179Z 
2020-03-03T11:18:17.9305185Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-03T11:18:17.9326485Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69519/merge to s
2020-03-03T11:18:17.9331099Z Task         : Get sources
2020-03-03T11:18:17.9331751Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T11:18:17.9332070Z Version      : 1.0.0
2020-03-03T11:18:17.9332307Z Author       : Microsoft
---
2020-03-03T11:18:19.1406773Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-03T11:18:19.1420827Z ##[command]git config gc.auto 0
2020-03-03T11:18:19.1425804Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-03T11:18:19.1429614Z ##[command]git config --get-all http.proxy
2020-03-03T11:18:19.1437345Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69519/merge:refs/remotes/pull/69519/merge
---
2020-03-03T11:24:34.6381410Z Done!
2020-03-03T11:24:34.6386193Z some tidy checks failed
2020-03-03T11:24:34.6392014Z 
2020-03-03T11:24:34.6392284Z 
2020-03-03T11:24:34.6394364Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-03-03T11:24:34.6395730Z 
2020-03-03T11:24:34.6395905Z 
2020-03-03T11:24:34.6401105Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-03T11:24:34.6401757Z Build completed unsuccessfully in 0:01:38
2020-03-03T11:24:34.6401757Z Build completed unsuccessfully in 0:01:38
2020-03-03T11:24:34.6447756Z == clock drift check ==
2020-03-03T11:24:34.6459880Z   local time: Tue Mar  3 11:24:34 UTC 2020
2020-03-03T11:24:34.9315152Z   network time: Tue, 03 Mar 2020 11:24:34 GMT
2020-03-03T11:24:34.9318820Z == end clock drift check ==
2020-03-03T11:24:35.6719311Z 
2020-03-03T11:24:35.6795637Z ##[error]Bash exited with code '1'.
2020-03-03T11:24:35.6810318Z ##[section]Finishing: Run build
2020-03-03T11:24:35.6859711Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69519/merge to s
2020-03-03T11:24:35.6865275Z Task         : Get sources
2020-03-03T11:24:35.6865657Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T11:24:35.6865990Z Version      : 1.0.0
2020-03-03T11:24:35.6866222Z Author       : Microsoft
2020-03-03T11:24:35.6866222Z Author       : Microsoft
2020-03-03T11:24:35.6866615Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-03T11:24:35.6867041Z ==============================================================================
2020-03-03T11:24:36.0140449Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-03T11:24:36.0189227Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69519/merge to s
2020-03-03T11:24:36.0276773Z Cleaning up task key
2020-03-03T11:24:36.0278117Z Start cleaning up orphan processes.
2020-03-03T11:24:36.0449468Z Terminate orphan process: pid (3742) (python)
2020-03-03T11:24:36.0579146Z ##[section]Finishing: Finalize Job
