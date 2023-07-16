
2020-07-02T03:23:50.7570916Z thread '[compile-fail] compile-fail/intrinsics/exact_div2.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.5.0/src/runtest.rs:1091:13
2020-07-02T03:23:50.7571167Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-07-02T03:23:50.7571860Z error: tests/compile-fail/intrinsics/exact_div2.rs:4: unexpected error: '4:14: 4:49: Undefined Behavior: exact_div: 2_u16 cannot be divided by 3_u16 without remainder'
2020-07-02T03:23:50.7572313Z 
2020-07-02T03:23:50.7572948Z error: tests/compile-fail/intrinsics/exact_div2.rs:4: expected error not found: 2u16 cannot be divided by 3u16 without remainder
2020-07-02T03:23:50.7573060Z 
2020-07-02T03:23:50.7573210Z error: 1 unexpected errors found, 1 expected errors not found
2020-07-02T03:23:50.7573368Z status: exit code: 1
2020-07-02T03:23:50.7579207Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/intrinsics/exact_div2.rs" "-L" "/tmp/compiletestPb2EL8" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestPb2EL8/intrinsics/exact_div2.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestPb2EL8/intrinsics/exact_div2.stage-id.aux"
2020-07-02T03:23:50.7579531Z unexpected errors (from JSON output): [
2020-07-02T03:23:50.7579685Z     Error {
2020-07-02T03:23:50.7579828Z         line_num: 4,
2020-07-02T03:23:50.7579970Z         kind: Some(
2020-07-02T03:23:50.7580106Z             Error,
2020-07-02T03:23:50.7580242Z         ),
2020-07-02T03:23:50.7649233Z         msg: "4:14: 4:49: Undefined Behavior: exact_div: 2_u16 cannot be divided by 3_u16 without remainder",
2020-07-02T03:23:50.7654938Z     },
2020-07-02T03:23:50.7655087Z ]
2020-07-02T03:23:50.7655171Z 
2020-07-02T03:23:50.7655309Z not found errors (from test file): [
2020-07-02T03:23:50.7655453Z     Error {
2020-07-02T03:23:50.7655627Z         line_num: 4,
2020-07-02T03:23:50.7655768Z         kind: Some(
2020-07-02T03:23:50.7655903Z             Error,
2020-07-02T03:23:50.7656039Z         ),
2020-07-02T03:23:50.7883001Z thread '[compile-fail] compile-fail/intrinsics/exact_div3.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.5.0/src/runtest.rs:1091:13
2020-07-02T03:23:50.7883293Z         msg: "2u16 cannot be divided by 3u16 without remainder",
2020-07-02T03:23:50.7883503Z     },
2020-07-02T03:23:50.7883634Z ]
2020-07-02T03:23:50.7883701Z 
2020-07-02T03:23:50.7884060Z test [compile-fail] compile-fail/intrinsics/exact_div2.rs ... FAILED
2020-07-02T03:23:50.7884169Z 
2020-07-02T03:23:50.7884620Z error: tests/compile-fail/intrinsics/exact_div3.rs:4: unexpected error: '4:14: 4:50: Undefined Behavior: exact_div: -19_i8 cannot be divided by 2_i8 without remainder'
2020-07-02T03:23:50.7884754Z 
2020-07-02T03:23:50.7885158Z error: tests/compile-fail/intrinsics/exact_div3.rs:4: expected error not found: -19i8 cannot be divided by 2i8 without remainder
2020-07-02T03:23:50.7885265Z 
2020-07-02T03:23:50.7983767Z error: 1 unexpected errors found, 1 expected errors not found
2020-07-02T03:23:50.7984094Z status: exit code: 1
2020-07-02T03:23:50.7985624Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/intrinsics/exact_div3.rs" "-L" "/tmp/compiletestPb2EL8" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestPb2EL8/intrinsics/exact_div3.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestPb2EL8/intrinsics/exact_div3.stage-id.aux"
2020-07-02T03:23:50.7985977Z unexpected errors (from JSON output): [
2020-07-02T03:23:50.7986150Z     Error {
2020-07-02T03:23:50.7986292Z         line_num: 4,
2020-07-02T03:23:50.7986435Z         kind: Some(
2020-07-02T03:23:50.7986575Z             Error,
2020-07-02T03:23:50.7986980Z         ),
2020-07-02T03:23:50.7987467Z         msg: "4:14: 4:50: Undefined Behavior: exact_div: -19_i8 cannot be divided by 2_i8 without remainder",
2020-07-02T03:23:50.7987636Z     },
2020-07-02T03:23:50.7987789Z ]
2020-07-02T03:23:50.7987858Z 
2020-07-02T03:23:50.7987996Z not found errors (from test file): [
2020-07-02T03:23:50.7988141Z     Error {
2020-07-02T03:23:50.7988278Z         line_num: 4,
2020-07-02T03:23:50.7988418Z         kind: Some(
2020-07-02T03:23:50.7988692Z             Error,
2020-07-02T03:23:50.7988827Z         ),
2020-07-02T03:23:50.7989191Z         msg: "-19i8 cannot be divided by 2i8 without remainder",
2020-07-02T03:23:50.7989362Z     },
2020-07-02T03:23:50.7989495Z ]
2020-07-02T03:23:50.7989559Z 
