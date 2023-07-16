plain
2020-01-18T11:04:16.6401579Z ========================== Starting Command Output ===========================
2020-01-18T11:04:16.6731359Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c678ff3a-bdf8-454c-8d3c-2c1286e899a0.sh
2020-01-18T11:04:16.6731582Z 
2020-01-18T11:04:16.6735240Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-18T11:04:16.6741693Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68338/merge to s
2020-01-18T11:04:16.6744083Z Task         : Get sources
2020-01-18T11:04:16.6744116Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T11:04:16.6744162Z Version      : 1.0.0
2020-01-18T11:04:16.6744192Z Author       : Microsoft
---
2020-01-18T11:04:17.7496513Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-18T11:04:17.7537418Z ##[command]git config gc.auto 0
2020-01-18T11:04:17.7545969Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-18T11:04:17.7596738Z ##[command]git config --get-all http.proxy
2020-01-18T11:04:20.1908257Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68338/merge:refs/remotes/pull/68338/merge
