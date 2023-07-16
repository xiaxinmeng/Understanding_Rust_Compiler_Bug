plain
2019-08-13T21:05:29.6285889Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-13T21:05:29.6299092Z ##[command]git config gc.auto 0
2019-08-13T21:05:30.6272037Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-13T21:05:30.6277301Z ##[command]git config --get-all http.proxy
2019-08-13T21:05:30.6282982Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63470/merge:refs/remotes/pull/63470/merge
---
2019-08-13T21:06:06.3462857Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-13T21:06:06.3462885Z 
2019-08-13T21:06:06.3463225Z   git checkout -b <new-branch-name>
2019-08-13T21:06:06.3463251Z 
2019-08-13T21:06:06.3463317Z HEAD is now at 13221cb90 Merge 417f9ea90cf9bcccd8a1fa569a11a4fc071e3b8c into 60960a260f7b5c695fd0717311d72ce62dd4eb43
2019-08-13T21:06:06.3629640Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-13T21:06:06.3632375Z ==============================================================================
2019-08-13T21:06:06.3632642Z Task         : Bash
2019-08-13T21:06:06.3632703Z Description  : Run a Bash script on macOS, Linux, or Windows
