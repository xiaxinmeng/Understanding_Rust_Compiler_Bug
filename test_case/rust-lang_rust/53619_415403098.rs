plain
[00:46:26] ....................................................................................................
[00:46:29] ....................................................................................................
[00:46:32] ....................................................................................................
[00:46:35] ..............i.....................................................................................
[00:46:40] ..............................F.....................................................................
[00:46:46] ..................ii.iii............................................................................
[00:46:49] ....................................................................................................
[00:46:51] ....................................................................................................
[00:46:53] ....................................................................................................
---
[00:48:07] ....................................................................................................
[00:48:11] .........................................................i..........................................
[00:48:15] ....................................................................................................
[00:48:18] ....................................................................................................
ld/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main/const_panic_libcore_main.stderr
[00:48:22] To update references, rerun the tests and pass the `--bless` flag
[00:48:22] To only update this specific test, also pass `--test-args consts/const-eval/const_panic_libcore_main.rs`
[00:48:22] error: 1 errors occurred comparing output.
[00:48:22] status: exit code: 1
[00:48:22] status: exit code: 1
[00:48:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main/auxiliary" "-A" "unused"
[00:48:22] ------------------------------------------
[00:48:22] 
[00:48:22] ------------------------------------------
[00:48:22] stderr:
[00:48:22] stderr:
[00:48:22] ------------------------------------------
[00:48:22] {"message":"use of deprecated attribute `panic_implementation`: This attribute was renamed to `panic_handler`. See https://github.com/rust-lang/rust/issues/44489#issuecomment-415140224","code":{"code":"deprecated","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs","byte_start":936,"byte_end":959,"line_start":34,"line_end":34,"column_sart":1,"highlight_end":46},{"text":"file ! (  ) , line ! (  ) , __rust_unstable_column ! (  ) ) ) } ) ;","highlight_start":1,"highlight_end":68}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"/checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs","byte_start":626,"byte_end":657,"line_start":20,"line_end":20,"column_start":1,"column_end":32,"is_primary":true,"text":[{"text":"const Z: () = panic!(\"cheese\");","highlight_start":1,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[deny(const_err)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this constant cannot be used\n  --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:20:1\n   |\nLL | const Z: () = panic!(\"cheese\");\n   | ^^^^^^^^^^^^^^----------------^\n   |               |\n   |               the evaluated program panicked at 'cheese', /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:20:15\n   |\n   = note: #[deny(const_err)] on by default\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[00:48:22] {"message":"this constant cannot be used","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"<panic macros>","byte_start":67,"byte_end":172,"line_start":3,"line_end":4,"column_start":1,"column_end":74,"is_primary":false,"text":[{"text":"$ crate :: panicking :: panic (","highlight_msg ) } ) ; ( $ fmt : expr , $ ( $ arg : tt ) * ) =>","highlight_start":1,"highlight_end":78},{"text":"(","highlight_start":1,"highlight_end":2},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"panic ! (","highlight_start":1,"highlight_end":10},{"text":"concat ! ( \"internal error: entered unreachable code: \" , $ fmt ) , $ ( $ arg","highlight_start":1,"highlight_end":78},{"text":") * ) } ) ;","highlight_start":1,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"panic!","def_site_span":{"file_name":"<panic macros>","byte_start":0,"byte_end":419,"line_start":1,"line_end":10,"column_start":1,"column_end":68,"is_primary":false,"text":[{"text":"(  ) => ( panic ! ( \"explicit panic\" ) ) ; ( $ msg : expr ) => (","highlight_start":1,"highlight_end":65},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"$ crate :: panicking :: panic (","highlight_start":1,"highlight_end":32},{"text":"& ( $ msg , file ! (  ) , line ! (  ) , __rust_unstable_column ! (  ) ) ) } )","highlight_start":1,"highlight_end":78},{"text":"; ( $ msg : expr , ) => ( panic ! ( $ msg ) ) ; (","highlight_start":1,"highlight_end":50},{"text":"$ fmt : expr , $ ( $ arg : tt ) + ) => (","highlight_start":1,"highlight_end":41},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"$ crate :: panicking :: panic_fmt (","highlight_start":1,"highlight_end":36},{"text":"format_args ! ( $ fmt , $ ( $ arg ) * ) , & (","highlight_start":1,"highlight_end":46},{"text":"file ! (  ) , line ! (  ) , __rust_unstable_column ! (  ) ) ) } ) ;","highlight_start":1,"highlight_end":68 at 'not yet implemented', /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:26:15\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[00:48:22] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:48:22] ------------------------------------------
[00:48:22] 
[00:48:22] thread '[ui] ui/consts/const-eval/const_panic_libcore_main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:48:22] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:48:22] 
[00:48:22] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:48:22] 
[00:48:22] 
[00:48:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:48:22] 
[00:48:22] 
[00:48:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:48:22] Build completed unsuccessfully in 0:03:08
[00:48:22] Build completed unsuccessfully in 0:03:08
[00:48:22] make: *** [check] Error 1
[00:48:22] Makefile:58: recipe for target 'check' failed
128740 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
126384 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
126380 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
123616 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
