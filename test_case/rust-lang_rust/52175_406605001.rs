plain
[00:42:12] ....................................................................................................
[00:42:14] ....................................................................................................
[00:42:17] ....................................................................................................
[00:42:20] ....................................................................................................
[00:42:22] ......................................................F.............................................
[00:42:29] .........................................i..........................................................
[00:42:32] ...............................i....................................................................
[00:42:35] ....................................................................................................
[00:42:39] ....................................................................................................
[00:42:39] ....................................................................................................
[00:42:42] ..................................................i.................................................
 -    |     unmatched `}` in format string
[00:42:44] -    |     in this macro invocation
[00:42:44] + LL |     println!("}"); //~ ERROR invalid
[00:42:44] +    |     ^^^^^^^^^^^^^^ unmatched `}` in format string
[00:42:44] 18    |
[00:42:44] 19    = note: if you intended to print `}`, you can escape it using `}}`
[00:42:44] 
[00:42:44] 
[00:42:44] The actual stderr differed from the expected stderr.
[00:42:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-51848/issue-51848.stderr
[00:42:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-51848/issue-51848.stderr
[00:42:44] To update references, rerun the tests and pass the `--bless` flag
[00:42:44] To only update this specific test, also pass `--test-args issue-51848.rs`
[00:42:44] error: 1 errors occurred comparing output.
[00:42:44] status: exit code: 1
[00:42:44] status: exit code: 1
[00:42:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-51848.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-51848/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-51848/auxiliary" "-A" "unused"
[00:42:44] ------------------------------------------
[00:42:44] 
[00:42:44] ------------------------------------------
[00:42:44] stderr:
[00:42:44] stderr:
[00:42:44] ------------------------------------------
[00:42:44] {"message":"invalid format string: expected `'}'` but string was terminated","code":null,"level":"error","spans":[{"file_name":"<println macros>","byte_start":66,"byte_end":66,"line_start":2,"line_end":2,"column_start":14,"column_end":14,"is_primary":true,"text":[{"text":"print ! ( concat ! ( $ fmt , \"\\n\" ) ) ) ; ( $ fmt : expr , $ ( $ arg : tt ) *","highlight_start":14,"highlight_end":14}],"label":"expected `'}'` in format string","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"<print macros>","byte_start":54,"byte_end":85,"line_start":2,"line_end":2,"column_start":27,"column_end":58,"is_primary":false,"text":[{"text":"$ crate :: io :: _print ( format_args ! ( $ ( $ arg ) * ) ) ) ;","highlight_start":27,"highlight_end":58}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"<println macros>","byte_start":53,"byte_end":90,"line_start":2,"line_end":2,"column_start":1,"column_end":38,"is_primary":false,"text":[{"text":"print ! ( concat ! ( $ fmt , \"\\n\" ) ) ) ; ( $ fmt : expr , $ ( $ arg : tt ) *","highlight_start":1,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/issue-51848.rs","byte_start":669,"byte_end":683,"line_start":16,"line_end":16,"column_start":9,"column_end":23,"is_primary":false,"text":[{"text":"        println!(\"{\"); //~ ERROR invalid","highlight_start":9,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/issue-51848.rs","byte_start":743,"byte_end":763,"line_start":25,"line_end":25,"column_start":5,"column_end":25,"is_primary":false,"text":[{"text":"    macro_with_error!();","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"macro_with_error!","def_site_span":{"file_name":"/checkout/src/test/ui/issue-51848.rs","byte_start":616,"byte_end":710,"line_start":14,"line_end":18,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro_rules! macro_with_error {","highlight_start":1,"highlight_end":32},{"text":"    ( ) => {","highlight_start":1,"highlight_end":13},{"text":"        println!(\"{\"); //~ ERROR invalid","highlight_start":1,"highlight_end":41},{"text":"    };","highlight_start":1,"highlight_end":7},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"println!","def_site_span":{"file_name":"<println macros>","byte_start":0,"byte_end":195,"line_start":1,"line_end":3,"column_start":1,"column_end":65,"is_primary":false,"text":[{"text":"(  ) => ( print ! ( \"\\n\" ) ) ; ( $ fmt : expr ) => (","highlight_start":1,"highlight_end":53},{"text":"print ! ( concat ! ( $ fmt , \"\\n\" ) ) ) ; ( $ fmt : expr , $ ( $ arg : tt ) *","highlight_start":1,"highlight_end":78},{"text":") => ( print ! ( concat ! ( $ fmt , \"\\n\" ) , $ ( $ arg ) * ) ) ;","highlight_start":1,"highlight_end":65}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":nueplacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"<print macros>","byte_start":54,"byte_end":85,"line_start":2,"line_end":2,"column_start":27,"column_end":58,"is_primary":false,"text":[{"text":"$ crate :: io :: _print ( format_args ! ( $ ( $ arg ) * ) ) ) ;","highlight_start":27,"highlight_end":58}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"<println macros>","byte_start":53,"byte_end":90,"line_start":2,"line_end":2,"column_start":1,"column_end":38,"is_primary":false,"text":[{"text":"print ! ( concat ! ( $ fmt , \"\\n\" ) ) ) ; ( $ fmt : expr , $ ( $ arg : tt ) *","highlight_start":1,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/issue-51848.rs","byte_start":868,"byte_end":882,"line_start":28,"line_end":28,"column_start":5,"column_end":19,"is_primary":false,"text":[{"text":"    println!(\"}\"); //~ ERROR invalid","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"println!","def_site_span":{"file_name":"<println macros>","byte_start":0,"byte_end":195,"line_start":1,"line_end":3,"column_start":1,"column_end":65,"is_primary":false,"text":[{"text":"(  ) => ( print ! ( \"\\n\" ) ) ; ( $ fmt : expr ) => (","highlight_start":1,"highlight_end":53},{"text":"print ! ( concat ! ( $ fmt , \"\\n\" ) ) ) ; ( $ fmt : expr , $ ( $ arg : tt ) *","highlight_start":1,"highlight_end":78},{"text":") => ( print ! ( concat ! ( $ fmt , \"\\n\" ) , $ ( $ arg ) * ) ) ;","highlight_start":1,"highlight_end":65}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"print!","def_site_span":{"file_name":"<print macros>","byte_start":0,"byte_end":91,"line_start":1,"line_end":2,"column_start":1,"column_end":64,"is_primary":false,"text":[{"text":"( $ ( $ arg : tt ) * ) => (","highlight_start":1,"highlight_end":28},{"text":"$ crate :: io :: _print ( format_args ! ( $ ( $ arg ) * ) ) ) ;","highlight_start":1,"highlight_end":64}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"format_args!","def_site_span":null}}],"children":[{"message":"if you intended to print `}`, you can escape it using `}}`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: invalid format string: unmatched `}` found\n  --> /checkout/src/test/ui/issue-51848.rs:28:5\n   |\nLL |     println!(\"}\"); //~ ERROR invalid\n   |     ^^^^^^^^^^^^^^ unmatched `}` in format string\n   |\n   = note: if you intended to print `}`, you can escape it using `}}`\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[00:42:44] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:42:44] ------------------------------------------
[00:42:44] 
[00:42:44] thread '[ui] ui/issue-51848.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3137:9
[00:42:44] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:42:44] 
[00:42:44] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:42:44] 
[00:42:44] 
[00:42:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:42:44] 
[00:42:44] 
[00:42:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:42:44] Build completed unsuccessfully in 0:01:34
[00:42:44] Build completed unsuccessfully in 0:01:34
[00:42:44] make: *** [check] Error 1
[00:42:44] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00ba6ecf
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
