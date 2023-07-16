plain
2020-01-18T21:23:56.7487009Z ========================== Starting Command Output ===========================
2020-01-18T21:23:56.7488659Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/28481c88-bcbf-403c-b2c7-317609519229.sh
2020-01-18T21:23:56.7488693Z 
2020-01-18T21:23:56.7491339Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-18T21:23:56.7497762Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/66383/merge to s
2020-01-18T21:23:56.7499514Z Task         : Get sources
2020-01-18T21:23:56.7499548Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T21:23:56.7499580Z Version      : 1.0.0
2020-01-18T21:23:56.7499612Z Author       : Microsoft
---
2020-01-18T21:24:02.3732822Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-18T21:24:02.3756804Z ##[command]git config gc.auto 0
2020-01-18T21:24:02.3762896Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-18T21:24:02.3768436Z ##[command]git config --get-all http.proxy
2020-01-18T21:24:02.3778227Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66383/merge:refs/remotes/pull/66383/merge
---
2020-01-18T21:30:17.6339155Z * 589 error codes
2020-01-18T21:30:17.6339274Z * highest error code: E0746
2020-01-18T21:30:17.7060074Z thread 'main' panicked at 'assertion failed: `(left != right)`
2020-01-18T21:30:17.7060200Z   left: `0`,
2020-01-18T21:30:17.7060569Z  right: `0`: "none" should be used when there is no issue, not "0"', src/tools/tidy/src/features.rs:417:21
2020-01-18T21:30:17.7119581Z 
2020-01-18T21:30:17.7120039Z 
2020-01-18T21:30:17.7120039Z 
2020-01-18T21:30:17.7121399Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-18T21:30:17.7122304Z 
2020-01-18T21:30:17.7122425Z 
2020-01-18T21:30:17.7122652Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-18T21:30:17.7122834Z Build completed unsuccessfully in 0:01:32
2020-01-18T21:30:17.7122834Z Build completed unsuccessfully in 0:01:32
2020-01-18T21:30:17.7132198Z == clock drift check ==
2020-01-18T21:30:17.7140311Z   local time: Sat Jan 18 21:30:17 UTC 2020
2020-01-18T21:30:17.8077554Z   network time: Sat, 18 Jan 2020 21:30:17 GMT
2020-01-18T21:30:17.8082675Z == end clock drift check ==
2020-01-18T21:30:18.6615572Z 
2020-01-18T21:30:18.6791710Z ##[error]Bash exited with code '1'.
2020-01-18T21:30:18.6805668Z ##[section]Finishing: Run build
2020-01-18T21:30:18.6822010Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/66383/merge to s
2020-01-18T21:30:18.6824619Z Task         : Get sources
2020-01-18T21:30:18.6824671Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T21:30:18.6824761Z Version      : 1.0.0
2020-01-18T21:30:18.6824807Z Author       : Microsoft
2020-01-18T21:30:18.6824807Z Author       : Microsoft
2020-01-18T21:30:18.6824860Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-18T21:30:18.6824931Z ==============================================================================
2020-01-18T21:30:19.1410863Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-18T21:30:19.1455235Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/66383/merge to s
2020-01-18T21:30:19.1581217Z Cleaning up task key
2020-01-18T21:30:19.1582110Z Start cleaning up orphan processes.
2020-01-18T21:30:19.1741798Z Terminate orphan process: pid (3779) (python)
2020-01-18T21:30:19.2169903Z ##[section]Finishing: Finalize Job
