plain
2020-04-02T15:52:23.7751623Z ========================== Starting Command Output ===========================
2020-04-02T15:52:23.7754032Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/76bb283b-1b00-4bc7-99df-3ba943183335.sh
2020-04-02T15:52:23.7754312Z 
2020-04-02T15:52:23.7758228Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-02T15:52:23.7776943Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70702/merge to s
2020-04-02T15:52:23.7780110Z Task         : Get sources
2020-04-02T15:52:23.7780418Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-02T15:52:23.7780734Z Version      : 1.0.0
2020-04-02T15:52:23.7780935Z Author       : Microsoft
---
2020-04-02T15:52:24.7701690Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-02T15:52:24.7708145Z ##[command]git config gc.auto 0
2020-04-02T15:52:24.7711709Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-02T15:52:24.7715010Z ##[command]git config --get-all http.proxy
2020-04-02T15:52:24.7720883Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70702/merge:refs/remotes/pull/70702/merge
2020-04-02T15:52:25.2885564Z fatal: couldn't find remote ref refs/pull/70702/merge
2020-04-02T15:52:25.3821030Z ##[warning]Git fetch failed with exit code 128, back off 4.425 seconds before retry.
2020-04-02T15:52:29.7334617Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70702/merge:refs/remotes/pull/70702/merge
2020-04-02T15:52:30.4108886Z fatal: couldn't find remote ref refs/pull/70702/merge
2020-04-02T15:52:30.4722013Z ##[warning]Git fetch failed with exit code 128, back off 7.508 seconds before retry.
2020-04-02T15:52:37.9241351Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70702/merge:refs/remotes/pull/70702/merge
2020-04-02T15:52:38.7049096Z fatal: couldn't find remote ref refs/pull/70702/merge
2020-04-02T15:52:38.7633126Z ##[error]Git fetch failed with exit code: 128
2020-04-02T15:52:38.7647030Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70702/merge to s
2020-04-02T15:52:38.8066380Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70702/merge to s
2020-04-02T15:52:38.8072022Z Task         : Get sources
2020-04-02T15:52:38.8072370Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-02T15:52:38.8072730Z Version      : 1.0.0
2020-04-02T15:52:38.8072955Z Author       : Microsoft
2020-04-02T15:52:38.8072955Z Author       : Microsoft
2020-04-02T15:52:38.8073304Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-02T15:52:38.8073721Z ==============================================================================
2020-04-02T15:52:39.1100390Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70702/merge to s
2020-04-02T15:52:39.1185645Z Cleaning up task key
2020-04-02T15:52:39.1187054Z Start cleaning up orphan processes.
2020-04-02T15:52:39.1327968Z ##[section]Finishing: Finalize Job
2020-04-02T15:52:39.1368093Z ##[section]Finishing: Linux x86_64-gnu-tools
