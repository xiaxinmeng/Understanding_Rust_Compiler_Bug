plain
2020-01-28T23:05:09.7519836Z ========================== Starting Command Output ===========================
2020-01-28T23:05:09.7522628Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/694cdaa9-acea-490f-9699-d54260de4423.sh
2020-01-28T23:05:09.7522842Z 
2020-01-28T23:05:09.7526174Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-28T23:05:09.7533757Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68583/merge to s
2020-01-28T23:05:09.7535844Z Task         : Get sources
2020-01-28T23:05:09.7535881Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-28T23:05:09.7535917Z Version      : 1.0.0
2020-01-28T23:05:09.7536005Z Author       : Microsoft
---
2020-01-28T23:05:10.7263313Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-28T23:05:10.7273955Z ##[command]git config gc.auto 0
2020-01-28T23:05:10.7276269Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-28T23:05:10.7278218Z ##[command]git config --get-all http.proxy
2020-01-28T23:05:10.7285601Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68583/merge:refs/remotes/pull/68583/merge
