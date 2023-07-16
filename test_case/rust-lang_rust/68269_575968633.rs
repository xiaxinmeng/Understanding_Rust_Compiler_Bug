plain
2020-01-19T05:00:54.1596456Z ========================== Starting Command Output ===========================
2020-01-19T05:00:54.1617345Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a914f82e-03a7-4a12-b099-c2758c6cb9ec.sh
2020-01-19T05:00:54.2093493Z 
2020-01-19T05:00:54.2157361Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-19T05:00:54.2164652Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68269/merge to s
2020-01-19T05:00:54.2167090Z Task         : Get sources
2020-01-19T05:00:54.2167162Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-19T05:00:54.2167197Z Version      : 1.0.0
2020-01-19T05:00:54.2167230Z Author       : Microsoft
---
2020-01-19T05:00:55.6249832Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-19T05:00:55.6262239Z ##[command]git config gc.auto 0
2020-01-19T05:00:55.6264652Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-19T05:00:55.6270407Z ##[command]git config --get-all http.proxy
2020-01-19T05:00:55.6277308Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68269/merge:refs/remotes/pull/68269/merge
