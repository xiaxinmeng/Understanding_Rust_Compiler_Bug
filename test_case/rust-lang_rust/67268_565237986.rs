plain
2019-12-12T23:28:45.3608081Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-12T23:28:45.3623779Z ##[command]git config gc.auto 0
2019-12-12T23:28:45.3628333Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-12T23:28:45.3630168Z ##[command]git config --get-all http.proxy
2019-12-12T23:28:45.3634482Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67268/merge:refs/remotes/pull/67268/merge
---
2019-12-12T23:34:14.0196957Z    Compiling serde_json v1.0.40
2019-12-12T23:34:15.5611356Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-12T23:34:25.3952105Z     Finished release [optimized] target(s) in 1m 18s
2019-12-12T23:34:25.4036757Z tidy check
2019-12-12T23:34:25.9176817Z tidy error: /checkout/src/test/ui/associated-type/associated-type-projection-from-multiple-supertraits.rs:23: TODO is deprecated; use FIXME
2019-12-12T23:34:26.3503271Z tidy error: /checkout/src/librustc_typeck/astconv.rs:1298: line longer than 100 chars
2019-12-12T23:34:27.7884712Z some tidy checks failed
2019-12-12T23:34:27.7886249Z Found 485 error codes
2019-12-12T23:34:27.7886521Z Found 0 error codes with no tests
2019-12-12T23:34:27.7886756Z Done!
2019-12-12T23:34:27.7886756Z Done!
2019-12-12T23:34:27.7887023Z 
2019-12-12T23:34:27.7887207Z 
2019-12-12T23:34:27.7888505Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-12T23:34:27.7889060Z 
2019-12-12T23:34:27.7889392Z 
2019-12-12T23:34:27.7891533Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-12T23:34:27.7892011Z Build completed unsuccessfully in 0:01:21
2019-12-12T23:34:27.7892011Z Build completed unsuccessfully in 0:01:21
2019-12-12T23:34:27.7940987Z == clock drift check ==
2019-12-12T23:34:27.7955922Z   local time: Thu Dec 12 23:34:27 UTC 2019
2019-12-12T23:34:28.0731318Z   network time: Thu, 12 Dec 2019 23:34:28 GMT
2019-12-12T23:34:28.0736134Z == end clock drift check ==
2019-12-12T23:34:29.4119357Z 
2019-12-12T23:34:29.4207266Z ##[error]Bash exited with code '1'.
2019-12-12T23:34:29.4237084Z ##[section]Starting: Checkout
2019-12-12T23:34:29.4238492Z ==============================================================================
2019-12-12T23:34:29.4238544Z Task         : Get sources
2019-12-12T23:34:29.4238581Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
