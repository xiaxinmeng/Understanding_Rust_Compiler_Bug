plain
2020-01-20T21:17:29.3002033Z ========================== Starting Command Output ===========================
2020-01-20T21:17:29.3004186Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d03a6f89-e91b-4439-a50e-9855b9b7ac15.sh
2020-01-20T21:17:29.3004222Z 
2020-01-20T21:17:29.3007845Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-20T21:17:29.3016076Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68388/merge to s
2020-01-20T21:17:29.3017835Z Task         : Get sources
2020-01-20T21:17:29.3017915Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-20T21:17:29.3017949Z Version      : 1.0.0
2020-01-20T21:17:29.3017983Z Author       : Microsoft
---
2020-01-20T21:17:30.3705095Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-20T21:17:30.3720006Z ##[command]git config gc.auto 0
2020-01-20T21:17:30.3724363Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-20T21:17:30.3729811Z ##[command]git config --get-all http.proxy
2020-01-20T21:17:30.3738886Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68388/merge:refs/remotes/pull/68388/merge
