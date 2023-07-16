plain
2020-01-27T21:55:41.9338132Z ========================== Starting Command Output ===========================
2020-01-27T21:55:41.9351703Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3af5cd63-0f31-41a4-9ef3-933cf07b9e8c.sh
2020-01-27T21:55:41.9558232Z 
2020-01-27T21:55:41.9607125Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-27T21:55:41.9612394Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67429/merge to s
2020-01-27T21:55:41.9613852Z Task         : Get sources
2020-01-27T21:55:41.9613886Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-27T21:55:41.9613920Z Version      : 1.0.0
2020-01-27T21:55:41.9613989Z Author       : Microsoft
---
2020-01-27T21:55:43.3317221Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-27T21:55:43.3392056Z ##[command]git config gc.auto 0
2020-01-27T21:55:43.3456063Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-27T21:55:43.3515959Z ##[command]git config --get-all http.proxy
2020-01-27T21:55:43.3652589Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67429/merge:refs/remotes/pull/67429/merge
