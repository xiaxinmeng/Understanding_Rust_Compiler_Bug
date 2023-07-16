plain
2019-10-27T18:55:16.2602718Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-27T18:55:16.2811262Z ##[command]git config gc.auto 0
2019-10-27T18:55:16.2872668Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-27T18:55:16.2927582Z ##[command]git config --get-all http.proxy
2019-10-27T18:55:16.3094649Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65794/merge:refs/remotes/pull/65794/merge
---
2019-10-27T19:01:41.6362711Z tidy check
2019-10-27T19:01:42.7967497Z * 586 error codes
2019-10-27T19:01:42.7967681Z * highest error code: E0741
2019-10-27T19:01:43.1743643Z * 267 features
2019-10-27T19:01:43.4131097Z tidy error: The Unstable Book has a 'language feature' section 'on-unimplemented' which doesn't correspond to an unstable language feature
2019-10-27T19:01:44.0925968Z Found 484 error codes
2019-10-27T19:01:44.0926120Z Found 0 error codes with no tests
2019-10-27T19:01:44.0926170Z Done!
2019-10-27T19:01:44.0926236Z some tidy checks failed
2019-10-27T19:01:44.0926236Z some tidy checks failed
2019-10-27T19:01:44.0926271Z 
2019-10-27T19:01:44.0926300Z 
2019-10-27T19:01:44.0927476Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-27T19:01:44.0927639Z 
2019-10-27T19:01:44.0927668Z 
2019-10-27T19:01:44.0931926Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-27T19:01:44.0932043Z Build completed unsuccessfully in 0:01:35
2019-10-27T19:01:44.0932043Z Build completed unsuccessfully in 0:01:35
2019-10-27T19:01:44.0982820Z == clock drift check ==
2019-10-27T19:01:44.0992916Z   local time: Sun Oct 27 19:01:44 UTC 2019
2019-10-27T19:01:44.1826124Z   network time: Sun, 27 Oct 2019 19:01:44 GMT
2019-10-27T19:01:44.1831164Z == end clock drift check ==
2019-10-27T19:01:45.5770878Z 
2019-10-27T19:01:45.5894247Z ##[error]Bash exited with code '1'.
2019-10-27T19:01:45.5929618Z ##[section]Starting: Checkout
2019-10-27T19:01:45.5931371Z ==============================================================================
2019-10-27T19:01:45.5931433Z Task         : Get sources
2019-10-27T19:01:45.5931504Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
