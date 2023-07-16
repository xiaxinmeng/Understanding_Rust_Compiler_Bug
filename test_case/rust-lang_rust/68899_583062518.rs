plain
2020-02-06T18:49:18.9527063Z ========================== Starting Command Output ===========================
2020-02-06T18:49:18.9528466Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/990b9e53-fb8c-4c93-92a4-2b764c756aef.sh
2020-02-06T18:49:18.9528500Z 
2020-02-06T18:49:18.9539795Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-06T18:49:18.9545558Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68899/merge to s
2020-02-06T18:49:18.9547392Z Task         : Get sources
2020-02-06T18:49:18.9547429Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-06T18:49:18.9547462Z Version      : 1.0.0
2020-02-06T18:49:18.9547494Z Author       : Microsoft
---
2020-02-06T18:49:19.9628063Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-06T18:49:19.9765817Z ##[command]git config gc.auto 0
2020-02-06T18:49:19.9833479Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-06T18:49:19.9870020Z ##[command]git config --get-all http.proxy
2020-02-06T18:49:20.0038732Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68899/merge:refs/remotes/pull/68899/merge
