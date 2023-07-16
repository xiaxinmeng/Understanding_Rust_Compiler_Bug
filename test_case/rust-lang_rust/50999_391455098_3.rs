\n"},"level":"error","spans":[{"file_name":"<print_me macros>","byte_start":26,"byte_end":34,"line_start":1,"line_end":1,"column_start":27,"column_end":35,"is_primary":true,"text":[{"text":"( $ p : path ) => { { use $ p as V ; println ! ( \"{}\" , V ) ; } }","highlight_start":27,"highlight_end":35}],"label":"no `y` in `x`","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/rust-2018/inject-2015-use-root-module-path.rs","byte_start":1024,"byte_end":1040,"line_start":28,"line_end":28,"column_start":5,"column_end":21,"is_primary":false,"text":[{"text":"    print_me!(x::y); //~ ERROR unresolved import `x::y`","highlight_start":5,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"print_me!","def_site_span":{"file_name":"<print_me macros>","byte_start":0,"byte_end":65,"line_start":1,"line_end":1,"column_start":1,"column_end":66,"is_primary":false,"text":[{"text":"( $ p : path ) => { { use $ p as V ; println ! ( \"{}\" , V ) ; } }","highlight_start":1,"highlight_end":66}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error[E0432]: unresolved import `x::y`\n  --> /checkout/src/test/ui/rust-2018/inject-2015-use-root-module-path.rs:28:5\n   |\nLL |     print_me!(x::y); //~ ERROR unresolved import `x::y`\n   |     ^^^^^^^^^^^^^^^^ no `y` in `x`\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[00:43:53] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:43:53] {"message":"For more information about this error, try `rustc --explain E0432`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0432`.\n"}
[00:43:53] ------------------------------------------
[00:43:53] 
[00:43:53] 
[00:43:53] thread '[ui] ui/rust-2018/inject-2015-use-root-module-path.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3044:9
[00:43:53] 
[00:43:53] 
[00:43:53] ---- [ui] ui/rust-2018/inject-2015-use-root-module.rs stdout ----
[00:43:53] 
[00:43:53] error: test compilation failed although it shouldn't!
[00:43:53] status: exit code: 101
[00:43:53] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/inject-2015-use-root-module.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/inject-2015-use-root-module/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/inject-2015-use-root-module/auxiliary" "-A" "unused"
[00:43:53] ------------------------------------------
[00:43:53] 
[00:43:53] ------------------------------------------
[00:43:53] stderr:
[00:43:53] stderr:
[00:43:53] ------------------------------------------
[00:43:53] {"message":"unresolved import `crate::x::y`","code":{"code":"E0432","explanation":"\nAn import was unresolved.\n\nErroneous code example:\n\n