plain
2020-01-27T21:10:45.8433022Z ========================== Starting Command Output ===========================
2020-01-27T21:10:45.8434578Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/099d8a7d-fc3e-4626-8430-442787b75a44.sh
2020-01-27T21:10:45.8434614Z 
2020-01-27T21:10:45.8437359Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-27T21:10:45.8443100Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67429/merge to s
2020-01-27T21:10:45.8444663Z Task         : Get sources
2020-01-27T21:10:45.8444698Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-27T21:10:45.8444771Z Version      : 1.0.0
2020-01-27T21:10:45.8444806Z Author       : Microsoft
---
2020-01-27T21:10:46.6300093Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-27T21:10:46.6378124Z ##[command]git config gc.auto 0
2020-01-27T21:10:46.6451793Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-27T21:10:46.6513209Z ##[command]git config --get-all http.proxy
2020-01-27T21:10:46.6656264Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67429/merge:refs/remotes/pull/67429/merge
---
2020-01-27T21:14:04.2285229Z   local time: Mon Jan 27 21:14:04 UTC 2020
2020-01-27T21:14:04.5156176Z   network time: Mon, 27 Jan 2020 21:14:04 GMT
2020-01-27T21:14:04.5160212Z == end clock drift check ==
2020-01-27T21:14:12.5942822Z 
2020-01-27T21:14:12.6032183Z ##[error]Bash exited with code '1'.
2020-01-27T21:14:12.6043428Z ##[section]Finishing: Run build
2020-01-27T21:14:12.6057817Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67429/merge to s
2020-01-27T21:14:12.6059577Z Task         : Get sources
2020-01-27T21:14:12.6059641Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-27T21:14:12.6059691Z Version      : 1.0.0
2020-01-27T21:14:12.6059735Z Author       : Microsoft
2020-01-27T21:14:12.6059735Z Author       : Microsoft
2020-01-27T21:14:12.6059800Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-27T21:14:12.6059853Z ==============================================================================
2020-01-27T21:14:12.9800590Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-27T21:14:12.9875029Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67429/merge to s
2020-01-27T21:14:12.9980280Z Cleaning up task key
2020-01-27T21:14:12.9981149Z Start cleaning up orphan processes.
2020-01-27T21:14:13.0081527Z Terminate orphan process: pid (3797) (python)
2020-01-27T21:14:13.0261892Z ##[section]Finishing: Finalize Job
