plain
2019-10-31T13:47:23.9099729Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-31T13:47:23.9296791Z ##[command]git config gc.auto 0
2019-10-31T13:47:23.9381583Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-31T13:47:23.9444354Z ##[command]git config --get-all http.proxy
2019-10-31T13:47:24.5793443Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65470/merge:refs/remotes/pull/65470/merge
---
2019-10-31T13:53:40.1753075Z    Compiling serde_json v1.0.40
2019-10-31T13:53:41.7251196Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-31T13:53:53.2483739Z     Finished release [optimized] target(s) in 1m 29s
2019-10-31T13:53:53.2566967Z tidy check
2019-10-31T13:53:54.0626443Z tidy error: /checkout/src/librustc_codegen_utils/lib.rs:48: line longer than 100 chars
2019-10-31T13:53:54.0923293Z tidy error: /checkout/src/libsyntax/feature_gate/builtin_attrs.rs:540: line longer than 100 chars
2019-10-31T13:53:55.6399756Z some tidy checks failed
2019-10-31T13:53:55.6401033Z Found 485 error codes
2019-10-31T13:53:55.6401222Z Found 0 error codes with no tests
2019-10-31T13:53:55.6401553Z Done!
2019-10-31T13:53:55.6401553Z Done!
2019-10-31T13:53:55.6404913Z 
2019-10-31T13:53:55.6405409Z 
2019-10-31T13:53:55.6409793Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-31T13:53:55.6409975Z 
2019-10-31T13:53:55.6410002Z 
2019-10-31T13:53:55.6423776Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-31T13:53:55.6423861Z Build completed unsuccessfully in 0:01:33
2019-10-31T13:53:55.6423861Z Build completed unsuccessfully in 0:01:33
2019-10-31T13:53:55.6474236Z == clock drift check ==
2019-10-31T13:53:55.6481729Z   local time: Thu Oct 31 13:53:55 UTC 2019
2019-10-31T13:53:55.7322112Z   network time: Thu, 31 Oct 2019 13:53:55 GMT
2019-10-31T13:53:55.7325205Z == end clock drift check ==
2019-10-31T13:53:57.1070772Z 
2019-10-31T13:53:57.1177636Z ##[error]Bash exited with code '1'.
2019-10-31T13:53:57.1218203Z ##[section]Starting: Checkout
2019-10-31T13:53:57.1220077Z ==============================================================================
2019-10-31T13:53:57.1220138Z Task         : Get sources
2019-10-31T13:53:57.1220192Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
