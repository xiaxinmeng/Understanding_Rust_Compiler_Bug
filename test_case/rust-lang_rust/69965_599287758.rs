plain
2020-03-16T00:13:20.4817356Z ========================== Starting Command Output ===========================
2020-03-16T00:13:20.4820179Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fdd17353-f487-47b1-be87-2f665dd8b1c5.sh
2020-03-16T00:13:20.4820464Z 
2020-03-16T00:13:20.4824470Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-16T00:13:20.4843661Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69965/merge to s
2020-03-16T00:13:20.4847780Z Task         : Get sources
2020-03-16T00:13:20.4848090Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-16T00:13:20.4848387Z Version      : 1.0.0
2020-03-16T00:13:20.4848587Z Author       : Microsoft
---
2020-03-16T00:13:21.4779168Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-16T00:13:21.4785220Z ##[command]git config gc.auto 0
2020-03-16T00:13:21.4790819Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-16T00:13:21.4795591Z ##[command]git config --get-all http.proxy
2020-03-16T00:13:21.4803462Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69965/merge:refs/remotes/pull/69965/merge
