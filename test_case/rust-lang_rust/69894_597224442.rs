plain
2020-03-10T17:48:17.7905392Z ========================== Starting Command Output ===========================
2020-03-10T17:48:17.7911936Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9bc4f1b4-9d75-49c1-8ef3-00b6f8f6cc58.sh
2020-03-10T17:48:17.7912538Z 
2020-03-10T17:48:17.7918801Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-10T17:48:17.7940747Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69894/merge to s
2020-03-10T17:48:17.7944742Z Task         : Get sources
2020-03-10T17:48:17.7945068Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-10T17:48:17.7945410Z Version      : 1.0.0
2020-03-10T17:48:17.7945625Z Author       : Microsoft
---
2020-03-10T17:48:18.7786216Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-10T17:48:18.7792099Z ##[command]git config gc.auto 0
2020-03-10T17:48:18.7795786Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-10T17:48:18.7799142Z ##[command]git config --get-all http.proxy
2020-03-10T17:48:18.7805481Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69894/merge:refs/remotes/pull/69894/merge
