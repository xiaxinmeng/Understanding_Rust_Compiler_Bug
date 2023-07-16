plain
2019-12-08T12:57:48.1171760Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-08T12:57:48.1188978Z ##[command]git config gc.auto 0
2019-12-08T12:57:48.1192627Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-08T12:57:48.1194909Z ##[command]git config --get-all http.proxy
2019-12-08T12:57:48.1201231Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67152/merge:refs/remotes/pull/67152/merge
---
2019-12-08T13:03:35.6961203Z    Compiling serde_json v1.0.40
2019-12-08T13:03:37.3861027Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-08T13:03:48.1633929Z     Finished release [optimized] target(s) in 1m 26s
2019-12-08T13:03:48.1741288Z tidy check
2019-12-08T13:03:48.8105376Z tidy error: /checkout/src/librustdoc/html/render.rs:2295: line longer than 100 chars
2019-12-08T13:03:50.9029129Z Found 485 error codes
2019-12-08T13:03:50.9029239Z Found 0 error codes with no tests
2019-12-08T13:03:50.9033440Z Done!
2019-12-08T13:03:50.9033494Z some tidy checks failed
2019-12-08T13:03:50.9033494Z some tidy checks failed
2019-12-08T13:03:50.9038792Z 
2019-12-08T13:03:50.9045036Z 
2019-12-08T13:03:50.9051743Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-08T13:03:50.9052257Z 
2019-12-08T13:03:50.9052294Z 
2019-12-08T13:03:50.9077043Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-08T13:03:50.9077122Z Build completed unsuccessfully in 0:01:30
2019-12-08T13:03:50.9077122Z Build completed unsuccessfully in 0:01:30
2019-12-08T13:03:50.9111204Z == clock drift check ==
2019-12-08T13:03:50.9125953Z   local time: Sun Dec  8 13:03:50 UTC 2019
2019-12-08T13:03:51.0422610Z   network time: Sun, 08 Dec 2019 13:03:51 GMT
2019-12-08T13:03:51.0427394Z == end clock drift check ==
2019-12-08T13:03:52.4660629Z 
2019-12-08T13:03:52.4768256Z ##[error]Bash exited with code '1'.
2019-12-08T13:03:52.4797707Z ##[section]Starting: Checkout
2019-12-08T13:03:52.4799647Z ==============================================================================
2019-12-08T13:03:52.4799702Z Task         : Get sources
2019-12-08T13:03:52.4799765Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
