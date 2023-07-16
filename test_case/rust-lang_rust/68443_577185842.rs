plain
2020-01-22T13:23:40.0086251Z ========================== Starting Command Output ===========================
2020-01-22T13:23:40.0087674Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/62d551e4-4419-4038-865d-6e28158ed51b.sh
2020-01-22T13:23:40.0087705Z 
2020-01-22T13:23:40.0090271Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-22T13:23:40.0095212Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68443/merge to s
2020-01-22T13:23:40.0096876Z Task         : Get sources
2020-01-22T13:23:40.0096904Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-22T13:23:40.0096963Z Version      : 1.0.0
2020-01-22T13:23:40.0096991Z Author       : Microsoft
---
2020-01-22T13:23:40.8435239Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-22T13:23:40.8531285Z ##[command]git config gc.auto 0
2020-01-22T13:23:40.8592698Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-22T13:23:40.8646653Z ##[command]git config --get-all http.proxy
2020-01-22T13:23:40.8779898Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68443/merge:refs/remotes/pull/68443/merge
---
2020-01-22T13:28:33.1297325Z    Compiling serde_json v1.0.40
2020-01-22T13:28:34.6346858Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-01-22T13:28:43.1872180Z     Finished release [optimized] target(s) in 1m 12s
2020-01-22T13:28:43.1970890Z tidy check
2020-01-22T13:28:44.1007182Z tidy error: /checkout/src/test/run-make-fulldeps/return-non-c-like-enum-from-c/nonclike.rs:34: trailing whitespace
2020-01-22T13:28:45.9195602Z Found 487 error codes
2020-01-22T13:28:45.9196212Z Found 0 error codes with no tests
2020-01-22T13:28:45.9196385Z Done!
2020-01-22T13:28:45.9196583Z some tidy checks failed
2020-01-22T13:28:45.9196583Z some tidy checks failed
2020-01-22T13:28:45.9200101Z 
2020-01-22T13:28:45.9200307Z 
2020-01-22T13:28:45.9201384Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-22T13:28:45.9202014Z 
2020-01-22T13:28:45.9202132Z 
2020-01-22T13:28:45.9211199Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-22T13:28:45.9211473Z Build completed unsuccessfully in 0:01:22
2020-01-22T13:28:45.9211473Z Build completed unsuccessfully in 0:01:22
2020-01-22T13:28:45.9260606Z == clock drift check ==
2020-01-22T13:28:45.9270992Z   local time: Wed Jan 22 13:28:45 UTC 2020
2020-01-22T13:28:46.1930644Z   network time: Wed, 22 Jan 2020 13:28:46 GMT
2020-01-22T13:28:46.1930758Z == end clock drift check ==
2020-01-22T13:28:46.8975674Z 
2020-01-22T13:28:46.9065833Z ##[error]Bash exited with code '1'.
2020-01-22T13:28:46.9076157Z ##[section]Finishing: Run build
2020-01-22T13:28:46.9090662Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68443/merge to s
2020-01-22T13:28:46.9092291Z Task         : Get sources
2020-01-22T13:28:46.9092331Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-22T13:28:46.9092389Z Version      : 1.0.0
2020-01-22T13:28:46.9092424Z Author       : Microsoft
2020-01-22T13:28:46.9092424Z Author       : Microsoft
2020-01-22T13:28:46.9092464Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-22T13:28:46.9092522Z ==============================================================================
2020-01-22T13:28:47.2815328Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-22T13:28:47.2858552Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68443/merge to s
2020-01-22T13:28:47.2969465Z Cleaning up task key
2020-01-22T13:28:47.2970218Z Start cleaning up orphan processes.
2020-01-22T13:28:47.3073831Z Terminate orphan process: pid (4368) (python)
2020-01-22T13:28:47.3257764Z ##[section]Finishing: Finalize Job
