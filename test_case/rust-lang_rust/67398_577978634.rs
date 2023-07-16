plain
2020-01-24T01:07:28.8804770Z ========================== Starting Command Output ===========================
2020-01-24T01:07:28.8806773Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b46e2957-b196-4098-8408-8e06c7cc1a19.sh
2020-01-24T01:07:28.8806809Z 
2020-01-24T01:07:28.8809584Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-24T01:07:28.8815360Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67398/merge to s
2020-01-24T01:07:28.8817338Z Task         : Get sources
2020-01-24T01:07:28.8817371Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-24T01:07:28.8817402Z Version      : 1.0.0
2020-01-24T01:07:28.8817434Z Author       : Microsoft
---
2020-01-24T01:07:29.8755899Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-24T01:07:29.8766730Z ##[command]git config gc.auto 0
2020-01-24T01:07:29.8768950Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-24T01:07:29.8770863Z ##[command]git config --get-all http.proxy
2020-01-24T01:07:29.8776480Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67398/merge:refs/remotes/pull/67398/merge
---
2020-01-24T01:12:38.3923991Z tidy check
2020-01-24T01:12:39.7411854Z * 589 error codes
2020-01-24T01:12:39.7413131Z * highest error code: E0746
2020-01-24T01:12:40.0900993Z * 278 features
2020-01-24T01:12:40.7342713Z invalid source: "git+https://github.com/alexcrichton/jobserver-rs#3fa404cfb39cfc77294e5492c0e40954b065f767"
2020-01-24T01:12:40.9818943Z some tidy checks failed
2020-01-24T01:12:40.9819689Z Found 487 error codes
2020-01-24T01:12:40.9820182Z Found 0 error codes with no tests
2020-01-24T01:12:40.9820760Z Done!
2020-01-24T01:12:40.9820760Z Done!
2020-01-24T01:12:40.9823753Z 
2020-01-24T01:12:40.9824099Z 
2020-01-24T01:12:40.9825279Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-24T01:12:40.9826146Z 
2020-01-24T01:12:40.9827028Z 
2020-01-24T01:12:40.9833491Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-24T01:12:40.9834412Z Build completed unsuccessfully in 0:01:24
2020-01-24T01:12:40.9834412Z Build completed unsuccessfully in 0:01:24
2020-01-24T01:12:40.9885990Z == clock drift check ==
2020-01-24T01:12:40.9895619Z   local time: Fri Jan 24 01:12:40 UTC 2020
2020-01-24T01:12:41.0137527Z   network time: Fri, 24 Jan 2020 01:12:41 GMT
2020-01-24T01:12:41.0137621Z == end clock drift check ==
2020-01-24T01:12:41.7571779Z 
2020-01-24T01:12:41.7681695Z ##[error]Bash exited with code '1'.
2020-01-24T01:12:41.7694628Z ##[section]Finishing: Run build
2020-01-24T01:12:41.7711269Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67398/merge to s
2020-01-24T01:12:41.7713218Z Task         : Get sources
2020-01-24T01:12:41.7713268Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-24T01:12:41.7713351Z Version      : 1.0.0
2020-01-24T01:12:41.7713395Z Author       : Microsoft
2020-01-24T01:12:41.7713395Z Author       : Microsoft
2020-01-24T01:12:41.7713443Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-24T01:12:41.7713512Z ==============================================================================
2020-01-24T01:12:42.1876851Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-24T01:12:42.1917327Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67398/merge to s
2020-01-24T01:12:42.2042781Z Cleaning up task key
2020-01-24T01:12:42.2043595Z Start cleaning up orphan processes.
2020-01-24T01:12:42.2161819Z Terminate orphan process: pid (3702) (python)
2020-01-24T01:12:42.2368623Z ##[section]Finishing: Finalize Job
