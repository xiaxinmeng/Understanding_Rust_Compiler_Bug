plain
2020-03-09T22:50:57.9656240Z Prepare build directory.
2020-03-09T22:50:58.0070934Z Set build variables.
2020-03-09T22:50:58.0108032Z Download all required tasks.
2020-03-09T22:50:58.0243867Z Downloading task: Bash (3.163.1)
2020-03-09T22:50:58.9132643Z Checking job knob settings.
2020-03-09T22:50:58.9154999Z Finished checking job knob settings.
2020-03-09T22:50:58.9752590Z ##[section]Finishing: Initialize job
2020-03-09T22:50:59.0133051Z ##[section]Starting: Configure Job Name
2020-03-09T22:50:59.1327334Z ==============================================================================
2020-03-09T22:50:59.1328234Z Task         : Bash
---
2020-03-09T22:51:00.7893516Z ========================== Starting Command Output ===========================
2020-03-09T22:51:00.7899432Z [command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/ab5b0663-e9f3-4958-8da0-95eed2dde931.sh
2020-03-09T22:51:00.8908290Z 
2020-03-09T22:51:00.8985574Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-09T22:51:00.9094538Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69872/merge to s
2020-03-09T22:51:00.9193822Z Task         : Get sources
2020-03-09T22:51:00.9194126Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-09T22:51:00.9194487Z Version      : 1.0.0
2020-03-09T22:51:00.9194682Z Author       : Microsoft
---
2020-03-09T22:51:03.3269064Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-09T22:51:03.4055616Z ##[command]git config gc.auto 0
2020-03-09T22:51:03.4596455Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-09T22:51:03.4970214Z ##[command]git config --get-all http.proxy
2020-03-09T22:51:03.5409890Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69872/merge:refs/remotes/pull/69872/merge
