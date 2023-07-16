plain
2020-03-06T03:58:50.3666486Z ========================== Starting Command Output ===========================
2020-03-06T03:58:50.3670419Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3e10b8ef-99b4-4fc6-91d5-a315ce6d0335.sh
2020-03-06T03:58:50.3670721Z 
2020-03-06T03:58:50.3675451Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-06T03:58:50.3698172Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69756/merge to s
2020-03-06T03:58:50.3702066Z Task         : Get sources
2020-03-06T03:58:50.3702392Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-06T03:58:50.3702707Z Version      : 1.0.0
2020-03-06T03:58:50.3702939Z Author       : Microsoft
---
2020-03-06T03:58:53.5647002Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-06T03:58:53.5654881Z ##[command]git config gc.auto 0
2020-03-06T03:58:53.5658461Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-06T03:58:53.5661461Z ##[command]git config --get-all http.proxy
2020-03-06T03:58:53.5669725Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69756/merge:refs/remotes/pull/69756/merge
