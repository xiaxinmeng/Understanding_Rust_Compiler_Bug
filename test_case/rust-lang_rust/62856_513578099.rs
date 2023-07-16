plain
2019-07-21T18:45:59.8107311Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-21T18:45:59.8323990Z ##[command]git config gc.auto 0
2019-07-21T18:45:59.8435843Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-21T18:45:59.8528096Z ##[command]git config --get-all http.proxy
2019-07-21T18:45:59.8683239Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62856/merge:refs/remotes/pull/62856/merge
---
2019-07-21T18:46:34.3287614Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-21T18:46:34.3288780Z 
2019-07-21T18:46:34.3289863Z   git checkout -b <new-branch-name>
2019-07-21T18:46:34.3290340Z 
2019-07-21T18:46:34.3290630Z HEAD is now at 2a8ebba54 Merge f82f329b5400a3159bad3123ff8be86d79368748 into 83dfe7b27cf2debecebedd3b038f9a1c2e05e051
2019-07-21T18:46:34.3433287Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-21T18:46:34.3436066Z ==============================================================================
2019-07-21T18:46:34.3436124Z Task         : Bash
2019-07-21T18:46:34.3436172Z Description  : Run a Bash script on macOS, Linux, or Windows
