plain
2019-10-30T00:47:45.4900913Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-30T00:47:45.5120733Z ##[command]git config gc.auto 0
2019-10-30T00:47:45.5184787Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-30T00:47:45.5239079Z ##[command]git config --get-all http.proxy
2019-10-30T00:47:45.5366975Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65951/merge:refs/remotes/pull/65951/merge
---
2019-10-30T00:54:37.9997032Z    Compiling serde_json v1.0.40
2019-10-30T00:54:40.1319743Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-30T00:54:52.9975619Z     Finished release [optimized] target(s) in 1m 35s
2019-10-30T00:54:53.0059972Z tidy check
2019-10-30T00:54:53.7614074Z tidy error: /checkout/src/librustc/infer/error_reporting/need_type_info.rs:237: line longer than 100 chars
2019-10-30T00:54:55.6377833Z some tidy checks failed
2019-10-30T00:54:55.6378738Z Found 485 error codes
2019-10-30T00:54:55.6378858Z Found 0 error codes with no tests
2019-10-30T00:54:55.6378905Z Done!
2019-10-30T00:54:55.6378905Z Done!
2019-10-30T00:54:55.6381445Z 
2019-10-30T00:54:55.6381855Z 
2019-10-30T00:54:55.6383980Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-30T00:54:55.6385241Z 
2019-10-30T00:54:55.6385371Z 
2019-10-30T00:54:55.6394435Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-30T00:54:55.6394674Z Build completed unsuccessfully in 0:01:39
2019-10-30T00:54:55.6394674Z Build completed unsuccessfully in 0:01:39
2019-10-30T00:54:55.6452609Z == clock drift check ==
2019-10-30T00:54:55.6463803Z   local time: Wed Oct 30 00:54:55 UTC 2019
2019-10-30T00:54:55.7508315Z   network time: Wed, 30 Oct 2019 00:54:55 GMT
2019-10-30T00:54:55.7515528Z == end clock drift check ==
2019-10-30T00:54:57.0841697Z 
2019-10-30T00:54:57.0952153Z ##[error]Bash exited with code '1'.
2019-10-30T00:54:57.0983712Z ##[section]Starting: Checkout
2019-10-30T00:54:57.0985138Z ==============================================================================
2019-10-30T00:54:57.0985201Z Task         : Get sources
2019-10-30T00:54:57.0985241Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
