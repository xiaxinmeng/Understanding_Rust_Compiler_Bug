plain
2020-02-04T00:41:46.8057423Z ========================== Starting Command Output ===========================
2020-02-04T00:41:46.8059115Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f2b78b9e-f0d3-47a7-acb5-ec7a8cd1177e.sh
2020-02-04T00:41:46.8059153Z 
2020-02-04T00:41:46.8061992Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-04T00:41:46.8067836Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68816/merge to s
2020-02-04T00:41:46.8069965Z Task         : Get sources
2020-02-04T00:41:46.8070000Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-04T00:41:46.8070032Z Version      : 1.0.0
2020-02-04T00:41:46.8070065Z Author       : Microsoft
---
2020-02-04T00:41:47.8001834Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-04T00:41:47.8010369Z ##[command]git config gc.auto 0
2020-02-04T00:41:47.8012190Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-04T00:41:47.8014139Z ##[command]git config --get-all http.proxy
2020-02-04T00:41:47.8021099Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68816/merge:refs/remotes/pull/68816/merge
---
2020-02-04T00:47:03.9491332Z Found 0 error codes with no tests
2020-02-04T00:47:03.9491464Z Done!
2020-02-04T00:47:03.9494173Z 
2020-02-04T00:47:03.9494357Z 
2020-02-04T00:47:03.9495329Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-02-04T00:47:03.9495679Z 
2020-02-04T00:47:03.9495780Z 
2020-02-04T00:47:03.9503441Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-02-04T00:47:03.9504777Z Build completed unsuccessfully in 0:01:24
2020-02-04T00:47:03.9504777Z Build completed unsuccessfully in 0:01:24
2020-02-04T00:47:03.9554095Z == clock drift check ==
2020-02-04T00:47:03.9562058Z   local time: Tue Feb  4 00:47:03 UTC 2020
2020-02-04T00:47:04.2468356Z   network time: Tue, 04 Feb 2020 00:47:04 GMT
2020-02-04T00:47:04.2473120Z == end clock drift check ==
2020-02-04T00:47:04.9934309Z 
2020-02-04T00:47:05.0014713Z ##[error]Bash exited with code '1'.
2020-02-04T00:47:05.0025045Z ##[section]Finishing: Run build
2020-02-04T00:47:05.0037940Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68816/merge to s
2020-02-04T00:47:05.0039455Z Task         : Get sources
2020-02-04T00:47:05.0039492Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-04T00:47:05.0039546Z Version      : 1.0.0
2020-02-04T00:47:05.0039578Z Author       : Microsoft
2020-02-04T00:47:05.0039578Z Author       : Microsoft
2020-02-04T00:47:05.0039612Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-04T00:47:05.0039682Z ==============================================================================
2020-02-04T00:47:05.3674610Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-04T00:47:05.3716863Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68816/merge to s
2020-02-04T00:47:05.3941642Z Cleaning up task key
2020-02-04T00:47:05.3943579Z Start cleaning up orphan processes.
2020-02-04T00:47:05.4085095Z Terminate orphan process: pid (3667) (python)
2020-02-04T00:47:05.4297677Z ##[section]Finishing: Finalize Job
