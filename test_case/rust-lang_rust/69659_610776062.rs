plain
2020-04-08T06:26:54.7491263Z ========================== Starting Command Output ===========================
2020-04-08T06:26:54.7496407Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e6dc611b-b412-44a4-985e-eec75b67c37e.sh
2020-04-08T06:26:54.7496839Z 
2020-04-08T06:26:54.7501804Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-08T06:26:54.7519915Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69659/merge to s
2020-04-08T06:26:54.7523018Z Task         : Get sources
2020-04-08T06:26:54.7523778Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-08T06:26:54.7524044Z Version      : 1.0.0
2020-04-08T06:26:54.7524223Z Author       : Microsoft
---
2020-04-08T06:26:55.7426489Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-08T06:26:55.7431588Z ##[command]git config gc.auto 0
2020-04-08T06:26:55.7436654Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-08T06:26:55.7440097Z ##[command]git config --get-all http.proxy
2020-04-08T06:26:55.7446723Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69659/merge:refs/remotes/pull/69659/merge
---
2020-04-08T06:28:07.9146411Z E: Unable to fetch some archives, maybe run apt-get update or try with --fix-missing?
2020-04-08T06:28:07.9188388Z 
2020-04-08T06:28:07.9259631Z ##[error]Bash exited with code '100'.
2020-04-08T06:28:07.9271259Z ##[section]Finishing: Install awscli
2020-04-08T06:28:07.9333529Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69659/merge to s
2020-04-08T06:28:07.9338189Z Task         : Get sources
2020-04-08T06:28:07.9338478Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-08T06:28:07.9339011Z Version      : 1.0.0
2020-04-08T06:28:07.9339199Z Author       : Microsoft
2020-04-08T06:28:07.9339199Z Author       : Microsoft
2020-04-08T06:28:07.9339492Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-08T06:28:07.9339849Z ==============================================================================
2020-04-08T06:28:08.2598811Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-08T06:28:08.2625899Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69659/merge to s
2020-04-08T06:28:08.2716878Z Cleaning up task key
2020-04-08T06:28:08.2718022Z Start cleaning up orphan processes.
2020-04-08T06:28:08.3044994Z Terminate orphan process: pid (3969) (python)
2020-04-08T06:28:08.3074308Z ##[section]Finishing: Finalize Job
