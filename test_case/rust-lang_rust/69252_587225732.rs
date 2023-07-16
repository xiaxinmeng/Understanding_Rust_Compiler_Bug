plain
2020-02-18T00:58:44.1191206Z ========================== Starting Command Output ===========================
2020-02-18T00:58:44.1193945Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b4018b75-9173-48e9-bcea-a382c75e6a5a.sh
2020-02-18T00:58:44.1193991Z 
2020-02-18T00:58:44.1197953Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-18T00:58:44.1204847Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69252/merge to s
2020-02-18T00:58:44.1206451Z Task         : Get sources
2020-02-18T00:58:44.1206485Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-18T00:58:44.1206562Z Version      : 1.0.0
2020-02-18T00:58:44.1206595Z Author       : Microsoft
---
2020-02-18T00:58:46.4227405Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-18T00:58:46.4240471Z ##[command]git config gc.auto 0
2020-02-18T00:58:46.4242666Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-18T00:58:46.4245472Z ##[command]git config --get-all http.proxy
2020-02-18T00:58:46.4251862Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69252/merge:refs/remotes/pull/69252/merge
