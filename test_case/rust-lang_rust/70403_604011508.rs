plain
2020-03-25T18:30:25.7227630Z ========================== Starting Command Output ===========================
2020-03-25T18:30:25.7230119Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/cd5ac80b-021c-4d8a-9f55-6bf0702b975b.sh
2020-03-25T18:30:25.7230399Z 
2020-03-25T18:30:25.7234790Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-25T18:30:25.7254209Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70403/merge to s
2020-03-25T18:30:25.7257396Z Task         : Get sources
2020-03-25T18:30:25.7257704Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-25T18:30:25.7258006Z Version      : 1.0.0
2020-03-25T18:30:25.7258205Z Author       : Microsoft
---
2020-03-25T18:30:26.7167435Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-25T18:30:26.7175321Z ##[command]git config gc.auto 0
2020-03-25T18:30:26.7181304Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-25T18:30:26.7186523Z ##[command]git config --get-all http.proxy
2020-03-25T18:30:26.7196760Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70403/merge:refs/remotes/pull/70403/merge
2020-03-25T18:30:27.0828151Z fatal: couldn't find remote ref refs/pull/70403/merge
2020-03-25T18:30:27.1610324Z ##[warning]Git fetch failed with exit code 128, back off 4.954 seconds before retry.
2020-03-25T18:30:32.0540650Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70403/merge:refs/remotes/pull/70403/merge
2020-03-25T18:30:32.7719402Z fatal: couldn't find remote ref refs/pull/70403/merge
2020-03-25T18:30:32.8341364Z ##[warning]Git fetch failed with exit code 128, back off 1.164 seconds before retry.
2020-03-25T18:30:33.9469831Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70403/merge:refs/remotes/pull/70403/merge
2020-03-25T18:30:34.5532509Z fatal: couldn't find remote ref refs/pull/70403/merge
2020-03-25T18:30:34.6097167Z ##[error]Git fetch failed with exit code: 128
2020-03-25T18:30:34.6109921Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70403/merge to s
2020-03-25T18:30:34.6527756Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70403/merge to s
2020-03-25T18:30:34.6533666Z Task         : Get sources
2020-03-25T18:30:34.6534029Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-25T18:30:34.6534378Z Version      : 1.0.0
2020-03-25T18:30:34.6534606Z Author       : Microsoft
2020-03-25T18:30:34.6534606Z Author       : Microsoft
2020-03-25T18:30:34.6534955Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-25T18:30:34.6535365Z ==============================================================================
2020-03-25T18:30:34.9569569Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70403/merge to s
2020-03-25T18:30:34.9656895Z Cleaning up task key
2020-03-25T18:30:34.9658229Z Start cleaning up orphan processes.
2020-03-25T18:30:34.9794518Z ##[section]Finishing: Finalize Job
2020-03-25T18:30:34.9834825Z ##[section]Finishing: Linux x86_64-gnu-tools
