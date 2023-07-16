plain
2020-02-23T17:33:07.6473617Z ========================== Starting Command Output ===========================
2020-02-23T17:33:07.6478461Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/24e28d17-59de-4c31-b311-c0af33ddbec1.sh
2020-02-23T17:33:07.6478963Z 
2020-02-23T17:33:07.6484038Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-23T17:33:07.6504591Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69402/merge to s
2020-02-23T17:33:07.6508175Z Task         : Get sources
2020-02-23T17:33:07.6508504Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-23T17:33:07.6508820Z Version      : 1.0.0
2020-02-23T17:33:07.6509051Z Author       : Microsoft
---
2020-02-23T17:33:08.7890423Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-23T17:33:08.7901343Z ##[command]git config gc.auto 0
2020-02-23T17:33:08.7908804Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-23T17:33:08.7915970Z ##[command]git config --get-all http.proxy
2020-02-23T17:33:08.7931648Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69402/merge:refs/remotes/pull/69402/merge
