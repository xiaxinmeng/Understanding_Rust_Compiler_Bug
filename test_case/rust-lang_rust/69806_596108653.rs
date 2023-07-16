plain
2020-03-07T16:45:46.1252141Z ========================== Starting Command Output ===========================
2020-03-07T16:45:46.1254623Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/510199fc-6b3a-43b8-8477-6a755178702a.sh
2020-03-07T16:45:46.1254874Z 
2020-03-07T16:45:46.1257984Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-07T16:45:46.1278329Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69806/merge to s
2020-03-07T16:45:46.1282128Z Task         : Get sources
2020-03-07T16:45:46.1282416Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-07T16:45:46.1282713Z Version      : 1.0.0
2020-03-07T16:45:46.1282900Z Author       : Microsoft
---
2020-03-07T16:45:47.6213775Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-07T16:45:47.6220257Z ##[command]git config gc.auto 0
2020-03-07T16:45:47.6224026Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-07T16:45:47.6227487Z ##[command]git config --get-all http.proxy
2020-03-07T16:45:47.6240360Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69806/merge:refs/remotes/pull/69806/merge
---
2020-03-07T16:51:04.3168642Z    Compiling serde_json v1.0.40
2020-03-07T16:51:06.2008176Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-03-07T16:51:15.1055065Z     Finished release [optimized] target(s) in 1m 15s
2020-03-07T16:51:15.1148994Z tidy check
2020-03-07T16:51:15.9197910Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0628.md:9: line longer than 80 chars
2020-03-07T16:51:15.9199082Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0628.md:10: trailing whitespace
2020-03-07T16:51:17.6204148Z Found 489 error codes
2020-03-07T16:51:17.6204574Z Found 0 error codes with no tests
2020-03-07T16:51:17.6204788Z Done!
2020-03-07T16:51:17.6204956Z some tidy checks failed
2020-03-07T16:51:17.6204956Z some tidy checks failed
2020-03-07T16:51:17.6218479Z 
2020-03-07T16:51:17.6222031Z 
2020-03-07T16:51:17.6223543Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-03-07T16:51:17.6224521Z 
2020-03-07T16:51:17.6224662Z 
2020-03-07T16:51:17.6231057Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-07T16:51:17.6231471Z Build completed unsuccessfully in 0:01:26
2020-03-07T16:51:17.6231471Z Build completed unsuccessfully in 0:01:26
2020-03-07T16:51:17.6270207Z == clock drift check ==
2020-03-07T16:51:17.6292532Z   local time: Sat Mar  7 16:51:17 UTC 2020
2020-03-07T16:51:17.9123029Z   network time: Sat, 07 Mar 2020 16:51:17 GMT
2020-03-07T16:51:17.9127515Z == end clock drift check ==
2020-03-07T16:51:18.7031504Z 
2020-03-07T16:51:18.7074878Z ##[error]Bash exited with code '1'.
2020-03-07T16:51:18.7088951Z ##[section]Finishing: Run build
2020-03-07T16:51:18.7136418Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69806/merge to s
2020-03-07T16:51:18.7141268Z Task         : Get sources
2020-03-07T16:51:18.7141613Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-07T16:51:18.7141945Z Version      : 1.0.0
2020-03-07T16:51:18.7142186Z Author       : Microsoft
2020-03-07T16:51:18.7142186Z Author       : Microsoft
2020-03-07T16:51:18.7142542Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-07T16:51:18.7142949Z ==============================================================================
2020-03-07T16:51:19.0326425Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-07T16:51:19.0369228Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69806/merge to s
2020-03-07T16:51:19.0460015Z Cleaning up task key
2020-03-07T16:51:19.0461520Z Start cleaning up orphan processes.
2020-03-07T16:51:19.0663935Z Terminate orphan process: pid (4752) (python)
2020-03-07T16:51:19.0804163Z ##[section]Finishing: Finalize Job
