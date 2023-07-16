plain
2020-01-22T20:26:37.6389416Z ========================== Starting Command Output ===========================
2020-01-22T20:26:37.6392802Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/22859c96-9fa8-4726-adef-27486109f5e1.sh
2020-01-22T20:26:37.6393060Z 
2020-01-22T20:26:37.6399542Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-22T20:26:37.6406485Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68463/merge to s
2020-01-22T20:26:37.6408444Z Task         : Get sources
2020-01-22T20:26:37.6408532Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-22T20:26:37.6408570Z Version      : 1.0.0
2020-01-22T20:26:37.6408604Z Author       : Microsoft
---
2020-01-22T20:26:38.6474082Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-22T20:26:38.6487275Z ##[command]git config gc.auto 0
2020-01-22T20:26:38.6489866Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-22T20:26:38.6492203Z ##[command]git config --get-all http.proxy
2020-01-22T20:26:38.6499484Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68463/merge:refs/remotes/pull/68463/merge
