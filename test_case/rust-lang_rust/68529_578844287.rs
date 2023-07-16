plain
2020-01-27T16:37:45.0621953Z ========================== Starting Command Output ===========================
2020-01-27T16:37:45.0623899Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/55581fff-bbc6-47a4-a0d5-6964c9507ba3.sh
2020-01-27T16:37:45.0623942Z 
2020-01-27T16:37:45.0627286Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-27T16:37:45.0635621Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68529/merge to s
2020-01-27T16:37:45.0637958Z Task         : Get sources
2020-01-27T16:37:45.0637997Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-27T16:37:45.0638034Z Version      : 1.0.0
2020-01-27T16:37:45.0638135Z Author       : Microsoft
---
2020-01-27T16:37:46.1330158Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-27T16:37:46.1414059Z ##[command]git config gc.auto 0
2020-01-27T16:37:46.1493933Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-27T16:37:46.1539520Z ##[command]git config --get-all http.proxy
2020-01-27T16:37:46.1701354Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68529/merge:refs/remotes/pull/68529/merge
