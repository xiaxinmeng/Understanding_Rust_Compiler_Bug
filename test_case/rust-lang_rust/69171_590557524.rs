plain
2020-02-24T21:26:47.7633309Z ========================== Starting Command Output ===========================
2020-02-24T21:26:47.7638767Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7bf5b5ec-7635-46dd-a7c3-22df4063c36e.sh
2020-02-24T21:26:47.7639292Z 
2020-02-24T21:26:47.7643536Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-24T21:26:47.7664466Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69171/merge to s
2020-02-24T21:26:47.7667990Z Task         : Get sources
2020-02-24T21:26:47.7668322Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-24T21:26:47.7668639Z Version      : 1.0.0
2020-02-24T21:26:47.7668855Z Author       : Microsoft
---
2020-02-24T21:26:48.7513795Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-24T21:26:48.7519777Z ##[command]git config gc.auto 0
2020-02-24T21:26:48.7523970Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-24T21:26:48.7527906Z ##[command]git config --get-all http.proxy
2020-02-24T21:26:48.7534792Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69171/merge:refs/remotes/pull/69171/merge
