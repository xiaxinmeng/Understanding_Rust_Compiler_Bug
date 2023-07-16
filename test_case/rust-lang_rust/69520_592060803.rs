plain
2020-02-27T16:30:30.8869384Z ========================== Starting Command Output ===========================
2020-02-27T16:30:30.8873430Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/facc7d7a-4c9d-4351-a2b9-7322dd38115f.sh
2020-02-27T16:30:30.8874280Z 
2020-02-27T16:30:30.8879664Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-27T16:30:30.8897168Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69520/merge to s
2020-02-27T16:30:30.8900070Z Task         : Get sources
2020-02-27T16:30:30.8900300Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-27T16:30:30.8900571Z Version      : 1.0.0
2020-02-27T16:30:30.8900880Z Author       : Microsoft
---
2020-02-27T16:30:32.1732279Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-27T16:30:32.1738686Z ##[command]git config gc.auto 0
2020-02-27T16:30:32.1742020Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-27T16:30:32.1745101Z ##[command]git config --get-all http.proxy
2020-02-27T16:30:32.1752114Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69520/merge:refs/remotes/pull/69520/merge
