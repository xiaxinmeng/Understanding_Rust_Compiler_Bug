plain
2019-08-20T16:25:42.7444085Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-20T16:25:42.7624033Z ##[command]git config gc.auto 0
2019-08-20T16:25:42.7702809Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-20T16:25:42.7759026Z ##[command]git config --get-all http.proxy
2019-08-20T16:25:42.7888184Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63749/merge:refs/remotes/pull/63749/merge
---
2019-08-20T16:26:18.6713740Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-20T16:26:18.6713994Z 
2019-08-20T16:26:18.6714478Z   git checkout -b <new-branch-name>
2019-08-20T16:26:18.6714707Z 
2019-08-20T16:26:18.6714912Z HEAD is now at 041058cbd Merge b1a9e9edb1004f725d253dd3bf444fa61b74d4ca into 51879c3abaedb926739095d19a2af638ee6a07d8
2019-08-20T16:26:18.6876822Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-20T16:26:18.6880097Z ==============================================================================
2019-08-20T16:26:18.6880167Z Task         : Bash
2019-08-20T16:26:18.6880219Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-20T16:58:03.6430192Z    Compiling smallvec v0.6.10
2019-08-20T16:58:03.7093301Z error[E0658]: specialization is unstable
2019-08-20T16:58:03.7094768Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/smallvec-0.6.10/lib.rs:1259:5
2019-08-20T16:58:03.7096485Z      |
2019-08-20T16:58:03.7097355Z 1259 |     default fn spec_from(slice: &'a [A::Item]) -> SmallVec<A> {
2019-08-20T16:58:03.7098357Z      |
2019-08-20T16:58:03.7098357Z      |
2019-08-20T16:58:03.7099059Z      = note: for more information, see ***/issues/31844
2019-08-20T16:58:03.7099693Z      = help: add `#![feature(specialization)]` to the crate attributes to enable
2019-08-20T16:58:03.9582354Z error: aborting due to previous error
2019-08-20T16:58:03.9586415Z 
2019-08-20T16:58:03.9595528Z For more information about this error, try `rustc --explain E0658`.
2019-08-20T16:58:03.9595528Z For more information about this error, try `rustc --explain E0658`.
2019-08-20T16:58:03.9707882Z error: Could not compile `smallvec`.
2019-08-20T16:58:04.3932754Z error: build failed
2019-08-20T16:58:04.3944379Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-20T16:58:04.3944581Z expected success, got: exit code: 101
2019-08-20T16:58:04.3958627Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-20T16:58:04.3958627Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-20T16:58:04.3958737Z Build completed unsuccessfully in 0:25:08
2019-08-20T16:58:04.4018117Z == clock drift check ==
2019-08-20T16:58:04.4034516Z   local time: Tue Aug 20 16:58:04 UTC 2019
2019-08-20T16:58:04.5616731Z   network time: Tue, 20 Aug 2019 16:58:04 GMT
2019-08-20T16:58:04.5617469Z == end clock drift check ==
2019-08-20T16:58:05.5193879Z ##[error]Bash exited with code '1'.
2019-08-20T16:58:05.5239608Z ##[section]Starting: Checkout
2019-08-20T16:58:05.5241836Z ==============================================================================
2019-08-20T16:58:05.5241901Z Task         : Get sources
2019-08-20T16:58:05.5241977Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
