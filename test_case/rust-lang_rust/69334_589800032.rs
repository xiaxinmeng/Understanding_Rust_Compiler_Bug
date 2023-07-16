plain
2020-02-21T17:31:43.4641094Z ========================== Starting Command Output ===========================
2020-02-21T17:31:43.4645260Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5ce49279-6ad8-4ed9-b312-81e6985caf36.sh
2020-02-21T17:31:43.4645705Z 
2020-02-21T17:31:43.4649725Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-21T17:31:43.4670589Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69334/merge to s
2020-02-21T17:31:43.4673981Z Task         : Get sources
2020-02-21T17:31:43.4674535Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-21T17:31:43.4674876Z Version      : 1.0.0
2020-02-21T17:31:43.4675089Z Author       : Microsoft
---
2020-02-21T17:31:44.4619701Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-21T17:31:44.4625275Z ##[command]git config gc.auto 0
2020-02-21T17:31:44.4628994Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-21T17:31:44.4632466Z ##[command]git config --get-all http.proxy
2020-02-21T17:31:44.4639717Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69334/merge:refs/remotes/pull/69334/merge
---
2020-02-21T17:33:22.3338664Z tar: Exiting with failure status due to previous errors
2020-02-21T17:33:22.3368144Z 
2020-02-21T17:33:22.3464731Z ##[error]Bash exited with code '2'.
2020-02-21T17:33:22.3480325Z ##[section]Finishing: Install awscli
2020-02-21T17:33:22.3554919Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69334/merge to s
2020-02-21T17:33:22.3562101Z Task         : Get sources
2020-02-21T17:33:22.3562952Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-21T17:33:22.3563311Z Version      : 1.0.0
2020-02-21T17:33:22.3563550Z Author       : Microsoft
2020-02-21T17:33:22.3563550Z Author       : Microsoft
2020-02-21T17:33:22.3563938Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-21T17:33:22.3564366Z ==============================================================================
2020-02-21T17:33:22.6836966Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-21T17:33:22.6893325Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69334/merge to s
2020-02-21T17:33:22.6983208Z Cleaning up task key
2020-02-21T17:33:22.6984505Z Start cleaning up orphan processes.
2020-02-21T17:33:22.7253289Z Terminate orphan process: pid (3831) (python)
2020-02-21T17:33:22.7290084Z ##[section]Finishing: Finalize Job
