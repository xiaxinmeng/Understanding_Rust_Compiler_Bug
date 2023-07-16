plain
2019-08-12T08:04:17.4910607Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-12T08:04:17.5094067Z ##[command]git config gc.auto 0
2019-08-12T08:04:17.5156669Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-12T08:04:17.5201337Z ##[command]git config --get-all http.proxy
2019-08-12T08:04:17.5333411Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63485/merge:refs/remotes/pull/63485/merge
---
2019-08-12T08:04:53.5826810Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-12T08:04:53.5827250Z 
2019-08-12T08:04:53.5827872Z   git checkout -b <new-branch-name>
2019-08-12T08:04:53.5828120Z 
2019-08-12T08:04:53.5828334Z HEAD is now at 5977ed075 Merge fb127a17605ee47fe9689bee655184e08fc63e90 into 72f8043d44a8925e469daf5c10e2630c80c2a7d4
2019-08-12T08:04:53.5977872Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-12T08:04:53.5980306Z ==============================================================================
2019-08-12T08:04:53.5980354Z Task         : Bash
2019-08-12T08:04:53.5980411Z Description  : Run a Bash script on macOS, Linux, or Windows
