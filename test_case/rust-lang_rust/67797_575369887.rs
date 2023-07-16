plain
2020-01-16T22:07:20.9408764Z ========================== Starting Command Output ===========================
2020-01-16T22:07:20.9411043Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/659cf19b-cf7f-4573-bb01-8b1b88d153b3.sh
2020-01-16T22:07:20.9411075Z 
2020-01-16T22:07:20.9413015Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-16T22:07:20.9419886Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67797/merge to s
2020-01-16T22:07:20.9421559Z Task         : Get sources
2020-01-16T22:07:20.9421587Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-16T22:07:20.9421620Z Version      : 1.0.0
2020-01-16T22:07:20.9421647Z Author       : Microsoft
---
2020-01-16T22:07:21.9148743Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-16T22:07:21.9160191Z ##[command]git config gc.auto 0
2020-01-16T22:07:21.9162605Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-16T22:07:21.9164223Z ##[command]git config --get-all http.proxy
2020-01-16T22:07:21.9170933Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67797/merge:refs/remotes/pull/67797/merge
