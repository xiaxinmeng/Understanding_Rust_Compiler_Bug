plain
2019-10-15T11:21:05.7728811Z    Compiling cc v1.0.35
2019-10-15T11:21:05.7729545Z    Compiling core v0.0.0 (/checkout/src/libcore)
2019-10-15T11:21:11.6793778Z [RUSTC-TIMING] cc test:false 5.909
2019-10-15T11:21:11.6836279Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-10-15T11:21:12.6246943Z thread 'rustc' panicked at 'attempt to add with overflow', /checkout/src/librustc_index/bit_set.rs:337:13
2019-10-15T11:21:12.6248022Z 
2019-10-15T11:21:12.6248090Z error: internal compiler error: unexpected panic
2019-10-15T11:21:12.6248170Z 
2019-10-15T11:21:12.6248237Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-15T11:21:12.6248237Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-15T11:21:12.6248286Z 
2019-10-15T11:21:12.6249307Z note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
2019-10-15T11:21:12.6249395Z 
2019-10-15T11:21:12.6249720Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-15T11:21:12.6249776Z 
2019-10-15T11:21:12.6250283Z note: compiler flags: -Z external-macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=y --crate-type lib
2019-10-15T11:21:12.6250508Z note: some of the compiler flags provided by cargo are hidden
2019-10-15T11:21:12.6250562Z 
2019-10-15T11:21:12.6250816Z [RUSTC-TIMING] core test:false 6.849
2019-10-15T11:21:12.6251088Z error: could not compile `core`.
2019-10-15T11:21:12.6251088Z error: could not compile `core`.
2019-10-15T11:21:12.6251426Z warning: build failed, waiting for other jobs to finish...
2019-10-15T11:21:12.7618681Z [RUSTC-TIMING] build_helper test:false 1.074
2019-10-15T11:21:12.7674613Z error: build failed
2019-10-15T11:21:12.7697371Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-10-15T11:21:12.7697756Z expected success, got: exit code: 101
2019-10-15T11:21:12.7713271Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/tools/cargo src/tools/cargotest
2019-10-15T11:21:12.7713416Z Build completed unsuccessfully in 0:32:59
2019-10-15T11:21:12.7760918Z make: *** [check-aux] Error 1
2019-10-15T11:21:12.7761225Z Makefile:50: recipe for target 'check-aux' failed
2019-10-15T11:21:12.7788249Z   local time: Tue Oct 15 11:21:12 UTC 2019
2019-10-15T11:21:12.9280513Z   network time: Tue, 15 Oct 2019 11:21:12 GMT
2019-10-15T11:21:12.9282568Z == end clock drift check ==
2019-10-15T11:21:12.9282568Z == end clock drift check ==
2019-10-15T11:21:16.2064429Z ##[error]Bash exited with code '2'.
2019-10-15T11:21:16.2110018Z ##[section]Starting: Upload CPU usage statistics
2019-10-15T11:21:16.2122303Z ==============================================================================
2019-10-15T11:21:16.2122582Z Task         : Bash
2019-10-15T11:21:16.2122817Z Description  : Run a Bash script on macOS, Linux, or Windows
