plain
2020-01-28T17:03:32.6223666Z ========================== Starting Command Output ===========================
2020-01-28T17:03:32.6226175Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b6fd0137-099f-4535-9c77-56bff62c9f91.sh
2020-01-28T17:03:32.6226215Z 
2020-01-28T17:03:32.6229001Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-28T17:03:32.6234209Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67028/merge to s
2020-01-28T17:03:32.6235792Z Task         : Get sources
2020-01-28T17:03:32.6235826Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-28T17:03:32.6235897Z Version      : 1.0.0
2020-01-28T17:03:32.6235931Z Author       : Microsoft
---
2020-01-28T17:03:33.6917398Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-28T17:03:33.6929172Z ##[command]git config gc.auto 0
2020-01-28T17:03:33.6932358Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-28T17:03:33.6934298Z ##[command]git config --get-all http.proxy
2020-01-28T17:03:33.6940951Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67028/merge:refs/remotes/pull/67028/merge
