plain
2019-10-08T13:37:57.1749985Z note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
2019-10-08T13:37:57.1750134Z 
2019-10-08T13:37:57.1750476Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-08T13:37:57.1750592Z 
2019-10-08T13:37:57.1751154Z note: compiler flags: -Z external-macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=y --crate-type lib
2019-10-08T13:37:57.1751417Z note: some of the compiler flags provided by cargo are hidden
2019-10-08T13:37:57.1751495Z 
2019-10-08T13:37:57.2667177Z [RUSTC-TIMING] rustc test:false 0.874
2019-10-08T13:37:57.2667690Z error: could not compile `rustc`.
2019-10-08T13:37:57.2667690Z error: could not compile `rustc`.
2019-10-08T13:37:57.2668064Z warning: build failed, waiting for other jobs to finish...
2019-10-08T13:39:44.6167192Z [RUSTC-TIMING] syntax test:false 169.689
2019-10-08T13:39:44.6277126Z error: build failed
2019-10-08T13:39:44.6300673Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-10-08T13:39:44.6301020Z expected success, got: exit code: 101
2019-10-08T13:39:44.6320816Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/tools/cargo src/tools/cargotest
2019-10-08T13:39:44.6321030Z Build completed unsuccessfully in 0:45:54
2019-10-08T13:39:44.6377871Z Makefile:50: recipe for target 'check-aux' failed
2019-10-08T13:39:44.6379143Z make: *** [check-aux] Error 1
2019-10-08T13:39:44.6404435Z   local time: Tue Oct  8 13:39:44 UTC 2019
2019-10-08T13:39:44.7954209Z   network time: Tue, 08 Oct 2019 13:39:44 GMT
2019-10-08T13:39:44.7954426Z == end clock drift check ==
2019-10-08T13:39:44.7954426Z == end clock drift check ==
2019-10-08T13:39:46.0090261Z ##[error]Bash exited with code '2'.
2019-10-08T13:39:46.0147691Z ##[section]Starting: Upload CPU usage statistics
2019-10-08T13:39:46.0157924Z ==============================================================================
2019-10-08T13:39:46.0158083Z Task         : Bash
2019-10-08T13:39:46.0158188Z Description  : Run a Bash script on macOS, Linux, or Windows
