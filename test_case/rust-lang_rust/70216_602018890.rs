plain
2020-03-21T09:27:57.1836487Z ========================== Starting Command Output ===========================
2020-03-21T09:27:57.1841167Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d751ef02-5c48-432c-b3fd-0e9a726818c2.sh
2020-03-21T09:27:57.1843972Z 
2020-03-21T09:27:57.1849460Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-21T09:27:57.1867058Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70216/merge to s
2020-03-21T09:27:57.1869867Z Task         : Get sources
2020-03-21T09:27:57.1870092Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-21T09:27:57.1870362Z Version      : 1.0.0
2020-03-21T09:27:57.1870513Z Author       : Microsoft
---
2020-03-21T09:27:58.1859957Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-21T09:27:58.1864577Z ##[command]git config gc.auto 0
2020-03-21T09:27:58.1867746Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-21T09:27:58.1870553Z ##[command]git config --get-all http.proxy
2020-03-21T09:27:58.1876172Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70216/merge:refs/remotes/pull/70216/merge
2020-03-21T09:27:58.3839576Z fatal: couldn't find remote ref refs/pull/70216/merge
2020-03-21T09:27:58.4415863Z ##[warning]Git fetch failed with exit code 128, back off 4.428 seconds before retry.
2020-03-21T09:28:02.8307060Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70216/merge:refs/remotes/pull/70216/merge
2020-03-21T09:28:03.4208964Z fatal: couldn't find remote ref refs/pull/70216/merge
2020-03-21T09:28:03.4720274Z ##[warning]Git fetch failed with exit code 128, back off 9.252 seconds before retry.
2020-03-21T09:28:12.6762308Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70216/merge:refs/remotes/pull/70216/merge
2020-03-21T09:28:13.2637339Z fatal: couldn't find remote ref refs/pull/70216/merge
2020-03-21T09:28:13.2934428Z ##[error]Git fetch failed with exit code: 128
2020-03-21T09:28:13.2948040Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70216/merge to s
2020-03-21T09:28:13.3324545Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70216/merge to s
2020-03-21T09:28:13.3329103Z Task         : Get sources
2020-03-21T09:28:13.3329367Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-21T09:28:13.3329626Z Version      : 1.0.0
2020-03-21T09:28:13.3329794Z Author       : Microsoft
2020-03-21T09:28:13.3329794Z Author       : Microsoft
2020-03-21T09:28:13.3330115Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-21T09:28:13.3330422Z ==============================================================================
2020-03-21T09:28:13.6111427Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70216/merge to s
2020-03-21T09:28:13.6187604Z Cleaning up task key
2020-03-21T09:28:13.6188708Z Start cleaning up orphan processes.
2020-03-21T09:28:13.6323036Z ##[section]Finishing: Finalize Job
2020-03-21T09:28:13.6358159Z ##[section]Finishing: Linux x86_64-gnu-tools
