plain
2019-10-01T01:37:14.2424477Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-01T01:37:14.2650169Z ##[command]git config gc.auto 0
2019-10-01T01:37:14.2713246Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-01T01:37:14.2787505Z ##[command]git config --get-all http.proxy
2019-10-01T01:37:14.2943444Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64938/merge:refs/remotes/pull/64938/merge
---
2019-10-01T01:40:12.5248018Z Looks like docker image is the same as before, not uploading
2019-10-01T01:40:23.0270268Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-10-01T01:40:23.2166328Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-10-01T01:40:23.2200333Z == clock drift check ==
2019-10-01T01:40:23.2209604Z   local time: Tue Oct  1 01:40:23 UTC 2019
2019-10-01T01:40:23.3060321Z   network time: Tue, 01 Oct 2019 01:40:23 GMT
2019-10-01T01:40:23.3083495Z Starting sccache server...
2019-10-01T01:40:23.4097113Z configure: processing command line
2019-10-01T01:40:23.4097322Z configure: 
2019-10-01T01:40:23.4098525Z configure: rust.dist-src        := False
---
2019-10-01T01:44:48.9105752Z    Compiling serde_json v1.0.40
2019-10-01T01:44:50.8001412Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-01T01:45:02.4706691Z     Finished release [optimized] target(s) in 1m 32s
2019-10-01T01:45:02.4802239Z tidy check
2019-10-01T01:45:02.6521902Z tidy error: /checkout/src/test/ui/associated-types/issue-64855.rs: too many trailing newlines (2)
2019-10-01T01:45:04.5484778Z some tidy checks failed
2019-10-01T01:45:04.5487639Z 
2019-10-01T01:45:04.5487639Z 
2019-10-01T01:45:04.5488896Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-01T01:45:04.5489058Z 
2019-10-01T01:45:04.5504307Z 
2019-10-01T01:45:04.5504605Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-01T01:45:04.5504699Z Build completed unsuccessfully in 0:01:35
2019-10-01T01:45:04.5504699Z Build completed unsuccessfully in 0:01:35
2019-10-01T01:45:04.5553291Z == clock drift check ==
2019-10-01T01:45:04.5567709Z   local time: Tue Oct  1 01:45:04 UTC 2019
2019-10-01T01:45:04.7049777Z   network time: Tue, 01 Oct 2019 01:45:04 GMT
2019-10-01T01:45:04.7054848Z == end clock drift check ==
2019-10-01T01:45:06.1526411Z ##[error]Bash exited with code '1'.
2019-10-01T01:45:06.1585935Z ##[section]Starting: Checkout
2019-10-01T01:45:06.1588202Z ==============================================================================
2019-10-01T01:45:06.1588287Z Task         : Get sources
2019-10-01T01:45:06.1588343Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
