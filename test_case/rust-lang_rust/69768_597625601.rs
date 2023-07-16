plain
2020-03-11T12:58:33.5647965Z ========================== Starting Command Output ===========================
2020-03-11T12:58:33.5650052Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/535e9a9f-3239-4232-8449-34a74f2763e2.sh
2020-03-11T12:58:33.5650247Z 
2020-03-11T12:58:33.5653727Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-11T12:58:33.5670483Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69768/merge to s
2020-03-11T12:58:33.5673250Z Task         : Get sources
2020-03-11T12:58:33.5673478Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-11T12:58:33.5673748Z Version      : 1.0.0
2020-03-11T12:58:33.5673896Z Author       : Microsoft
---
2020-03-11T12:58:34.5851779Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-11T12:58:34.5856354Z ##[command]git config gc.auto 0
2020-03-11T12:58:34.5859528Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-11T12:58:34.5864291Z ##[command]git config --get-all http.proxy
2020-03-11T12:58:34.5872260Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69768/merge:refs/remotes/pull/69768/merge
