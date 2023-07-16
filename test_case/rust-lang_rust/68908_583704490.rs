plain
2020-02-08T05:06:09.2615966Z ========================== Starting Command Output ===========================
2020-02-08T05:06:09.2633806Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a3a33976-acc8-4801-925e-8acc7f793914.sh
2020-02-08T05:06:09.7742108Z 
2020-02-08T05:06:09.7801648Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-08T05:06:09.7811163Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68908/merge to s
2020-02-08T05:06:09.7812924Z Task         : Get sources
2020-02-08T05:06:09.7813004Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-08T05:06:09.7813146Z Version      : 1.0.0
2020-02-08T05:06:09.7813175Z Author       : Microsoft
---
2020-02-08T05:06:16.7416562Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-08T05:06:16.7983433Z ##[command]git config gc.auto 0
2020-02-08T05:06:16.8057383Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-08T05:06:16.8116922Z ##[command]git config --get-all http.proxy
2020-02-08T05:06:16.8250808Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68908/merge:refs/remotes/pull/68908/merge
---
2020-02-08T05:13:05.8808578Z Found 0 error codes with no tests
2020-02-08T05:13:05.8808619Z Done!
2020-02-08T05:13:05.8818426Z 
2020-02-08T05:13:05.8818491Z 
2020-02-08T05:13:05.8819528Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-02-08T05:13:05.8819642Z 
2020-02-08T05:13:05.8819687Z 
2020-02-08T05:13:05.8825693Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-02-08T05:13:05.8825764Z Build completed unsuccessfully in 0:01:35
2020-02-08T05:13:05.8825764Z Build completed unsuccessfully in 0:01:35
2020-02-08T05:13:05.8874351Z == clock drift check ==
2020-02-08T05:13:05.8905739Z   local time: Sat Feb  8 05:13:05 UTC 2020
2020-02-08T05:13:06.1785688Z   network time: Sat, 08 Feb 2020 05:13:06 GMT
2020-02-08T05:13:06.1789098Z == end clock drift check ==
2020-02-08T05:13:07.0359661Z 
2020-02-08T05:13:07.0453109Z ##[error]Bash exited with code '1'.
2020-02-08T05:13:07.0463665Z ##[section]Finishing: Run build
2020-02-08T05:13:07.0476435Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68908/merge to s
2020-02-08T05:13:07.0478088Z Task         : Get sources
2020-02-08T05:13:07.0478127Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-08T05:13:07.0478168Z Version      : 1.0.0
2020-02-08T05:13:07.0478233Z Author       : Microsoft
2020-02-08T05:13:07.0478233Z Author       : Microsoft
2020-02-08T05:13:07.0478271Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-08T05:13:07.0478313Z ==============================================================================
2020-02-08T05:13:07.4339805Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-08T05:13:07.4377852Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68908/merge to s
2020-02-08T05:13:07.4504131Z Cleaning up task key
2020-02-08T05:13:07.4504912Z Start cleaning up orphan processes.
2020-02-08T05:13:07.4625866Z Terminate orphan process: pid (3837) (python)
2020-02-08T05:13:07.5130681Z ##[section]Finishing: Finalize Job
