plain
[00:06:36]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:06:50] error: unreachable pattern
[00:06:50]    --> libsyntax/test.rs:361:21
[00:06:50]     |
[00:06:50] 361 |                     (true, _) => No(BadTestSignature::WrongTypeSignature),
[00:06:50]     |
[00:06:50]     = note: `-D unreachable-patterns` implied by `-D warnings`
[00:06:50] 
[00:06:50] error: aborting due to previous error
[00:06:50] error: aborting due to previous error
[00:06:50] 
[00:06:51] error: Could not compile `syntax`.
[00:06:51] 
[00:06:51] Caused by:
[00:06:51]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax libsyntax/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=4c5434c80172b18c -C extra-filename=-4c5434c80172b18c --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-58741ed9de9aae4f.so --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-b76c070114255d98.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-5073f1296cd24b67.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-41b116eaee1e5535.so --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-99b4534960f92449.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-ary/Logs/DiagnosticReports/: No such file or directory
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0d6266f7
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
