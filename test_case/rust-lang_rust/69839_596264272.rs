plain
2020-03-08T22:52:43.6640229Z ========================== Starting Command Output ===========================
2020-03-08T22:52:43.6643047Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/41ff1b9f-0165-4437-942c-19711d11b6bc.sh
2020-03-08T22:52:43.6643347Z 
2020-03-08T22:52:43.6647244Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-08T22:52:43.6670700Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69839/merge to s
2020-03-08T22:52:43.6675268Z Task         : Get sources
2020-03-08T22:52:43.6675623Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-08T22:52:43.6675940Z Version      : 1.0.0
2020-03-08T22:52:43.6676158Z Author       : Microsoft
---
2020-03-08T22:52:46.7141514Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-08T22:52:46.7152817Z ##[command]git config gc.auto 0
2020-03-08T22:52:46.7161119Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-08T22:52:46.7165657Z ##[command]git config --get-all http.proxy
2020-03-08T22:52:46.7174965Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69839/merge:refs/remotes/pull/69839/merge
