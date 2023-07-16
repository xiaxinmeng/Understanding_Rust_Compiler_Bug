plain
2020-03-16T19:53:04.2306165Z ========================== Starting Command Output ===========================
2020-03-16T19:53:04.2310052Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1e317d35-dc34-4115-ad75-364a75cc4181.sh
2020-03-16T19:53:04.2310364Z 
2020-03-16T19:53:04.2313811Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-16T19:53:04.2334244Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70051/merge to s
2020-03-16T19:53:04.2338359Z Task         : Get sources
2020-03-16T19:53:04.2338692Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-16T19:53:04.2339001Z Version      : 1.0.0
2020-03-16T19:53:04.2339209Z Author       : Microsoft
---
2020-03-16T19:53:06.9945137Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-16T19:53:06.9953499Z ##[command]git config gc.auto 0
2020-03-16T19:53:06.9959182Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-16T19:53:06.9964637Z ##[command]git config --get-all http.proxy
2020-03-16T19:53:06.9976267Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70051/merge:refs/remotes/pull/70051/merge
