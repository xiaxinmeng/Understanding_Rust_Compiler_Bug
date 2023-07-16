plain
2020-02-13T00:51:19.8440837Z ========================== Starting Command Output ===========================
2020-02-13T00:51:19.8443353Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b8a04fa5-7464-42ea-95ff-385d4ea1f1d0.sh
2020-02-13T00:51:19.8443414Z 
2020-02-13T00:51:19.8451343Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-13T00:51:19.8461644Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69082/merge to s
2020-02-13T00:51:19.8463429Z Task         : Get sources
2020-02-13T00:51:19.8463468Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-13T00:51:19.8463506Z Version      : 1.0.0
2020-02-13T00:51:19.8463542Z Author       : Microsoft
---
2020-02-13T00:51:20.8185308Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-13T00:51:20.8196864Z ##[command]git config gc.auto 0
2020-02-13T00:51:20.8199698Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-13T00:51:20.8202040Z ##[command]git config --get-all http.proxy
2020-02-13T00:51:20.8208851Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69082/merge:refs/remotes/pull/69082/merge
---
2020-02-13T00:57:27.3011191Z Found 0 error codes with no tests
2020-02-13T00:57:27.3011232Z Done!
2020-02-13T00:57:27.3011258Z 
2020-02-13T00:57:27.3011302Z 
2020-02-13T00:57:27.3012133Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-02-13T00:57:27.3012496Z 
2020-02-13T00:57:27.3012520Z 
2020-02-13T00:57:27.3016558Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-02-13T00:57:27.3016639Z Build completed unsuccessfully in 0:01:41
2020-02-13T00:57:27.3016639Z Build completed unsuccessfully in 0:01:41
2020-02-13T00:57:27.3074799Z == clock drift check ==
2020-02-13T00:57:27.3074863Z   local time: Thu Feb 13 00:57:27 UTC 2020
2020-02-13T00:57:27.5949628Z   network time: Thu, 13 Feb 2020 00:57:27 GMT
2020-02-13T00:57:27.5949767Z == end clock drift check ==
2020-02-13T00:57:28.2701843Z 
2020-02-13T00:57:28.2816531Z ##[error]Bash exited with code '1'.
2020-02-13T00:57:28.2829905Z ##[section]Finishing: Run build
2020-02-13T00:57:28.2851654Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69082/merge to s
2020-02-13T00:57:28.2853780Z Task         : Get sources
2020-02-13T00:57:28.2853856Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-13T00:57:28.2853910Z Version      : 1.0.0
2020-02-13T00:57:28.2853958Z Author       : Microsoft
2020-02-13T00:57:28.2853958Z Author       : Microsoft
2020-02-13T00:57:28.2854030Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-13T00:57:28.2854089Z ==============================================================================
2020-02-13T00:57:28.6700637Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-13T00:57:28.6746764Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69082/merge to s
2020-02-13T00:57:28.6845148Z Cleaning up task key
2020-02-13T00:57:28.6845861Z Start cleaning up orphan processes.
2020-02-13T00:57:28.7126527Z Terminate orphan process: pid (7225) (python)
2020-02-13T00:57:28.7147610Z ##[section]Finishing: Finalize Job
