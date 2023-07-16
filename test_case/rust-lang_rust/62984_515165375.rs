plain
2019-07-25T17:57:36.9718948Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-25T17:57:36.9917972Z ##[command]git config gc.auto 0
2019-07-25T17:57:37.0021529Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-25T17:57:37.0117833Z ##[command]git config --get-all http.proxy
2019-07-25T17:57:37.0273406Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62984/merge:refs/remotes/pull/62984/merge
---
2019-07-25T17:58:13.2661424Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T17:58:13.2662651Z 
2019-07-25T17:58:13.2664278Z   git checkout -b <new-branch-name>
2019-07-25T17:58:13.2666058Z 
2019-07-25T17:58:13.2668261Z HEAD is now at 067db91dc Merge 2716d6d80386d4a111586b6a5d4493477596a692 into eedf6ce4ef54bb03818ab21d714f1b9f13a6b31c
2019-07-25T17:58:13.2819110Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-25T17:58:13.2822024Z ==============================================================================
2019-07-25T17:58:13.2822079Z Task         : Bash
2019-07-25T17:58:13.2822123Z Description  : Run a Bash script on macOS, Linux, or Windows
