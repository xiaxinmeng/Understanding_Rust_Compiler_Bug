plain
2019-07-26T20:47:03.6141427Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-26T20:47:03.6319647Z ##[command]git config gc.auto 0
2019-07-26T20:47:03.6392858Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-26T20:47:03.6449775Z ##[command]git config --get-all http.proxy
2019-07-26T20:47:03.6591384Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63025/merge:refs/remotes/pull/63025/merge
---
2019-07-26T20:47:39.9118929Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T20:47:39.9118976Z 
2019-07-26T20:47:39.9119310Z   git checkout -b <new-branch-name>
2019-07-26T20:47:39.9119344Z 
2019-07-26T20:47:39.9119418Z HEAD is now at 7f11f0725 Merge f12327b565316c168150a075317b67457b26dafd into c43753f910aae000f8bcb0a502407ea332afc74b
2019-07-26T20:47:39.9267667Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-26T20:47:39.9317883Z ==============================================================================
2019-07-26T20:47:39.9317950Z Task         : Bash
2019-07-26T20:47:39.9318016Z Description  : Run a Bash script on macOS, Linux, or Windows
