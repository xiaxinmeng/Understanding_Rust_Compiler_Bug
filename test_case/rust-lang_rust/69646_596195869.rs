plain
2020-03-08T11:23:02.9591376Z ========================== Starting Command Output ===========================
2020-03-08T11:23:02.9594183Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a90f5641-5b9d-4fbb-af67-74269ab9f291.sh
2020-03-08T11:23:02.9594597Z 
2020-03-08T11:23:02.9598537Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-08T11:23:02.9619568Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69646/merge to s
2020-03-08T11:23:02.9622865Z Task         : Get sources
2020-03-08T11:23:02.9623112Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-08T11:23:02.9623345Z Version      : 1.0.0
2020-03-08T11:23:02.9623502Z Author       : Microsoft
---
2020-03-08T11:23:04.2253854Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-08T11:23:04.2260270Z ##[command]git config gc.auto 0
2020-03-08T11:23:04.2269963Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-08T11:23:04.2277325Z ##[command]git config --get-all http.proxy
2020-03-08T11:23:05.2172346Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69646/merge:refs/remotes/pull/69646/merge
---
2020-03-08T11:28:17.3497003Z Done!
2020-03-08T11:28:17.3497343Z some tidy checks failed
2020-03-08T11:28:17.3512988Z 
2020-03-08T11:28:17.3513277Z 
2020-03-08T11:28:17.3514985Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-03-08T11:28:17.3516011Z 
2020-03-08T11:28:17.3516113Z 
2020-03-08T11:28:17.3520692Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-08T11:28:17.3521181Z Build completed unsuccessfully in 0:01:23
2020-03-08T11:28:17.3521181Z Build completed unsuccessfully in 0:01:23
2020-03-08T11:28:17.3570029Z == clock drift check ==
2020-03-08T11:28:17.3577704Z   local time: Sun Mar  8 11:28:17 UTC 2020
2020-03-08T11:28:17.4438132Z   network time: Sun, 08 Mar 2020 11:28:17 GMT
2020-03-08T11:28:17.4438397Z == end clock drift check ==
2020-03-08T11:28:18.2423122Z 
2020-03-08T11:28:18.2488420Z ##[error]Bash exited with code '1'.
2020-03-08T11:28:18.2499861Z ##[section]Finishing: Run build
2020-03-08T11:28:18.2540171Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69646/merge to s
2020-03-08T11:28:18.2545408Z Task         : Get sources
2020-03-08T11:28:18.2545691Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-08T11:28:18.2545965Z Version      : 1.0.0
2020-03-08T11:28:18.2546147Z Author       : Microsoft
2020-03-08T11:28:18.2546147Z Author       : Microsoft
2020-03-08T11:28:18.2546430Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-08T11:28:18.2546779Z ==============================================================================
2020-03-08T11:28:18.5595510Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-08T11:28:18.5648207Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69646/merge to s
2020-03-08T11:28:18.5754331Z Cleaning up task key
2020-03-08T11:28:18.5755950Z Start cleaning up orphan processes.
2020-03-08T11:28:18.5935566Z Terminate orphan process: pid (3800) (python)
2020-03-08T11:28:18.6088478Z ##[section]Finishing: Finalize Job
