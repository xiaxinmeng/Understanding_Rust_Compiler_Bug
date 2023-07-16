plain
2019-10-30T08:15:43.7273674Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-30T08:15:43.7488177Z ##[command]git config gc.auto 0
2019-10-30T08:15:43.7568970Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-30T08:15:43.7642316Z ##[command]git config --get-all http.proxy
2019-10-30T08:15:43.7779793Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65955/merge:refs/remotes/pull/65955/merge
---
2019-10-30T08:22:07.1207925Z    Compiling serde_json v1.0.40
2019-10-30T08:22:09.1340323Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-30T08:22:21.0786748Z     Finished release [optimized] target(s) in 1m 30s
2019-10-30T08:22:21.0871331Z tidy check
2019-10-30T08:22:21.5535907Z tidy error: /checkout/src/ci/scripts/install-msys2.sh: ignoring line length unnecessarily
2019-10-30T08:22:23.7018221Z Found 485 error codes
2019-10-30T08:22:23.7019604Z Found 0 error codes with no tests
2019-10-30T08:22:23.7019692Z Done!
2019-10-30T08:22:23.7022868Z some tidy checks failed
2019-10-30T08:22:23.7022868Z some tidy checks failed
2019-10-30T08:22:23.7025933Z 
2019-10-30T08:22:23.7026595Z 
2019-10-30T08:22:23.7027528Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-30T08:22:23.7027916Z 
2019-10-30T08:22:23.7028029Z 
2019-10-30T08:22:23.7034491Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-30T08:22:23.7034712Z Build completed unsuccessfully in 0:01:34
2019-10-30T08:22:23.7034712Z Build completed unsuccessfully in 0:01:34
2019-10-30T08:22:23.7086683Z == clock drift check ==
2019-10-30T08:22:23.7096543Z   local time: Wed Oct 30 08:22:23 UTC 2019
2019-10-30T08:22:23.8606699Z   network time: Wed, 30 Oct 2019 08:22:23 GMT
2019-10-30T08:22:23.8610682Z == end clock drift check ==
2019-10-30T08:22:25.1786937Z 
2019-10-30T08:22:25.1893514Z ##[error]Bash exited with code '1'.
2019-10-30T08:22:25.1927653Z ##[section]Starting: Checkout
2019-10-30T08:22:25.1929877Z ==============================================================================
2019-10-30T08:22:25.1929933Z Task         : Get sources
2019-10-30T08:22:25.1930001Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
