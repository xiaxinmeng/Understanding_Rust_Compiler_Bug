plain
2019-10-23T08:48:26.2997671Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-23T08:48:26.3242118Z ##[command]git config gc.auto 0
2019-10-23T08:48:26.3325912Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-23T08:48:26.3382004Z ##[command]git config --get-all http.proxy
2019-10-23T08:48:26.3541441Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65405/merge:refs/remotes/pull/65405/merge
---
2019-10-23T08:54:50.0276431Z    Compiling serde_json v1.0.40
2019-10-23T08:54:51.9274902Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-23T08:55:04.2346186Z     Finished release [optimized] target(s) in 1m 34s
2019-10-23T08:55:04.2435676Z tidy check
2019-10-23T08:55:05.4915209Z tidy error: duplicate error code: 740
2019-10-23T08:55:05.4916183Z tidy error: /checkout/src/librustc_resolve/error_codes.rs:1909: E0740: r##"
2019-10-23T08:55:05.4916476Z tidy error: /checkout/src/librustc_typeck/error_codes.rs:4866: E0740: r##"
2019-10-23T08:55:06.8853889Z Found 483 error codes
2019-10-23T08:55:06.8854021Z Found 0 error codes with no tests
2019-10-23T08:55:06.8854199Z Done!
2019-10-23T08:55:06.8859619Z some tidy checks failed
2019-10-23T08:55:06.8859619Z some tidy checks failed
2019-10-23T08:55:06.8860394Z 
2019-10-23T08:55:06.8866021Z 
2019-10-23T08:55:06.8867236Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-23T08:55:06.8867910Z 
2019-10-23T08:55:06.8868100Z 
2019-10-23T08:55:06.8868366Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-23T08:55:06.8868534Z Build completed unsuccessfully in 0:01:38
2019-10-23T08:55:06.8868534Z Build completed unsuccessfully in 0:01:38
2019-10-23T08:55:06.8925742Z == clock drift check ==
2019-10-23T08:55:06.8958716Z   local time: Wed Oct 23 08:55:06 UTC 2019
2019-10-23T08:55:07.1740317Z   network time: Wed, 23 Oct 2019 08:55:07 GMT
2019-10-23T08:55:07.1743489Z == end clock drift check ==
2019-10-23T08:55:09.1765666Z 
2019-10-23T08:55:09.1880875Z ##[error]Bash exited with code '1'.
2019-10-23T08:55:09.1914069Z ##[section]Starting: Checkout
2019-10-23T08:55:09.1916150Z ==============================================================================
2019-10-23T08:55:09.1916222Z Task         : Get sources
2019-10-23T08:55:09.1916285Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
