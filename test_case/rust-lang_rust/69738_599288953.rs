plain
2020-03-16T00:18:00.1639722Z ========================== Starting Command Output ===========================
2020-03-16T00:18:00.1643653Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/38ac25cc-51ad-44b6-9e91-6d8b376407a2.sh
2020-03-16T00:18:00.1644062Z 
2020-03-16T00:18:00.1648503Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-16T00:18:00.1668187Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69738/merge to s
2020-03-16T00:18:00.1671587Z Task         : Get sources
2020-03-16T00:18:00.1671908Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-16T00:18:00.1672228Z Version      : 1.0.0
2020-03-16T00:18:00.1672439Z Author       : Microsoft
---
2020-03-16T00:18:01.1518720Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-16T00:18:01.1523799Z ##[command]git config gc.auto 0
2020-03-16T00:18:01.1527145Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-16T00:18:01.1530204Z ##[command]git config --get-all http.proxy
2020-03-16T00:18:01.1536966Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69738/merge:refs/remotes/pull/69738/merge
