plain
2020-02-08T17:14:48.1320188Z ========================== Starting Command Output ===========================
2020-02-08T17:14:48.1341636Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0a522f6d-2d91-406c-bc61-1ac7ef346503.sh
2020-02-08T17:14:48.1551794Z 
2020-02-08T17:14:48.1613500Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-08T17:14:48.1619786Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68959/merge to s
2020-02-08T17:14:48.1621451Z Task         : Get sources
2020-02-08T17:14:48.1621480Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-08T17:14:48.1621510Z Version      : 1.0.0
2020-02-08T17:14:48.1621554Z Author       : Microsoft
---
2020-02-08T17:14:49.0057694Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-08T17:14:49.0134702Z ##[command]git config gc.auto 0
2020-02-08T17:14:49.0211378Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-08T17:14:49.0427355Z ##[command]git config --get-all http.proxy
2020-02-08T17:14:49.0609620Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68959/merge:refs/remotes/pull/68959/merge
