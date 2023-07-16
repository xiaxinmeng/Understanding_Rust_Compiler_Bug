plain
2020-02-19T10:38:21.9654048Z ========================== Starting Command Output ===========================
2020-02-19T10:38:21.9656486Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/531a2c58-b909-4fa8-81d0-065ded9be7dd.sh
2020-02-19T10:38:21.9656530Z 
2020-02-19T10:38:21.9659400Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-19T10:38:21.9665740Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69285/merge to s
2020-02-19T10:38:21.9667443Z Task         : Get sources
2020-02-19T10:38:21.9667482Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-19T10:38:21.9667566Z Version      : 1.0.0
2020-02-19T10:38:21.9667604Z Author       : Microsoft
---
2020-02-19T10:38:22.9448203Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-19T10:38:22.9541074Z ##[command]git config gc.auto 0
2020-02-19T10:38:22.9616640Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-19T10:38:22.9647625Z ##[command]git config --get-all http.proxy
2020-02-19T10:38:22.9821858Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69285/merge:refs/remotes/pull/69285/merge
