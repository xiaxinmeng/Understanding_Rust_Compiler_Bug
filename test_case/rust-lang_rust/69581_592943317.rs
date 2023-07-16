plain
2020-02-29T12:41:22.0278814Z ========================== Starting Command Output ===========================
2020-02-29T12:41:22.0282082Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6fbfa46a-3d45-4a8f-8b3b-3d73d3461671.sh
2020-02-29T12:41:22.0282626Z 
2020-02-29T12:41:22.0286746Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-29T12:41:22.0307718Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69581/merge to s
2020-02-29T12:41:22.0311402Z Task         : Get sources
2020-02-29T12:41:22.0311778Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-29T12:41:22.0312104Z Version      : 1.0.0
2020-02-29T12:41:22.0312332Z Author       : Microsoft
---
2020-02-29T12:41:23.0183660Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-29T12:41:23.0197899Z ##[command]git config gc.auto 0
2020-02-29T12:41:23.0201899Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-29T12:41:23.0205531Z ##[command]git config --get-all http.proxy
2020-02-29T12:41:23.0212088Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69581/merge:refs/remotes/pull/69581/merge
