plain
[01:05:07] .............i.i..ii................................................................................ 3200/4601
[01:05:11] .................................................................................................... 3300/4601
[01:05:13] .................................................................i.................................. 3400/4601
[01:05:16] .................................................................................................... 3500/4601
[01:05:19] ......................................................................F............................. 3600/4601
[01:05:25] .................................................................................................... 3800/4601
[01:05:29] .............i...................................................................................... 3900/4601
[01:05:33] .................................................................................................... 4000/4601
[01:05:36] .................................................................................................... 4100/4601
---
[01:05:51] -   --> $DIR/expr_match.rs:19:5
[01:05:51] + error: unreachable statement
[01:05:51] +   --> $DIR/expr_match.rs:24:5
[01:05:51] 3    |
[01:05:51] - LL |     match {return} { } //~ ERROR unreachable
[01:05:51]:05:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/reachable/expr_match.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reachable/expr_match/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reachable/expr_match/auxiliary" "-A" "unused"
[01:05:51] ------------------------------------------
[01:05:51] 
[01:05:51] ------------------------------------------
[01:05:51] stderr:
[01:05:51] stderr:
[01:05:51] ------------------------------------------
[01:05:51] {"message":"unreachable statement","code":{"code":"unreachable_code","explanation":null},"level":"error","spans":[{"file_name":"<::std::macros::println macros>","byte_start":59,"byte_end":127,"line_start":2,"line_end":2,"column_start":1,"column_end":69,"is_primary":true,"text":[{"text":"{ $ crate :: io :: _print ( format_args_nl ! ( $ ( $ arg ) * ) ) ; } )","highlight_start":1,"highlight_end":69}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/reachable/expr_match.rs","byte_start":761,"byte_end":783,"line_start":24,"line_end":24,"column_start":5,"column_end":27,"is_primary":false,"text":[{"text":"    println!(\"I am dead\");","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"println!","def_site_span":{"file_name":"<::std::macros::println macros>","byte_start":0,"byte_end":129,"line_start":1,"line_end":2,"column_start":1,"column_end":71,"is_primary":false,"text":[{"text":"(  ) => ( print ! ( \"\\n\" ) ) ; ( $ ( $ arg : tt ) * ) => (","highlight_start":1,"highlight_end":59},{"text":"{ $ crate :: io :: _print ( format_args_nl ! ( $ ( $ arg ) * ) ) ; } )","highlight_start":1,"highlight_end":71}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/reachable/expr_match.rs","byte_start":554,"byte_end":570,"line_start":14,"line_end":14,"column_start":9,"column_end":25,"is_primary":true,"text":[{"text":"#![deny(unreachable_code)]","highlight_start":9,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: unreachable statement\n  --> /checkout/src/test/ui/reachable/expr_match.rs:24:5\n   |\nLL |     println!(\"I am dead\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/reachable/expr_match.rs:14:9\n   |\nLL | #![deny(unreachable_code)]\n   |         ^^^^^^^^^^^^^^^^\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:05:51] {"message":"unreachable statement","code":{"code":"unreachable_code","explanation":null},"level":"error","spans":[{"file_name":"<::std::macros::println macros>","byte_start":59,"byte_end":127,"line_start":2,"line_end":2,"column_start":1,"column_end":69,"is_primary":true,"text":[{"text":"{ $ crate :: io :: _print ( format_args_nl ! ( $ ( $ arg ) * ) ) ; } )","highlight_start":1,"highlight_end":69}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/reachable/expr_match.rs","byte_start":982,"byte_end":1004,"line_start":35,"line_end":35,"column_start":5,"column_end":27,"is_primary":false,"text":[{"text":"    println!(\"I am dead\");","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"println!","def_site_span":{"file_name":"<::std::macros::println macros>","byte_start":0,"byte_end":129,"line_start":1,"line_end":2,"column_start":1,"column_end":71,"is_primary":false,"text":[{"text":"(  ) => ( print ! ( \"\\n\" ) ) ; ( $ ( $ arg : tt ) * ) => (","highlight_start":1,"highlight_end":59},{"text":"{ $ crate :: io :: _print ( format_args_nl ! ( $ ( $ arg ) * ) ) ; } )","highlight_start":1,"highlight_end":71}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: unreachable statement\n  --> /checkout/src/test/ui/reachable/expr_match.rs:35:5\n   |\nLL |     println!(\"I am dead\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:05:51] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"chibin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:51] 
[01:05:51] 
[01:05:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:51] Build completed unsuccessfully in 0:03:22
[01:05:51] Build completed unsuccessfully in 0:03:22
[01:05:51] Makefile:58: recipe for target 'check' failed
[01:05:51] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:070fc128
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
