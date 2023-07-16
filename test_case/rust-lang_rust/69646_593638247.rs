plain
2020-03-02T21:25:07.6849907Z ========================== Starting Command Output ===========================
2020-03-02T21:25:07.6854355Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/340fc354-f8f1-4fca-aa73-291175ae90c4.sh
2020-03-02T21:25:07.6854773Z 
2020-03-02T21:25:07.6858782Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-02T21:25:07.6877158Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69646/merge to s
2020-03-02T21:25:07.6880005Z Task         : Get sources
2020-03-02T21:25:07.6880253Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-02T21:25:07.6880506Z Version      : 1.0.0
2020-03-02T21:25:07.6880670Z Author       : Microsoft
---
2020-03-02T21:25:08.6805574Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-02T21:25:08.6810914Z ##[command]git config gc.auto 0
2020-03-02T21:25:08.6814675Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-02T21:25:08.6818046Z ##[command]git config --get-all http.proxy
2020-03-02T21:25:08.6824306Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69646/merge:refs/remotes/pull/69646/merge
