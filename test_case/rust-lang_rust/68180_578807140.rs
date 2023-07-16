plain
2020-01-27T15:18:03.4052000Z ========================== Starting Command Output ===========================
2020-01-27T15:18:03.4054599Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/900a3905-9dfa-4ff9-88bf-554e55e5563c.sh
2020-01-27T15:18:03.4054761Z 
2020-01-27T15:18:03.4058061Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-27T15:18:03.4063731Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68180/merge to s
2020-01-27T15:18:03.4065317Z Task         : Get sources
2020-01-27T15:18:03.4065353Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-27T15:18:03.4065431Z Version      : 1.0.0
2020-01-27T15:18:03.4065466Z Author       : Microsoft
---
2020-01-27T15:18:04.1845464Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-27T15:18:04.1939013Z ##[command]git config gc.auto 0
2020-01-27T15:18:04.1997939Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-27T15:18:04.2054176Z ##[command]git config --get-all http.proxy
2020-01-27T15:18:04.2190961Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68180/merge:refs/remotes/pull/68180/merge
