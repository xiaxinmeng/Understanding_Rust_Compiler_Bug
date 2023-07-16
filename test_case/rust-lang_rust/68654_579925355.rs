plain
2020-01-29T19:19:46.9856407Z ========================== Starting Command Output ===========================
2020-01-29T19:19:46.9861319Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0d355db8-09c6-47a6-b47b-5fb62bb4d568.sh
2020-01-29T19:19:46.9861366Z 
2020-01-29T19:19:46.9865200Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-29T19:19:46.9872304Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68654/merge to s
2020-01-29T19:19:46.9874186Z Task         : Get sources
2020-01-29T19:19:46.9874225Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-29T19:19:46.9874311Z Version      : 1.0.0
2020-01-29T19:19:46.9874346Z Author       : Microsoft
---
2020-01-29T19:19:48.0319798Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-29T19:19:48.0334509Z ##[command]git config gc.auto 0
2020-01-29T19:19:48.0339505Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-29T19:19:48.0343681Z ##[command]git config --get-all http.proxy
2020-01-29T19:19:48.0361329Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68654/merge:refs/remotes/pull/68654/merge
