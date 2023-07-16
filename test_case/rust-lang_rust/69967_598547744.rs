plain
2020-03-13T04:36:41.6477066Z ========================== Starting Command Output ===========================
2020-03-13T04:36:41.6480006Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b84985df-c226-4e76-9f00-69a9645593cb.sh
2020-03-13T04:36:41.6480331Z 
2020-03-13T04:36:41.6484135Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-13T04:36:41.6506795Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69967/merge to s
2020-03-13T04:36:41.6510582Z Task         : Get sources
2020-03-13T04:36:41.6510955Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-13T04:36:41.6511318Z Version      : 1.0.0
2020-03-13T04:36:41.6511583Z Author       : Microsoft
---
2020-03-13T04:36:42.6508218Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-13T04:36:42.6516730Z ##[command]git config gc.auto 0
2020-03-13T04:36:42.6521533Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-13T04:36:42.6526100Z ##[command]git config --get-all http.proxy
2020-03-13T04:36:42.6533620Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69967/merge:refs/remotes/pull/69967/merge
