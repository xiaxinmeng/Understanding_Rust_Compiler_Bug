plain
2020-01-19T12:10:44.8229490Z ========================== Starting Command Output ===========================
2020-01-19T12:10:44.8269317Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d96239a8-6e89-49d2-9276-aae83c16a230.sh
2020-01-19T12:10:44.8269535Z 
2020-01-19T12:10:44.8273180Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-19T12:10:44.8279510Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67195/merge to s
2020-01-19T12:10:44.8281661Z Task         : Get sources
2020-01-19T12:10:44.8281694Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-19T12:10:44.8281773Z Version      : 1.0.0
2020-01-19T12:10:44.8281806Z Author       : Microsoft
---
2020-01-19T12:10:45.8802517Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-19T12:10:45.8822915Z ##[command]git config gc.auto 0
2020-01-19T12:10:45.8827461Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-19T12:10:45.8829833Z ##[command]git config --get-all http.proxy
2020-01-19T12:10:45.8841907Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67195/merge:refs/remotes/pull/67195/merge
