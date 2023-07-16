plain
2020-01-25T07:31:59.4689436Z ========================== Starting Command Output ===========================
2020-01-25T07:31:59.4691144Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b320d523-0356-4573-8701-85b0434aca3b.sh
2020-01-25T07:31:59.4691179Z 
2020-01-25T07:31:59.4694391Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-25T07:31:59.4699703Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67330/merge to s
2020-01-25T07:31:59.4701182Z Task         : Get sources
2020-01-25T07:31:59.4701214Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-25T07:31:59.4701246Z Version      : 1.0.0
2020-01-25T07:31:59.4701318Z Author       : Microsoft
---
2020-01-25T07:32:00.2087520Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-25T07:32:00.2181956Z ##[command]git config gc.auto 0
2020-01-25T07:32:00.2236256Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-25T07:32:00.2288029Z ##[command]git config --get-all http.proxy
2020-01-25T07:32:00.2433159Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67330/merge:refs/remotes/pull/67330/merge
