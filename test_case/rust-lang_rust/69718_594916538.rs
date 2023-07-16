plain
2020-03-04T22:42:11.0015940Z ========================== Starting Command Output ===========================
2020-03-04T22:42:11.0019900Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/97a346ad-b12d-426f-b158-1976690e7c01.sh
2020-03-04T22:42:11.0020285Z 
2020-03-04T22:42:11.0029128Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-04T22:42:11.0060763Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69718/merge to s
2020-03-04T22:42:11.0063562Z Task         : Get sources
2020-03-04T22:42:11.0063788Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-04T22:42:11.0064003Z Version      : 1.0.0
2020-03-04T22:42:11.0064150Z Author       : Microsoft
---
2020-03-04T22:42:12.0033993Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-04T22:42:12.0042265Z ##[command]git config gc.auto 0
2020-03-04T22:42:12.0047421Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-04T22:42:12.0051855Z ##[command]git config --get-all http.proxy
2020-03-04T22:42:12.0059057Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69718/merge:refs/remotes/pull/69718/merge
