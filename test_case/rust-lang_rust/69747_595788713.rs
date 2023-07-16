plain
2020-03-06T14:17:57.8890968Z ========================== Starting Command Output ===========================
2020-03-06T14:17:57.8896480Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6c373c06-869a-440e-95ff-5a14f3028eab.sh
2020-03-06T14:17:57.8896992Z 
2020-03-06T14:17:57.8901721Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-06T14:17:57.8922818Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69747/merge to s
2020-03-06T14:17:57.8926433Z Task         : Get sources
2020-03-06T14:17:57.8926764Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-06T14:17:57.8927099Z Version      : 1.0.0
2020-03-06T14:17:57.8927317Z Author       : Microsoft
---
2020-03-06T14:17:59.4953557Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-06T14:17:59.4960623Z ##[command]git config gc.auto 0
2020-03-06T14:17:59.4965329Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-06T14:17:59.4969273Z ##[command]git config --get-all http.proxy
2020-03-06T14:17:59.4978120Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69747/merge:refs/remotes/pull/69747/merge
