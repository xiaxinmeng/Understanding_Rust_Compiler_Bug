plain
2020-01-28T12:49:21.7695359Z ========================== Starting Command Output ===========================
2020-01-28T12:49:21.7714417Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/118662eb-0c48-471d-99ca-f1d95257267f.sh
2020-01-28T12:49:21.8012328Z 
2020-01-28T12:49:21.8052957Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-28T12:49:21.8058767Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68598/merge to s
2020-01-28T12:49:21.8060387Z Task         : Get sources
2020-01-28T12:49:21.8060425Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-28T12:49:21.8060462Z Version      : 1.0.0
2020-01-28T12:49:21.8060542Z Author       : Microsoft
---
2020-01-28T12:49:22.6943018Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-28T12:49:22.7056128Z ##[command]git config gc.auto 0
2020-01-28T12:49:22.7130788Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-28T12:49:22.7182727Z ##[command]git config --get-all http.proxy
2020-01-28T12:49:22.7328353Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68598/merge:refs/remotes/pull/68598/merge
---
2020-01-28T13:15:28.4965256Z    Compiling serde_json v1.0.40
2020-01-28T13:15:30.0268271Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-01-28T13:15:39.1227660Z     Finished release [optimized] target(s) in 1m 16s
2020-01-28T13:15:39.1335538Z tidy check
2020-01-28T13:15:40.2722931Z tidy error: /checkout/src/librustdoc/html/static/main.js:1902: line longer than 100 chars
2020-01-28T13:15:41.9424896Z some tidy checks failed
2020-01-28T13:15:41.9425054Z Found 487 error codes
2020-01-28T13:15:41.9425112Z Found 0 error codes with no tests
2020-01-28T13:15:41.9425165Z Done!
2020-01-28T13:15:41.9425165Z Done!
2020-01-28T13:15:41.9425200Z 
2020-01-28T13:15:41.9425252Z 
2020-01-28T13:15:41.9426375Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-28T13:15:41.9426564Z 
2020-01-28T13:15:41.9426596Z 
2020-01-28T13:15:41.9435190Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-28T13:15:41.9435277Z Build completed unsuccessfully in 0:01:27
2020-01-28T13:15:41.9435277Z Build completed unsuccessfully in 0:01:27
2020-01-28T13:15:41.9489069Z == clock drift check ==
2020-01-28T13:15:41.9500442Z   local time: Tue Jan 28 13:15:41 UTC 2020
2020-01-28T13:15:42.4933755Z   network time: Tue, 28 Jan 2020 13:15:42 GMT
2020-01-28T13:15:42.4934942Z == end clock drift check ==
2020-01-28T13:15:43.2105706Z 
2020-01-28T13:15:43.2226734Z ##[error]Bash exited with code '1'.
2020-01-28T13:15:43.2239901Z ##[section]Finishing: Run build
2020-01-28T13:15:43.2255256Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68598/merge to s
2020-01-28T13:15:43.2257120Z Task         : Get sources
2020-01-28T13:15:43.2257173Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-28T13:15:43.2257240Z Version      : 1.0.0
2020-01-28T13:15:43.2257287Z Author       : Microsoft
2020-01-28T13:15:43.2257287Z Author       : Microsoft
2020-01-28T13:15:43.2257338Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-28T13:15:43.2257406Z ==============================================================================
2020-01-28T13:15:43.6539830Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-28T13:15:43.6581909Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68598/merge to s
2020-01-28T13:15:43.6695896Z Cleaning up task key
2020-01-28T13:15:43.6696682Z Start cleaning up orphan processes.
2020-01-28T13:15:43.6804375Z Terminate orphan process: pid (3238) (python)
2020-01-28T13:15:43.7019407Z ##[section]Finishing: Finalize Job
