plain
2020-01-22T11:27:09.7167033Z ========================== Starting Command Output ===========================
2020-01-22T11:27:09.7168749Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4a42801c-9c8a-428b-baec-28893d352f05.sh
2020-01-22T11:27:09.7168846Z 
2020-01-22T11:27:09.7172061Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-22T11:27:09.7180161Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68414/merge to s
2020-01-22T11:27:09.7182177Z Task         : Get sources
2020-01-22T11:27:09.7182265Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-22T11:27:09.7182301Z Version      : 1.0.0
2020-01-22T11:27:09.7182336Z Author       : Microsoft
---
2020-01-22T11:27:10.7199941Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-22T11:27:10.7219682Z ##[command]git config gc.auto 0
2020-01-22T11:27:10.7221822Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-22T11:27:10.7224284Z ##[command]git config --get-all http.proxy
2020-01-22T11:27:10.7230682Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68414/merge:refs/remotes/pull/68414/merge
