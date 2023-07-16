plain
2020-04-06T21:35:05.1516885Z ========================== Starting Command Output ===========================
2020-04-06T21:35:05.1519654Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/284fb69c-f363-4881-87fa-c7c7f11b3e4b.sh
2020-04-06T21:35:05.1520341Z 
2020-04-06T21:35:05.1524209Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-06T21:35:05.1544795Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70858/merge to s
2020-04-06T21:35:05.1548723Z Task         : Get sources
2020-04-06T21:35:05.1549021Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T21:35:05.1549461Z Version      : 1.0.0
2020-04-06T21:35:05.1549649Z Author       : Microsoft
---
2020-04-06T21:35:06.3311173Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-06T21:35:06.3318180Z ##[command]git config gc.auto 0
2020-04-06T21:35:06.3323008Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-06T21:35:06.3328103Z ##[command]git config --get-all http.proxy
2020-04-06T21:35:06.3339284Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70858/merge:refs/remotes/pull/70858/merge
2020-04-06T21:35:06.3858048Z fatal: couldn't find remote ref refs/pull/70858/merge
2020-04-06T21:35:06.4810549Z ##[warning]Git fetch failed with exit code 128, back off 9.981 seconds before retry.
2020-04-06T21:35:16.4085119Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70858/merge:refs/remotes/pull/70858/merge
2020-04-06T21:35:16.9032709Z fatal: couldn't find remote ref refs/pull/70858/merge
2020-04-06T21:35:16.9844087Z ##[warning]Git fetch failed with exit code 128, back off 3.279 seconds before retry.
2020-04-06T21:35:20.2113494Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70858/merge:refs/remotes/pull/70858/merge
2020-04-06T21:35:20.6839890Z fatal: couldn't find remote ref refs/pull/70858/merge
2020-04-06T21:35:20.7382476Z ##[error]Git fetch failed with exit code: 128
2020-04-06T21:35:20.7403376Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70858/merge to s
2020-04-06T21:35:20.8147205Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70858/merge to s
2020-04-06T21:35:20.8154257Z Task         : Get sources
2020-04-06T21:35:20.8154629Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T21:35:20.8154984Z Version      : 1.0.0
2020-04-06T21:35:20.8155217Z Author       : Microsoft
2020-04-06T21:35:20.8155217Z Author       : Microsoft
2020-04-06T21:35:20.8155593Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-06T21:35:20.8156015Z ==============================================================================
2020-04-06T21:35:21.1776838Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70858/merge to s
2020-04-06T21:35:21.1878896Z Cleaning up task key
2020-04-06T21:35:21.1880355Z Start cleaning up orphan processes.
2020-04-06T21:35:21.2104280Z ##[section]Finishing: Finalize Job
2020-04-06T21:35:21.2154692Z ##[section]Finishing: Linux x86_64-gnu-tools
