plain
2020-02-26T10:21:01.2357032Z ========================== Starting Command Output ===========================
2020-02-26T10:21:01.2360195Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f6ec0bca-acd1-42d6-8f70-02ad124295c9.sh
2020-02-26T10:21:01.2360605Z 
2020-02-26T10:21:01.2365062Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-26T10:21:01.2382285Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-02-26T10:21:01.2384974Z Task         : Get sources
2020-02-26T10:21:01.2385226Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-26T10:21:01.2385485Z Version      : 1.0.0
2020-02-26T10:21:01.2385647Z Author       : Microsoft
---
2020-02-26T10:21:02.2331884Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-26T10:21:02.2337193Z ##[command]git config gc.auto 0
2020-02-26T10:21:02.2340627Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-26T10:21:02.2343693Z ##[command]git config --get-all http.proxy
2020-02-26T10:21:02.2349247Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69478/merge:refs/remotes/pull/69478/merge
2020-02-26T10:21:02.6789524Z fatal: couldn't find remote ref refs/pull/69478/merge
2020-02-26T10:21:02.7414737Z ##[warning]Git fetch failed with exit code 128, back off 7.043 seconds before retry.
2020-02-26T10:21:09.7308754Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69478/merge:refs/remotes/pull/69478/merge
2020-02-26T10:21:10.3253502Z fatal: couldn't find remote ref refs/pull/69478/merge
2020-02-26T10:21:10.3898315Z ##[warning]Git fetch failed with exit code 128, back off 1.498 seconds before retry.
2020-02-26T10:21:11.8349278Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69478/merge:refs/remotes/pull/69478/merge
2020-02-26T10:21:12.4499751Z fatal: couldn't find remote ref refs/pull/69478/merge
2020-02-26T10:21:12.5092850Z ##[error]Git fetch failed with exit code: 128
2020-02-26T10:21:12.5106301Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-02-26T10:21:12.5499290Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-02-26T10:21:12.5504483Z Task         : Get sources
2020-02-26T10:21:12.5504814Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-26T10:21:12.5505145Z Version      : 1.0.0
2020-02-26T10:21:12.5505373Z Author       : Microsoft
2020-02-26T10:21:12.5505373Z Author       : Microsoft
2020-02-26T10:21:12.5505706Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-26T10:21:12.5506096Z ==============================================================================
2020-02-26T10:21:12.8045201Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-02-26T10:21:12.8119378Z Cleaning up task key
2020-02-26T10:21:12.8120721Z Start cleaning up orphan processes.
2020-02-26T10:21:12.8238420Z ##[section]Finishing: Finalize Job
2020-02-26T10:21:12.8277157Z ##[section]Finishing: Linux x86_64-gnu-tools
