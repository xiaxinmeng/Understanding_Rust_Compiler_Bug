
2020-07-01T21:00:40.5186999Z 
2020-07-01T21:00:40.5188488Z error: tests/compile-fail/intrinsics/exact_div3.rs:4: unexpected error: '4:14: 4:50: Undefined Behavior: exact_div: -19_i8 cannot be divided by 2_i8 without remainder'
2020-07-01T21:00:40.5188643Z 
2020-07-01T21:00:40.5189120Z error: tests/compile-fail/intrinsics/exact_div3.rs:4: expected error not found: -19i8 cannot be divided by 2i8 without remainder
2020-07-01T21:00:40.5189241Z 
2020-07-01T21:00:40.5189397Z error: 1 unexpected errors found, 1 expected errors not found
2020-07-01T21:00:40.5189551Z status: exit code: 1
2020-07-01T21:00:40.5236387Z thread '[compile-fail] compile-fail/intrinsics/exact_div3.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.5.0/src/runtest.rs:1091:13
2020-07-01T21:00:40.5237301Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/intrinsics/exact_div3.rs" "-L" "/tmp/compiletestA5dhBv" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestA5dhBv/intrinsics/exact_div3.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestA5dhBv/intrinsics/exact_div3.stage-id.aux"
2020-07-01T21:00:40.5237565Z unexpected errors (from JSON output): [
2020-07-01T21:00:40.5237714Z     Error {
2020-07-01T21:00:40.5237849Z         line_num: 4,
2020-07-01T21:00:40.5237987Z         kind: Some(
2020-07-01T21:00:40.5238117Z             Error,
2020-07-01T21:00:40.5238245Z         ),
2020-07-01T21:00:40.5238656Z         msg: "4:14: 4:50: Undefined Behavior: exact_div: -19_i8 cannot be divided by 2_i8 without remainder",
2020-07-01T21:00:40.5238841Z     },
2020-07-01T21:00:40.5238965Z ]
2020-07-01T21:00:40.5397157Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-07-01T21:00:40.5397719Z 
2020-07-01T21:00:40.5397863Z not found errors (from test file): [
2020-07-01T21:00:40.5398002Z     Error {
2020-07-01T21:00:40.5398133Z         line_num: 4,
2020-07-01T21:00:40.5398266Z         kind: Some(
2020-07-01T21:00:40.5476635Z thread '[compile-fail] compile-fail/intrinsics/exact_div2.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.5.0/src/runtest.rs:1091:13
2020-07-01T21:00:40.5640575Z             Error,
2020-07-01T21:00:40.5641183Z         ),
2020-07-01T21:00:40.5642120Z         msg: "-19i8 cannot be divided by 2i8 without remainder",
2020-07-01T21:00:40.5642303Z     },
2020-07-01T21:00:40.5642431Z ]
2020-07-01T21:00:40.5642494Z 
2020-07-01T21:00:40.5642832Z test [compile-fail] compile-fail/intrinsics/exact_div3.rs ... FAILED
2020-07-01T21:00:40.5643173Z test [compile-fail] compile-fail/intrinsics/exact_div4.rs ... ok
2020-07-01T21:00:40.5643275Z 
2020-07-01T21:00:40.5643726Z error: tests/compile-fail/intrinsics/exact_div2.rs:4: unexpected error: '4:14: 4:49: Undefined Behavior: exact_div: 2_u16 cannot be divided by 3_u16 without remainder'
2020-07-01T21:00:40.5643874Z 
2020-07-01T21:00:40.5644274Z error: tests/compile-fail/intrinsics/exact_div2.rs:4: expected error not found: 2u16 cannot be divided by 3u16 without remainder
2020-07-01T21:00:40.5644375Z 
2020-07-01T21:00:40.5644534Z error: 1 unexpected errors found, 1 expected errors not found
2020-07-01T21:00:40.5644682Z status: exit code: 1
2020-07-01T21:00:40.5645490Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/intrinsics/exact_div2.rs" "-L" "/tmp/compiletestA5dhBv" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestA5dhBv/intrinsics/exact_div2.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestA5dhBv/intrinsics/exact_div2.stage-id.aux"
2020-07-01T21:00:40.5645729Z unexpected errors (from JSON output): [
2020-07-01T21:00:40.5645874Z     Error {
2020-07-01T21:00:40.5646007Z         line_num: 4,
2020-07-01T21:00:40.5646142Z         kind: Some(
2020-07-01T21:00:40.5646269Z             Error,
2020-07-01T21:00:40.5646398Z         ),
2020-07-01T21:00:40.5646577Z         msg: "4:14: 4:49: Undefined Behavior: exact_div: 2_u16 cannot be divided by 3_u16 without remainder",
2020-07-01T21:00:40.5646737Z     },
2020-07-01T21:00:40.5646862Z ]
2020-07-01T21:00:40.5646922Z 
2020-07-01T21:00:40.5647056Z not found errors (from test file): [
2020-07-01T21:00:40.5647193Z     Error {
2020-07-01T21:00:40.5647323Z         line_num: 4,
2020-07-01T21:00:40.5647471Z         kind: Some(
2020-07-01T21:00:40.5647599Z             Error,
2020-07-01T21:00:40.5647726Z         ),
2020-07-01T21:00:40.5647871Z         msg: "2u16 cannot be divided by 3u16 without remainder",
2020-07-01T21:00:40.5648015Z     },
2020-07-01T21:00:40.5648139Z ]
2020-07-01T21:00:40.5648200Z 
