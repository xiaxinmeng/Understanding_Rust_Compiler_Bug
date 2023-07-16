plain
2019-10-31T04:42:00.6124808Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-31T04:42:00.6326856Z ##[command]git config gc.auto 0
2019-10-31T04:42:00.6402982Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-31T04:42:00.6458596Z ##[command]git config --get-all http.proxy
2019-10-31T04:42:00.6594322Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65994/merge:refs/remotes/pull/65994/merge
---
2019-10-31T04:48:06.1483673Z    Compiling serde_json v1.0.40
2019-10-31T04:48:07.9328704Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-31T04:48:19.1451449Z     Finished release [optimized] target(s) in 1m 26s
2019-10-31T04:48:19.1525585Z tidy check
2019-10-31T04:48:19.7984570Z tidy error: /checkout/src/librustc/ty/wf.rs:247: line longer than 100 chars
2019-10-31T04:48:19.7984687Z tidy error: /checkout/src/librustc/ty/wf.rs:305: line longer than 100 chars
2019-10-31T04:48:21.3597198Z Found 485 error codes
2019-10-31T04:48:21.3598220Z Found 0 error codes with no tests
2019-10-31T04:48:21.3598464Z Done!
2019-10-31T04:48:21.3598744Z some tidy checks failed
2019-10-31T04:48:21.3598744Z some tidy checks failed
2019-10-31T04:48:21.3601720Z 
2019-10-31T04:48:21.3602014Z 
2019-10-31T04:48:21.3602968Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-31T04:48:21.3629265Z 
2019-10-31T04:48:21.3629784Z 
2019-10-31T04:48:21.3634106Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-31T04:48:21.3634368Z Build completed unsuccessfully in 0:01:30
2019-10-31T04:48:21.3634368Z Build completed unsuccessfully in 0:01:30
2019-10-31T04:48:21.3657925Z == clock drift check ==
2019-10-31T04:48:21.3668414Z   local time: Thu Oct 31 04:48:21 UTC 2019
2019-10-31T04:48:21.5149449Z   network time: Thu, 31 Oct 2019 04:48:21 GMT
2019-10-31T04:48:21.5151540Z == end clock drift check ==
2019-10-31T04:48:22.8868541Z 
2019-10-31T04:48:22.8969739Z ##[error]Bash exited with code '1'.
2019-10-31T04:48:22.9001353Z ##[section]Starting: Checkout
2019-10-31T04:48:22.9002778Z ==============================================================================
2019-10-31T04:48:22.9002823Z Task         : Get sources
2019-10-31T04:48:22.9002891Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
