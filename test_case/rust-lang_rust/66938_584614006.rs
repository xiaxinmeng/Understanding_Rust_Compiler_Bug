plain
2020-02-11T12:17:35.5035168Z ========================== Starting Command Output ===========================
2020-02-11T12:17:35.5037432Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/897feab8-30f8-4e05-aacf-6a73f341c255.sh
2020-02-11T12:17:35.5037573Z 
2020-02-11T12:17:35.5040233Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-11T12:17:35.5045078Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/66938/merge to s
2020-02-11T12:17:35.5046473Z Task         : Get sources
2020-02-11T12:17:35.5046539Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-11T12:17:35.5046570Z Version      : 1.0.0
2020-02-11T12:17:35.5046599Z Author       : Microsoft
---
2020-02-11T12:17:36.2959486Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-11T12:17:36.3050244Z ##[command]git config gc.auto 0
2020-02-11T12:17:36.3110687Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-11T12:17:36.3172170Z ##[command]git config --get-all http.proxy
2020-02-11T12:17:36.3317105Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66938/merge:refs/remotes/pull/66938/merge
