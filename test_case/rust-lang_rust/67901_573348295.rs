plain
2020-01-11T19:23:44.7686733Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-11T19:23:44.7764823Z ##[command]git config gc.auto 0
2020-01-11T19:23:44.7844752Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-11T19:23:44.7903166Z ##[command]git config --get-all http.proxy
2020-01-11T19:23:44.8043437Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67901/merge:refs/remotes/pull/67901/merge
---
2020-01-11T19:29:02.0208612Z    Compiling serde_json v1.0.40
2020-01-11T19:29:03.6836108Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-01-11T19:29:13.2756453Z     Finished release [optimized] target(s) in 1m 19s
2020-01-11T19:29:13.2875240Z tidy check
2020-01-11T19:29:14.1942742Z tidy error: /checkout/src/librustc/mir/mod.rs: ignoring file length unnecessarily
2020-01-11T19:29:16.1057966Z some tidy checks failed
2020-01-11T19:29:16.1058750Z Found 486 error codes
2020-01-11T19:29:16.1059022Z Found 0 error codes with no tests
2020-01-11T19:29:16.1059177Z Done!
2020-01-11T19:29:16.1059177Z Done!
2020-01-11T19:29:16.1067655Z 
2020-01-11T19:29:16.1068126Z 
2020-01-11T19:29:16.1069385Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-11T19:29:16.1069835Z 
2020-01-11T19:29:16.1069952Z 
2020-01-11T19:29:16.1076491Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-11T19:29:16.1076897Z Build completed unsuccessfully in 0:01:30
2020-01-11T19:29:16.1076897Z Build completed unsuccessfully in 0:01:30
2020-01-11T19:29:16.1134224Z == clock drift check ==
2020-01-11T19:29:16.1199390Z   local time: Sat Jan 11 19:29:16 UTC 2020
2020-01-11T19:29:16.4147807Z   network time: Sat, 11 Jan 2020 19:29:16 GMT
2020-01-11T19:29:16.4147957Z == end clock drift check ==
2020-01-11T19:29:17.1441089Z 
2020-01-11T19:29:17.1548975Z ##[error]Bash exited with code '1'.
2020-01-11T19:29:17.1577438Z ##[section]Starting: Checkout
2020-01-11T19:29:17.1579353Z ==============================================================================
2020-01-11T19:29:17.1579409Z Task         : Get sources
2020-01-11T19:29:17.1579458Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
