plain
2020-03-07T00:26:14.8118716Z ========================== Starting Command Output ===========================
2020-03-07T00:26:14.8120805Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2ef6ce12-5229-482c-af0c-9d98cd509ed0.sh
2020-03-07T00:26:14.8120996Z 
2020-03-07T00:26:14.8123892Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-07T00:26:14.8140965Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69793/merge to s
2020-03-07T00:26:14.8144091Z Task         : Get sources
2020-03-07T00:26:14.8144358Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-07T00:26:14.8144576Z Version      : 1.0.0
2020-03-07T00:26:14.8144726Z Author       : Microsoft
---
2020-03-07T00:26:16.0552387Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-07T00:26:16.0559046Z ##[command]git config gc.auto 0
2020-03-07T00:26:16.0563490Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-07T00:26:16.0568165Z ##[command]git config --get-all http.proxy
2020-03-07T00:26:16.0577725Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69793/merge:refs/remotes/pull/69793/merge
