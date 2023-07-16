plain
2020-03-09T21:33:49.8521386Z Prepare build directory.
2020-03-09T21:33:49.8855236Z Set build variables.
2020-03-09T21:33:49.8889216Z Download all required tasks.
2020-03-09T21:33:49.9010025Z Downloading task: Bash (3.163.1)
2020-03-09T21:33:50.9701833Z Checking job knob settings.
2020-03-09T21:33:50.9724271Z Finished checking job knob settings.
2020-03-09T21:33:51.0357197Z ##[section]Finishing: Initialize job
2020-03-09T21:33:51.0696557Z ##[section]Starting: Configure Job Name
2020-03-09T21:33:51.0898876Z ==============================================================================
2020-03-09T21:33:51.0899623Z Task         : Bash
---
2020-03-09T21:33:52.4222767Z ========================== Starting Command Output ===========================
2020-03-09T21:33:52.4225545Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c7330566-4616-4dbd-ae9f-516df2491ff9.sh
2020-03-09T21:33:52.4225804Z 
2020-03-09T21:33:52.4230880Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-09T21:33:52.4248977Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69747/merge to s
2020-03-09T21:33:52.4252360Z Task         : Get sources
2020-03-09T21:33:52.4252619Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-09T21:33:52.4252902Z Version      : 1.0.0
2020-03-09T21:33:52.4253072Z Author       : Microsoft
---
2020-03-09T21:33:53.4345742Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-09T21:33:53.4352962Z ##[command]git config gc.auto 0
2020-03-09T21:33:53.4357321Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-09T21:33:53.4360777Z ##[command]git config --get-all http.proxy
2020-03-09T21:33:53.4367680Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69747/merge:refs/remotes/pull/69747/merge
