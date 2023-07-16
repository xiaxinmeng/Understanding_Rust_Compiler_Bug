\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/nested-item-spans.rs","byte_start":622,"byte_end":625,"line_start":20,"line_end":20,"column_start":22,"column_end":25,"is_primary":true,"text":[{"text":"        let x: u32 = \"x\"; //~ ERROR: mismatched types","highlight_start":22,"highlight_end":25}],"label":"expected u32, found reference","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `u32`\n   found type `&'static str`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui-fulldeps/proc-macro/nested-item-spans.rs:20:22\n   |\nLL |         let x: u32 = \"x\"; //~ ERROR: mismatched types\n   |                      ^^^ expected u32, found reference\n   |\n   = note: expected type `u32`\n              found type `&'static str`\n\n"}
[01:04:51] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[01:04:51] {"message":"Some errors occurred: E0308, E0425.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0308, E0425.\n"}
[01:04:51] {"message":"For more information about an error, try `rustc --explain E0308`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0308`.\n"}
[01:04:51] ------------------------------------------
[01:04:51] 
[01:04:51] thread '[ui] ui-fulldeps/proc-macro/nested-item-spans.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:04:51] 
[01:04:51] 
[01:04:51] ---- [ui] ui-fulldeps/proc-macro/parent-source-spans.rs stdout ----
[01:04:51] diff of stderr:
[01:04:51] 
[01:04:51] 7 LL |     one!("hello", "world");
[01:04:51] 9 
[01:04:51] 9 
[01:04:51] - error: second final: "world"
[01:04:51] + error: second final: "hello"
[01:04:51] 12    |
[01:04:51] 12    |
[01:04:51] 13 LL |     three!($a, $b);
[01:04:51] 
[01:04:51] 25 LL |     one!("hello", "world");
[01:04:51] 27 
[01:04:51] 27 
[01:04:51] - error: second parent: "world"
[01:04:51] + error: second parent: "hello"
[01:04:51] 30    |
[01:04:51] 30    |
[01:04:51] 31 LL |     two!($a, $b);
[01:04:51] 
[01:04:51] 40 LL |     one!("hello", "world");
[01:04:51] 42 
[01:04:51] 42 
[01:04:51] - error: second grandparent: "world"
[01:04:51] + error: second grandparent: "hello"
[01:04:51] 45    |
[01:04:51] 45    |
[01:04:51] 46 LL |     one!("hello", "world");
[01:04:51] 
[01:04:51] 52 LL |     one!("hello", "world");
[01:04:51] 54 
[01:04:51] 54 
[01:04:51] - error: second source: "world"
[01:04:51] + error: second source: "hello"
[01:04:51] 57    |
[01:04:51] 57    |
[01:04:51] 58 LL |     one!("hello", "world");
[01:04:51] 59    |     ^^^^^^^^^^^^^^^^^^^^^^^
[01:04:51] 60 
[01:04:51] 60 
[01:04:51] - error: first final: "yay"
[01:04:51] + error: first final: "hello"
[01:04:51] 63    |
[01:04:51] 63    |
[01:04:51] 64 LL |     three!($a, $b);
[01:04:51] 
[01:04:51] 67 LL |     two!("yay", "rust");
[01:04:51] 69 
[01:04:51] 69 
[01:04:51] - error: second final: "rust"
[01:04:51] + error: second final: "hello"
[01:04:51] 72    |
[01:04:51] 72    |
[01:04:51] 73 LL |     three!($a, $b);
[01:04:51] 
[01:04:51] 76 LL |     two!("yay", "rust");
[01:04:51] 78 
[01:04:51] 78 
[01:04:51] - error: first parent: "yay"
[01:04:51] + error: first parent: "hello"
[01:04:51] 81    |
[01:04:51] 81    |
[01:04:51] 82 LL |     two!("yay", "rust");
[01:04:51] 83    |     ^^^^^^^^^^^^^^^^^^^^
[01:04:51] 84 
[01:04:51] 84 
[01:04:51] - error: second parent: "rust"
[01:04:51] + error: second parent: "hello"
[01:04:51] 87    |
[01:04:51] 87    |
[01:04:51] 88 LL |     two!("yay", "rust");
[01:04:51] 89    |     ^^^^^^^^^^^^^^^^^^^^
[01:04:51] 90 
[01:04:51] 90 
[01:04:51] - error: first source: "yay"
[01:04:51] + error: first source: "hello"
[01:04:51] 93    |
[01:04:51] 93    |
[01:04:51] 94 LL |     two!("yay", "rust");
[01:04:51] 95    |     ^^^^^^^^^^^^^^^^^^^^
[01:04:51] 96 
[01:04:51] 96 
[01:04:51] - error: second source: "rust"
[01:04:51] + error: second source: "hello"
[01:04:51] 99    |
[01:04:51] 99    |
[01:04:51] 100 LL |     two!("yay", "rust");
[01:04:51] 
[01:04:51] The actual stderr differed from the expected stderr.
[01:04:51] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/parent-source-spans/parent-source-spans.stderr
[01:04:51] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/parent-source-spans/parent-source-spans.stderr
[01:04:51] To update references, rerun the tests and pass the `--bless` flag
[01:04:51] To only update this specific test, also pass `--test-args proc-macro/parent-source-spans.rs`
[01:04:51] error: 1 errors occurred comparing output.
[01:04:51] status: exit code: 1
[01:04:51] status: exit code: 1
[01:04:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/parent-source-spans/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/parent-source-spans/auxiliary" "-A" "unused"
[01:04:51] ------------------------------------------
[01:04:51] 
[01:04:51] ------------------------------------------
[01:04:51] stderr:
[01:04:51] stderr:
[01:04:51] ------------------------------------------
[01:04:51] {"message":"first final: \"hello\"","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":815,"byte_end":817,"line_start":27,"line_end":27,"column_start":12,"column_end":14,"is_primary":true,"text":[{"text":"    three!($a, $b);","highlight_start":12,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":682,"byte_end":695,"line_start":21,"line_end":21,"column_start":5,"column_end":18,"is_primary":false,"text":[{"text":"    two!($a, $b);","highlight_start":5,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":1178,"byte_end":1201,"line_start":44,"line_end":44,"column_start":5,"column_end":28,"is_primary":false,"text":[{"text":"    one!(\"hello\", \"world\");","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"one!","def_site_span":{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":648,"byte_end":772,"line_start":20,"line_end":24,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro one($a:expr, $b:expr) {","highlight_start":1,"highlight_end":30},{"text":"    two!($a, $b);","highlight_start":1,"highlight_end":18},{"text":"    //~^ ERROR first parent: \"hello\"","highlight_start":1,"highlight_end":37},{"text":"    //~| ERROR second parent: \"world\"","highlight_start":1,"highlight_end":38},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"two!","def_site_span":{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":774,"byte_end":968,"line_start":26,"line_end":32,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro two($a:expr, $b:expr) {","highlight_start":1,"highlight_end":30},{"text":"    three!($a, $b);","highlight_start":1,"highlight_end":20},{"text":"    //~^ ERROR first final: \"hello\"","highlight_start":1,"highlight_end":36},{"text":"    //~| ERROR second final: \"world\"","highlight_start":1,"highlight_end":37},{"text":"    //~| ERROR first final: \"yay\"","highlight_start":1,"highlight_end":34},{"text":"    //~| ERROR second final: \"rust\"","highlight_start":1,"highlight_end":36},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: first final: \"hello\"\n  --> /checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs:27:12\n   |\nLL |     three!($a, $b);\n   |            ^^\n...\nLL |     one!(\"hello\", \"world\");\n   |     ----------------------- in this macro invocation\n\n"}
[01:04:51] {"message":"second final: \"hello\"","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":819,"byte_end":821,"line_start":27,"line_end":27,"column_start":16,"column_end":18,"is_primary":true,"text":[{"text":"    three!($a, $b);","highlight_start":16,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":682,"byte_end":695,"line_start":21,"line_end":21,"column_start":5,"column_end":18,"is_primary":false,"text":[{"text":"    two!($a, $b);","highlight_start":5,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":1178,"byte_end":1201,"line_start":44,"line_end":44,"column_start":5,"column_end":28,"is_primary":false,"text":[{"text":"    one!(\"hello\", \"world\");","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"one!","def_site_span":{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":648,"byte_end":772,"line_start":20,"line_end":24,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro one($a:expr, $b:expr) {","highlight_start":1,"highlight_end":30},{"text":"    two!($a, $b);","highlight_start":1,"highlight_end":18},{"text":"    //~^ ERROR first parent: \"hello\"","highlight_start":1,"highlight_end":37},{"text":"    //~| ERROR second parent: \"world\"","highlight_start":1,"highlight_end":38},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"two!","def_site_span":{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":774,"byte_end":968,"line_start":26,"line_end":32,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro two($a:expr, $b:expr) {","highlight_start":1,"highlight_end":30},{"text":"    three!($a, $b);","highlight_start":1,"highlight_end":20},{"text":"    //~^ ERROR first final: \"hello\"","highlight_start":1,"highlight_end":36},{"text":"    //~| ERROR second final: \"world\"","highlight_start":1,"highlight_end":37},{"text":"    //~| ERROR first final: \"yay\"","highlight_start":1,"highlight_end":34},{"text":"    //~| ERROR second final: \"rust\"","highlight_start":1,"highlight_end":36},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: second final: \"hello\"\n  --> /checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs:27:16\n   |\nLL |     three!($a, $b);\n   |                ^^\n...\nLL |     one!(\"hello\", \"world\");\n   |     ----------------------- in this macro invocation\n\n"}
[01:04:51] {"message":"first parent: \"hello\"","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":682,"byte_end":695,"line_start":21,"line_end":21,"column_start":5,"column_end":18,"is_primary":true,"text":[{"text":"    two!($a, $b);","highlight_start":5,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":1178,"byte_end":1201,"line_start":44,"line_end":44,"column_start":5,"column_end":28,"is_primary":false,"text":[{"text":"    one!(\"hello\", \"world\");","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"one!","def_site_span":{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":648,"byte_end":772,"line_start":20,"line_end":24,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro one($a:expr, $b:expr) {","highlight_start":1,"highlight_end":30},{"text":"    two!($a, $b);","highlight_start":1,"highlight_end":18},{"text":"    //~^ ERROR first parent: \"hello\"","highlight_start":1,"highlight_end":37},{"text":"    //~| ERROR second parent: \"world\"","highlight_start":1,"highlight_end":38},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: first parent: \"hello\"\n  --> /checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs:21:5\n   |\nLL |     two!($a, $b);\n   |     ^^^^^^^^^^^^^\n...\nLL |     one!(\"hello\", \"world\");\n   |     ----------------------- in this macro invocation\n\n"}
[01:04:51] {"message":"second parent: \"hello\"","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":682,"byte_end":695,"line_start":21,"line_end":21,"column_start":5,"column_end":18,"is_primary":true,"text":[{"text":"    two!($a, $b);","highlight_start":5,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":1178,"byte_end":1201,"line_start":44,"line_end":44,"column_start":5,"column_end":28,"is_primary":false,"text":[{"text":"    one!(\"hello\", \"world\");","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"one!","def_site_span":{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":648,"byte_end":772,"line_start":20,"line_end":24,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro one($a:expr, $b:expr) {","highlight_start":1,"highlight_end":30},{"text":"    two!($a, $b);","highlight_start":1,"highlight_end":18},{"text":"    //~^ ERROR first parent: \"hello\"","highlight_start":1,"highlight_end":37},{"text":"    //~| ERROR second parent: \"world\"","highlight_start":1,"highlight_end":38},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: second parent: \"hello\"\n  --> /checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs:21:5\n   |\nLL |     two!($a, $b);\n   |     ^^^^^^^^^^^^^\n...\nLL |     one!(\"hello\", \"world\");\n   |     ----------------------- in this macro invocation\n\n"}
[01:04:51] {"message":"first grandparent: \"hello\"","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":1178,"byte_end":1201,"line_start":44,"line_end":44,"column_start":5,"column_end":28,"is_primary":true,"text":[{"text":"    one!(\"hello\", \"world\");","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: first grandparent: \"hello\"\n  --> /checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs:44:5\n   |\nLL |     one!(\"hello\", \"world\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:04:51] {"message":"second grandparent: \"hello\"","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":1178,"byte_end":1201,"line_start":44,"line_end":44,"column_start":5,"column_end":28,"is_primary":true,"text":[{"text":"    one!(\"hello\", \"world\");","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: second grandparent: \"hello\"\n  --> /checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs:44:5\n   |\nLL |     one!(\"hello\", \"world\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:04:51] {"message":"first source: \"hello\"","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":1178,"byte_end":1201,"line_start":44,"line_end":44,"column_start":5,"column_end":28,"is_primary":true,"text":[{"text":"    one!(\"hello\", \"world\");","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: first source: \"hello\"\n  --> /checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs:44:5\n   |\nLL |     one!(\"hello\", \"world\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:04:51] {"message":"second source: \"hello\"","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":1178,"byte_end":1201,"line_start":44,"line_end":44,"column_start":5,"column_end":28,"is_primary":true,"text":[{"text":"    one!(\"hello\", \"world\");","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: second source: \"hello\"\n  --> /checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs:44:5\n   |\nLL |     one!(\"hello\", \"world\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:04:51] {"message":"first final: \"hello\"","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":815,"byte_end":817,"line_start":27,"line_end":27,"column_start":12,"column_end":14,"is_primary":true,"text":[{"text":"    three!($a, $b);","highlight_start":12,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":1367,"byte_end":1387,"line_start":50,"line_end":50,"column_start":5,"column_end":25,"is_primary":false,"text":[{"text":"    two!(\"yay\", \"rust\");","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"two!","def_site_span":{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":774,"byte_end":968,"line_start":26,"line_end":32,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro two($a:expr, $b:expr) {","highlight_start":1,"highlight_end":30},{"text":"    three!($a, $b);","highlight_start":1,"highlight_end":20},{"text":"    //~^ ERROR first final: \"hello\"","highlight_start":1,"highlight_end":36},{"text":"    //~| ERROR second final: \"world\"","highlight_start":1,"highlight_end":37},{"text":"    //~| ERROR first final: \"yay\"","highlight_start":1,"highlight_end":34},{"text":"    //~| ERROR second final: \"rust\"","highlight_start":1,"highlight_end":36},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: first final: \"hello\"\n  --> /checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs:27:12\n   |\nLL |     three!($a, $b);\n   |            ^^\n...\nLL |     two!(\"yay\", \"rust\");\n   |     -------------------- in this macro invocation\n\n"}
[01:04:51] {"message":"second final: \"hello\"","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":819,"byte_end":821,"line_start":27,"line_end":27,"column_start":16,"column_end":18,"is_primary":true,"text":[{"text":"    three!($a, $b);","highlight_start":16,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":1367,"byte_end":1387,"line_start":50,"line_end":50,"column_start":5,"column_end":25,"is_primary":false,"text":[{"text":"    two!(\"yay\", \"rust\");","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"two!","def_site_span":{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":774,"byte_end":968,"line_start":26,"line_end":32,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro two($a:expr, $b:expr) {","highlight_start":1,"highlight_end":30},{"text":"    three!($a, $b);","highlight_start":1,"highlight_end":20},{"text":"    //~^ ERROR first final: \"hello\"","highlight_start":1,"highlight_end":36},{"text":"    //~| ERROR second final: \"world\"","highlight_start":1,"highlight_end":37},{"text":"    //~| ERROR first final: \"yay\"","highlight_start":1,"highlight_end":34},{"text":"    //~| ERROR second final: \"rust\"","highlight_start":1,"highlight_end":36},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: second final: \"hello\"\n  --> /checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs:27:16\n   |\nLL |     three!($a, $b);\n   |                ^^\n...\nLL |     two!(\"yay\", \"rust\");\n   |     -------------------- in this macro invocation\n\n"}
[01:04:51] {"message":"first parent: \"hello\"","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":1367,"byte_end":1387,"line_start":50,"line_end":50,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    two!(\"yay\", \"rust\");","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: first parent: \"hello\"\n  --> /checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs:50:5\n   |\nLL |     two!(\"yay\", \"rust\");\n   |     ^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:04:51] {"message":"second parent: \"hello\"","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":1367,"byte_end":1387,"line_start":50,"line_end":50,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    two!(\"yay\", \"rust\");","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: second parent: \"hello\"\n  --> /checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs:50:5\n   |\nLL |     two!(\"yay\", \"rust\");\n   |     ^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:04:51] {"message":"first source: \"hello\"","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":1367,"byte_end":1387,"line_start":50,"line_end":50,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    two!(\"yay\", \"rust\");","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: first source: \"hello\"\n  --> /checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs:50:5\n   |\nLL |     two!(\"yay\", \"rust\");\n   |     ^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:04:51] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:04:51] {"message":"second source: \"hello\"","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":1367,"byte_end":1387,"line_start":50,"line_end":50,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    two!(\"yay\", \"rust\");","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: second source: \"hello\"\n  --> /checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs:50:5\n   |\nLL |     two!(\"yay\", \"rust\");\n   |     ^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:04:51] {"message":"first final: \"hip\"","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":1544,"byte_end":1549,"line_start":56,"line_end":56,"column_start":12,"column_end":17,"is_primary":true,"text":[{"text":"    three!(\"hip\", \"hop\");","highlight_start":12,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: first final: \"hip\"\n  --> /checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs:56:12\n   |\nLL |     three!(\"hip\", \"hop\");\n   |            ^^^^^\n\n"}
[01:04:51] {"message":"second final: \"hop\"","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":1551,"byte_end":1556,"line_start":56,"line_end":56,"column_start":19,"column_end":24,"is_primary":true,"text":[{"text":"    three!(\"hip\", \"hop\");","highlight_start":19,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: second final: \"hop\"\n  --> /checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs:56:19\n   |\nLL |     three!(\"hip\", \"hop\");\n   |                   ^^^^^\n\n"}
[01:04:51] {"message":"first source: \"hip\"","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":1544,"byte_end":1549,"line_start":56,"line_end":56,"column_start":12,"column_end":17,"is_primary":true,"text":[{"text":"    three!(\"hip\", \"hop\");","highlight_start":12,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: first source: \"hip\"\n  --> /checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs:56:12\n   |\nLL |     three!(\"hip\", \"hop\");\n   |            ^^^^^\n\n"}
[01:04:51] {"message":"second source: \"hop\"","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs","byte_start":1551,"byte_end":1556,"line_start":56,"line_end":56,"column_start":19,"column_end":24,"is_primary":true,"text":[{"text":"    three!(\"hip\", \"hop\");","highlight_start":19,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: second source: \"hop\"\n  --> /checkout/src/test/ui-fulldeps/proc-macro/parent-source-spans.rs:56:19\n   |\nLL |     three!(\"hip\", \"hop\");\n   |                   ^^^^^\n\n"}
[01:04:51] {"message":"aborting due to 18 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 18 previous errors\n\n"}
[01:04:51] ------------------------------------------
[01:04:51] 
[01:04:51] thread '[ui] ui-fulldeps/proc-macro/parent-source-spans.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:04:51] 
---
[01:04:51] test result: FAILED. 37 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out
[01:04:51] 
[01:04:51] 
[01:04:51] 
[01:04:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:51] 
[01:04:51] 
[01:04:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:51] Build completed unsuccessfully in 0:19:41
[01:04:51] Build completed unsuccessfully in 0:19:41
[01:04:51] Makefile:58: recipe for target 'check' failed
[01:04:51] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07a3d432
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
