plain
2020-01-21T15:52:34.4016176Z ========================== Starting Command Output ===========================
2020-01-21T15:52:34.4028648Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/36eb0214-b3c5-4d30-8490-dd7f8ccceab6.sh
2020-01-21T15:52:34.4276415Z 
2020-01-21T15:52:34.4301906Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-21T15:52:34.4308258Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68404/merge to s
2020-01-21T15:52:34.4309806Z Task         : Get sources
2020-01-21T15:52:34.4309840Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-21T15:52:34.4309911Z Version      : 1.0.0
2020-01-21T15:52:34.4309944Z Author       : Microsoft
---
2020-01-21T15:52:35.8735130Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-21T15:52:35.8799767Z ##[command]git config gc.auto 0
2020-01-21T15:52:35.8858688Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-21T15:52:35.8908256Z ##[command]git config --get-all http.proxy
2020-01-21T15:52:35.9017583Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68404/merge:refs/remotes/pull/68404/merge
