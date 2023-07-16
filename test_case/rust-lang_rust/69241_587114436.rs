plain
2020-02-17T18:12:35.7364435Z ========================== Starting Command Output ===========================
2020-02-17T18:12:35.7384641Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/91845100-bc90-4b8a-9665-ca2a6964191a.sh
2020-02-17T18:12:35.7734765Z 
2020-02-17T18:12:35.7740523Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-17T18:12:35.7749509Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69241/merge to s
2020-02-17T18:12:35.7751685Z Task         : Get sources
2020-02-17T18:12:35.7751733Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-17T18:12:35.7751766Z Version      : 1.0.0
2020-02-17T18:12:35.7751797Z Author       : Microsoft
---
2020-02-17T18:12:36.7009395Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-17T18:12:36.7099963Z ##[command]git config gc.auto 0
2020-02-17T18:12:36.7166888Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-17T18:12:36.7217613Z ##[command]git config --get-all http.proxy
2020-02-17T18:12:36.7358765Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69241/merge:refs/remotes/pull/69241/merge
