plain
2020-03-10T03:07:25.2211237Z ========================== Starting Command Output ===========================
2020-03-10T03:07:25.2215738Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4992f08a-956f-4003-8658-7a76d923004c.sh
2020-03-10T03:07:25.2216276Z 
2020-03-10T03:07:25.2221697Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-10T03:07:25.2240768Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69876/merge to s
2020-03-10T03:07:25.2244119Z Task         : Get sources
2020-03-10T03:07:25.2244375Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-10T03:07:25.2244622Z Version      : 1.0.0
2020-03-10T03:07:25.2244794Z Author       : Microsoft
---
2020-03-10T03:07:26.2090905Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-10T03:07:26.2096840Z ##[command]git config gc.auto 0
2020-03-10T03:07:26.2100627Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-10T03:07:26.2103891Z ##[command]git config --get-all http.proxy
2020-03-10T03:07:26.2109563Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69876/merge:refs/remotes/pull/69876/merge
---
2020-03-10T03:14:36.4583395Z Found 0 error codes with no tests
2020-03-10T03:14:36.4583805Z Done!
2020-03-10T03:14:36.4586468Z 
2020-03-10T03:14:36.4586808Z 
2020-03-10T03:14:36.4588260Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-03-10T03:14:36.4589372Z 
2020-03-10T03:14:36.4589653Z 
2020-03-10T03:14:36.4630383Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-10T03:14:36.4630888Z Build completed unsuccessfully in 0:01:21
2020-03-10T03:14:36.4630888Z Build completed unsuccessfully in 0:01:21
2020-03-10T03:14:36.4641618Z == clock drift check ==
2020-03-10T03:14:36.4648836Z   local time: Tue Mar 10 03:14:36 UTC 2020
2020-03-10T03:14:36.5619775Z   network time: Tue, 10 Mar 2020 03:14:36 GMT
2020-03-10T03:14:36.5623414Z == end clock drift check ==
2020-03-10T03:14:37.4935776Z 
2020-03-10T03:14:37.5020658Z ##[error]Bash exited with code '1'.
2020-03-10T03:14:37.5041374Z ##[section]Finishing: Run build
2020-03-10T03:14:37.5086038Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69876/merge to s
2020-03-10T03:14:37.5090855Z Task         : Get sources
2020-03-10T03:14:37.5091181Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-10T03:14:37.5091459Z Version      : 1.0.0
2020-03-10T03:14:37.5091653Z Author       : Microsoft
2020-03-10T03:14:37.5091653Z Author       : Microsoft
2020-03-10T03:14:37.5091977Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-10T03:14:37.5092332Z ==============================================================================
2020-03-10T03:14:37.8211719Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-10T03:14:37.8256844Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69876/merge to s
2020-03-10T03:14:37.8348914Z Cleaning up task key
2020-03-10T03:14:37.8350359Z Start cleaning up orphan processes.
2020-03-10T03:14:37.8533341Z Terminate orphan process: pid (3994) (python)
2020-03-10T03:14:37.8675997Z ##[section]Finishing: Finalize Job
