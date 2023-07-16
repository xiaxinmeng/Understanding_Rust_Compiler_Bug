plain
2020-02-16T15:26:23.1491593Z ========================== Starting Command Output ===========================
2020-02-16T15:26:23.1511826Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e054d444-9bfe-436c-83e7-760b4accd279.sh
2020-02-16T15:26:23.1697193Z 
2020-02-16T15:26:23.1774837Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-16T15:26:23.1779127Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69209/merge to s
2020-02-16T15:26:23.1780563Z Task         : Get sources
2020-02-16T15:26:23.1780588Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T15:26:23.1780649Z Version      : 1.0.0
2020-02-16T15:26:23.1780672Z Author       : Microsoft
---
2020-02-16T15:26:24.0282483Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-16T15:26:24.0372730Z ##[command]git config gc.auto 0
2020-02-16T15:26:24.0445721Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-16T15:26:24.0500964Z ##[command]git config --get-all http.proxy
2020-02-16T15:26:24.0630568Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69209/merge:refs/remotes/pull/69209/merge
