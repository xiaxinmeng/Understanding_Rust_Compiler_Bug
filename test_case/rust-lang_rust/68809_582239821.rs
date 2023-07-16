plain
2020-02-05T04:37:21.0663012Z ========================== Starting Command Output ===========================
2020-02-05T04:37:21.0665773Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7e462e2e-b1bc-42d0-91bb-672149a06716.sh
2020-02-05T04:37:21.0665810Z 
2020-02-05T04:37:21.0670863Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-05T04:37:21.0676641Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68809/merge to s
2020-02-05T04:37:21.0678514Z Task         : Get sources
2020-02-05T04:37:21.0678582Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-05T04:37:21.0678732Z Version      : 1.0.0
2020-02-05T04:37:21.0678771Z Author       : Microsoft
---
2020-02-05T04:37:21.8963367Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-05T04:37:21.9071221Z ##[command]git config gc.auto 0
2020-02-05T04:37:21.9106863Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-05T04:37:21.9152781Z ##[command]git config --get-all http.proxy
2020-02-05T04:37:21.9283810Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68809/merge:refs/remotes/pull/68809/merge
---
2020-02-05T04:40:38.0165042Z E: Unable to fetch some archives, maybe run apt-get update or try with --fix-missing?
2020-02-05T04:40:38.0208152Z 
2020-02-05T04:40:38.0322225Z ##[error]Bash exited with code '100'.
2020-02-05T04:40:38.0335205Z ##[section]Finishing: Install awscli
2020-02-05T04:40:38.0356356Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68809/merge to s
2020-02-05T04:40:38.0358076Z Task         : Get sources
2020-02-05T04:40:38.0358127Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-05T04:40:38.0358196Z Version      : 1.0.0
2020-02-05T04:40:38.0358241Z Author       : Microsoft
2020-02-05T04:40:38.0358241Z Author       : Microsoft
2020-02-05T04:40:38.0358291Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-05T04:40:38.0358362Z ==============================================================================
2020-02-05T04:40:38.4381860Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-05T04:40:38.4425004Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68809/merge to s
2020-02-05T04:40:38.4555636Z Cleaning up task key
2020-02-05T04:40:38.4556585Z Start cleaning up orphan processes.
2020-02-05T04:40:38.4690695Z Terminate orphan process: pid (5090) (python)
2020-02-05T04:40:38.4882654Z ##[section]Finishing: Finalize Job
