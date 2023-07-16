plain
2020-01-31T13:44:57.0266080Z ========================== Starting Command Output ===========================
2020-01-31T13:44:57.0270844Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b0500972-e0b4-45ff-bad9-8acf365b3e19.sh
2020-01-31T13:44:57.0271085Z 
2020-01-31T13:44:57.0275137Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-31T13:44:57.0281389Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68700/merge to s
2020-01-31T13:44:57.0283044Z Task         : Get sources
2020-01-31T13:44:57.0283078Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-31T13:44:57.0283108Z Version      : 1.0.0
2020-01-31T13:44:57.0283187Z Author       : Microsoft
---
2020-01-31T13:44:58.0304877Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-31T13:44:58.0319158Z ##[command]git config gc.auto 0
2020-01-31T13:44:58.0321546Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-31T13:44:58.0323464Z ##[command]git config --get-all http.proxy
2020-01-31T13:44:58.0459345Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68700/merge:refs/remotes/pull/68700/merge
