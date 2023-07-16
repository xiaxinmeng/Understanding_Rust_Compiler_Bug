plain
2020-04-14T20:52:00.7923011Z ========================== Starting Command Output ===========================
2020-04-14T20:52:00.7925766Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d4b3b2c3-8984-4063-a677-dbbc23fe8d14.sh
2020-04-14T20:52:00.7926152Z 
2020-04-14T20:52:00.7929632Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-14T20:52:00.7947563Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71149/merge to s
2020-04-14T20:52:00.7950142Z Task         : Get sources
2020-04-14T20:52:00.7950365Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T20:52:00.7950585Z Version      : 1.0.0
2020-04-14T20:52:00.7950732Z Author       : Microsoft
---
2020-04-14T20:52:01.7940029Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-14T20:52:01.7948203Z ##[command]git config gc.auto 0
2020-04-14T20:52:01.7953484Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-14T20:52:01.7959868Z ##[command]git config --get-all http.proxy
2020-04-14T20:52:01.7967909Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71149/merge:refs/remotes/pull/71149/merge
---
2020-04-14T20:53:21.4970619Z E: Unable to fetch some archives, maybe run apt-get update or try with --fix-missing?
2020-04-14T20:53:21.5010797Z 
2020-04-14T20:53:21.5072455Z ##[error]Bash exited with code '100'.
2020-04-14T20:53:21.5086480Z ##[section]Finishing: Install awscli
2020-04-14T20:53:21.5144784Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71149/merge to s
2020-04-14T20:53:21.5149309Z Task         : Get sources
2020-04-14T20:53:21.5149621Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T20:53:21.5150055Z Version      : 1.0.0
2020-04-14T20:53:21.5150262Z Author       : Microsoft
2020-04-14T20:53:21.5150262Z Author       : Microsoft
2020-04-14T20:53:21.5150595Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-14T20:53:21.5150977Z ==============================================================================
2020-04-14T20:53:21.8280348Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-14T20:53:21.8332251Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71149/merge to s
2020-04-14T20:53:21.8418746Z Cleaning up task key
2020-04-14T20:53:21.8419972Z Start cleaning up orphan processes.
2020-04-14T20:53:21.8599536Z Terminate orphan process: pid (3511) (python)
2020-04-14T20:53:22.7939152Z ##[section]Finishing: Finalize Job
