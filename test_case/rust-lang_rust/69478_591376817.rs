plain
2020-02-26T11:05:17.0080663Z ========================== Starting Command Output ===========================
2020-02-26T11:05:17.0083792Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a8ccabd2-f485-44eb-ab44-d259119e607b.sh
2020-02-26T11:05:17.0084368Z 
2020-02-26T11:05:17.0087820Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-26T11:05:17.0104021Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-02-26T11:05:17.0106720Z Task         : Get sources
2020-02-26T11:05:17.0106937Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-26T11:05:17.0107244Z Version      : 1.0.0
2020-02-26T11:05:17.0107388Z Author       : Microsoft
---
2020-02-26T11:05:18.0284353Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-26T11:05:18.0289388Z ##[command]git config gc.auto 0
2020-02-26T11:05:18.0293193Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-26T11:05:18.0295813Z ##[command]git config --get-all http.proxy
2020-02-26T11:05:18.0302584Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69478/merge:refs/remotes/pull/69478/merge
