plain
2019-12-22T11:49:43.5779060Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-22T11:49:43.5788203Z ##[command]git config gc.auto 0
2019-12-22T11:49:43.5791241Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-22T11:49:43.5794015Z ##[command]git config --get-all http.proxy
2019-12-22T11:49:43.5797478Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67516/merge:refs/remotes/pull/67516/merge
---
2019-12-22T11:54:39.1812109Z    Compiling serde_json v1.0.40
2019-12-22T11:54:39.8486759Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-22T11:54:47.7992367Z     Finished release [optimized] target(s) in 1m 03s
2019-12-22T11:54:47.8069013Z tidy check
2019-12-22T11:54:48.4069688Z tidy error: /checkout/src/librustc_codegen_ssa/base.rs:735: line longer than 100 chars
2019-12-22T11:54:48.4069820Z tidy error: /checkout/src/librustc_codegen_ssa/base.rs:736: line longer than 100 chars
2019-12-22T11:54:49.7934059Z Found 485 error codes
2019-12-22T11:54:49.7934125Z Found 0 error codes with no tests
2019-12-22T11:54:49.7934164Z Done!
2019-12-22T11:54:49.7934215Z some tidy checks failed
2019-12-22T11:54:49.7934215Z some tidy checks failed
2019-12-22T11:54:49.7938008Z 
2019-12-22T11:54:49.7938064Z 
2019-12-22T11:54:49.7938818Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-22T11:54:49.7938929Z 
2019-12-22T11:54:49.7938949Z 
2019-12-22T11:54:49.7945002Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-22T11:54:49.7945256Z Build completed unsuccessfully in 0:01:06
2019-12-22T11:54:49.7945256Z Build completed unsuccessfully in 0:01:06
2019-12-22T11:54:49.7987930Z == clock drift check ==
2019-12-22T11:54:49.7995023Z   local time: Sun Dec 22 11:54:49 UTC 2019
2019-12-22T11:54:50.3205937Z   network time: Sun, 22 Dec 2019 11:54:50 GMT
2019-12-22T11:54:50.3214416Z == end clock drift check ==
2019-12-22T11:54:51.9954185Z 
2019-12-22T11:54:52.0048165Z ##[error]Bash exited with code '1'.
2019-12-22T11:54:52.0072505Z ##[section]Starting: Checkout
2019-12-22T11:54:52.0073851Z ==============================================================================
2019-12-22T11:54:52.0073893Z Task         : Get sources
2019-12-22T11:54:52.0073946Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
