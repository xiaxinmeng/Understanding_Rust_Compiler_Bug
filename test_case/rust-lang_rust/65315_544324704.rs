plain
2019-10-21T02:09:54.6767624Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-21T02:09:54.6941563Z ##[command]git config gc.auto 0
2019-10-21T02:09:54.7006571Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-21T02:09:54.7049793Z ##[command]git config --get-all http.proxy
2019-10-21T02:09:54.7176383Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65315/merge:refs/remotes/pull/65315/merge
---
2019-10-21T02:15:54.1579712Z    Compiling serde_json v1.0.40
2019-10-21T02:15:55.8448125Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-21T02:16:06.8851578Z     Finished release [optimized] target(s) in 1m 24s
2019-10-21T02:16:06.8932452Z tidy check
2019-10-21T02:16:07.3095566Z tidy error: /checkout/src/librustc_mir/transform/copy_prop.rs:135: line longer than 100 chars
2019-10-21T02:16:07.3103452Z tidy error: /checkout/src/librustc_mir/transform/rustc_peek.rs:213: line longer than 100 chars
2019-10-21T02:16:07.3103636Z tidy error: /checkout/src/librustc_mir/transform/rustc_peek.rs:217: line longer than 100 chars
2019-10-21T02:16:07.3127611Z tidy error: /checkout/src/librustc_mir/transform/check_consts/validation.rs:397: line longer than 100 chars
2019-10-21T02:16:07.3154360Z tidy error: /checkout/src/librustc_mir/util/def_use.rs:51: line longer than 100 chars
2019-10-21T02:16:07.3154456Z tidy error: /checkout/src/librustc_mir/util/def_use.rs:127: line longer than 100 chars
2019-10-21T02:16:07.3198400Z tidy error: /checkout/src/librustc_mir/build/matches/util.rs:17: line longer than 100 chars
2019-10-21T02:16:07.3202025Z tidy error: /checkout/src/librustc_mir/build/matches/test.rs:756: line longer than 100 chars
2019-10-21T02:16:07.5127632Z tidy error: /checkout/src/librustc/ty/context.rs:2608: line longer than 100 chars
2019-10-21T02:16:07.5127812Z tidy error: /checkout/src/librustc/ty/context.rs:2615: line longer than 100 chars
2019-10-21T02:16:07.5128891Z tidy error: /checkout/src/librustc/ty/context.rs: too many lines (3029) (add `// ignore-tidy-filelength` to the file to suppress this error)
2019-10-21T02:16:08.8798522Z some tidy checks failed
2019-10-21T02:16:08.8800818Z Found 482 error codes
2019-10-21T02:16:08.8801541Z Found 0 error codes with no tests
2019-10-21T02:16:08.8801793Z Done!
2019-10-21T02:16:08.8801793Z Done!
2019-10-21T02:16:08.8802181Z 
2019-10-21T02:16:08.8802356Z 
2019-10-21T02:16:08.8803338Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-21T02:16:08.8804411Z 
2019-10-21T02:16:08.8804614Z 
2019-10-21T02:16:08.8810561Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-21T02:16:08.8810894Z Build completed unsuccessfully in 0:01:27
2019-10-21T02:16:08.8810894Z Build completed unsuccessfully in 0:01:27
2019-10-21T02:16:08.8858442Z == clock drift check ==
2019-10-21T02:16:08.8873393Z   local time: Mon Oct 21 02:16:08 UTC 2019
2019-10-21T02:16:09.0297794Z   network time: Mon, 21 Oct 2019 02:16:09 GMT
2019-10-21T02:16:09.0301036Z == end clock drift check ==
2019-10-21T02:16:10.4011116Z 
2019-10-21T02:16:10.4117359Z ##[error]Bash exited with code '1'.
2019-10-21T02:16:10.4155587Z ##[section]Starting: Checkout
2019-10-21T02:16:10.4157272Z ==============================================================================
2019-10-21T02:16:10.4157329Z Task         : Get sources
2019-10-21T02:16:10.4157382Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
