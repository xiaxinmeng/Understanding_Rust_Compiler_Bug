plain
2020-01-16T23:20:11.6016917Z ========================== Starting Command Output ===========================
2020-01-16T23:20:11.6309659Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/bc8f35a3-36c9-41e1-82f2-8e4118fe911a.sh
2020-01-16T23:20:11.6323392Z 
2020-01-16T23:20:11.6369457Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-16T23:20:11.6375773Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68294/merge to s
2020-01-16T23:20:11.6377581Z Task         : Get sources
2020-01-16T23:20:11.6377648Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-16T23:20:11.6377862Z Version      : 1.0.0
2020-01-16T23:20:11.6377894Z Author       : Microsoft
---
2020-01-16T23:20:12.7166034Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-16T23:20:12.7177309Z ##[command]git config gc.auto 0
2020-01-16T23:20:12.7179934Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-16T23:20:12.7181756Z ##[command]git config --get-all http.proxy
2020-01-16T23:20:12.7188271Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68294/merge:refs/remotes/pull/68294/merge
