plain
2020-02-21T22:45:13.8193451Z ========================== Starting Command Output ===========================
2020-02-21T22:45:13.8196547Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c27abfde-37d6-4761-8c36-bd0ae759c838.sh
2020-02-21T22:45:13.8197192Z 
2020-02-21T22:45:13.8202121Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-21T22:45:13.8222508Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69084/merge to s
2020-02-21T22:45:13.8225466Z Task         : Get sources
2020-02-21T22:45:13.8282809Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-21T22:45:13.8283263Z Version      : 1.0.0
2020-02-21T22:45:13.8283427Z Author       : Microsoft
---
2020-02-21T22:45:14.8308447Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-21T22:45:14.8317343Z ##[command]git config gc.auto 0
2020-02-21T22:45:14.8324291Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-21T22:45:14.8330862Z ##[command]git config --get-all http.proxy
2020-02-21T22:45:14.8339017Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69084/merge:refs/remotes/pull/69084/merge
