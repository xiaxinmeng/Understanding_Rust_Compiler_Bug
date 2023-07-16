plain
2019-11-14T10:29:56.8999175Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-14T10:29:56.9196883Z ##[command]git config gc.auto 0
2019-11-14T10:29:56.9265240Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-14T10:29:56.9326669Z ##[command]git config --get-all http.proxy
2019-11-14T10:29:56.9466820Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66379/merge:refs/remotes/pull/66379/merge
---
2019-11-14T10:35:28.5432025Z    Compiling serde_json v1.0.40
2019-11-14T10:35:30.1334418Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-14T10:35:40.5795849Z     Finished release [optimized] target(s) in 1m 18s
2019-11-14T10:35:40.5884740Z tidy check
2019-11-14T10:35:41.3783448Z tidy error: /checkout/src/libcore/ptr/mod.rs:1077: trailing whitespace
2019-11-14T10:35:41.3783707Z tidy error: /checkout/src/libcore/ptr/mod.rs:1080: trailing whitespace
2019-11-14T10:35:41.3783762Z tidy error: /checkout/src/libcore/ptr/mod.rs:1088: trailing whitespace
2019-11-14T10:35:41.3783812Z tidy error: /checkout/src/libcore/ptr/mod.rs:1089: trailing whitespace
2019-11-14T10:35:41.3783908Z tidy error: /checkout/src/libcore/ptr/mod.rs:1092: trailing whitespace
2019-11-14T10:35:41.3783960Z tidy error: /checkout/src/libcore/ptr/mod.rs:1093: trailing whitespace
2019-11-14T10:35:41.3784009Z tidy error: /checkout/src/libcore/ptr/mod.rs:1941: trailing whitespace
2019-11-14T10:35:41.3784078Z tidy error: /checkout/src/libcore/ptr/mod.rs:1949: trailing whitespace
2019-11-14T10:35:41.3784127Z tidy error: /checkout/src/libcore/ptr/mod.rs:1950: trailing whitespace
2019-11-14T10:35:41.3784196Z tidy error: /checkout/src/libcore/ptr/mod.rs:1951: line longer than 100 chars
2019-11-14T10:35:41.3784263Z tidy error: /checkout/src/libcore/ptr/mod.rs:1953: trailing whitespace
2019-11-14T10:35:41.3784473Z tidy error: /checkout/src/libcore/ptr/mod.rs:1954: trailing whitespace
2019-11-14T10:35:42.8864145Z Found 486 error codes
2019-11-14T10:35:42.8864243Z Found 0 error codes with no tests
2019-11-14T10:35:42.8864319Z Done!
2019-11-14T10:35:42.8864585Z some tidy checks failed
2019-11-14T10:35:42.8864585Z some tidy checks failed
2019-11-14T10:35:42.8864819Z 
2019-11-14T10:35:42.8864854Z 
2019-11-14T10:35:42.8865706Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-14T10:35:42.8865803Z 
2019-11-14T10:35:42.8865859Z 
2019-11-14T10:35:42.8872087Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-14T10:35:42.8872146Z Build completed unsuccessfully in 0:01:22
2019-11-14T10:35:42.8872146Z Build completed unsuccessfully in 0:01:22
2019-11-14T10:35:42.8917201Z == clock drift check ==
2019-11-14T10:35:42.8928284Z   local time: Thu Nov 14 10:35:42 UTC 2019
2019-11-14T10:35:42.9760286Z   network time: Thu, 14 Nov 2019 10:35:42 GMT
2019-11-14T10:35:42.9764209Z == end clock drift check ==
2019-11-14T10:35:44.3746041Z 
2019-11-14T10:35:44.3817348Z ##[error]Bash exited with code '1'.
2019-11-14T10:35:44.3849435Z ##[section]Starting: Checkout
2019-11-14T10:35:44.3850904Z ==============================================================================
2019-11-14T10:35:44.3850951Z Task         : Get sources
2019-11-14T10:35:44.3851008Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
