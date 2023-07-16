plain
2020-02-04T07:23:38.9951774Z ========================== Starting Command Output ===========================
2020-02-04T07:23:38.9954619Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/525878a4-f6a0-4f83-a711-9a1fb7dd6b0e.sh
2020-02-04T07:23:38.9954885Z 
2020-02-04T07:23:38.9958475Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-04T07:23:38.9965915Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67272/merge to s
2020-02-04T07:23:38.9967824Z Task         : Get sources
2020-02-04T07:23:38.9967863Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-04T07:23:38.9967901Z Version      : 1.0.0
2020-02-04T07:23:38.9967938Z Author       : Microsoft
---
2020-02-04T07:23:40.0953494Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-04T07:23:40.1062251Z ##[command]git config gc.auto 0
2020-02-04T07:23:40.1150471Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-04T07:23:40.1260782Z ##[command]git config --get-all http.proxy
2020-02-04T07:23:40.1387643Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67272/merge:refs/remotes/pull/67272/merge
---
2020-02-04T07:29:10.1440962Z    Compiling serde_json v1.0.40
2020-02-04T07:29:11.8915846Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-02-04T07:29:22.5569870Z     Finished release [optimized] target(s) in 1m 26s
2020-02-04T07:29:22.5697227Z tidy check
2020-02-04T07:29:23.3189297Z tidy error: /checkout/src/test/ui/recursion_limit/overflow.rs:3: line longer than 100 chars
2020-02-04T07:29:25.4810383Z some tidy checks failed
2020-02-04T07:29:25.4810493Z Found 487 error codes
2020-02-04T07:29:25.4810545Z Found 0 error codes with no tests
2020-02-04T07:29:25.4810638Z Done!
2020-02-04T07:29:25.4810638Z Done!
2020-02-04T07:29:25.4823037Z 
2020-02-04T07:29:25.4823165Z 
2020-02-04T07:29:25.4824273Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-02-04T07:29:25.4824431Z 
2020-02-04T07:29:25.4824494Z 
2020-02-04T07:29:25.4828999Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-02-04T07:29:25.4829270Z Build completed unsuccessfully in 0:01:37
2020-02-04T07:29:25.4829270Z Build completed unsuccessfully in 0:01:37
2020-02-04T07:29:25.4901497Z == clock drift check ==
2020-02-04T07:29:25.4912456Z   local time: Tue Feb  4 07:29:25 UTC 2020
2020-02-04T07:29:25.7831982Z   network time: Tue, 04 Feb 2020 07:29:25 GMT
2020-02-04T07:29:25.7833564Z == end clock drift check ==
2020-02-04T07:29:26.4913578Z 
2020-02-04T07:29:26.5030969Z ##[error]Bash exited with code '1'.
2020-02-04T07:29:26.5061623Z ##[section]Finishing: Run build
2020-02-04T07:29:26.5077949Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67272/merge to s
2020-02-04T07:29:26.5080420Z Task         : Get sources
2020-02-04T07:29:26.5080474Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-04T07:29:26.5080545Z Version      : 1.0.0
2020-02-04T07:29:26.5080593Z Author       : Microsoft
2020-02-04T07:29:26.5080593Z Author       : Microsoft
2020-02-04T07:29:26.5080646Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-04T07:29:26.5080724Z ==============================================================================
2020-02-04T07:29:26.9647445Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-04T07:29:26.9699288Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67272/merge to s
2020-02-04T07:29:26.9833912Z Cleaning up task key
2020-02-04T07:29:26.9835012Z Start cleaning up orphan processes.
2020-02-04T07:29:26.9984618Z Terminate orphan process: pid (4367) (python)
2020-02-04T07:29:27.0204208Z ##[section]Finishing: Finalize Job
