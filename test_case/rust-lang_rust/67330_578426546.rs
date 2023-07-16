plain
2020-01-25T17:29:57.5905053Z ========================== Starting Command Output ===========================
2020-01-25T17:29:57.5908627Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/042bc763-ad79-4907-b393-0b772a70119a.sh
2020-01-25T17:29:57.5908677Z 
2020-01-25T17:29:57.5912585Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-25T17:29:57.5919692Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67330/merge to s
2020-01-25T17:29:57.5921716Z Task         : Get sources
2020-01-25T17:29:57.5921753Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-25T17:29:57.5921843Z Version      : 1.0.0
2020-01-25T17:29:57.5921880Z Author       : Microsoft
---
2020-01-25T17:29:58.6377899Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-25T17:29:58.6393026Z ##[command]git config gc.auto 0
2020-01-25T17:29:58.6398857Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-25T17:29:58.6404454Z ##[command]git config --get-all http.proxy
2020-01-25T17:29:58.6413437Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67330/merge:refs/remotes/pull/67330/merge
