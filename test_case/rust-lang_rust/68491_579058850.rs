plain
2020-01-28T02:50:46.8117781Z ========================== Starting Command Output ===========================
2020-01-28T02:50:46.8121494Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/91596910-63bc-4168-9e17-62d03d9c6cb3.sh
2020-01-28T02:50:46.8952355Z 
2020-01-28T02:50:46.9031384Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-28T02:50:46.9037215Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68491/merge to s
2020-01-28T02:50:46.9038785Z Task         : Get sources
2020-01-28T02:50:46.9038822Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-28T02:50:46.9038902Z Version      : 1.0.0
2020-01-28T02:50:46.9038938Z Author       : Microsoft
---
2020-01-28T02:50:51.9835491Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-28T02:50:52.0091384Z ##[command]git config gc.auto 0
2020-01-28T02:50:52.0175660Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-28T02:50:52.0230339Z ##[command]git config --get-all http.proxy
2020-01-28T02:50:52.0358153Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68491/merge:refs/remotes/pull/68491/merge
---
2020-01-28T02:56:53.0717845Z     Finished release [optimized] target(s) in 1m 10s
2020-01-28T02:56:53.0850225Z tidy check
2020-01-28T02:56:54.4871956Z * 589 error codes
2020-01-28T02:56:54.4872986Z * highest error code: E0746
2020-01-28T02:56:54.5270073Z tidy error: /checkout/src/librustc_feature/active.rs:194: feature compiler_builtins is not sorted by "since" (version number)
2020-01-28T02:56:55.6930770Z some tidy checks failed
2020-01-28T02:56:55.6963400Z Found 487 error codes
2020-01-28T02:56:55.6963734Z Found 0 error codes with no tests
2020-01-28T02:56:55.6963899Z Done!
2020-01-28T02:56:55.6963899Z Done!
2020-01-28T02:56:55.6964027Z 
2020-01-28T02:56:55.6964171Z 
2020-01-28T02:56:55.6965246Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-28T02:56:55.6965775Z 
2020-01-28T02:56:55.6965896Z 
2020-01-28T02:56:55.6966059Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-28T02:56:55.6966212Z Build completed unsuccessfully in 0:01:20
2020-01-28T02:56:55.6966212Z Build completed unsuccessfully in 0:01:20
2020-01-28T02:56:55.6991702Z == clock drift check ==
2020-01-28T02:56:55.7005084Z   local time: Tue Jan 28 02:56:55 UTC 2020
2020-01-28T02:56:55.8657110Z   network time: Tue, 28 Jan 2020 02:56:55 GMT
2020-01-28T02:56:55.8663758Z == end clock drift check ==
2020-01-28T02:56:56.7325129Z 
2020-01-28T02:56:56.7390247Z ##[error]Bash exited with code '1'.
2020-01-28T02:56:56.7401547Z ##[section]Finishing: Run build
2020-01-28T02:56:56.7415630Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68491/merge to s
2020-01-28T02:56:56.7417389Z Task         : Get sources
2020-01-28T02:56:56.7417441Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-28T02:56:56.7417507Z Version      : 1.0.0
2020-01-28T02:56:56.7417551Z Author       : Microsoft
2020-01-28T02:56:56.7417551Z Author       : Microsoft
2020-01-28T02:56:56.7417599Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-28T02:56:56.7417667Z ==============================================================================
2020-01-28T02:56:57.1222336Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-28T02:56:57.1262197Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68491/merge to s
2020-01-28T02:56:57.1370699Z Cleaning up task key
2020-01-28T02:56:57.1371489Z Start cleaning up orphan processes.
2020-01-28T02:56:57.2323758Z Terminate orphan process: pid (4042) (python)
2020-01-28T02:56:57.2341992Z ##[section]Finishing: Finalize Job
