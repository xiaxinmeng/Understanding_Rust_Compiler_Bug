plain
2019-08-26T16:33:22.6263961Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-26T16:33:22.6466136Z ##[command]git config gc.auto 0
2019-08-26T16:33:22.6545393Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-26T16:33:22.6609516Z ##[command]git config --get-all http.proxy
2019-08-26T16:33:22.6754042Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63860/merge:refs/remotes/pull/63860/merge
---
2019-08-26T16:33:57.9873068Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-26T16:33:57.9873105Z 
2019-08-26T16:33:57.9873920Z   git checkout -b <new-branch-name>
2019-08-26T16:33:57.9873968Z 
2019-08-26T16:33:57.9874025Z HEAD is now at 44e21c956 Merge ff0d2119d74458e8a7b4212afd1e0ffb77b3082b into 555d7a2fd6165b614cfc01136d8e3f5c465a1582
2019-08-26T16:33:58.0052120Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-26T16:33:58.0055939Z ==============================================================================
2019-08-26T16:33:58.0056010Z Task         : Bash
2019-08-26T16:33:58.0056061Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-26T16:40:35.2787272Z    Compiling serde_json v1.0.40
2019-08-26T16:40:37.1578827Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-26T16:40:48.6161790Z     Finished release [optimized] target(s) in 1m 35s
2019-08-26T16:40:48.6246194Z tidy check
2019-08-26T16:40:49.1157123Z tidy error: /checkout/src/librustc_mir/dataflow/generic.rs:262: line longer than 100 chars
2019-08-26T16:40:50.7392236Z some tidy checks failed
2019-08-26T16:40:50.7396740Z 
2019-08-26T16:40:50.7396740Z 
2019-08-26T16:40:50.7397834Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-26T16:40:50.7398433Z 
2019-08-26T16:40:50.7398649Z 
2019-08-26T16:40:50.7408141Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-26T16:40:50.7408622Z Build completed unsuccessfully in 0:01:39
2019-08-26T16:40:50.7408622Z Build completed unsuccessfully in 0:01:39
2019-08-26T16:40:50.7469808Z == clock drift check ==
2019-08-26T16:40:50.7506708Z   local time: Mon Aug 26 16:40:50 UTC 2019
2019-08-26T16:40:50.9090481Z   network time: Mon, 26 Aug 2019 16:40:50 GMT
2019-08-26T16:40:50.9094965Z == end clock drift check ==
2019-08-26T16:40:52.2849985Z ##[error]Bash exited with code '1'.
2019-08-26T16:40:52.2906789Z ##[section]Starting: Checkout
2019-08-26T16:40:52.2908570Z ==============================================================================
2019-08-26T16:40:52.2908625Z Task         : Get sources
2019-08-26T16:40:52.2908690Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
