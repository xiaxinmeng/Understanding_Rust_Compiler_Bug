plain
2019-07-27T15:51:20.2984022Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-27T15:51:20.3206259Z ##[command]git config gc.auto 0
2019-07-27T15:51:20.3262550Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-27T15:51:20.3322567Z ##[command]git config --get-all http.proxy
2019-07-27T15:51:20.3456124Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61511/merge:refs/remotes/pull/61511/merge
---
2019-07-27T15:51:54.0671496Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-27T15:51:54.0671708Z 
2019-07-27T15:51:54.0672114Z   git checkout -b <new-branch-name>
2019-07-27T15:51:54.0681900Z 
2019-07-27T15:51:54.0682383Z HEAD is now at 486a93803 Merge 421d9ea1bd462133341e0b906d60cd0fea82d3f3 into 0e9b465d729d07101b29b4d096d83edf9be82df0
2019-07-27T15:51:54.0824468Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-27T15:51:54.0827591Z ==============================================================================
2019-07-27T15:51:54.0827672Z Task         : Bash
2019-07-27T15:51:54.0827719Z Description  : Run a Bash script on macOS, Linux, or Windows
