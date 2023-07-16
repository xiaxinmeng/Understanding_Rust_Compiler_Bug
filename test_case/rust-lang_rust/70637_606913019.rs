plain
2020-03-31T22:22:35.9454834Z ========================== Starting Command Output ===========================
2020-03-31T22:22:35.9457282Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/753728db-95c0-495c-9c83-c8674b89f442.sh
2020-03-31T22:22:35.9457565Z 
2020-03-31T22:22:35.9461357Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-31T22:22:35.9479878Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70637/merge to s
2020-03-31T22:22:35.9482915Z Task         : Get sources
2020-03-31T22:22:35.9483202Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T22:22:35.9483483Z Version      : 1.0.0
2020-03-31T22:22:35.9483672Z Author       : Microsoft
---
2020-03-31T22:22:36.9423266Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-31T22:22:36.9429553Z ##[command]git config gc.auto 0
2020-03-31T22:22:36.9433929Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-31T22:22:36.9438305Z ##[command]git config --get-all http.proxy
2020-03-31T22:22:36.9445974Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70637/merge:refs/remotes/pull/70637/merge
2020-03-31T22:22:37.2949731Z fatal: couldn't find remote ref refs/pull/70637/merge
2020-03-31T22:22:37.3762321Z ##[warning]Git fetch failed with exit code 128, back off 3.622 seconds before retry.
2020-03-31T22:22:40.9364323Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70637/merge:refs/remotes/pull/70637/merge
2020-03-31T22:22:41.9366454Z fatal: couldn't find remote ref refs/pull/70637/merge
2020-03-31T22:22:41.9952808Z ##[warning]Git fetch failed with exit code 128, back off 1.611 seconds before retry.
2020-03-31T22:22:43.1309879Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70637/merge:refs/remotes/pull/70637/merge
2020-03-31T22:22:43.7336961Z fatal: couldn't find remote ref refs/pull/70637/merge
2020-03-31T22:22:43.7899879Z ##[error]Git fetch failed with exit code: 128
2020-03-31T22:22:43.7914215Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70637/merge to s
2020-03-31T22:22:43.8326808Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70637/merge to s
2020-03-31T22:22:43.8332079Z Task         : Get sources
2020-03-31T22:22:43.8332407Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T22:22:43.8332729Z Version      : 1.0.0
2020-03-31T22:22:43.8332958Z Author       : Microsoft
2020-03-31T22:22:43.8332958Z Author       : Microsoft
2020-03-31T22:22:43.8333290Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-31T22:22:43.8333675Z ==============================================================================
2020-03-31T22:22:44.1238871Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70637/merge to s
2020-03-31T22:22:44.1319913Z Cleaning up task key
2020-03-31T22:22:44.1321265Z Start cleaning up orphan processes.
2020-03-31T22:22:44.1453113Z ##[section]Finishing: Finalize Job
2020-03-31T22:22:44.1493390Z ##[section]Finishing: Linux x86_64-gnu-tools
