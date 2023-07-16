plain
2019-10-20T18:06:04.0858664Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-20T18:06:04.1020033Z ##[command]git config gc.auto 0
2019-10-20T18:06:04.1099826Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-20T18:06:04.1148869Z ##[command]git config --get-all http.proxy
2019-10-20T18:06:04.1305096Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65630/merge:refs/remotes/pull/65630/merge
---
2019-10-20T18:12:28.5049834Z    Compiling serde_json v1.0.40
2019-10-20T18:12:30.3351734Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-20T18:12:41.7830511Z     Finished release [optimized] target(s) in 1m 29s
2019-10-20T18:12:41.7913749Z tidy check
2019-10-20T18:12:42.8833897Z tidy error: /checkout/src/test/ui/consts/const-eval/issue-65394.rs:5: `borrowck_graphviz_postflow` attribute in test
2019-10-20T18:12:44.2095976Z some tidy checks failed
2019-10-20T18:12:44.2099861Z Found 482 error codes
2019-10-20T18:12:44.2100326Z Found 0 error codes with no tests
2019-10-20T18:12:44.2101008Z Done!
2019-10-20T18:12:44.2101008Z Done!
2019-10-20T18:12:44.2101279Z 
2019-10-20T18:12:44.2101483Z 
2019-10-20T18:12:44.2102722Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-20T18:12:44.2105033Z 
2019-10-20T18:12:44.2105287Z 
2019-10-20T18:12:44.2105567Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-20T18:12:44.2105761Z Build completed unsuccessfully in 0:01:32
2019-10-20T18:12:44.2105761Z Build completed unsuccessfully in 0:01:32
2019-10-20T18:12:44.2157023Z == clock drift check ==
2019-10-20T18:12:44.2172212Z   local time: Sun Oct 20 18:12:44 UTC 2019
2019-10-20T18:12:44.3816839Z   network time: Sun, 20 Oct 2019 18:12:44 GMT
2019-10-20T18:12:44.3817905Z == end clock drift check ==
2019-10-20T18:12:45.7020683Z 
2019-10-20T18:12:45.7129000Z ##[error]Bash exited with code '1'.
2019-10-20T18:12:45.7162568Z ##[section]Starting: Checkout
2019-10-20T18:12:45.7164165Z ==============================================================================
2019-10-20T18:12:45.7164210Z Task         : Get sources
2019-10-20T18:12:45.7164248Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
