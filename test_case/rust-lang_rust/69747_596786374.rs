plain
2020-03-09T21:20:32.7158765Z Prepare build directory.
2020-03-09T21:20:32.7523171Z Set build variables.
2020-03-09T21:20:32.7560771Z Download all required tasks.
2020-03-09T21:20:32.7699288Z Downloading task: Bash (3.163.1)
2020-03-09T21:20:33.7266086Z Checking job knob settings.
2020-03-09T21:20:33.7289905Z Finished checking job knob settings.
2020-03-09T21:20:33.7898932Z ##[section]Finishing: Initialize job
2020-03-09T21:20:33.8268591Z ##[section]Starting: Configure Job Name
2020-03-09T21:20:33.8502645Z ==============================================================================
2020-03-09T21:20:33.8503437Z Task         : Bash
---
2020-03-09T21:20:35.1747156Z ========================== Starting Command Output ===========================
2020-03-09T21:20:35.1750466Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2e558ba0-15b9-4feb-9a4f-b56cea4576d3.sh
2020-03-09T21:20:35.1750963Z 
2020-03-09T21:20:35.1755895Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-09T21:20:35.1775641Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69747/merge to s
2020-03-09T21:20:35.1779090Z Task         : Get sources
2020-03-09T21:20:35.1779506Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-09T21:20:35.1779797Z Version      : 1.0.0
2020-03-09T21:20:35.1779995Z Author       : Microsoft
---
2020-03-09T21:20:36.1707927Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-09T21:20:36.1713702Z ##[command]git config gc.auto 0
2020-03-09T21:20:36.1717401Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-09T21:20:36.1720784Z ##[command]git config --get-all http.proxy
2020-03-09T21:20:36.1727028Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69747/merge:refs/remotes/pull/69747/merge
