plain
2020-02-10T14:49:03.6652346Z ========================== Starting Command Output ===========================
2020-02-10T14:49:03.6669792Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/135719e1-3aa1-4232-8b1b-2e2177705451.sh
2020-02-10T14:49:03.6869039Z 
2020-02-10T14:49:03.6928806Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-10T14:49:03.6934348Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68824/merge to s
2020-02-10T14:49:03.6936203Z Task         : Get sources
2020-02-10T14:49:03.6936233Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-10T14:49:03.6936280Z Version      : 1.0.0
2020-02-10T14:49:03.6936310Z Author       : Microsoft
---
2020-02-10T14:49:04.5516567Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-10T14:49:04.5604040Z ##[command]git config gc.auto 0
2020-02-10T14:49:04.5683988Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-10T14:49:04.5781797Z ##[command]git config --get-all http.proxy
2020-02-10T14:49:04.5922275Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68824/merge:refs/remotes/pull/68824/merge
