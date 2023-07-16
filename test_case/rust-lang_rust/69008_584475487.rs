plain
2020-02-11T04:36:30.4245512Z ========================== Starting Command Output ===========================
2020-02-11T04:36:30.4249548Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/89cadf7c-3664-4f55-ac5b-1ac7273cb69f.sh
2020-02-11T04:36:30.4249725Z 
2020-02-11T04:36:30.4256584Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-11T04:36:30.4262849Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69008/merge to s
2020-02-11T04:36:30.4265258Z Task         : Get sources
2020-02-11T04:36:30.4265293Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-11T04:36:30.4265325Z Version      : 1.0.0
2020-02-11T04:36:30.4265358Z Author       : Microsoft
---
2020-02-11T04:36:31.5210578Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-11T04:36:31.5293843Z ##[command]git config gc.auto 0
2020-02-11T04:36:31.5371821Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-11T04:36:31.5431761Z ##[command]git config --get-all http.proxy
2020-02-11T04:36:31.5580578Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69008/merge:refs/remotes/pull/69008/merge
---
2020-02-11T04:39:49.8195905Z E: Unable to fetch some archives, maybe run apt-get update or try with --fix-missing?
2020-02-11T04:39:49.8243165Z 
2020-02-11T04:39:49.8349447Z ##[error]Bash exited with code '100'.
2020-02-11T04:39:49.8362514Z ##[section]Finishing: Install awscli
2020-02-11T04:39:49.8383976Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69008/merge to s
2020-02-11T04:39:49.8386158Z Task         : Get sources
2020-02-11T04:39:49.8386205Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-11T04:39:49.8386252Z Version      : 1.0.0
2020-02-11T04:39:49.8386312Z Author       : Microsoft
2020-02-11T04:39:49.8386312Z Author       : Microsoft
2020-02-11T04:39:49.8386361Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-11T04:39:49.8386411Z ==============================================================================
2020-02-11T04:39:50.2610920Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-11T04:39:50.2652804Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69008/merge to s
2020-02-11T04:39:50.2772815Z Cleaning up task key
2020-02-11T04:39:50.2773585Z Start cleaning up orphan processes.
2020-02-11T04:39:50.2883175Z Terminate orphan process: pid (4110) (python)
2020-02-11T04:39:50.3091810Z ##[section]Finishing: Finalize Job
