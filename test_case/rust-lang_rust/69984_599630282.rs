plain
2020-03-16T16:21:20.3782402Z ========================== Starting Command Output ===========================
2020-03-16T16:21:20.3784856Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/33d15ff7-a5d3-41e6-b2f1-13e71616ed45.sh
2020-03-16T16:21:20.3785130Z 
2020-03-16T16:21:20.3791429Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-16T16:21:20.3811195Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69984/merge to s
2020-03-16T16:21:20.3814471Z Task         : Get sources
2020-03-16T16:21:20.3814779Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-16T16:21:20.3815077Z Version      : 1.0.0
2020-03-16T16:21:20.3815295Z Author       : Microsoft
---
2020-03-16T16:21:21.6622575Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-16T16:21:21.6638527Z ##[command]git config gc.auto 0
2020-03-16T16:21:21.6648495Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-16T16:21:22.6585443Z ##[command]git config --get-all http.proxy
2020-03-16T16:21:22.6600588Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69984/merge:refs/remotes/pull/69984/merge
