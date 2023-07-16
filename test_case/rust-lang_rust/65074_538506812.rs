plain
2019-10-04T17:56:28.8054148Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-04T17:56:28.8312524Z ##[command]git config gc.auto 0
2019-10-04T17:56:28.8400880Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-04T17:56:28.8465965Z ##[command]git config --get-all http.proxy
2019-10-04T17:56:28.8618800Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65074/merge:refs/remotes/pull/65074/merge
---
2019-10-04T18:03:47.7157516Z    Compiling serde_json v1.0.40
2019-10-04T18:03:50.5272471Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-04T18:04:01.2609780Z     Finished release [optimized] target(s) in 1m 33s
2019-10-04T18:04:01.2696648Z tidy check
2019-10-04T18:04:01.5555780Z tidy error: /checkout/src/test/ui/json-bom-plus-crlf.rs:1: CR character
2019-10-04T18:04:01.5556628Z tidy error: /checkout/src/test/ui/json-bom-plus-crlf.rs:2: CR character
2019-10-04T18:04:01.5557125Z tidy error: /checkout/src/test/ui/json-bom-plus-crlf.rs:3: CR character
2019-10-04T18:04:01.5557564Z tidy error: /checkout/src/test/ui/json-bom-plus-crlf.rs:4: CR character
2019-10-04T18:04:01.5558038Z tidy error: /checkout/src/test/ui/json-bom-plus-crlf.rs:5: CR character
2019-10-04T18:04:01.5558454Z tidy error: /checkout/src/test/ui/json-bom-plus-crlf.rs:6: CR character
2019-10-04T18:04:01.5558887Z tidy error: /checkout/src/test/ui/json-bom-plus-crlf.rs:7: CR character
2019-10-04T18:04:01.5559310Z tidy error: /checkout/src/test/ui/json-bom-plus-crlf.rs:8: CR character
2019-10-04T18:04:01.5559720Z tidy error: /checkout/src/test/ui/json-bom-plus-crlf.rs:9: CR character
2019-10-04T18:04:01.5560301Z tidy error: /checkout/src/test/ui/json-bom-plus-crlf.rs:10: CR character
2019-10-04T18:04:01.5560712Z tidy error: /checkout/src/test/ui/json-bom-plus-crlf.rs:11: CR character
2019-10-04T18:04:01.5561146Z tidy error: /checkout/src/test/ui/json-bom-plus-crlf.rs:12: CR character
2019-10-04T18:04:01.5561808Z tidy error: /checkout/src/test/ui/json-bom-plus-crlf.rs:13: CR character
2019-10-04T18:04:01.5562305Z tidy error: /checkout/src/test/ui/json-bom-plus-crlf.rs:14: CR character
2019-10-04T18:04:01.5562940Z tidy error: /checkout/src/test/ui/json-bom-plus-crlf.rs:15: CR character
2019-10-04T18:04:01.5563358Z tidy error: /checkout/src/test/ui/json-bom-plus-crlf.rs:16: CR character
2019-10-04T18:04:02.3036707Z tidy error: /checkout/src/libsyntax/json/tests.rs: too many trailing newlines (2)
2019-10-04T18:04:03.5156055Z some tidy checks failed
2019-10-04T18:04:03.5156226Z 
2019-10-04T18:04:03.5156226Z 
2019-10-04T18:04:03.5157170Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-04T18:04:03.5157284Z 
2019-10-04T18:04:03.5157312Z 
2019-10-04T18:04:03.5162382Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-04T18:04:03.5162484Z Build completed unsuccessfully in 0:01:36
2019-10-04T18:04:03.5162484Z Build completed unsuccessfully in 0:01:36
2019-10-04T18:04:03.5217091Z == clock drift check ==
2019-10-04T18:04:03.5246201Z   local time: Fri Oct  4 18:04:03 UTC 2019
2019-10-04T18:04:03.8019457Z   network time: Fri, 04 Oct 2019 18:04:03 GMT
2019-10-04T18:04:03.8020054Z == end clock drift check ==
2019-10-04T18:04:05.1954794Z ##[error]Bash exited with code '1'.
2019-10-04T18:04:05.1990377Z ##[section]Starting: Checkout
2019-10-04T18:04:05.1992408Z ==============================================================================
2019-10-04T18:04:05.1992481Z Task         : Get sources
2019-10-04T18:04:05.1992528Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
