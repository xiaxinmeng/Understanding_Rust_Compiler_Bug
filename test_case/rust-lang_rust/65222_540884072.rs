plain
2019-10-11T03:03:25.5422768Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-11T03:03:25.5535141Z ##[command]git config gc.auto 0
2019-10-11T03:03:25.5633817Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-11T03:03:25.5705074Z ##[command]git config --get-all http.proxy
2019-10-11T03:03:25.5853145Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65222/merge:refs/remotes/pull/65222/merge
---
2019-10-11T03:09:39.8174004Z    Compiling serde_json v1.0.40
2019-10-11T03:09:41.7155909Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-11T03:09:53.6923241Z     Finished release [optimized] target(s) in 1m 33s
2019-10-11T03:09:53.7016654Z tidy check
2019-10-11T03:09:54.8489548Z tidy error: /checkout/src/libcore/iter/traits/iterator.rs: too many lines (3043) (add `// ignore-tidy-filelength` to the file to suppress this error)
2019-10-11T03:09:56.2570936Z Found 482 error codes
2019-10-11T03:09:56.2571284Z Found 0 error codes with no tests
2019-10-11T03:09:56.2571334Z Done!
2019-10-11T03:09:56.2571380Z some tidy checks failed
2019-10-11T03:09:56.2571380Z some tidy checks failed
2019-10-11T03:09:56.2571413Z 
2019-10-11T03:09:56.2571467Z 
2019-10-11T03:09:56.2576452Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-11T03:09:56.2576651Z 
2019-10-11T03:09:56.2576671Z 
2019-10-11T03:09:56.2587787Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-11T03:09:56.2587898Z Build completed unsuccessfully in 0:01:37
2019-10-11T03:09:56.2587898Z Build completed unsuccessfully in 0:01:37
2019-10-11T03:09:56.2643256Z == clock drift check ==
2019-10-11T03:09:56.2665049Z   local time: Fri Oct 11 03:09:56 UTC 2019
2019-10-11T03:09:56.4184295Z   network time: Fri, 11 Oct 2019 03:09:56 GMT
2019-10-11T03:09:56.4189488Z == end clock drift check ==
2019-10-11T03:09:57.2132799Z ##[error]Bash exited with code '1'.
2019-10-11T03:09:57.2180847Z ##[section]Starting: Checkout
2019-10-11T03:09:57.2183463Z ==============================================================================
2019-10-11T03:09:57.2184493Z Task         : Get sources
2019-10-11T03:09:57.2184571Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
