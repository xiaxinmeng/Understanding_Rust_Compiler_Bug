plain
2020-01-27T16:13:38.6731696Z ========================== Starting Command Output ===========================
2020-01-27T16:13:38.6733136Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/07f670d6-5a33-4e83-b35d-cef237c46c06.sh
2020-01-27T16:13:38.6733167Z 
2020-01-27T16:13:38.6735482Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-27T16:13:38.6741072Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67086/merge to s
2020-01-27T16:13:38.6742626Z Task         : Get sources
2020-01-27T16:13:38.6742661Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-27T16:13:38.6742695Z Version      : 1.0.0
2020-01-27T16:13:38.6742778Z Author       : Microsoft
---
2020-01-27T16:13:39.6930443Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-27T16:13:39.6941759Z ##[command]git config gc.auto 0
2020-01-27T16:13:39.6944128Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-27T16:13:39.6945832Z ##[command]git config --get-all http.proxy
2020-01-27T16:13:39.6951950Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67086/merge:refs/remotes/pull/67086/merge
