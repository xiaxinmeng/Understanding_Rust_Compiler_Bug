plain
2020-03-13T19:50:27.6802344Z ========================== Starting Command Output ===========================
2020-03-13T19:50:27.6806911Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/dc7a94af-f746-42e2-bf15-d604616cc1fc.sh
2020-03-13T19:50:27.6807366Z 
2020-03-13T19:50:27.6811381Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-13T19:50:27.6828153Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69955/merge to s
2020-03-13T19:50:27.6831464Z Task         : Get sources
2020-03-13T19:50:27.6831861Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T19:50:27.6832083Z Version      : 1.0.0
2020-03-13T19:50:27.6832276Z Author       : Microsoft
---
2020-03-13T19:50:28.9418467Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-13T19:50:28.9490026Z ##[command]git config gc.auto 0
2020-03-13T19:50:28.9497254Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-13T19:50:28.9502793Z ##[command]git config --get-all http.proxy
2020-03-13T19:50:28.9510737Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69955/merge:refs/remotes/pull/69955/merge
