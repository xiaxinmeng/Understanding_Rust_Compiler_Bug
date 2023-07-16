plain
2020-01-21T18:05:13.5527553Z ========================== Starting Command Output ===========================
2020-01-21T18:05:13.5531691Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c22da964-3b9a-4ac0-ba81-1adfd7784d5e.sh
2020-01-21T18:05:13.5531798Z 
2020-01-21T18:05:13.5536050Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-21T18:05:13.5542651Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68046/merge to s
2020-01-21T18:05:13.5544420Z Task         : Get sources
2020-01-21T18:05:13.5544501Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-21T18:05:13.5544535Z Version      : 1.0.0
2020-01-21T18:05:13.5544566Z Author       : Microsoft
---
2020-01-21T18:05:14.5844515Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-21T18:05:14.5926728Z ##[command]git config gc.auto 0
2020-01-21T18:05:14.5933752Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-21T18:05:14.5938643Z ##[command]git config --get-all http.proxy
2020-01-21T18:05:14.5946458Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68046/merge:refs/remotes/pull/68046/merge
