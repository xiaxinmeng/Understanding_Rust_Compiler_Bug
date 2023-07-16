plain
2020-03-10T23:16:15.0738544Z ========================== Starting Command Output ===========================
2020-03-10T23:16:15.0743677Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ba6fa79a-18ca-446f-bb7d-1b8e88e4dbeb.sh
2020-03-10T23:16:15.0744207Z 
2020-03-10T23:16:15.0749598Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-10T23:16:15.0770358Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68692/merge to s
2020-03-10T23:16:15.0774055Z Task         : Get sources
2020-03-10T23:16:15.0774403Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-10T23:16:15.0774717Z Version      : 1.0.0
2020-03-10T23:16:15.0774932Z Author       : Microsoft
---
2020-03-10T23:16:16.0619286Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-10T23:16:16.0624218Z ##[command]git config gc.auto 0
2020-03-10T23:16:16.0627657Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-10T23:16:16.0630834Z ##[command]git config --get-all http.proxy
2020-03-10T23:16:16.0637640Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68692/merge:refs/remotes/pull/68692/merge
---
2020-03-10T23:22:35.6959411Z    Compiling serde_json v1.0.40
2020-03-10T23:22:37.3355995Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-03-10T23:22:46.9968289Z     Finished release [optimized] target(s) in 1m 18s
2020-03-10T23:22:47.0078440Z tidy check
2020-03-10T23:22:47.9989104Z tidy error: /checkout/src/liballoc/vec.rs: too many lines (3003) (add `// ignore-tidy-filelength` to the file to suppress this error)
2020-03-10T23:22:49.6691518Z Found 489 error codes
2020-03-10T23:22:49.6691839Z Found 0 error codes with no tests
2020-03-10T23:22:49.6692061Z Done!
2020-03-10T23:22:49.6692302Z some tidy checks failed
2020-03-10T23:22:49.6692302Z some tidy checks failed
2020-03-10T23:22:49.6692466Z 
2020-03-10T23:22:49.6692579Z 
2020-03-10T23:22:49.6697696Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-03-10T23:22:49.6698601Z 
2020-03-10T23:22:49.6755551Z 
2020-03-10T23:22:49.6756002Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-10T23:22:49.6756418Z Build completed unsuccessfully in 0:01:30
2020-03-10T23:22:49.6756418Z Build completed unsuccessfully in 0:01:30
2020-03-10T23:22:49.6761819Z == clock drift check ==
2020-03-10T23:22:49.6771363Z   local time: Tue Mar 10 23:22:49 UTC 2020
2020-03-10T23:22:49.9651667Z   network time: Tue, 10 Mar 2020 23:22:49 GMT
2020-03-10T23:22:49.9654536Z == end clock drift check ==
2020-03-10T23:22:50.7935838Z 
2020-03-10T23:22:50.8026693Z ##[error]Bash exited with code '1'.
2020-03-10T23:22:50.8042965Z ##[section]Finishing: Run build
2020-03-10T23:22:50.8090972Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68692/merge to s
2020-03-10T23:22:50.8096311Z Task         : Get sources
2020-03-10T23:22:50.8096710Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-10T23:22:50.8097077Z Version      : 1.0.0
2020-03-10T23:22:50.8097354Z Author       : Microsoft
2020-03-10T23:22:50.8097354Z Author       : Microsoft
2020-03-10T23:22:50.8097762Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-10T23:22:50.8098231Z ==============================================================================
2020-03-10T23:22:51.1597397Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-10T23:22:51.1655185Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68692/merge to s
2020-03-10T23:22:51.1765133Z Cleaning up task key
2020-03-10T23:22:51.1766569Z Start cleaning up orphan processes.
2020-03-10T23:22:51.1975749Z Terminate orphan process: pid (4707) (python)
2020-03-10T23:22:51.2147267Z ##[section]Finishing: Finalize Job
