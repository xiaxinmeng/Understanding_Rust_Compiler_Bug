plain
2019-10-24T15:41:15.5283966Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-24T15:41:15.5497097Z ##[command]git config gc.auto 0
2019-10-24T15:41:15.5588937Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-24T15:41:15.5643144Z ##[command]git config --get-all http.proxy
2019-10-24T15:41:15.5788082Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65762/merge:refs/remotes/pull/65762/merge
---
2019-10-24T15:41:55.7293404Z Generating script.
2019-10-24T15:41:55.7321781Z ========================== Starting Command Output ===========================
2019-10-24T15:41:55.7344281Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/42a1072a-e118-43b6-bea2-55354ed5d484.sh
2019-10-24T15:41:55.7452527Z 
2019-10-24T15:41:55.7454116Z /home/vsts/work/_temp/42a1072a-e118-43b6-bea2-55354ed5d484.sh: line 1: pacman: command not found
2019-10-24T15:41:55.7454460Z /home/vsts/work/_temp/42a1072a-e118-43b6-bea2-55354ed5d484.sh: line 3: pacman: command not found
2019-10-24T15:41:55.7454840Z rm: cannot remove 'mingw-w64-x86_64-ca-certificates-20180409-1-any.pkg.tar.xz': No such file or directory
2019-10-24T15:41:55.7554186Z ##[error]Bash exited with code '1'.
2019-10-24T15:41:55.7660620Z ##[section]Starting: Checkout
2019-10-24T15:41:55.7663242Z ==============================================================================
2019-10-24T15:41:55.7663304Z Task         : Get sources
2019-10-24T15:41:55.7663390Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
