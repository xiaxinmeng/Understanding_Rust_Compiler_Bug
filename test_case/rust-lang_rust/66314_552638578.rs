plain
2019-11-11T21:48:34.7772724Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-11T21:48:34.7979702Z ##[command]git config gc.auto 0
2019-11-11T21:48:34.8092431Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-11T21:48:34.8129258Z ##[command]git config --get-all http.proxy
2019-11-11T21:48:34.8352882Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66314/merge:refs/remotes/pull/66314/merge
---
2019-11-11T21:55:12.3783740Z    Compiling serde_json v1.0.40
2019-11-11T21:55:14.3430139Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-11T21:55:26.6474798Z     Finished release [optimized] target(s) in 1m 34s
2019-11-11T21:55:26.6549811Z tidy check
2019-11-11T21:55:27.5143618Z tidy error: /checkout/src/librustc_error_codes/error_codes.rs: too many lines (13613) (add `// ignore-tidy-filelength` to the file to suppress this error)
2019-11-11T21:55:29.5452694Z Found 485 error codes
2019-11-11T21:55:29.5452844Z Found 0 error codes with no tests
2019-11-11T21:55:29.5452935Z Done!
2019-11-11T21:55:29.5452979Z some tidy checks failed
2019-11-11T21:55:29.5452979Z some tidy checks failed
2019-11-11T21:55:29.5453009Z 
2019-11-11T21:55:29.5453036Z 
2019-11-11T21:55:29.5453903Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-11T21:55:29.5454003Z 
2019-11-11T21:55:29.5454044Z 
2019-11-11T21:55:29.5461316Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-11T21:55:29.5461394Z Build completed unsuccessfully in 0:01:39
2019-11-11T21:55:29.5461394Z Build completed unsuccessfully in 0:01:39
2019-11-11T21:55:29.5507045Z == clock drift check ==
2019-11-11T21:55:29.5515397Z   local time: Mon Nov 11 21:55:29 UTC 2019
2019-11-11T21:55:29.7124165Z   network time: Mon, 11 Nov 2019 21:55:29 GMT
2019-11-11T21:55:29.7130776Z == end clock drift check ==
2019-11-11T21:55:31.0276072Z 
2019-11-11T21:55:31.0390713Z ##[error]Bash exited with code '1'.
2019-11-11T21:55:31.0431424Z ##[section]Starting: Checkout
2019-11-11T21:55:31.0433518Z ==============================================================================
2019-11-11T21:55:31.0433597Z Task         : Get sources
2019-11-11T21:55:31.0433700Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
