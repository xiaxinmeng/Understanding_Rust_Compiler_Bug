plain
2020-02-12T13:40:57.4340642Z ========================== Starting Command Output ===========================
2020-02-12T13:40:57.4342147Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9dbab29a-2123-4747-acb7-bdae29593a14.sh
2020-02-12T13:40:57.4342183Z 
2020-02-12T13:40:57.4344965Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-12T13:40:57.4352770Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69095/merge to s
2020-02-12T13:40:57.4354622Z Task         : Get sources
2020-02-12T13:40:57.4354654Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-12T13:40:57.4354735Z Version      : 1.0.0
2020-02-12T13:40:57.4354768Z Author       : Microsoft
---
2020-02-12T13:40:58.4386190Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-12T13:40:58.4407130Z ##[command]git config gc.auto 0
2020-02-12T13:40:58.4415032Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-12T13:40:58.4420473Z ##[command]git config --get-all http.proxy
2020-02-12T13:40:58.4430666Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69095/merge:refs/remotes/pull/69095/merge
