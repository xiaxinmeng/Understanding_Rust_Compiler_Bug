plain
2019-12-11T22:52:12.4888401Z ========================== Starting Command Output ===========================
2019-12-11T22:52:12.4892514Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/be45803e-018a-4843-ac2f-4545a75cbccf.sh
2019-12-11T22:52:12.4893053Z 
2019-12-11T22:52:12.4898046Z ##[section]Finishing: Disable git automatic line ending conversion
2019-12-11T22:52:12.4904826Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67241/merge to s
2019-12-11T22:52:12.4906310Z Task         : Get sources
2019-12-11T22:52:12.4906341Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2019-12-11T22:52:12.4906414Z Version      : 1.0.0
2019-12-11T22:52:12.4906444Z Author       : Microsoft
---
2019-12-11T22:52:14.5439410Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-11T22:52:15.1893226Z ##[command]git config gc.auto 0
2019-12-11T22:52:15.1897776Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-11T22:52:15.1902257Z ##[command]git config --get-all http.proxy
2019-12-11T22:52:15.1911274Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67241/merge:refs/remotes/pull/67241/merge
---
2019-12-11T22:58:22.7539345Z    Compiling serde_json v1.0.40
2019-12-11T22:58:24.6367223Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-11T22:58:36.6133405Z     Finished release [optimized] target(s) in 1m 34s
2019-12-11T22:58:36.6245952Z tidy check
2019-12-11T22:58:37.6304437Z tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1566: TODO is deprecated; use FIXME
2019-12-11T22:58:37.6477665Z tidy error: /checkout/src/librustc_mir/borrow_check/region_infer/mod.rs:485: TODO is deprecated; use FIXME
2019-12-11T22:58:39.5841201Z Found 485 error codes
2019-12-11T22:58:39.5846696Z Found 0 error codes with no tests
2019-12-11T22:58:39.5846865Z Done!
2019-12-11T22:58:39.5846916Z some tidy checks failed
2019-12-11T22:58:39.5846916Z some tidy checks failed
2019-12-11T22:58:39.5846949Z 
2019-12-11T22:58:39.5846977Z 
2019-12-11T22:58:39.5847971Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-11T22:58:39.5848121Z 
2019-12-11T22:58:39.5848148Z 
2019-12-11T22:58:39.5852679Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-11T22:58:39.5852951Z Build completed unsuccessfully in 0:01:38
2019-12-11T22:58:39.5852951Z Build completed unsuccessfully in 0:01:38
2019-12-11T22:58:39.5902774Z == clock drift check ==
2019-12-11T22:58:39.5912670Z   local time: Wed Dec 11 22:58:39 UTC 2019
2019-12-11T22:58:39.8737427Z   network time: Wed, 11 Dec 2019 22:58:39 GMT
2019-12-11T22:58:39.8744167Z == end clock drift check ==
2019-12-11T22:58:41.1903851Z 
2019-12-11T22:58:41.2030046Z ##[error]Bash exited with code '1'.
2019-12-11T22:58:41.2044974Z ##[section]Finishing: Run build
2019-12-11T22:58:41.2061287Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67241/merge to s
2019-12-11T22:58:41.2063545Z Task         : Get sources
2019-12-11T22:58:41.2063607Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2019-12-11T22:58:41.2063673Z Version      : 1.0.0
2019-12-11T22:58:41.2063714Z Author       : Microsoft
2019-12-11T22:58:41.2063714Z Author       : Microsoft
2019-12-11T22:58:41.2063778Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2019-12-11T22:58:41.2063828Z ==============================================================================
2019-12-11T22:58:41.7023368Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2019-12-11T22:58:41.7072498Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67241/merge to s
2019-12-11T22:58:41.7197471Z Start cleaning up orphan processes.
2019-12-11T22:58:41.7357203Z Terminate orphan process: pid (3591) (python)
2019-12-11T22:58:41.7684046Z ##[section]Finishing: Finalize Job
2019-12-11T22:58:41.7745846Z ##[section]Finishing: Linux x86_64-gnu-llvm-7
