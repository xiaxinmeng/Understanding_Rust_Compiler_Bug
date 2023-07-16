plain
2019-10-22T11:01:46.0069080Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-22T11:01:46.0265594Z ##[command]git config gc.auto 0
2019-10-22T11:01:46.0333631Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-22T11:01:46.0401731Z ##[command]git config --get-all http.proxy
2019-10-22T11:01:46.0529179Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65627/merge:refs/remotes/pull/65627/merge
---
2019-10-22T11:08:07.7078156Z    Compiling serde_json v1.0.40
2019-10-22T11:08:09.3997784Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-22T11:08:20.6663755Z     Finished release [optimized] target(s) in 1m 27s
2019-10-22T11:08:20.6748365Z tidy check
2019-10-22T11:08:21.7466678Z tidy error: duplicate error code: 740
2019-10-22T11:08:21.7466806Z tidy error: /checkout/src/librustc_typeck/error_codes.rs:4866: E0740: r##"
2019-10-22T11:08:21.7466862Z tidy error: /checkout/src/librustc_typeck/error_codes.rs:4981: E0740: r##"
2019-10-22T11:08:23.0193757Z Found 483 error codes
2019-10-22T11:08:23.0195522Z Found 0 error codes with no tests
2019-10-22T11:08:23.0195871Z Done!
2019-10-22T11:08:23.0196358Z some tidy checks failed
2019-10-22T11:08:23.0196358Z some tidy checks failed
2019-10-22T11:08:23.0197571Z 
2019-10-22T11:08:23.0197866Z 
2019-10-22T11:08:23.0199121Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-22T11:08:23.0199712Z 
2019-10-22T11:08:23.0199935Z 
2019-10-22T11:08:23.0200916Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-22T11:08:23.0201207Z Build completed unsuccessfully in 0:01:30
2019-10-22T11:08:23.0201207Z Build completed unsuccessfully in 0:01:30
2019-10-22T11:08:23.0244616Z == clock drift check ==
2019-10-22T11:08:23.0253687Z   local time: Tue Oct 22 11:08:23 UTC 2019
2019-10-22T11:08:23.2896209Z   network time: Tue, 22 Oct 2019 11:08:23 GMT
2019-10-22T11:08:23.2903120Z == end clock drift check ==
2019-10-22T11:08:24.7084510Z 
2019-10-22T11:08:24.7195218Z ##[error]Bash exited with code '1'.
2019-10-22T11:08:24.7226114Z ##[section]Starting: Checkout
2019-10-22T11:08:24.7227735Z ==============================================================================
2019-10-22T11:08:24.7227784Z Task         : Get sources
2019-10-22T11:08:24.7227826Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
