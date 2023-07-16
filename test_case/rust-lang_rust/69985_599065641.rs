plain
2020-03-14T13:46:41.2288453Z ========================== Starting Command Output ===========================
2020-03-14T13:46:41.2305149Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/06eab72d-e15f-479c-a549-42fa7565a004.sh
2020-03-14T13:46:41.4384723Z 
2020-03-14T13:46:41.4451449Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-14T13:46:41.4485845Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-03-14T13:46:41.4497525Z Task         : Get sources
2020-03-14T13:46:41.4498092Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-14T13:46:41.4498640Z Version      : 1.0.0
2020-03-14T13:46:41.4499060Z Author       : Microsoft
---
2020-03-14T13:46:43.6597986Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-14T13:46:43.6758697Z ##[command]git config gc.auto 0
2020-03-14T13:46:43.6803715Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-14T13:46:43.6829189Z ##[command]git config --get-all http.proxy
2020-03-14T13:46:43.6930517Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69985/merge:refs/remotes/pull/69985/merge
