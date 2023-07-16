plain
2020-02-22T02:36:28.8377938Z ========================== Starting Command Output ===========================
2020-02-22T02:36:28.8380945Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/329cb1fd-4786-467a-ad53-100ffddcecfd.sh
2020-02-22T02:36:28.8381166Z 
2020-02-22T02:36:28.8385052Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-22T02:36:28.8406108Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68434/merge to s
2020-02-22T02:36:28.8410059Z Task         : Get sources
2020-02-22T02:36:28.8410307Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T02:36:28.8411269Z Version      : 1.0.0
2020-02-22T02:36:28.8411447Z Author       : Microsoft
---
2020-02-22T02:36:30.4658369Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-22T02:36:30.4809836Z ##[command]git config gc.auto 0
2020-02-22T02:36:30.4851864Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-22T02:36:30.4882765Z ##[command]git config --get-all http.proxy
2020-02-22T02:36:30.4968550Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68434/merge:refs/remotes/pull/68434/merge
