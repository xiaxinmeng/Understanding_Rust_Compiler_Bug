plain
2020-01-30T07:54:52.7372613Z ========================== Starting Command Output ===========================
2020-01-30T07:54:52.7393876Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0645bc57-73d2-4062-a5ee-9b326afe05e2.sh
2020-01-30T07:54:53.0032542Z 
2020-01-30T07:54:53.0112031Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-30T07:54:53.0118935Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68554/merge to s
2020-01-30T07:54:53.0129049Z Task         : Get sources
2020-01-30T07:54:53.0129086Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-30T07:54:53.0129121Z Version      : 1.0.0
2020-01-30T07:54:53.0129208Z Author       : Microsoft
---
2020-01-30T07:54:55.8537811Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-30T07:54:55.8750102Z ##[command]git config gc.auto 0
2020-01-30T07:54:55.8838314Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-30T07:54:55.8903790Z ##[command]git config --get-all http.proxy
2020-01-30T07:54:55.9074466Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68554/merge:refs/remotes/pull/68554/merge
