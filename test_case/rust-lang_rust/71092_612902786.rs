plain
2020-04-13T13:38:20.8168879Z ========================== Starting Command Output ===========================
2020-04-13T13:38:20.8172158Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6129d24d-441d-41d4-a9d0-09e180bb6a59.sh
2020-04-13T13:38:20.8172571Z 
2020-04-13T13:38:20.8177060Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-13T13:38:20.8196449Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71092/merge to s
2020-04-13T13:38:20.8199740Z Task         : Get sources
2020-04-13T13:38:20.8200070Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T13:38:20.8200371Z Version      : 1.0.0
2020-04-13T13:38:20.8200575Z Author       : Microsoft
---
2020-04-13T13:38:22.0660482Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-13T13:38:22.0667647Z ##[command]git config gc.auto 0
2020-04-13T13:38:22.0671336Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-13T13:38:22.0674728Z ##[command]git config --get-all http.proxy
2020-04-13T13:38:22.0683454Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71092/merge:refs/remotes/pull/71092/merge
---
2020-04-13T13:39:17.1937821Z E: Unable to fetch some archives, maybe run apt-get update or try with --fix-missing?
2020-04-13T13:39:17.1976755Z 
2020-04-13T13:39:17.2102990Z ##[error]Bash exited with code '100'.
2020-04-13T13:39:17.2147106Z ##[section]Finishing: Install awscli
2020-04-13T13:39:17.2236500Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71092/merge to s
2020-04-13T13:39:17.2241601Z Task         : Get sources
2020-04-13T13:39:17.2241963Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T13:39:17.2242312Z Version      : 1.0.0
2020-04-13T13:39:17.2242542Z Author       : Microsoft
2020-04-13T13:39:17.2242542Z Author       : Microsoft
2020-04-13T13:39:17.2242925Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-13T13:39:17.2243366Z ==============================================================================
2020-04-13T13:39:17.5519814Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-13T13:39:17.5562360Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71092/merge to s
2020-04-13T13:39:17.5644777Z Cleaning up task key
2020-04-13T13:39:17.5646065Z Start cleaning up orphan processes.
2020-04-13T13:39:17.5832898Z Terminate orphan process: pid (3710) (python)
2020-04-13T13:39:17.5946468Z ##[section]Finishing: Finalize Job
