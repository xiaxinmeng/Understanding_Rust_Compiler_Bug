plain
2020-03-16T13:58:03.5224095Z ========================== Starting Command Output ===========================
2020-03-16T13:58:03.5226659Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/10661d78-a180-456f-8948-fa580ee1a53e.sh
2020-03-16T13:58:03.5227019Z 
2020-03-16T13:58:03.5230562Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-16T13:58:03.5249493Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70042/merge to s
2020-03-16T13:58:03.5252827Z Task         : Get sources
2020-03-16T13:58:03.5253076Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-16T13:58:03.5253317Z Version      : 1.0.0
2020-03-16T13:58:03.5253497Z Author       : Microsoft
---
2020-03-16T13:58:04.5405125Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-16T13:58:04.5410866Z ##[command]git config gc.auto 0
2020-03-16T13:58:04.5414911Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-16T13:58:04.5418634Z ##[command]git config --get-all http.proxy
2020-03-16T13:58:04.5425398Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70042/merge:refs/remotes/pull/70042/merge
---
2020-03-16T14:05:02.8163105Z Done!
2020-03-16T14:05:02.8163614Z some tidy checks failed
2020-03-16T14:05:02.8169752Z 
2020-03-16T14:05:02.8169949Z 
2020-03-16T14:05:02.8175379Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-03-16T14:05:02.8179917Z 
2020-03-16T14:05:02.8180183Z 
2020-03-16T14:05:02.8190538Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-16T14:05:02.8190914Z Build completed unsuccessfully in 0:01:33
2020-03-16T14:05:02.8190914Z Build completed unsuccessfully in 0:01:33
2020-03-16T14:05:02.8242536Z == clock drift check ==
2020-03-16T14:05:02.8252913Z   local time: Mon Mar 16 14:05:02 UTC 2020
2020-03-16T14:05:03.3732118Z   network time: Mon, 16 Mar 2020 14:05:03 GMT
2020-03-16T14:05:03.3732524Z == end clock drift check ==
2020-03-16T14:05:04.1566242Z 
2020-03-16T14:05:04.1655861Z ##[error]Bash exited with code '1'.
2020-03-16T14:05:04.1670339Z ##[section]Finishing: Run build
2020-03-16T14:05:04.1728118Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70042/merge to s
2020-03-16T14:05:04.1733344Z Task         : Get sources
2020-03-16T14:05:04.1733728Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-16T14:05:04.1734102Z Version      : 1.0.0
2020-03-16T14:05:04.1734357Z Author       : Microsoft
2020-03-16T14:05:04.1734357Z Author       : Microsoft
2020-03-16T14:05:04.1734747Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-16T14:05:04.1735222Z ==============================================================================
2020-03-16T14:05:04.5294060Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-16T14:05:04.5349237Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70042/merge to s
2020-03-16T14:05:04.5445851Z Cleaning up task key
2020-03-16T14:05:04.5447206Z Start cleaning up orphan processes.
2020-03-16T14:05:04.5685638Z Terminate orphan process: pid (3688) (python)
2020-03-16T14:05:04.5837900Z ##[section]Finishing: Finalize Job
