plain
2020-02-23T18:44:19.5735486Z ========================== Starting Command Output ===========================
2020-02-23T18:44:19.5740472Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ddf4fc90-4688-4a04-8819-89d904d1b982.sh
2020-02-23T18:44:19.5740989Z 
2020-02-23T18:44:19.5745203Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-23T18:44:19.5766209Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69404/merge to s
2020-02-23T18:44:19.5769651Z Task         : Get sources
2020-02-23T18:44:19.5769999Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-23T18:44:19.5770315Z Version      : 1.0.0
2020-02-23T18:44:19.5770531Z Author       : Microsoft
---
2020-02-23T18:44:20.5622647Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-23T18:44:20.5628397Z ##[command]git config gc.auto 0
2020-02-23T18:44:20.5659734Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-23T18:44:20.5663484Z ##[command]git config --get-all http.proxy
2020-02-23T18:44:20.5670659Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69404/merge:refs/remotes/pull/69404/merge
---
2020-02-23T18:47:29.8754254Z   local time: Sun Feb 23 18:47:29 UTC 2020
2020-02-23T18:47:30.0326971Z   network time: Sun, 23 Feb 2020 18:47:30 GMT
2020-02-23T18:47:30.0338352Z == end clock drift check ==
2020-02-23T18:47:30.5533927Z 
2020-02-23T18:47:30.5609132Z ##[error]Bash exited with code '1'.
2020-02-23T18:47:30.5626283Z ##[section]Finishing: Run build
2020-02-23T18:47:30.5676380Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69404/merge to s
2020-02-23T18:47:30.5681684Z Task         : Get sources
2020-02-23T18:47:30.5682074Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-23T18:47:30.5682450Z Version      : 1.0.0
2020-02-23T18:47:30.5682700Z Author       : Microsoft
2020-02-23T18:47:30.5682700Z Author       : Microsoft
2020-02-23T18:47:30.5683298Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-23T18:47:30.5683778Z ==============================================================================
2020-02-23T18:47:30.9212658Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-23T18:47:30.9256404Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69404/merge to s
2020-02-23T18:47:30.9338449Z Cleaning up task key
2020-02-23T18:47:30.9339774Z Start cleaning up orphan processes.
2020-02-23T18:47:30.9500257Z Terminate orphan process: pid (4525) (python)
2020-02-23T18:47:30.9622935Z ##[section]Finishing: Finalize Job
