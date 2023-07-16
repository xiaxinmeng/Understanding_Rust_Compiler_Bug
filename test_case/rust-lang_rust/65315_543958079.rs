plain
2019-10-18T21:10:57.7442495Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-18T21:10:57.7691507Z ##[command]git config gc.auto 0
2019-10-18T21:10:57.7760658Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-18T21:10:57.7817248Z ##[command]git config --get-all http.proxy
2019-10-18T21:10:57.7948450Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65315/merge:refs/remotes/pull/65315/merge
---
2019-10-18T21:17:18.4775895Z    Compiling serde_json v1.0.40
2019-10-18T21:17:20.1701695Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-18T21:17:31.4956299Z     Finished release [optimized] target(s) in 1m 28s
2019-10-18T21:17:31.5035272Z tidy check
2019-10-18T21:17:31.9616116Z tidy error: /checkout/src/librustc_mir/transform/copy_prop.rs:135: line longer than 100 chars
2019-10-18T21:17:31.9616229Z tidy error: /checkout/src/librustc_mir/transform/rustc_peek.rs:213: line longer than 100 chars
2019-10-18T21:17:31.9616564Z tidy error: /checkout/src/librustc_mir/transform/rustc_peek.rs:217: line longer than 100 chars
2019-10-18T21:17:31.9645429Z tidy error: /checkout/src/librustc_mir/transform/check_consts/validation.rs:397: line longer than 100 chars
2019-10-18T21:17:31.9669346Z tidy error: /checkout/src/librustc_mir/util/def_use.rs:51: line longer than 100 chars
2019-10-18T21:17:31.9669465Z tidy error: /checkout/src/librustc_mir/util/def_use.rs:127: line longer than 100 chars
2019-10-18T21:17:32.1491380Z tidy error: /checkout/src/librustc/ty/context.rs: too many lines (3007) (add `// ignore-tidy-filelength` to the file to suppress this error)
2019-10-18T21:17:32.1561492Z tidy error: /checkout/src/librustc/mir/mod.rs:1873: line longer than 100 chars
2019-10-18T21:17:33.6195482Z some tidy checks failed
2019-10-18T21:17:33.6195585Z Found 482 error codes
2019-10-18T21:17:33.6195629Z Found 0 error codes with no tests
2019-10-18T21:17:33.6195754Z Done!
2019-10-18T21:17:33.6195754Z Done!
2019-10-18T21:17:33.6195779Z 
2019-10-18T21:17:33.6195801Z 
2019-10-18T21:17:33.6196591Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-18T21:17:33.6197019Z 
2019-10-18T21:17:33.6197045Z 
2019-10-18T21:17:33.6201133Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-18T21:17:33.6201488Z Build completed unsuccessfully in 0:01:31
2019-10-18T21:17:33.6201488Z Build completed unsuccessfully in 0:01:31
2019-10-18T21:17:33.6249670Z == clock drift check ==
2019-10-18T21:17:33.6262263Z   local time: Fri Oct 18 21:17:33 UTC 2019
2019-10-18T21:17:33.6958690Z   network time: Fri, 18 Oct 2019 21:17:33 GMT
2019-10-18T21:17:33.6962503Z == end clock drift check ==
2019-10-18T21:17:35.1651767Z 
2019-10-18T21:17:35.1750784Z ##[error]Bash exited with code '1'.
2019-10-18T21:17:35.1791736Z ##[section]Starting: Checkout
2019-10-18T21:17:35.1793177Z ==============================================================================
2019-10-18T21:17:35.1793242Z Task         : Get sources
2019-10-18T21:17:35.1793280Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
