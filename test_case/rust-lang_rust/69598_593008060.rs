plain
2020-02-29T23:48:30.7656068Z ========================== Starting Command Output ===========================
2020-02-29T23:48:30.7659052Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/52a7c53f-6511-4298-b3de-a811358cb2bc.sh
2020-02-29T23:48:30.7659337Z 
2020-02-29T23:48:30.7661946Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-29T23:48:30.7681862Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69598/merge to s
2020-02-29T23:48:30.7685246Z Task         : Get sources
2020-02-29T23:48:30.7685461Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-29T23:48:30.7685707Z Version      : 1.0.0
2020-02-29T23:48:30.7685847Z Author       : Microsoft
---
2020-02-29T23:48:32.0279930Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-29T23:48:32.0290801Z ##[command]git config gc.auto 0
2020-02-29T23:48:32.0299163Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-29T23:48:32.0305561Z ##[command]git config --get-all http.proxy
2020-02-29T23:48:32.0318760Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69598/merge:refs/remotes/pull/69598/merge
