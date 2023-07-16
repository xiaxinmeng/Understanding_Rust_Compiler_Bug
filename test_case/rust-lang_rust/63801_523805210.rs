plain
2019-08-22T08:07:48.5857572Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-22T08:07:48.6055863Z ##[command]git config gc.auto 0
2019-08-22T08:07:48.6128887Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-22T08:07:48.6179561Z ##[command]git config --get-all http.proxy
2019-08-22T08:07:48.6304125Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63801/merge:refs/remotes/pull/63801/merge
2019-08-22T08:07:50.8805778Z remote:                                                                                         
---
2019-08-22T08:08:25.2764105Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-22T08:08:25.2765141Z 
2019-08-22T08:08:25.2766322Z   git checkout -b <new-branch-name>
2019-08-22T08:08:25.2767645Z 
2019-08-22T08:08:25.2768690Z HEAD is now at ff37b3b4d Merge 6ce242fb6e46bad46c64ae662091e18594521a4b into 42dcd4b7c5fb7b61bc2f4c0842f66e5ad40057e4
2019-08-22T08:08:25.2929067Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-22T08:08:25.2931815Z ==============================================================================
2019-08-22T08:08:25.2931860Z Task         : Bash
2019-08-22T08:08:25.2931908Z Description  : Run a Bash script on macOS, Linux, or Windows
