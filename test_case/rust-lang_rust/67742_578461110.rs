plain
2020-01-26T01:32:20.6219419Z ========================== Starting Command Output ===========================
2020-01-26T01:32:20.6220740Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c46c7564-5e8c-43b0-831b-2527b378a1cd.sh
2020-01-26T01:32:20.6220769Z 
2020-01-26T01:32:20.6222892Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-26T01:32:20.6227942Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67742/merge to s
2020-01-26T01:32:20.6229699Z Task         : Get sources
2020-01-26T01:32:20.6229725Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-26T01:32:20.6229751Z Version      : 1.0.0
2020-01-26T01:32:20.6229783Z Author       : Microsoft
---
2020-01-26T01:32:21.6023826Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-26T01:32:21.6034869Z ##[command]git config gc.auto 0
2020-01-26T01:32:21.6037269Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-26T01:32:21.6039097Z ##[command]git config --get-all http.proxy
2020-01-26T01:32:21.6044039Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67742/merge:refs/remotes/pull/67742/merge
---
2020-01-26T01:37:22.6085342Z    Compiling serde_json v1.0.40
2020-01-26T01:37:23.7806323Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-01-26T01:37:30.9327117Z     Finished release [optimized] target(s) in 59.47s
2020-01-26T01:37:30.9408897Z tidy check
2020-01-26T01:37:31.2488721Z tidy error: /checkout/src/librustc_typeck/collect.rs: too many lines (3004) (add `// ignore-tidy-filelength` to the file to suppress this error)
2020-01-26T01:37:33.0423478Z Found 487 error codes
2020-01-26T01:37:33.0423616Z Found 0 error codes with no tests
2020-01-26T01:37:33.0423651Z Done!
2020-01-26T01:37:33.0428624Z some tidy checks failed
2020-01-26T01:37:33.0428624Z some tidy checks failed
2020-01-26T01:37:33.0430672Z 
2020-01-26T01:37:33.0432629Z 
2020-01-26T01:37:33.0435794Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-26T01:37:33.0442443Z 
2020-01-26T01:37:33.0442617Z 
2020-01-26T01:37:33.0446891Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-26T01:37:33.0447272Z Build completed unsuccessfully in 0:01:07
2020-01-26T01:37:33.0447272Z Build completed unsuccessfully in 0:01:07
2020-01-26T01:37:33.0487802Z == clock drift check ==
2020-01-26T01:37:33.0497728Z   local time: Sun Jan 26 01:37:33 UTC 2020
2020-01-26T01:37:33.3136764Z   network time: Sun, 26 Jan 2020 01:37:33 GMT
2020-01-26T01:37:33.3136827Z == end clock drift check ==
2020-01-26T01:37:34.2513802Z 
2020-01-26T01:37:34.2602713Z ##[error]Bash exited with code '1'.
2020-01-26T01:37:34.2612409Z ##[section]Finishing: Run build
2020-01-26T01:37:34.2626651Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67742/merge to s
2020-01-26T01:37:34.2628218Z Task         : Get sources
2020-01-26T01:37:34.2628255Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-26T01:37:34.2628290Z Version      : 1.0.0
2020-01-26T01:37:34.2628342Z Author       : Microsoft
2020-01-26T01:37:34.2628342Z Author       : Microsoft
2020-01-26T01:37:34.2628378Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-26T01:37:34.2628428Z ==============================================================================
2020-01-26T01:37:34.5713209Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-26T01:37:34.5741453Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67742/merge to s
2020-01-26T01:37:34.5833102Z Cleaning up task key
2020-01-26T01:37:34.5833791Z Start cleaning up orphan processes.
2020-01-26T01:37:34.6067273Z Terminate orphan process: pid (4625) (python)
2020-01-26T01:37:34.6085260Z ##[section]Finishing: Finalize Job
