plain
2020-02-22T14:36:10.6934281Z ========================== Starting Command Output ===========================
2020-02-22T14:36:10.6954354Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2900ef50-7f80-4179-9f6b-7e289381ad77.sh
2020-02-22T14:36:10.9531267Z 
2020-02-22T14:36:10.9607118Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-22T14:36:10.9666909Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69377/merge to s
2020-02-22T14:36:10.9679014Z Task         : Get sources
2020-02-22T14:36:10.9679524Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T14:36:10.9680097Z Version      : 1.0.0
2020-02-22T14:36:10.9680503Z Author       : Microsoft
---
2020-02-22T14:36:13.5461946Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-22T14:36:13.5665117Z ##[command]git config gc.auto 0
2020-02-22T14:36:13.5679454Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-22T14:36:13.5717159Z ##[command]git config --get-all http.proxy
2020-02-22T14:36:13.5802055Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69377/merge:refs/remotes/pull/69377/merge
2020-02-22T14:36:15.1945296Z fatal: remote error: upload-pack: not our ref 9476f64a5e0e8e2e3084d5d6e9c270eb4e62e369
2020-02-22T14:36:15.2120826Z fatal: the remote end hung up unexpectedly
2020-02-22T14:36:15.2801653Z ##[warning]Git fetch failed with exit code 128, back off 9.268 seconds before retry.
2020-02-22T14:36:24.4997823Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69377/merge:refs/remotes/pull/69377/merge
---
2020-02-22T14:40:36.7641160Z E: Unable to fetch some archives, maybe run apt-get update or try with --fix-missing?
2020-02-22T14:40:36.7683728Z 
2020-02-22T14:40:36.7722720Z ##[error]Bash exited with code '100'.
2020-02-22T14:40:36.7747626Z ##[section]Finishing: Install awscli
2020-02-22T14:40:36.7812921Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69377/merge to s
2020-02-22T14:40:36.7817741Z Task         : Get sources
2020-02-22T14:40:36.7818073Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T14:40:36.7818385Z Version      : 1.0.0
2020-02-22T14:40:36.7818611Z Author       : Microsoft
2020-02-22T14:40:36.7818611Z Author       : Microsoft
2020-02-22T14:40:36.7819135Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-22T14:40:36.7819542Z ==============================================================================
2020-02-22T14:40:37.1252874Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-22T14:40:37.1295146Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69377/merge to s
2020-02-22T14:40:37.1376523Z Cleaning up task key
2020-02-22T14:40:37.1377604Z Start cleaning up orphan processes.
2020-02-22T14:40:37.1542836Z Terminate orphan process: pid (3990) (python)
2020-02-22T14:40:37.1671473Z ##[section]Finishing: Finalize Job
