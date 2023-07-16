plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:43:46] 
[00:43:46] running 1376 tests
[00:43:51] ..................................................................................i.................
[00:43:57] ......................F.......i.....................................................................
[00:44:05] ....................................................................................................
[00:44:09] ....................................................................................................
[00:44:13] ....................................................................................................
[00:44:18] ....................................................................................................
[00:44:18] ....................................................................................................
[00:44:24] ......................................F...........................................................F.
[00:44:30] ..F.................................................................................................
[00:44:37] ..................................i....................................F.F....F....F.F.F..F.........
[00:44:48] ..........................ii........................................................................
[00:44:55] ....................................................................................................
[00:45:00] .......i....................................................................
[00:45:00] failures:
[00:45:00] failures:
[00:45:00] 
[00:45:00] ---- [ui] ui/const-eval/promoted_errors.rs stdout ----
[00:45:00]  diff of stderr:
[00:45:00] 
[00:45:00] 15 warning: constant evaluation error
[00:45:00] 16   --> $DIR/promoted_errors.rs:17:14
[00:45:00] 17    |
[00:45:00] + LL |     println!("{}", 0u32 - 1);
[00:45:00] +    |     ------------------------- attempted to do overflowing math
[00:45:00] + ...
[00:45:00] 18 LL |     let _x = 0u32 - 1;
[00:45:00] -    |              ^^^^^^^^ attempted to do overflowing math
[00:45:00] +    |
[00:45:00] +    |
[00:45:00] +    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backd":64,"is_primary":false,"text":[{"text":"( $ ( $ arg : tt ) * ) => (","highlight_start":1,"highlight_end":28},{"text":"$ crate :: io :: _print ( format_args ! ( $ ( $ arg ) * ) ) ) ;","highlight_start":1,"highlight_end":64}],"label":null,"suggested_replacement":null,"expansion":null}}},"macro_decl_name":"format_args!","def_site_span":null}},{"file_name":"/checkout/src/test/ui/const-eval/promoted_errors.rs","byte_start":731,"byte_end":738,"line_start":22,"line_end":22,"column_start":14,"column_end":21,"is_primary":true,"text":[{"text":"    let _x = 1/(1-1);","highlight_start":14,"highlight_end":21}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning: constant evaluation error\n  --> /checkout/src/test/ui/const-eval/promoted_errors.rs:22:14\n   |\nLL |     println!(\"{}\", 1/(1-1));\n   |     ------------------------ attempted to do overflowing math\n...\nLL |     let _x = 1/(1-1);\n   |              ^^^^^^^\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[00:45:00] {"message":"constant evaluation error","code":{"code":"const_err","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/const-eval/promoted_errors.rs","byte_start":807,"byte_end":823,"line_start":25,"line_end":25,"column_start":20,"column_end":36,"is_primary":true,"text":[{"text":"    println!(\"{}\", 1/(false as u32));","highlight_start":20,"highlight_end":36}],"label":"attempted to do overflowing math","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"warning_64-unknown-linux-gnu/test/ui/nll/decl-macro-illegal-copy.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/decl-macro-illegal-copy.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:45:00] ------------------------------------------
[00:45:00] 
[00:45:00] ------------------------------------------
[00:45:00] stderr:
[00:45:00] stderr:
[00:45:00] ------------------------------------------
[00:45:00] thread 'main' panicked at 'constants cannot have local types', libcore/option.rs:914:5
[00:45:00] 
[00:45:00] ------------------------------------------
[00:45:00] 
[00:45:00] thread '[ui] ui/nll/decl-macro-illegal-copy.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
[00:45:00] thread '[ui] ui/nll/decl-macro-illegal-copy.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
[00:45:00] 
[00:45:00] ---- [ui] ui/nll/issue-16223.rs stdout ----
[00:45:00]  
[00:45:00] error: test compilation failed although it shouldn't!
[00:45:00] status: signal: 4
[00:45:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-16223.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-16223.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:31aa69e1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
