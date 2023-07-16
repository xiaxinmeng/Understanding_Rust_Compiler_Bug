
2020-06-16T06:21:09.6227910Z ---- [mir-opt] mir-opt/instrument_coverage.rs stdout ----
2020-06-16T06:21:09.6318830Z 
2020-06-16T06:21:09.6388710Z error: compilation failed!
2020-06-16T06:21:09.6422990Z status: exit code: 1
2020-06-16T06:21:09.6493760Z command: "/Users/runner/runners/2.170.1/work/1/s/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.170.1/work/1/s/src/test/mir-opt/instrument_coverage.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "-Zdump-mir=all" "-Zmir-opt-level=3" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/Users/runner/runners/2.170.1/work/1/s/build/x86_64-apple-darwin/test/mir-opt/instrument_coverage" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.170.1/work/1/s/build/x86_64-apple-darwin/test/mir-opt/instrument_coverage" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.170.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "-Zinstrument-coverage" "-L" "/Users/runner/runners/2.170.1/work/1/s/build/x86_64-apple-darwin/test/mir-opt/instrument_coverage/auxiliary"
2020-06-16T06:21:09.6522630Z stdout:
2020-06-16T06:21:09.6560550Z ------------------------------------------
2020-06-16T06:21:09.6619440Z 
2020-06-16T06:21:09.6653760Z ------------------------------------------
2020-06-16T06:21:09.6694350Z stderr:
2020-06-16T06:21:09.6746420Z ------------------------------------------
2020-06-16T06:21:09.6783130Z error: linking with `cc` failed: exit code: 1
2020-06-16T06:21:09.6846280Z   |
2020-06-16T06:21:09.6933950Z   = note: "cc" "-m64" "-L" "/Users/runner/runners/2.170.1/work/1/s/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/runner/runners/2.170.1/work/1/s/build/x86_64-apple-darwin/test/mir-opt/instrument_coverage/instrument_coverage.instrument_coverage.7rcbfp3g-cgu.0.rcgu.o" "/Users/runner/runners/2.170.1/work/1/s/build/x86_64-apple-darwin/test/mir-opt/instrument_coverage/instrument_coverage.instrument_coverage.7rcbfp3g-cgu.1.rcgu.o" "-o" "/Users/runner/runners/2.170.1/work/1/s/build/x86_64-apple-darwin/test/mir-opt/instrument_coverage/instrument_coverage" "-Wl,-dead_strip" "-no-pie" "-nodefaultlibs" "-L" "/Users/runner/runners/2.170.1/work/1/s/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/runners/2.170.1/work/1/s/build/x86_64-apple-darwin/test/mir-opt/instrument_coverage/auxiliary" "-L" "/Users/runner/runners/2.170.1/work/1/s/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "-L" "/Users/runner/runners/2.170.1/work/1/s/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "-lstd-4ee1eeb3ff245df9" "/Users/runner/runners/2.170.1/work/1/s/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-5ae1570bfb7fe005.rlib" "-lSystem" "-lresolv" "-lc" "-lm" "-Wl,-rpath,@loader_path/../../../stage2/lib/rustlib/x86_64-apple-darwin/lib" "-Wl,-rpath,/Users/runner/runners/2.170.1/work/1/s/lib/rustlib/x86_64-apple-darwin/lib"
2020-06-16T06:21:09.6966660Z   = note: clang: warning: argument unused during compilation: '-nopie' [-Wunused-command-line-argument]
2020-06-16T06:21:09.7028860Z           ld: warning: directory not found for option '-L/Users/runner/runners/2.170.1/work/1/s/build/x86_64-apple-darwin/test/mir-opt/instrument_coverage/auxiliary'
2020-06-16T06:21:09.7070920Z           Undefined symbols for architecture x86_64:
2020-06-16T06:21:09.7108460Z             "___llvm_profile_runtime", referenced from:
2020-06-16T06:21:09.7169740Z                 ___llvm_profile_runtime_user in instrument_coverage.instrument_coverage.7rcbfp3g-cgu.0.rcgu.o
2020-06-16T06:21:09.7233580Z                (maybe you meant: ___llvm_profile_runtime_user)
2020-06-16T06:21:09.7334980Z           ld: symbol(s) not found for architecture x86_64
2020-06-16T06:21:09.7410200Z           clang: error: linker command failed with exit code 1 (use -v to see invocation)
2020-06-16T06:21:09.7480460Z           
2020-06-16T06:21:09.7562460Z 
2020-06-16T06:21:09.7625570Z error: aborting due to previous error
