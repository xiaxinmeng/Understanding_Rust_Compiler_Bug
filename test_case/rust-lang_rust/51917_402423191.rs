plain
Building stage0 tool remote-test-server (arm-unknown-linux-gnueabihf)
[00:48:08]    Compiling remote-test-server v0.1.0 (file:///checkout/src/tools/remote-test-server)
[00:48:08] error[E0463]: can't find crate for `std`
[00:48:08]   |
[00:48:08]   = note: the `arm-unknown-linux-gnueabihf` target may not be installed
[00:48:08] error: aborting due to previous error
[00:48:08] 
[00:48:08] For more information about this error, try `rustc --explain E0463`.
[00:48:08] [RUSTC-TIMING] remote_test_server test:false 0.078
[00:48:08] [RUSTC-TIMING] remote_test_server test:false 0.078
[00:48:08] error: Could not compile `remote-test-server`.
[00:48:08] 
[00:48:08] Caused by:
[00:48:08]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name remote_test_server tools/remote-test-server/src/main.rs --color always --error-format json --crate-type bin --emit=dep-info,link -C opt-level=2 -C metadata=476435fa0cc36d80 -C extra-filename=-476435fa0cc36d80 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/arm-unknown-linux-gnueabihf/release/deps --target arm-unknown-linux-gnueabihf -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/arm-unknown-linux-gnueabihf/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps` (exit code: 101)
[00:48:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "arm-unknown-linux-gnueabihf" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/remote-test-server/Cargo.toml" "--features" "" "--message-format" "json"
[00:48:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-unknown-linux-gnueabihf
[00:48:08] Build completed unsuccessfully in 0:44:53
travis_time:end:0e381e9b:start=1530693773481352566,finish=1530696661872009014,duration=2888390656448

---
travis_time:end:381d2526:start=1530696662455264350,finish=1530696662461458291,duration=6193941
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1092d090
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0162e4c0
$ dmesg | grep -i kill
