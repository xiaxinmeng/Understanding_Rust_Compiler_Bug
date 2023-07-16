plain
2019-08-10T14:38:48.8100217Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-10T14:38:48.8283982Z ##[command]git config gc.auto 0
2019-08-10T14:38:48.8352433Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-10T14:38:48.8414626Z ##[command]git config --get-all http.proxy
2019-08-10T14:38:48.8549249Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63440/merge:refs/remotes/pull/63440/merge
---
2019-08-10T14:39:22.9781529Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-10T14:39:22.9781581Z 
2019-08-10T14:39:22.9781830Z   git checkout -b <new-branch-name>
2019-08-10T14:39:22.9781858Z 
2019-08-10T14:39:22.9781906Z HEAD is now at f363fda2a Merge 440a5c810029088649918d738169f8e0bb65bb35 into 6f70adcb18e5dc8df0672898a8404fd05a9c32cb
2019-08-10T14:39:22.9953696Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-10T14:39:22.9957001Z ==============================================================================
2019-08-10T14:39:22.9957061Z Task         : Bash
2019-08-10T14:39:22.9957107Z Description  : Run a Bash script on macOS, Linux, or Windows
