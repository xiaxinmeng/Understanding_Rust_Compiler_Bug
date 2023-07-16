plain
2020-01-23T14:23:23.3185199Z ========================== Starting Command Output ===========================
2020-01-23T14:23:23.3186715Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/389ceeb2-2df0-4602-b6a5-5c0b583a44f8.sh
2020-01-23T14:23:23.3186750Z 
2020-01-23T14:23:23.3189696Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-23T14:23:23.3196525Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68414/merge to s
2020-01-23T14:23:23.3198258Z Task         : Get sources
2020-01-23T14:23:23.3198292Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-23T14:23:23.3198325Z Version      : 1.0.0
2020-01-23T14:23:23.3198356Z Author       : Microsoft
---
2020-01-23T14:23:25.9491471Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-23T14:23:25.9670146Z ##[command]git config gc.auto 0
2020-01-23T14:23:25.9741969Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-23T14:23:25.9794229Z ##[command]git config --get-all http.proxy
2020-01-23T14:23:25.9949052Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68414/merge:refs/remotes/pull/68414/merge
