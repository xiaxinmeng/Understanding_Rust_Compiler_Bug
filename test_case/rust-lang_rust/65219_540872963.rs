plain
2019-10-11T02:08:19.9285545Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-11T02:08:19.9605255Z ##[command]git config gc.auto 0
2019-10-11T02:08:19.9675613Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-11T02:08:19.9735881Z ##[command]git config --get-all http.proxy
2019-10-11T02:08:19.9915471Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65219/merge:refs/remotes/pull/65219/merge
---
2019-10-11T02:16:04.4928728Z    Compiling serde_json v1.0.40
2019-10-11T02:16:06.4186430Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-11T02:16:18.6636761Z     Finished release [optimized] target(s) in 1m 34s
2019-10-11T02:16:18.6727655Z tidy check
2019-10-11T02:16:19.5265793Z tidy error: /checkout/src/libcore/iter/traits/iterator.rs: too many lines (3002) (add `// ignore-tidy-filelength` to the file to suppress this error)
2019-10-11T02:16:21.1578876Z some tidy checks failed
2019-10-11T02:16:21.1587196Z Found 482 error codes
2019-10-11T02:16:21.1589474Z Found 0 error codes with no tests
2019-10-11T02:16:21.1592118Z Done!
2019-10-11T02:16:21.1592118Z Done!
2019-10-11T02:16:21.1592341Z 
2019-10-11T02:16:21.1592513Z 
2019-10-11T02:16:21.1595633Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-11T02:16:21.1596546Z 
2019-10-11T02:16:21.1596803Z 
2019-10-11T02:16:21.1600861Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-11T02:16:21.1601249Z Build completed unsuccessfully in 0:01:38
2019-10-11T02:16:21.1601249Z Build completed unsuccessfully in 0:01:38
2019-10-11T02:16:21.1662536Z == clock drift check ==
2019-10-11T02:16:21.1676919Z   local time: Fri Oct 11 02:16:21 UTC 2019
2019-10-11T02:16:21.3262924Z   network time: Fri, 11 Oct 2019 02:16:21 GMT
2019-10-11T02:16:21.3263059Z == end clock drift check ==
2019-10-11T02:16:22.3016641Z ##[error]Bash exited with code '1'.
2019-10-11T02:16:22.3050076Z ##[section]Starting: Checkout
2019-10-11T02:16:22.3051946Z ==============================================================================
2019-10-11T02:16:22.3052001Z Task         : Get sources
2019-10-11T02:16:22.3052046Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
