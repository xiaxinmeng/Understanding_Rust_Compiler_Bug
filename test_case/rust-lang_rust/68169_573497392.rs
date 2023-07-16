plain
2020-01-13T03:17:12.8763203Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-13T03:17:12.8774507Z ##[command]git config gc.auto 0
2020-01-13T03:17:12.8777196Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-13T03:17:12.8779458Z ##[command]git config --get-all http.proxy
2020-01-13T03:17:12.8782645Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68169/merge:refs/remotes/pull/68169/merge
---
2020-01-13T03:22:57.1715261Z    Compiling serde_json v1.0.40
2020-01-13T03:22:58.8398233Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-01-13T03:23:08.9108943Z     Finished release [optimized] target(s) in 1m 23s
2020-01-13T03:23:08.9205240Z tidy check
2020-01-13T03:23:09.1727195Z tidy error: /checkout/src/libcore/ptr/mod.rs:301: TODO is deprecated; use FIXME
2020-01-13T03:23:11.8038528Z some tidy checks failed
2020-01-13T03:23:11.8039222Z Found 486 error codes
2020-01-13T03:23:11.8039480Z Found 0 error codes with no tests
2020-01-13T03:23:11.8039668Z Done!
2020-01-13T03:23:11.8039668Z Done!
2020-01-13T03:23:11.8044455Z 
2020-01-13T03:23:11.8044723Z 
2020-01-13T03:23:11.8045954Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-13T03:23:11.8046142Z 
2020-01-13T03:23:11.8046169Z 
2020-01-13T03:23:11.8056054Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-13T03:23:11.8056379Z Build completed unsuccessfully in 0:01:35
2020-01-13T03:23:11.8056379Z Build completed unsuccessfully in 0:01:35
2020-01-13T03:23:11.8111790Z == clock drift check ==
2020-01-13T03:23:11.8123379Z   local time: Mon Jan 13 03:23:11 UTC 2020
2020-01-13T03:23:12.1006636Z   network time: Mon, 13 Jan 2020 03:23:12 GMT
2020-01-13T03:23:12.1006957Z == end clock drift check ==
2020-01-13T03:23:12.8221054Z 
2020-01-13T03:23:12.8310705Z ##[error]Bash exited with code '1'.
2020-01-13T03:23:12.8340570Z ##[section]Starting: Checkout
2020-01-13T03:23:12.8342301Z ==============================================================================
2020-01-13T03:23:12.8342383Z Task         : Get sources
2020-01-13T03:23:12.8342438Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
