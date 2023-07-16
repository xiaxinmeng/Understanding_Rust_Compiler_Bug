plain
2019-10-08T21:30:16.0316650Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-08T21:30:16.0557726Z ##[command]git config gc.auto 0
2019-10-08T21:30:16.0612012Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-08T21:30:16.7892906Z ##[command]git config --get-all http.proxy
2019-10-08T21:30:16.7901860Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65222/merge:refs/remotes/pull/65222/merge
---
2019-10-08T21:37:15.3485061Z    Compiling serde_json v1.0.40
2019-10-08T21:37:17.2088709Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-08T21:37:28.9645060Z     Finished release [optimized] target(s) in 1m 33s
2019-10-08T21:37:28.9732997Z tidy check
2019-10-08T21:37:29.7303798Z tidy error: /checkout/src/libcore/iter/traits/iterator.rs: too many lines (3055) (add `// ignore-tidy-filelength` to the file to suppress this error)
2019-10-08T21:37:31.0899737Z some tidy checks failed
2019-10-08T21:37:31.0902804Z 
2019-10-08T21:37:31.0902804Z 
2019-10-08T21:37:31.0903994Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-08T21:37:31.0904457Z 
2019-10-08T21:37:31.0904632Z 
2019-10-08T21:37:31.0910354Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-08T21:37:31.0910619Z Build completed unsuccessfully in 0:01:36
2019-10-08T21:37:31.0910619Z Build completed unsuccessfully in 0:01:36
2019-10-08T21:37:31.0963026Z == clock drift check ==
2019-10-08T21:37:31.0978960Z   local time: Tue Oct  8 21:37:31 UTC 2019
2019-10-08T21:37:31.1835808Z   network time: Tue, 08 Oct 2019 21:37:31 GMT
2019-10-08T21:37:31.1835929Z == end clock drift check ==
2019-10-08T21:37:32.5183247Z ##[error]Bash exited with code '1'.
2019-10-08T21:37:32.5237744Z ##[section]Starting: Checkout
2019-10-08T21:37:32.5239549Z ==============================================================================
2019-10-08T21:37:32.5239599Z Task         : Get sources
2019-10-08T21:37:32.5239661Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
