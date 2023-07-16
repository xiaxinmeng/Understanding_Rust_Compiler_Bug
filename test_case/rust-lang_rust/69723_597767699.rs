plain
2020-03-11T17:16:58.8329690Z ========================== Starting Command Output ===========================
2020-03-11T17:16:58.8332335Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3788a8f2-78a9-4dca-abe5-7b529cc5b7c2.sh
2020-03-11T17:16:58.8332595Z 
2020-03-11T17:16:58.8337320Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-11T17:16:58.8357656Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69723/merge to s
2020-03-11T17:16:58.8361441Z Task         : Get sources
2020-03-11T17:16:58.8361732Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-11T17:16:58.8362008Z Version      : 1.0.0
2020-03-11T17:16:58.8362196Z Author       : Microsoft
---
2020-03-11T17:16:59.8485393Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-11T17:16:59.8491022Z ##[command]git config gc.auto 0
2020-03-11T17:16:59.8494692Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-11T17:16:59.8498714Z ##[command]git config --get-all http.proxy
2020-03-11T17:16:59.8505194Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69723/merge:refs/remotes/pull/69723/merge
