plain
2019-12-10T12:35:15.6609777Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-10T12:35:15.6817518Z ##[command]git config gc.auto 0
2019-12-10T12:35:16.2452065Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-10T12:35:16.2457204Z ##[command]git config --get-all http.proxy
2019-12-10T12:35:16.2463412Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67195/merge:refs/remotes/pull/67195/merge
---
2019-12-10T12:41:09.8847172Z    Compiling serde_json v1.0.40
2019-12-10T12:41:11.6221690Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-10T12:41:22.6207840Z     Finished release [optimized] target(s) in 1m 28s
2019-12-10T12:41:22.6322224Z tidy check
2019-12-10T12:41:23.4268625Z tidy error: /checkout/src/librustc_session/config.rs: too many lines (3015) (add `// ignore-tidy-filelength` to the file to suppress this error)
2019-12-10T12:41:25.3800276Z some tidy checks failed
2019-12-10T12:41:25.3803694Z Found 485 error codes
2019-12-10T12:41:25.3804509Z Found 0 error codes with no tests
2019-12-10T12:41:25.3804796Z Done!
2019-12-10T12:41:25.3804796Z Done!
2019-12-10T12:41:25.3805117Z 
2019-12-10T12:41:25.3805345Z 
2019-12-10T12:41:25.3806701Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-10T12:41:25.3809219Z 
2019-12-10T12:41:25.3809446Z 
2019-12-10T12:41:25.3809791Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-10T12:41:25.3810052Z Build completed unsuccessfully in 0:01:32
2019-12-10T12:41:25.3810052Z Build completed unsuccessfully in 0:01:32
2019-12-10T12:41:25.3864524Z == clock drift check ==
2019-12-10T12:41:25.3872832Z   local time: Tue Dec 10 12:41:25 UTC 2019
2019-12-10T12:41:25.5375810Z   network time: Tue, 10 Dec 2019 12:41:25 GMT
2019-12-10T12:41:25.5378009Z == end clock drift check ==
2019-12-10T12:41:26.8769521Z 
2019-12-10T12:41:26.8887665Z ##[error]Bash exited with code '1'.
2019-12-10T12:41:26.8924335Z ##[section]Starting: Checkout
2019-12-10T12:41:26.8926199Z ==============================================================================
2019-12-10T12:41:26.8926260Z Task         : Get sources
2019-12-10T12:41:26.8926311Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
