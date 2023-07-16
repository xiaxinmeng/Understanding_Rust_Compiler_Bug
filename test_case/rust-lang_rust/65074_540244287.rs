plain
2019-10-09T22:50:30.8819782Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-09T22:50:30.8908229Z ##[command]git config gc.auto 0
2019-10-09T22:50:30.8955436Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-09T22:50:30.9003513Z ##[command]git config --get-all http.proxy
2019-10-09T22:50:30.9148622Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65074/merge:refs/remotes/pull/65074/merge
---
2019-10-09T22:57:08.8105398Z    Compiling serde_json v1.0.40
2019-10-09T22:57:10.6437503Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-09T22:57:22.5208806Z     Finished release [optimized] target(s) in 1m 30s
2019-10-09T22:57:22.5293426Z tidy check
2019-10-09T22:57:22.9111773Z tidy error: /checkout/src/test/ui/json-bom-plus-crlf-multifile-aux.rs: ignoring CR characters unnecessarily
2019-10-09T22:57:24.7303554Z some tidy checks failed
2019-10-09T22:57:24.7303958Z Found 482 error codes
2019-10-09T22:57:24.7304022Z Found 0 error codes with no tests
2019-10-09T22:57:24.7304817Z Done!
2019-10-09T22:57:24.7304817Z Done!
2019-10-09T22:57:24.7304855Z 
2019-10-09T22:57:24.7304881Z 
2019-10-09T22:57:24.7305783Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-09T22:57:24.7305928Z 
2019-10-09T22:57:24.7305953Z 
2019-10-09T22:57:24.7312895Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-09T22:57:24.7312998Z Build completed unsuccessfully in 0:01:34
2019-10-09T22:57:24.7312998Z Build completed unsuccessfully in 0:01:34
2019-10-09T22:57:24.7368326Z == clock drift check ==
2019-10-09T22:57:24.7395545Z   local time: Wed Oct  9 22:57:24 UTC 2019
2019-10-09T22:57:24.8486483Z   network time: Wed, 09 Oct 2019 22:57:24 GMT
2019-10-09T22:57:24.8492906Z == end clock drift check ==
2019-10-09T22:57:25.8533123Z ##[error]Bash exited with code '1'.
2019-10-09T22:57:25.8582671Z ##[section]Starting: Checkout
2019-10-09T22:57:25.8584518Z ==============================================================================
2019-10-09T22:57:25.8584575Z Task         : Get sources
2019-10-09T22:57:25.8584622Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
