plain
2020-03-04T13:58:32.7440062Z ========================== Starting Command Output ===========================
2020-03-04T13:58:32.7447402Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4f15b2eb-6ed4-4580-9d63-35ca78d40c49.sh
2020-03-04T13:58:32.7448362Z 
2020-03-04T13:58:32.7454322Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-04T13:58:32.7476765Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69033/merge to s
2020-03-04T13:58:32.7480567Z Task         : Get sources
2020-03-04T13:58:32.7480960Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-04T13:58:32.7481321Z Version      : 1.0.0
2020-03-04T13:58:32.7481567Z Author       : Microsoft
---
2020-03-04T13:58:33.7429603Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-04T13:58:33.7435688Z ##[command]git config gc.auto 0
2020-03-04T13:58:33.7439856Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-04T13:58:33.7443968Z ##[command]git config --get-all http.proxy
2020-03-04T13:58:33.7450859Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69033/merge:refs/remotes/pull/69033/merge
