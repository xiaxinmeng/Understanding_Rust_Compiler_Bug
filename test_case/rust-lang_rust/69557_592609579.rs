plain
2020-02-28T16:50:21.0097277Z ========================== Starting Command Output ===========================
2020-02-28T16:50:21.0101557Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7f1e1812-d0fd-4f3b-b7e4-61bfa1fc9ab3.sh
2020-02-28T16:50:21.0101993Z 
2020-02-28T16:50:21.0106142Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-28T16:50:21.0141352Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69557/merge to s
2020-02-28T16:50:21.0145347Z Task         : Get sources
2020-02-28T16:50:21.0145635Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-28T16:50:21.0145912Z Version      : 1.0.0
2020-02-28T16:50:21.0146120Z Author       : Microsoft
---
2020-02-28T16:50:22.0012348Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-28T16:50:22.0019959Z ##[command]git config gc.auto 0
2020-02-28T16:50:22.0026363Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-28T16:50:22.0032675Z ##[command]git config --get-all http.proxy
2020-02-28T16:50:22.0039772Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69557/merge:refs/remotes/pull/69557/merge
