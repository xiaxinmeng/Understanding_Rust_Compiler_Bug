plain
2020-03-02T20:16:47.3015407Z ========================== Starting Command Output ===========================
2020-03-02T20:16:47.3018334Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5971294d-6919-4080-ac50-a819bc5f88f5.sh
2020-03-02T20:16:47.3018730Z 
2020-03-02T20:16:47.3024025Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-02T20:16:47.3044334Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69609/merge to s
2020-03-02T20:16:47.3047203Z Task         : Get sources
2020-03-02T20:16:47.3047437Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-02T20:16:47.3047663Z Version      : 1.0.0
2020-03-02T20:16:47.3047890Z Author       : Microsoft
---
2020-03-02T20:16:48.5963412Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-02T20:16:48.5970906Z ##[command]git config gc.auto 0
2020-03-02T20:16:48.5995786Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-02T20:16:48.6004214Z ##[command]git config --get-all http.proxy
2020-03-02T20:16:48.6020234Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69609/merge:refs/remotes/pull/69609/merge
