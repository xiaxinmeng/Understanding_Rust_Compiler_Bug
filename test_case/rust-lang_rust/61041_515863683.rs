plain
2019-07-29T06:00:28.7017167Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-29T06:00:28.7206287Z ##[command]git config gc.auto 0
2019-07-29T06:00:28.7280021Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-29T06:00:28.7327593Z ##[command]git config --get-all http.proxy
2019-07-29T06:00:28.7463657Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61041/merge:refs/remotes/pull/61041/merge
---
2019-07-29T06:01:04.2269787Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-29T06:01:04.2269819Z 
2019-07-29T06:01:04.2270043Z   git checkout -b <new-branch-name>
2019-07-29T06:01:04.2270074Z 
2019-07-29T06:01:04.2270138Z HEAD is now at 20e3d2a99 Merge 266f17f41b4de1eecb524b3b1a07c08d8b368dee into 8b94e9e9188b65df38a5f1ae723617dc2dfb3155
2019-07-29T06:01:04.2441751Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-29T06:01:04.2444986Z ==============================================================================
2019-07-29T06:01:04.2445050Z Task         : Bash
2019-07-29T06:01:04.2445120Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-29T06:33:26.9323672Z    Compiling rand v0.6.1
2019-07-29T06:33:27.1507252Z    Compiling parking_lot_core v0.4.0
2019-07-29T06:33:27.3317910Z    Compiling rustc_apfloat v0.0.0 (/checkout/src/librustc_apfloat)
2019-07-29T06:33:27.6781736Z    Compiling quote v0.6.12
2019-07-29T06:33:28.3789314Z error: internal compiler error: src/librustc/traits/codegen/mod.rs:154: Encountered errors `[FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<ppc::FallbackS<F> as Float>)), depth=1),Unimplemented)]` resolving bounds after type-checking
2019-07-29T06:33:28.3789460Z 
2019-07-29T06:33:28.3798410Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:644:9
2019-07-29T06:33:28.3911732Z error: aborting due to previous error
2019-07-29T06:33:28.3911795Z 
2019-07-29T06:33:28.3955930Z 
2019-07-29T06:33:28.3955930Z 
2019-07-29T06:33:28.3956045Z note: the compiler unexpectedly panicked. this is a bug.
2019-07-29T06:33:28.3956082Z 
2019-07-29T06:33:28.3956917Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-07-29T06:33:28.3956958Z 
2019-07-29T06:33:28.3957378Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-07-29T06:33:28.3957459Z 
2019-07-29T06:33:28.3957821Z note: compiler flags: -Z unstable-options -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C prefer-dynamic -C debug-assertions=n -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
2019-07-29T06:33:28.3957881Z 
2019-07-29T06:33:28.3957949Z note: some of the compiler flags provided by cargo are hidden
2019-07-29T06:33:28.3996019Z error: Could not compile `rustc_apfloat`.
2019-07-29T06:33:28.4012314Z warning: build failed, waiting for other jobs to finish...
2019-07-29T06:33:29.1578395Z error: build failed
2019-07-29T06:33:29.1604725Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-29T06:33:29.1604725Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-29T06:33:29.1606268Z expected success, got: exit code: 101
2019-07-29T06:33:29.1616247Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-29T06:33:29.1616370Z Build completed unsuccessfully in 0:26:06
2019-07-29T06:33:30.9402831Z ##[error]Bash exited with code '1'.
2019-07-29T06:33:30.9440192Z ##[section]Starting: Checkout
2019-07-29T06:33:30.9441638Z ==============================================================================
2019-07-29T06:33:30.9441703Z Task         : Get sources
2019-07-29T06:33:30.9441747Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
