plain
2020-01-29T05:30:14.7822716Z ========================== Starting Command Output ===========================
2020-01-29T05:30:14.7839439Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/adf21447-1729-41f3-9eee-41f736845a06.sh
2020-01-29T05:30:14.8055175Z 
2020-01-29T05:30:14.8144491Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-29T05:30:14.8150370Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68601/merge to s
2020-01-29T05:30:14.8152020Z Task         : Get sources
2020-01-29T05:30:14.8152052Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-29T05:30:14.8152085Z Version      : 1.0.0
2020-01-29T05:30:14.8152155Z Author       : Microsoft
---
2020-01-29T05:30:15.6343019Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-29T05:30:15.6420777Z ##[command]git config gc.auto 0
2020-01-29T05:30:15.6487497Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-29T05:30:15.6540910Z ##[command]git config --get-all http.proxy
2020-01-29T05:30:15.6670805Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68601/merge:refs/remotes/pull/68601/merge
