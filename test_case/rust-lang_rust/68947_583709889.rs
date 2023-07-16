plain
2020-02-08T06:37:25.2203639Z ========================== Starting Command Output ===========================
2020-02-08T06:37:25.2205346Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1ba8c4c4-4c8b-4826-8216-b17f56575bb7.sh
2020-02-08T06:37:25.2205375Z 
2020-02-08T06:37:25.2208329Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-08T06:37:25.2214760Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68947/merge to s
2020-02-08T06:37:25.2217204Z Task         : Get sources
2020-02-08T06:37:25.2217231Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-08T06:37:25.2217256Z Version      : 1.0.0
2020-02-08T06:37:25.2217457Z Author       : Microsoft
---
2020-02-08T06:37:26.2198160Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-08T06:37:26.2210697Z ##[command]git config gc.auto 0
2020-02-08T06:37:26.2214777Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-08T06:37:26.2218687Z ##[command]git config --get-all http.proxy
2020-02-08T06:37:26.2226300Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68947/merge:refs/remotes/pull/68947/merge
---
2020-02-08T06:43:07.8023051Z    Compiling serde_json v1.0.40
2020-02-08T06:43:09.3787527Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-02-08T06:43:18.8489817Z     Finished release [optimized] target(s) in 1m 25s
2020-02-08T06:43:18.8580097Z tidy check
2020-02-08T06:43:19.3664453Z tidy error: /checkout/src/etc/lldb_rust_formatters.py:160: line longer than 100 chars
2020-02-08T06:43:19.3665350Z tidy error: /checkout/src/etc/lldb_rust_formatters.py:162: line longer than 100 chars
2020-02-08T06:43:19.3665641Z tidy error: /checkout/src/etc/lldb_rust_formatters.py:268: line longer than 100 chars
2020-02-08T06:43:19.3666019Z tidy error: /checkout/src/etc/lldb_rust_formatters.py:270: line longer than 100 chars
2020-02-08T06:43:21.3009562Z Found 487 error codes
2020-02-08T06:43:21.3010309Z Found 0 error codes with no tests
2020-02-08T06:43:21.3010480Z Done!
2020-02-08T06:43:21.3010721Z some tidy checks failed
2020-02-08T06:43:21.3010721Z some tidy checks failed
2020-02-08T06:43:21.3022774Z 
2020-02-08T06:43:21.3023025Z 
2020-02-08T06:43:21.3023970Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-02-08T06:43:21.3024351Z 
2020-02-08T06:43:21.3024467Z 
2020-02-08T06:43:21.3029079Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-02-08T06:43:21.3029324Z Build completed unsuccessfully in 0:01:34
2020-02-08T06:43:21.3029324Z Build completed unsuccessfully in 0:01:34
2020-02-08T06:43:21.3074544Z == clock drift check ==
2020-02-08T06:43:21.3102016Z   local time: Sat Feb  8 06:43:21 UTC 2020
2020-02-08T06:43:21.6395058Z   network time: Sat, 08 Feb 2020 06:43:21 GMT
2020-02-08T06:43:21.6395514Z == end clock drift check ==
2020-02-08T06:43:22.3877138Z 
2020-02-08T06:43:22.3968747Z ##[error]Bash exited with code '1'.
2020-02-08T06:43:22.3979641Z ##[section]Finishing: Run build
2020-02-08T06:43:22.3992218Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68947/merge to s
2020-02-08T06:43:22.3994671Z Task         : Get sources
2020-02-08T06:43:22.3994726Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-08T06:43:22.3994765Z Version      : 1.0.0
2020-02-08T06:43:22.3994798Z Author       : Microsoft
2020-02-08T06:43:22.3994798Z Author       : Microsoft
2020-02-08T06:43:22.3994857Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-08T06:43:22.3994898Z ==============================================================================
2020-02-08T06:43:22.7836465Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-08T06:43:22.7873708Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68947/merge to s
2020-02-08T06:43:22.7985140Z Cleaning up task key
2020-02-08T06:43:22.7985822Z Start cleaning up orphan processes.
2020-02-08T06:43:22.8097811Z Terminate orphan process: pid (3991) (python)
2020-02-08T06:43:22.8308603Z ##[section]Finishing: Finalize Job
