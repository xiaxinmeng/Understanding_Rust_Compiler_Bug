plain
2019-12-21T02:49:11.3440188Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-21T02:49:11.3455961Z ##[command]git config gc.auto 0
2019-12-21T02:49:11.3460140Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-21T02:49:11.3462809Z ##[command]git config --get-all http.proxy
2019-12-21T02:49:11.3474894Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67398/merge:refs/remotes/pull/67398/merge
---
2019-12-21T02:55:14.8690094Z    Compiling serde_json v1.0.40
2019-12-21T02:55:16.5664524Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-21T02:55:27.5955874Z     Finished release [optimized] target(s) in 1m 27s
2019-12-21T02:55:27.6067072Z tidy check
2019-12-21T02:55:27.9102848Z tidy error: /checkout/src/librustc_jobserver/lib.rs:235: line longer than 100 chars
2019-12-21T02:55:30.3999584Z some tidy checks failed
2019-12-21T02:55:30.4002573Z Found 485 error codes
2019-12-21T02:55:30.4004029Z Found 0 error codes with no tests
2019-12-21T02:55:30.4004397Z Done!
2019-12-21T02:55:30.4004397Z Done!
2019-12-21T02:55:30.4004722Z 
2019-12-21T02:55:30.4004986Z 
2019-12-21T02:55:30.4006229Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-21T02:55:30.4009400Z 
2019-12-21T02:55:30.4009991Z 
2019-12-21T02:55:30.4010559Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-21T02:55:30.4010938Z Build completed unsuccessfully in 0:01:31
2019-12-21T02:55:30.4010938Z Build completed unsuccessfully in 0:01:31
2019-12-21T02:55:30.4062910Z == clock drift check ==
2019-12-21T02:55:30.4073057Z   local time: Sat Dec 21 02:55:30 UTC 2019
2019-12-21T02:55:30.6878871Z   network time: Sat, 21 Dec 2019 02:55:30 GMT
2019-12-21T02:55:30.6885110Z == end clock drift check ==
2019-12-21T02:55:32.0591595Z 
2019-12-21T02:55:32.0707993Z ##[error]Bash exited with code '1'.
2019-12-21T02:55:32.0739264Z ##[section]Starting: Checkout
2019-12-21T02:55:32.0741496Z ==============================================================================
2019-12-21T02:55:32.0741577Z Task         : Get sources
2019-12-21T02:55:32.0741629Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
