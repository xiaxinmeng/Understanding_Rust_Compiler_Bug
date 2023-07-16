plain
2020-01-26T22:29:26.4854243Z ========================== Starting Command Output ===========================
2020-01-26T22:29:26.4872304Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b901e653-9795-4fc0-abab-36e0a6d692d0.sh
2020-01-26T22:29:26.5076425Z 
2020-01-26T22:29:26.5136184Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-26T22:29:26.5143220Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68558/merge to s
2020-01-26T22:29:26.5144850Z Task         : Get sources
2020-01-26T22:29:26.5144890Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-26T22:29:26.5144926Z Version      : 1.0.0
2020-01-26T22:29:26.5145013Z Author       : Microsoft
---
2020-01-26T22:29:27.3720341Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-26T22:29:27.3802697Z ##[command]git config gc.auto 0
2020-01-26T22:29:27.3887718Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-26T22:29:27.3942469Z ##[command]git config --get-all http.proxy
2020-01-26T22:29:27.4108997Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68558/merge:refs/remotes/pull/68558/merge
---
2020-01-26T22:34:04.7423350Z   local time: Sun Jan 26 22:34:04 UTC 2020
2020-01-26T22:34:05.0264118Z   network time: Sun, 26 Jan 2020 22:34:05 GMT
2020-01-26T22:34:05.0267530Z == end clock drift check ==
2020-01-26T22:34:06.0457033Z 
2020-01-26T22:34:06.0554169Z ##[error]Bash exited with code '1'.
2020-01-26T22:34:06.0566926Z ##[section]Finishing: Run build
2020-01-26T22:34:06.0582628Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68558/merge to s
2020-01-26T22:34:06.0584979Z Task         : Get sources
2020-01-26T22:34:06.0585049Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-26T22:34:06.0585101Z Version      : 1.0.0
2020-01-26T22:34:06.0585149Z Author       : Microsoft
2020-01-26T22:34:06.0585149Z Author       : Microsoft
2020-01-26T22:34:06.0585217Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-26T22:34:06.0585275Z ==============================================================================
2020-01-26T22:34:06.4831226Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-26T22:34:06.4874023Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68558/merge to s
2020-01-26T22:34:06.4989019Z Cleaning up task key
2020-01-26T22:34:06.4990028Z Start cleaning up orphan processes.
2020-01-26T22:34:06.5115831Z Terminate orphan process: pid (3367) (python)
2020-01-26T22:34:06.5342428Z ##[section]Finishing: Finalize Job
