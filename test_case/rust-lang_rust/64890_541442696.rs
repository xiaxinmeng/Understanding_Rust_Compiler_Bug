plain
2019-10-13T17:54:51.8725963Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-13T17:54:51.8811969Z ##[command]git config gc.auto 0
2019-10-13T17:54:51.8884398Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-13T17:54:51.8951477Z ##[command]git config --get-all http.proxy
2019-10-13T17:54:51.9086488Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64890/merge:refs/remotes/pull/64890/merge
---
2019-10-13T18:00:56.8250253Z    Compiling serde_json v1.0.40
2019-10-13T18:00:58.5211437Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-13T18:01:09.8766757Z     Finished release [optimized] target(s) in 1m 27s
2019-10-13T18:01:09.8849896Z tidy check
2019-10-13T18:01:10.3135384Z tidy error: /checkout/src/librustc_mir/transform/const_prop.rs:442: trailing whitespace
2019-10-13T18:01:12.1912599Z some tidy checks failed
2019-10-13T18:01:12.1912796Z Found 482 error codes
2019-10-13T18:01:12.1924080Z Found 0 error codes with no tests
2019-10-13T18:01:12.1924506Z Done!
2019-10-13T18:01:12.1924506Z Done!
2019-10-13T18:01:12.1924819Z 
2019-10-13T18:01:12.1925026Z 
2019-10-13T18:01:12.1925953Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-13T18:01:12.1926567Z 
2019-10-13T18:01:12.1926671Z 
2019-10-13T18:01:12.1931617Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-13T18:01:12.1931877Z Build completed unsuccessfully in 0:01:30
2019-10-13T18:01:12.1931877Z Build completed unsuccessfully in 0:01:30
2019-10-13T18:01:12.1970673Z == clock drift check ==
2019-10-13T18:01:12.1987749Z   local time: Sun Oct 13 18:01:12 UTC 2019
2019-10-13T18:01:12.3490573Z   network time: Sun, 13 Oct 2019 18:01:12 GMT
2019-10-13T18:01:12.3493623Z == end clock drift check ==
2019-10-13T18:01:13.1564650Z ##[error]Bash exited with code '1'.
2019-10-13T18:01:13.1612191Z ##[section]Starting: Checkout
2019-10-13T18:01:13.1613741Z ==============================================================================
2019-10-13T18:01:13.1613804Z Task         : Get sources
2019-10-13T18:01:13.1613854Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
