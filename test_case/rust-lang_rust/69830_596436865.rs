plain
2020-03-09T09:46:16.3989685Z ========================== Starting Command Output ===========================
2020-03-09T09:46:16.3994108Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c91bfa8b-a750-478d-ae1f-56703b195d6c.sh
2020-03-09T09:46:16.3994423Z 
2020-03-09T09:46:16.3999315Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-09T09:46:16.4028852Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69830/merge to s
2020-03-09T09:46:16.4032965Z Task         : Get sources
2020-03-09T09:46:16.4033338Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-09T09:46:16.4033660Z Version      : 1.0.0
2020-03-09T09:46:16.4033879Z Author       : Microsoft
---
2020-03-09T09:46:17.3838556Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-09T09:46:17.3843744Z ##[command]git config gc.auto 0
2020-03-09T09:46:17.3847256Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-09T09:46:17.3850462Z ##[command]git config --get-all http.proxy
2020-03-09T09:46:17.3856712Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69830/merge:refs/remotes/pull/69830/merge
