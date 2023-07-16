plain
2020-02-22T12:08:18.9681515Z ========================== Starting Command Output ===========================
2020-02-22T12:08:18.9684742Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d9ca5165-2eb3-4adf-9404-19530a42bb23.sh
2020-02-22T12:08:18.9685047Z 
2020-02-22T12:08:18.9689636Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-22T12:08:18.9712700Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-22T12:08:18.9717019Z Task         : Get sources
2020-02-22T12:08:18.9717354Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T12:08:18.9717673Z Version      : 1.0.0
2020-02-22T12:08:18.9717922Z Author       : Microsoft
---
2020-02-22T12:08:19.9537172Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-22T12:08:19.9543142Z ##[command]git config gc.auto 0
2020-02-22T12:08:19.9547301Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-22T12:08:19.9552138Z ##[command]git config --get-all http.proxy
2020-02-22T12:08:19.9559439Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69274/merge:refs/remotes/pull/69274/merge
