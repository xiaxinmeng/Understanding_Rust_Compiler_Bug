plain
2020-04-06T22:35:04.1790806Z ========================== Starting Command Output ===========================
2020-04-06T22:35:04.1793177Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e9c8ff20-88a1-432f-b306-441f76b64397.sh
2020-04-06T22:35:04.1793381Z 
2020-04-06T22:35:04.1799877Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-06T22:35:04.1818067Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70864/merge to s
2020-04-06T22:35:04.1821338Z Task         : Get sources
2020-04-06T22:35:04.1821561Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T22:35:04.1821773Z Version      : 1.0.0
2020-04-06T22:35:04.1821918Z Author       : Microsoft
---
2020-04-06T22:35:05.1756274Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-06T22:35:05.1762537Z ##[command]git config gc.auto 0
2020-04-06T22:35:05.1768126Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-06T22:35:05.1772177Z ##[command]git config --get-all http.proxy
2020-04-06T22:35:05.1780136Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70864/merge:refs/remotes/pull/70864/merge
2020-04-06T22:35:05.2254155Z fatal: couldn't find remote ref refs/pull/70864/merge
2020-04-06T22:35:05.2986238Z ##[warning]Git fetch failed with exit code 128, back off 8.419 seconds before retry.
2020-04-06T22:35:13.6839988Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70864/merge:refs/remotes/pull/70864/merge
2020-04-06T22:35:14.1185402Z fatal: couldn't find remote ref refs/pull/70864/merge
2020-04-06T22:35:14.1949657Z ##[warning]Git fetch failed with exit code 128, back off 3.602 seconds before retry.
2020-04-06T22:35:17.7490655Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70864/merge:refs/remotes/pull/70864/merge
2020-04-06T22:35:18.1890567Z fatal: couldn't find remote ref refs/pull/70864/merge
2020-04-06T22:35:18.2420329Z ##[error]Git fetch failed with exit code: 128
2020-04-06T22:35:18.2435961Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70864/merge to s
2020-04-06T22:35:18.2998290Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70864/merge to s
2020-04-06T22:35:18.3003748Z Task         : Get sources
2020-04-06T22:35:18.3004059Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T22:35:18.3004357Z Version      : 1.0.0
2020-04-06T22:35:18.3004547Z Author       : Microsoft
2020-04-06T22:35:18.3004547Z Author       : Microsoft
2020-04-06T22:35:18.3004854Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-06T22:35:18.3005216Z ==============================================================================
2020-04-06T22:35:18.5878700Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70864/merge to s
2020-04-06T22:35:18.5956304Z Cleaning up task key
2020-04-06T22:35:18.5957309Z Start cleaning up orphan processes.
2020-04-06T22:35:18.6078721Z ##[section]Finishing: Finalize Job
2020-04-06T22:35:18.6118611Z ##[section]Finishing: Linux x86_64-gnu-tools
