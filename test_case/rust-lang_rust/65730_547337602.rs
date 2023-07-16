plain
2019-10-29T09:24:49.1813466Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-29T09:24:49.2015881Z ##[command]git config gc.auto 0
2019-10-29T09:24:49.2091747Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-29T09:24:49.2125732Z ##[command]git config --get-all http.proxy
2019-10-29T09:24:49.2282878Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65730/merge:refs/remotes/pull/65730/merge
---
2019-10-29T09:30:58.2023504Z    Compiling serde_json v1.0.40
2019-10-29T09:30:59.8117415Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-29T09:31:10.4159996Z     Finished release [optimized] target(s) in 1m 22s
2019-10-29T09:31:10.4243268Z tidy check
2019-10-29T09:31:11.2132127Z tidy error: /checkout/src/librustc/infer/error_reporting/nice_region_error/static_impl_trait.rs:57: line longer than 100 chars
2019-10-29T09:31:11.2132512Z tidy error: /checkout/src/librustc/infer/error_reporting/nice_region_error/static_impl_trait.rs:58: line longer than 100 chars
2019-10-29T09:31:12.7019149Z some tidy checks failed
2019-10-29T09:31:12.7019911Z Found 484 error codes
2019-10-29T09:31:12.7020211Z Found 0 error codes with no tests
2019-10-29T09:31:12.7020442Z Done!
2019-10-29T09:31:12.7020442Z Done!
2019-10-29T09:31:12.7020626Z 
2019-10-29T09:31:12.7020796Z 
2019-10-29T09:31:12.7021992Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-29T09:31:12.7022560Z 
2019-10-29T09:31:12.7022644Z 
2019-10-29T09:31:12.7029231Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-29T09:31:12.7029464Z Build completed unsuccessfully in 0:01:25
2019-10-29T09:31:12.7029464Z Build completed unsuccessfully in 0:01:25
2019-10-29T09:31:12.7077720Z == clock drift check ==
2019-10-29T09:31:12.7095282Z   local time: Tue Oct 29 09:31:12 UTC 2019
2019-10-29T09:31:12.9776692Z   network time: Tue, 29 Oct 2019 09:31:12 GMT
2019-10-29T09:31:12.9777558Z == end clock drift check ==
2019-10-29T09:31:14.3722587Z 
2019-10-29T09:31:14.3819106Z ##[error]Bash exited with code '1'.
2019-10-29T09:31:14.3871120Z ##[section]Starting: Checkout
2019-10-29T09:31:14.3873142Z ==============================================================================
2019-10-29T09:31:14.3873199Z Task         : Get sources
2019-10-29T09:31:14.3873251Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
