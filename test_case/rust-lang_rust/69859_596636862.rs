plain
2020-03-09T16:14:34.7384113Z Prepare build directory.
2020-03-09T16:14:34.7708255Z Set build variables.
2020-03-09T16:14:34.7739164Z Download all required tasks.
2020-03-09T16:14:34.7853082Z Downloading task: Bash (3.163.1)
2020-03-09T16:14:36.8633109Z Checking job knob settings.
2020-03-09T16:14:36.8652547Z Finished checking job knob settings.
2020-03-09T16:14:36.9239852Z ##[section]Finishing: Initialize job
2020-03-09T16:14:36.9586259Z ##[section]Starting: Configure Job Name
2020-03-09T16:14:36.9801012Z ==============================================================================
2020-03-09T16:14:36.9801840Z Task         : Bash
---
2020-03-09T16:14:37.6127258Z ========================== Starting Command Output ===========================
2020-03-09T16:14:37.6131165Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b314ec7c-36a0-46ef-b4fe-2c88fd2490f0.sh
2020-03-09T16:14:37.6131657Z 
2020-03-09T16:14:37.6136907Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-09T16:14:37.6155920Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69859/merge to s
2020-03-09T16:14:37.6158962Z Task         : Get sources
2020-03-09T16:14:37.6159233Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-09T16:14:37.6159543Z Version      : 1.0.0
2020-03-09T16:14:37.6159724Z Author       : Microsoft
---
2020-03-09T16:14:38.8484071Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-09T16:14:38.8490310Z ##[command]git config gc.auto 0
2020-03-09T16:14:38.8493747Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-09T16:14:38.8498656Z ##[command]git config --get-all http.proxy
2020-03-09T16:14:38.8507039Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69859/merge:refs/remotes/pull/69859/merge
