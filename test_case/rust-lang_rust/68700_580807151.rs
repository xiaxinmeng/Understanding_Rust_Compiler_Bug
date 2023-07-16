plain
2020-01-31T16:15:05.3638631Z ========================== Starting Command Output ===========================
2020-01-31T16:15:05.3659998Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6c9e28e6-71ba-49dd-8cd5-21242af713f0.sh
2020-01-31T16:15:05.3931565Z 
2020-01-31T16:15:05.3939622Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-31T16:15:05.3945487Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68700/merge to s
2020-01-31T16:15:05.3947192Z Task         : Get sources
2020-01-31T16:15:05.3947243Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-31T16:15:05.3947276Z Version      : 1.0.0
2020-01-31T16:15:05.3947310Z Author       : Microsoft
---
2020-01-31T16:15:06.4591690Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-31T16:15:06.4605863Z ##[command]git config gc.auto 0
2020-01-31T16:15:06.4611113Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-31T16:15:06.4615566Z ##[command]git config --get-all http.proxy
2020-01-31T16:15:06.4624183Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68700/merge:refs/remotes/pull/68700/merge
---
2020-01-31T16:20:46.3759933Z * 589 error codes
2020-01-31T16:20:46.3760688Z * highest error code: E0746
2020-01-31T16:20:46.4659007Z thread 'main' panicked at 'assertion failed: `(left != right)`
2020-01-31T16:20:46.4659832Z   left: `0`,
2020-01-31T16:20:46.4660767Z  right: `0`: "none" should be used when there is no issue, not "0"', src/tools/tidy/src/features.rs:417:21
2020-01-31T16:20:46.4664469Z 
2020-01-31T16:20:46.4664767Z 
2020-01-31T16:20:46.4664767Z 
2020-01-31T16:20:46.4665594Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-31T16:20:46.4666662Z 
2020-01-31T16:20:46.4666867Z 
2020-01-31T16:20:46.4675192Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-31T16:20:46.4676338Z Build completed unsuccessfully in 0:01:35
2020-01-31T16:20:46.4676338Z Build completed unsuccessfully in 0:01:35
2020-01-31T16:20:46.4726342Z == clock drift check ==
2020-01-31T16:20:46.4736787Z   local time: Fri Jan 31 16:20:46 UTC 2020
2020-01-31T16:20:46.6330126Z   network time: Fri, 31 Jan 2020 16:20:46 GMT
2020-01-31T16:20:46.6336348Z == end clock drift check ==
2020-01-31T16:20:47.3851644Z 
2020-01-31T16:20:47.3955483Z ##[error]Bash exited with code '1'.
2020-01-31T16:20:47.3968246Z ##[section]Finishing: Run build
2020-01-31T16:20:47.3983208Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68700/merge to s
2020-01-31T16:20:47.3984981Z Task         : Get sources
2020-01-31T16:20:47.3985048Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-31T16:20:47.3985098Z Version      : 1.0.0
2020-01-31T16:20:47.3985141Z Author       : Microsoft
2020-01-31T16:20:47.3985141Z Author       : Microsoft
2020-01-31T16:20:47.3985219Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-31T16:20:47.3985272Z ==============================================================================
2020-01-31T16:20:47.8399674Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-31T16:20:47.8442657Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68700/merge to s
2020-01-31T16:20:47.8561273Z Cleaning up task key
2020-01-31T16:20:47.8562356Z Start cleaning up orphan processes.
2020-01-31T16:20:47.8681815Z Terminate orphan process: pid (3896) (python)
2020-01-31T16:20:47.8893305Z ##[section]Finishing: Finalize Job
