plain
2020-02-02T15:52:44.6597906Z ========================== Starting Command Output ===========================
2020-02-02T15:52:44.6599890Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/05a112d7-4169-448c-af76-ddb09955816c.sh
2020-02-02T15:52:44.6599925Z 
2020-02-02T15:52:44.6602767Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-02T15:52:44.6607658Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68700/merge to s
2020-02-02T15:52:44.6609808Z Task         : Get sources
2020-02-02T15:52:44.6609834Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-02T15:52:44.6609860Z Version      : 1.0.0
2020-02-02T15:52:44.6609892Z Author       : Microsoft
---
2020-02-02T15:52:45.4755220Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-02T15:52:45.4844260Z ##[command]git config gc.auto 0
2020-02-02T15:52:45.4897914Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-02T15:52:45.4944460Z ##[command]git config --get-all http.proxy
2020-02-02T15:52:45.5080201Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68700/merge:refs/remotes/pull/68700/merge
---
2020-02-02T15:57:52.1583998Z * 589 error codes
2020-02-02T15:57:52.1584962Z * highest error code: E0746
2020-02-02T15:57:52.2434408Z thread 'main' panicked at 'assertion failed: `(left != right)`
2020-02-02T15:57:52.2437304Z   left: `0`,
2020-02-02T15:57:52.2438681Z  right: `0`: "none" should be used when there is no issue, not "0"', src/tools/tidy/src/features.rs:417:21
2020-02-02T15:57:52.2446962Z 
2020-02-02T15:57:52.2447321Z 
2020-02-02T15:57:52.2447321Z 
2020-02-02T15:57:52.2448246Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-02-02T15:57:52.2448829Z 
2020-02-02T15:57:52.2449201Z 
2020-02-02T15:57:52.2451649Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-02-02T15:57:52.2452111Z Build completed unsuccessfully in 0:01:20
2020-02-02T15:57:52.2452111Z Build completed unsuccessfully in 0:01:20
2020-02-02T15:57:52.2495500Z == clock drift check ==
2020-02-02T15:57:52.2503642Z   local time: Sun Feb  2 15:57:52 UTC 2020
2020-02-02T15:57:52.4172018Z   network time: Sun, 02 Feb 2020 15:57:52 GMT
2020-02-02T15:57:52.4178376Z == end clock drift check ==
2020-02-02T15:57:53.1970241Z 
2020-02-02T15:57:53.2035649Z ##[error]Bash exited with code '1'.
2020-02-02T15:57:53.2046900Z ##[section]Finishing: Run build
2020-02-02T15:57:53.2060199Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68700/merge to s
2020-02-02T15:57:53.2062062Z Task         : Get sources
2020-02-02T15:57:53.2062104Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-02T15:57:53.2062161Z Version      : 1.0.0
2020-02-02T15:57:53.2062198Z Author       : Microsoft
2020-02-02T15:57:53.2062198Z Author       : Microsoft
2020-02-02T15:57:53.2062255Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-02T15:57:53.2062315Z ==============================================================================
2020-02-02T15:57:53.5702836Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-02T15:57:53.5740037Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68700/merge to s
2020-02-02T15:57:53.5846063Z Cleaning up task key
2020-02-02T15:57:53.5847165Z Start cleaning up orphan processes.
2020-02-02T15:57:53.5955071Z Terminate orphan process: pid (3756) (python)
2020-02-02T15:57:53.6135047Z ##[section]Finishing: Finalize Job
