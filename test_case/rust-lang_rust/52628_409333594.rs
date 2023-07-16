plain
[01:13:10]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[01:13:14] error[E0308]: mismatched types
[01:13:14]    --> librustdoc/html/markdown.rs:973:9
[01:13:14]     |
[01:13:14] 973 |         Markdown(markdown, &[], RefCell::new(&mut map), ErrorCodes::Yes).to_string()
[01:13:14]     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found struct `std::string::String`
[01:13:14]     = note: expected type `()`
[01:13:14]                found type `std::string::String`
[01:13:14] help: try adding a semicolon
[01:13:14]     |
[01:13:14]     |
[01:13:14] 973 |         Markdown(markdown, &[], RefCell::new(&mut map), ErrorCodes::Yes).to_string();
[01:13:14] help: try adding a return type
[01:13:14]     |
[01:13:14] 970 |     fn issue_17736() -> std::string::String {
[01:13:14]     |                      ^^^^^^^^^^^^^^^^^^^^^^
---
[01:13:16] 
[01:13:16] To learn more, run the command again with --verbose.
[01:13:16] 
[01:13:16] 
[01:13:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"
[01:13:16] 
[01:13:16] 
[01:13:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:16] Build completed unsuccessfully in 0:28:59
[01:13:16] Build completed unsuccessfully in 0:28:59
[01:13:16] Makefile:58: recipe for target 'check' failed
[01:13:16] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:053d86be
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0ccabd24:start=1533064015851188629,finish=1533064015858108174,duration=6919545
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:050590b0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:195d028c
travis_time:start:195d028c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09c66a52
$ dmesg | grep -i kill
