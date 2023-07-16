plain
2020-03-14T14:31:23.8001665Z ========================== Starting Command Output ===========================
2020-03-14T14:31:23.8023167Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/833bc518-c827-4d85-a672-3ea74e667caf.sh
2020-03-14T14:31:24.0378499Z 
2020-03-14T14:31:24.0443857Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-14T14:31:24.0470121Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69981/merge to s
2020-03-14T14:31:24.0481558Z Task         : Get sources
2020-03-14T14:31:24.0482033Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-14T14:31:24.0482498Z Version      : 1.0.0
2020-03-14T14:31:24.0482845Z Author       : Microsoft
---
2020-03-14T14:31:26.3851170Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-14T14:31:26.4030037Z ##[command]git config gc.auto 0
2020-03-14T14:31:26.4067653Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-14T14:31:26.4104329Z ##[command]git config --get-all http.proxy
2020-03-14T14:31:26.4233709Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69981/merge:refs/remotes/pull/69981/merge
