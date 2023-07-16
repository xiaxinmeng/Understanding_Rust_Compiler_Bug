plain
2020-03-26T18:44:53.9193819Z ========================== Starting Command Output ===========================
2020-03-26T18:44:53.9197310Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/bd6abcf3-6602-4398-8ff5-22a55959e35f.sh
2020-03-26T18:44:53.9198071Z 
2020-03-26T18:44:53.9202559Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-26T18:44:53.9220060Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-03-26T18:44:53.9223175Z Task         : Get sources
2020-03-26T18:44:53.9223486Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-26T18:44:53.9223766Z Version      : 1.0.0
2020-03-26T18:44:53.9223956Z Author       : Microsoft
---
2020-03-26T18:44:55.1695714Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-26T18:44:55.1703678Z ##[command]git config gc.auto 0
2020-03-26T18:44:55.1708691Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-26T18:44:55.1713491Z ##[command]git config --get-all http.proxy
2020-03-26T18:44:55.1722419Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69274/merge:refs/remotes/pull/69274/merge
---
2020-03-26T18:46:26.4745876Z E: Unable to fetch some archives, maybe run apt-get update or try with --fix-missing?
2020-03-26T18:46:26.4785819Z 
2020-03-26T18:46:26.4848617Z ##[error]Bash exited with code '100'.
2020-03-26T18:46:26.4864771Z ##[section]Finishing: Install awscli
2020-03-26T18:46:26.4949287Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-03-26T18:46:26.4956016Z Task         : Get sources
2020-03-26T18:46:26.4956443Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-26T18:46:26.4956837Z Version      : 1.0.0
2020-03-26T18:46:26.4957110Z Author       : Microsoft
2020-03-26T18:46:26.4957110Z Author       : Microsoft
2020-03-26T18:46:26.4957530Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-26T18:46:26.4958192Z ==============================================================================
2020-03-26T18:46:26.7940844Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-26T18:46:26.7983195Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-03-26T18:46:26.8068449Z Cleaning up task key
2020-03-26T18:46:26.8069766Z Start cleaning up orphan processes.
2020-03-26T18:46:26.8254387Z Terminate orphan process: pid (3729) (python)
2020-03-26T18:46:26.8397157Z ##[section]Finishing: Finalize Job
