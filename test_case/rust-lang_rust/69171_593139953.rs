plain
2020-03-01T19:57:56.9989108Z ========================== Starting Command Output ===========================
2020-03-01T19:57:56.9994391Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/336cd1ac-3392-4351-8892-839fa38b1f89.sh
2020-03-01T19:57:56.9994851Z 
2020-03-01T19:57:56.9998941Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-01T19:57:57.0019518Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69171/merge to s
2020-03-01T19:57:57.0023370Z Task         : Get sources
2020-03-01T19:57:57.0023669Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-01T19:57:57.0024196Z Version      : 1.0.0
2020-03-01T19:57:57.0024390Z Author       : Microsoft
---
2020-03-01T19:57:58.2581842Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-01T19:57:58.3101075Z ##[command]git config gc.auto 0
2020-03-01T19:57:58.3105643Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-01T19:57:58.3109793Z ##[command]git config --get-all http.proxy
2020-03-01T19:57:58.3120341Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69171/merge:refs/remotes/pull/69171/merge
