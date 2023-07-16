plain
2019-09-10T05:46:45.0546588Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-10T05:46:45.0744274Z ##[command]git config gc.auto 0
2019-09-10T05:46:45.0821908Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-10T05:46:45.0878206Z ##[command]git config --get-all http.proxy
2019-09-10T05:46:45.1021325Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64334/merge:refs/remotes/pull/64334/merge
---
2019-09-10T05:53:41.5776415Z    Compiling serde_json v1.0.40
2019-09-10T05:53:43.4567224Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-10T05:53:54.3404941Z     Finished release [optimized] target(s) in 1m 29s
2019-09-10T05:53:54.3494055Z tidy check
2019-09-10T05:53:54.5032909Z tidy error: /checkout/src/librustc_target/spec/i686_unknown_uefi.rs:3: line longer than 100 chars
2019-09-10T05:53:54.5033937Z tidy error: /checkout/src/librustc_target/spec/i686_unknown_uefi.rs:81: line longer than 100 chars
2019-09-10T05:53:54.5034289Z tidy error: /checkout/src/librustc_target/spec/i686_unknown_uefi.rs:82: line longer than 100 chars
2019-09-10T05:53:56.3215278Z some tidy checks failed
2019-09-10T05:53:56.3221303Z 
2019-09-10T05:53:56.3221303Z 
2019-09-10T05:53:56.3222619Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-10T05:53:56.3223150Z 
2019-09-10T05:53:56.3223266Z 
2019-09-10T05:53:56.3229491Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-10T05:53:56.3229933Z Build completed unsuccessfully in 0:01:32
2019-09-10T05:53:56.3229933Z Build completed unsuccessfully in 0:01:32
2019-09-10T05:53:56.3283365Z == clock drift check ==
2019-09-10T05:53:56.3298757Z   local time: Tue Sep 10 05:53:56 UTC 2019
2019-09-10T05:53:56.4022934Z   network time: Tue, 10 Sep 2019 05:53:56 GMT
2019-09-10T05:53:56.4025567Z == end clock drift check ==
2019-09-10T05:53:57.8546981Z ##[error]Bash exited with code '1'.
2019-09-10T05:53:57.8580959Z ##[section]Starting: Checkout
2019-09-10T05:53:57.8582963Z ==============================================================================
2019-09-10T05:53:57.8583033Z Task         : Get sources
2019-09-10T05:53:57.8583074Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
