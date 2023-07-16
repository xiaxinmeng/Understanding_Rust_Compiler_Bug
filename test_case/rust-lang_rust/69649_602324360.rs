plain
2020-03-23T00:56:42.8008459Z ========================== Starting Command Output ===========================
2020-03-23T00:56:42.8013363Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d839ad68-2701-4648-a3f3-029a84492b9c.sh
2020-03-23T00:56:42.8013880Z 
2020-03-23T00:56:42.8018095Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-23T00:56:42.8038929Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69649/merge to s
2020-03-23T00:56:42.8042479Z Task         : Get sources
2020-03-23T00:56:42.8042830Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T00:56:42.8043148Z Version      : 1.0.0
2020-03-23T00:56:42.8043364Z Author       : Microsoft
---
2020-03-23T00:56:43.9473320Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-23T00:56:43.9481561Z ##[command]git config gc.auto 0
2020-03-23T00:56:43.9489644Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-23T00:56:43.9495376Z ##[command]git config --get-all http.proxy
2020-03-23T00:56:43.9507836Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69649/merge:refs/remotes/pull/69649/merge
---
2020-03-23T01:06:10.9253271Z   local time: Mon Mar 23 01:06:10 UTC 2020
2020-03-23T01:06:11.0128042Z   network time: Mon, 23 Mar 2020 01:06:11 GMT
2020-03-23T01:06:11.0132409Z == end clock drift check ==
2020-03-23T01:06:11.5192914Z 
2020-03-23T01:06:11.5277263Z ##[error]Bash exited with code '1'.
2020-03-23T01:06:11.5293268Z ##[section]Finishing: Run build
2020-03-23T01:06:11.5454659Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69649/merge to s
2020-03-23T01:06:11.5460130Z Task         : Get sources
2020-03-23T01:06:11.5460517Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T01:06:11.5460892Z Version      : 1.0.0
2020-03-23T01:06:11.5461140Z Author       : Microsoft
2020-03-23T01:06:11.5461140Z Author       : Microsoft
2020-03-23T01:06:11.5461536Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-23T01:06:11.5462014Z ==============================================================================
2020-03-23T01:06:11.9019676Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-23T01:06:11.9068105Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69649/merge to s
2020-03-23T01:06:11.9163002Z Cleaning up task key
2020-03-23T01:06:11.9164422Z Start cleaning up orphan processes.
2020-03-23T01:06:11.9385163Z Terminate orphan process: pid (3715) (python)
2020-03-23T01:06:12.0166204Z ##[section]Finishing: Finalize Job
