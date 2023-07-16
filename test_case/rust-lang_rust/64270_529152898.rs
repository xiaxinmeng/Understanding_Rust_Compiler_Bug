plain
2019-09-07T22:26:49.7008011Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-07T22:26:49.7230150Z ##[command]git config gc.auto 0
2019-09-07T22:26:49.7291572Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-07T22:26:49.7349905Z ##[command]git config --get-all http.proxy
2019-09-07T22:26:49.7497670Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64270/merge:refs/remotes/pull/64270/merge
---
2019-09-07T22:33:51.4294006Z    Compiling serde_json v1.0.40
2019-09-07T22:33:53.3358081Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-07T22:34:04.5890355Z     Finished release [optimized] target(s) in 1m 34s
2019-09-07T22:34:04.5976974Z tidy check
2019-09-07T22:34:05.5673871Z tidy error: /checkout/src/libstd/keyword_docs.rs:35: trailing whitespace
2019-09-07T22:34:05.5674859Z tidy error: /checkout/src/libstd/keyword_docs.rs:36: trailing whitespace
2019-09-07T22:34:05.5675305Z tidy error: /checkout/src/libstd/keyword_docs.rs:37: trailing whitespace
2019-09-07T22:34:05.5676063Z tidy error: /checkout/src/libstd/keyword_docs.rs:38: trailing whitespace
2019-09-07T22:34:05.5676429Z tidy error: /checkout/src/libstd/keyword_docs.rs:40: trailing whitespace
2019-09-07T22:34:05.5677019Z tidy error: /checkout/src/libstd/keyword_docs.rs:47: trailing whitespace
2019-09-07T22:34:05.5677447Z tidy error: /checkout/src/libstd/keyword_docs.rs:52: trailing whitespace
2019-09-07T22:34:05.5677701Z tidy error: /checkout/src/libstd/keyword_docs.rs:54: trailing whitespace
2019-09-07T22:34:05.5677911Z tidy error: /checkout/src/libstd/keyword_docs.rs:65: trailing whitespace
2019-09-07T22:34:05.5678141Z tidy error: /checkout/src/libstd/keyword_docs.rs:68: trailing whitespace
2019-09-07T22:34:05.5678352Z tidy error: /checkout/src/libstd/keyword_docs.rs:81: trailing whitespace
2019-09-07T22:34:06.7426504Z some tidy checks failed
2019-09-07T22:34:06.7493224Z 
2019-09-07T22:34:06.7493224Z 
2019-09-07T22:34:06.7495067Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-07T22:34:06.7495250Z 
2019-09-07T22:34:06.7495396Z 
2019-09-07T22:34:06.7495473Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-07T22:34:06.7495526Z Build completed unsuccessfully in 0:01:37
2019-09-07T22:34:06.7495526Z Build completed unsuccessfully in 0:01:37
2019-09-07T22:34:06.7511372Z == clock drift check ==
2019-09-07T22:34:06.7526878Z   local time: Sat Sep  7 22:34:06 UTC 2019
2019-09-07T22:34:06.9046708Z   network time: Sat, 07 Sep 2019 22:34:06 GMT
2019-09-07T22:34:06.9046800Z == end clock drift check ==
2019-09-07T22:34:08.3114866Z ##[error]Bash exited with code '1'.
2019-09-07T22:34:08.3171836Z ##[section]Starting: Checkout
2019-09-07T22:34:08.3173489Z ==============================================================================
2019-09-07T22:34:08.3173538Z Task         : Get sources
2019-09-07T22:34:08.3173580Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
