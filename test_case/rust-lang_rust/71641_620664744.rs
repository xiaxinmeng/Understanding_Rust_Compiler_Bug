plain
2020-04-28T14:48:11.6821787Z ========================== Starting Command Output ===========================
2020-04-28T14:48:11.6824919Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b636010f-c5b1-49e4-bbd1-07c1e1d4b1b0.sh
2020-04-28T14:48:11.6825191Z 
2020-04-28T14:48:11.6830536Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-28T14:48:11.6855205Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71641/merge to s
2020-04-28T14:48:11.6866387Z Task         : Get sources
2020-04-28T14:48:11.6866694Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-28T14:48:11.6866984Z Version      : 1.0.0
2020-04-28T14:48:11.6867176Z Author       : Microsoft
---
2020-04-28T14:48:12.6828939Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-28T14:48:12.6834574Z ##[command]git config gc.auto 0
2020-04-28T14:48:12.6839000Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-28T14:48:12.6843566Z ##[command]git config --get-all http.proxy
2020-04-28T14:48:12.6856231Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71641/merge:refs/remotes/pull/71641/merge
---
2020-04-28T14:50:30.7391770Z E: Unable to fetch some archives, maybe run apt-get update or try with --fix-missing?
2020-04-28T14:50:30.7438224Z 
2020-04-28T14:50:30.7526887Z ##[error]Bash exited with code '100'.
2020-04-28T14:50:30.7541444Z ##[section]Finishing: Install awscli
2020-04-28T14:50:30.7599044Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71641/merge to s
2020-04-28T14:50:30.7603801Z Task         : Get sources
2020-04-28T14:50:30.7604130Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-28T14:50:30.7604435Z Version      : 1.0.0
2020-04-28T14:50:30.7604639Z Author       : Microsoft
2020-04-28T14:50:30.7604639Z Author       : Microsoft
2020-04-28T14:50:30.7604963Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-28T14:50:30.7605358Z ==============================================================================
2020-04-28T14:50:31.1581953Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-28T14:50:31.1634081Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71641/merge to s
2020-04-28T14:50:31.1782499Z Cleaning up task key
2020-04-28T14:50:31.1783827Z Start cleaning up orphan processes.
2020-04-28T14:50:31.2006460Z Terminate orphan process: pid (3749) (python)
2020-04-28T14:50:31.2153809Z ##[section]Finishing: Finalize Job
