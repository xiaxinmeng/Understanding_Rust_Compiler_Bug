plain
2020-02-22T15:57:02.4168248Z ========================== Starting Command Output ===========================
2020-02-22T15:57:02.4170374Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a50a7cad-0fe6-499e-8710-0179aa386d84.sh
2020-02-22T15:57:02.4170584Z 
2020-02-22T15:57:02.4176633Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-22T15:57:02.4200928Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68963/merge to s
2020-02-22T15:57:02.4205199Z Task         : Get sources
2020-02-22T15:57:02.4205438Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T15:57:02.4206178Z Version      : 1.0.0
2020-02-22T15:57:02.4206338Z Author       : Microsoft
---
2020-02-22T15:57:05.6229080Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-22T15:57:05.6238398Z ##[command]git config gc.auto 0
2020-02-22T15:57:05.6246012Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-22T15:57:05.6250467Z ##[command]git config --get-all http.proxy
2020-02-22T15:57:05.6261041Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68963/merge:refs/remotes/pull/68963/merge
