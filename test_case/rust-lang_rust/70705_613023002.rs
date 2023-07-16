plain
2020-04-13T17:55:27.7535738Z ========================== Starting Command Output ===========================
2020-04-13T17:55:27.7541354Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a319e94f-8493-4d88-b1d1-f20a0fcb59b8.sh
2020-04-13T17:55:27.7541556Z 
2020-04-13T17:55:27.7546067Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-13T17:55:27.7565143Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-13T17:55:27.7569539Z Task         : Get sources
2020-04-13T17:55:27.7569753Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T17:55:27.7570019Z Version      : 1.0.0
2020-04-13T17:55:27.7570163Z Author       : Microsoft
---
2020-04-13T17:55:28.7606965Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-13T17:55:28.7612532Z ##[command]git config gc.auto 0
2020-04-13T17:55:28.7615607Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-13T17:55:28.7618728Z ##[command]git config --get-all http.proxy
2020-04-13T17:55:28.7624943Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70705/merge:refs/remotes/pull/70705/merge
---
2020-04-13T17:57:23.7359700Z E: Unable to fetch some archives, maybe run apt-get update or try with --fix-missing?
2020-04-13T17:57:23.7408203Z 
2020-04-13T17:57:23.7493003Z ##[error]Bash exited with code '100'.
2020-04-13T17:57:23.7508891Z ##[section]Finishing: Install awscli
2020-04-13T17:57:23.7568017Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-13T17:57:23.7572385Z Task         : Get sources
2020-04-13T17:57:23.7572677Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T17:57:23.7573114Z Version      : 1.0.0
2020-04-13T17:57:23.7573313Z Author       : Microsoft
2020-04-13T17:57:23.7573313Z Author       : Microsoft
2020-04-13T17:57:23.7573613Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-13T17:57:23.7573980Z ==============================================================================
2020-04-13T17:57:24.1016460Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-13T17:57:24.1062215Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-13T17:57:24.1160264Z Cleaning up task key
2020-04-13T17:57:24.1161312Z Start cleaning up orphan processes.
2020-04-13T17:57:24.1419962Z Terminate orphan process: pid (3370) (python)
2020-04-13T17:57:24.1537749Z ##[section]Finishing: Finalize Job
