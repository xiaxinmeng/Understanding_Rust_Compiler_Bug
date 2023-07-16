plain
2020-03-13T21:01:38.3746732Z ========================== Starting Command Output ===========================
2020-03-13T21:01:38.3753100Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c79d09e7-417b-4f41-9569-0ed05bedc684.sh
2020-03-13T21:01:38.3753690Z 
2020-03-13T21:01:38.3758834Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-13T21:01:38.3786796Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-03-13T21:01:38.3791538Z Task         : Get sources
2020-03-13T21:01:38.3791867Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T21:01:38.3792967Z Version      : 1.0.0
2020-03-13T21:01:38.3793196Z Author       : Microsoft
---
2020-03-13T21:01:41.1342452Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-13T21:01:41.1350242Z ##[command]git config gc.auto 0
2020-03-13T21:01:41.1354838Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-13T21:01:41.1361235Z ##[command]git config --get-all http.proxy
2020-03-13T21:01:41.1371789Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69985/merge:refs/remotes/pull/69985/merge
