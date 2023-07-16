
2019-08-30T09:21:21.4479192Z error: failure produced the wrong error: exit code: 101
2019-08-30T09:21:21.4479270Z status: exit code: 101
2019-08-30T09:21:21.4479913Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/validity/invalid_char.rs" "-L" "/tmp/compiletestkZNor4" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestkZNor4/validity/invalid_char.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestkZNor4/validity/invalid_char.stage-id.aux" "-A" "unused"
2019-08-30T09:21:21.4480668Z stdout:
2019-08-30T09:21:21.4481022Z ------------------------------------------
2019-08-30T09:21:21.4481081Z 
2019-08-30T09:21:21.4481328Z ------------------------------------------
2019-08-30T09:21:21.4481434Z stderr:
2019-08-30T09:21:21.4481671Z ------------------------------------------
2019-08-30T09:21:21.4482018Z thread 'rustc' panicked at 'attempt to subtract with overflow', src/librustc_errors/emitter.rs:154:38
2019-08-30T09:21:21.4482129Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-30T09:21:21.4482208Z 
2019-08-30T09:21:21.4482275Z error: internal compiler error: unexpected panic
2019-08-30T09:21:21.4482342Z 
2019-08-30T09:21:21.4482411Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-30T09:21:21.4482462Z 
2019-08-30T09:21:21.4482813Z note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
2019-08-30T09:21:21.4482896Z 
2019-08-30T09:21:21.4483177Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-30T09:21:21.4483234Z 
2019-08-30T09:21:21.4483485Z note: compiler flags: -C prefer-dynamic
2019-08-30T09:21:21.4483538Z 
2019-08-30T09:21:21.4483574Z 
2019-08-30T09:21:21.4484155Z ------------------------------------------
2019-08-30T09:21:21.4484197Z 
2019-08-30T09:21:21.4484526Z thread '[compile-fail] compile-fail/validity/invalid_char.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-08-30T09:21:21.4484644Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-30T09:21:21.4493103Z test [compile-fail] compile-fail/validity/invalid_char.rs ... FAILED
