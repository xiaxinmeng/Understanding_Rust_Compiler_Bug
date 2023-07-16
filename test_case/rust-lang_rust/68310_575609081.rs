plain
2020-01-17T12:34:57.1902979Z ========================== Starting Command Output ===========================
2020-01-17T12:34:57.1904845Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ede12ae4-65c8-438d-ae33-3460122b02d4.sh
2020-01-17T12:34:57.1904886Z 
2020-01-17T12:34:57.1908057Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-17T12:34:57.1914716Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68310/merge to s
2020-01-17T12:34:57.1916918Z Task         : Get sources
2020-01-17T12:34:57.1916952Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-17T12:34:57.1917033Z Version      : 1.0.0
2020-01-17T12:34:57.1917066Z Author       : Microsoft
---
2020-01-17T12:34:58.2154941Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-17T12:34:58.2188528Z ##[command]git config gc.auto 0
2020-01-17T12:34:58.2194396Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-17T12:34:58.2199213Z ##[command]git config --get-all http.proxy
2020-01-17T12:34:58.2214788Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68310/merge:refs/remotes/pull/68310/merge
