plain
2020-03-09T10:17:59.0008873Z ========================== Starting Command Output ===========================
2020-03-09T10:17:59.0012953Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2b210fb9-5366-497d-b26a-73f46e9695b6.sh
2020-03-09T10:17:59.0013499Z 
2020-03-09T10:17:59.0017536Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-09T10:17:59.0037620Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69076/merge to s
2020-03-09T10:17:59.0041306Z Task         : Get sources
2020-03-09T10:17:59.0041682Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-09T10:17:59.0041976Z Version      : 1.0.0
2020-03-09T10:17:59.0042173Z Author       : Microsoft
---
2020-03-09T10:18:00.0162228Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-09T10:18:00.0184372Z ##[command]git config gc.auto 0
2020-03-09T10:18:00.0192584Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-09T10:18:00.0200491Z ##[command]git config --get-all http.proxy
2020-03-09T10:18:00.0207999Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69076/merge:refs/remotes/pull/69076/merge
