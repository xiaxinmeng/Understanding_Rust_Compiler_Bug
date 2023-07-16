plain
2019-07-21T18:52:50.2555436Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-21T18:52:50.2723782Z ##[command]git config gc.auto 0
2019-07-21T18:52:50.2800154Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-21T18:52:50.2853519Z ##[command]git config --get-all http.proxy
2019-07-21T18:52:50.2988280Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62856/merge:refs/remotes/pull/62856/merge
---
2019-07-21T18:53:24.3054243Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-21T18:53:24.3054277Z 
2019-07-21T18:53:24.3054526Z   git checkout -b <new-branch-name>
2019-07-21T18:53:24.3054559Z 
2019-07-21T18:53:24.3054614Z HEAD is now at ae9924726 Merge 18f3534b241a18e442df3d14fcefbe925b6a5133 into 83dfe7b27cf2debecebedd3b038f9a1c2e05e051
2019-07-21T18:53:24.3185709Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-21T18:53:24.3188785Z ==============================================================================
2019-07-21T18:53:24.3188845Z Task         : Bash
2019-07-21T18:53:24.3188891Z Description  : Run a Bash script on macOS, Linux, or Windows
