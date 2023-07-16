plain
2020-01-24T17:34:48.1979487Z ========================== Starting Command Output ===========================
2020-01-24T17:34:48.1996059Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a15ed74f-525a-4fcc-9316-95b2f8e6f486.sh
2020-01-24T17:34:48.2488961Z 
2020-01-24T17:34:48.2559151Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-24T17:34:48.2565194Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68515/merge to s
2020-01-24T17:34:48.2566834Z Task         : Get sources
2020-01-24T17:34:48.2566915Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-24T17:34:48.2566950Z Version      : 1.0.0
2020-01-24T17:34:48.2566984Z Author       : Microsoft
---
2020-01-24T17:34:49.4269409Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-24T17:34:49.4280857Z ##[command]git config gc.auto 0
2020-01-24T17:34:49.4283352Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-24T17:34:49.4285239Z ##[command]git config --get-all http.proxy
2020-01-24T17:34:49.4291255Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68515/merge:refs/remotes/pull/68515/merge
