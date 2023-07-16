plain
2020-01-17T20:15:45.1104115Z ========================== Starting Command Output ===========================
2020-01-17T20:15:45.1106973Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d7b46924-3bb9-429a-9a15-ada146a8ec8d.sh
2020-01-17T20:15:45.1107157Z 
2020-01-17T20:15:45.1110923Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-17T20:15:45.1116131Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67681/merge to s
2020-01-17T20:15:45.1117725Z Task         : Get sources
2020-01-17T20:15:45.1117754Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-17T20:15:45.1117781Z Version      : 1.0.0
2020-01-17T20:15:45.1117828Z Author       : Microsoft
---
2020-01-17T20:15:45.9808621Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-17T20:15:45.9924995Z ##[command]git config gc.auto 0
2020-01-17T20:15:45.9975367Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-17T20:15:46.0027306Z ##[command]git config --get-all http.proxy
2020-01-17T20:15:46.0185784Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67681/merge:refs/remotes/pull/67681/merge
---
2020-01-17T20:21:23.3545892Z    Compiling serde_json v1.0.40
2020-01-17T20:21:24.9982732Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-01-17T20:21:34.5980413Z     Finished release [optimized] target(s) in 1m 20s
2020-01-17T20:21:34.6095465Z tidy check
2020-01-17T20:21:34.9316471Z tidy error: /checkout/src/librustc_typeck/collect.rs: too many lines (3003) (add `// ignore-tidy-filelength` to the file to suppress this error)
2020-01-17T20:21:37.3840097Z Found 487 error codes
2020-01-17T20:21:37.3840909Z Found 0 error codes with no tests
2020-01-17T20:21:37.3845610Z Done!
2020-01-17T20:21:37.3845660Z some tidy checks failed
2020-01-17T20:21:37.3845660Z some tidy checks failed
2020-01-17T20:21:37.3845859Z 
2020-01-17T20:21:37.3845886Z 
2020-01-17T20:21:37.3848310Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-17T20:21:37.3848468Z 
2020-01-17T20:21:37.3848517Z 
2020-01-17T20:21:37.3862695Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-17T20:21:37.3863021Z Build completed unsuccessfully in 0:01:31
2020-01-17T20:21:37.3863021Z Build completed unsuccessfully in 0:01:31
2020-01-17T20:21:37.3920031Z == clock drift check ==
2020-01-17T20:21:37.3930501Z   local time: Fri Jan 17 20:21:37 UTC 2020
2020-01-17T20:21:37.5582327Z   network time: Fri, 17 Jan 2020 20:21:37 GMT
2020-01-17T20:21:37.5585453Z == end clock drift check ==
2020-01-17T20:21:38.2882136Z 
2020-01-17T20:21:38.2992441Z ##[error]Bash exited with code '1'.
2020-01-17T20:21:38.3009404Z ##[section]Finishing: Run build
2020-01-17T20:21:38.3024424Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67681/merge to s
2020-01-17T20:21:38.3026240Z Task         : Get sources
2020-01-17T20:21:38.3026281Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-17T20:21:38.3026632Z Version      : 1.0.0
2020-01-17T20:21:38.3026686Z Author       : Microsoft
2020-01-17T20:21:38.3026686Z Author       : Microsoft
2020-01-17T20:21:38.3026725Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-17T20:21:38.3026766Z ==============================================================================
2020-01-17T20:21:38.7222436Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-17T20:21:38.7265315Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67681/merge to s
2020-01-17T20:21:38.7402258Z Cleaning up task key
2020-01-17T20:21:38.7403026Z Start cleaning up orphan processes.
2020-01-17T20:21:38.7560398Z Terminate orphan process: pid (3462) (python)
2020-01-17T20:21:38.7761813Z ##[section]Finishing: Finalize Job
