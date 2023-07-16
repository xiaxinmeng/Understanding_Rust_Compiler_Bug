\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/resolve/privacy-enum-ctor.rs","byte_start":944,"byte_end":949,"line_start":37,"line_end":37,"column_start":20,"column_end":25,"is_primary":true,"text":[{"text":"        let _: Z = Z::Fn;","highlight_start":20,"highlight_end":25}],"label":"expected enum `m::n::Z`, found fn item","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `m::n::Z`\n   found type `fn(u8) -> m::n::Z {m::n::Z::Fn}`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:37:20\n   |\nLL |         let _: Z = Z::Fn;\n   |                    ^^^^^ expected enum `m::n::Z`, found fn item\n   |\n   = note: expected type `m::n::Z`\n              found type `fn(u8) -> m::n::Z {m::n::Z::Fn}`\n\n"}
[01:05:22] thread 'main' panicked at 'attempt to subtract with overflow', librustc_errors/emitter.rs:1288:34
[01:05:22] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:05:22] {"message":"aborting due to 18 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 18 previous errors\n\n"}
[01:05:22] {"message":"Some errors occurred: E0308, E0412, E0423, E0603, E0618.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0308, E0412, E0423, E0603, E0618.\n"}
[01:05:22] {"message":"For more information about an error, try `rustc --explain E0308`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0308`.\n"}
[01:05:22] error: internal compiler error: unexpected panic
[01:05:22] 
[01:05:22] 
[01:05:22] note: the compiler unexpectedly panicked. this is a bug.
[01:05:22] 
[01:05:22] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:05:22Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:22] 
[01:05:22] 
[01:05:22] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:05:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:22] Build completed unsuccessfully in 0:03:35
[01:05:22] make: *** [check] Error 1
[01:05:22] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1a117df2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
