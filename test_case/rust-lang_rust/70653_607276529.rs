plain
2020-04-01T12:33:00.3587852Z ========================== Starting Command Output ===========================
2020-04-01T12:33:00.3590201Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/673fa107-69ae-44bf-96df-54a165936421.sh
2020-04-01T12:33:00.3590433Z 
2020-04-01T12:33:00.3593447Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-01T12:33:00.3611291Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70653/merge to s
2020-04-01T12:33:00.3614534Z Task         : Get sources
2020-04-01T12:33:00.3614800Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-01T12:33:00.3615057Z Version      : 1.0.0
2020-04-01T12:33:00.3615231Z Author       : Microsoft
---
2020-04-01T12:33:01.5570746Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-01T12:33:01.5583641Z ##[command]git config gc.auto 0
2020-04-01T12:33:01.5590966Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-01T12:33:01.5596435Z ##[command]git config --get-all http.proxy
2020-04-01T12:33:01.5613315Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70653/merge:refs/remotes/pull/70653/merge
---
2020-04-01T12:34:20.8748255Z E: Unable to fetch some archives, maybe run apt-get update or try with --fix-missing?
2020-04-01T12:34:20.8791433Z 
2020-04-01T12:34:20.8874137Z ##[error]Bash exited with code '100'.
2020-04-01T12:34:20.8888282Z ##[section]Finishing: Install awscli
2020-04-01T12:34:20.8954770Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70653/merge to s
2020-04-01T12:34:20.8959599Z Task         : Get sources
2020-04-01T12:34:20.8959963Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-01T12:34:20.8960277Z Version      : 1.0.0
2020-04-01T12:34:20.8960499Z Author       : Microsoft
2020-04-01T12:34:20.8960499Z Author       : Microsoft
2020-04-01T12:34:20.8960874Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-01T12:34:20.8961282Z ==============================================================================
2020-04-01T12:34:21.2452384Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-01T12:34:21.2514782Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70653/merge to s
2020-04-01T12:34:21.2616437Z Cleaning up task key
2020-04-01T12:34:21.2617718Z Start cleaning up orphan processes.
2020-04-01T12:34:21.2804140Z Terminate orphan process: pid (3672) (python)
2020-04-01T12:34:21.3017358Z ##[section]Finishing: Finalize Job
