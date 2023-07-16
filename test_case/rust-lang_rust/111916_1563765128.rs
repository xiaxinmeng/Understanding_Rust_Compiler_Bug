plain

 error: explicit `deref` method call
   --> $DIR/explicit_deref_methods.rs:36:19
    |
 LL |     let b: &str = a.deref();
    |                   ^^^^^^^^^ help: try this: `&*a`
    |
    = note: `-D clippy::explicit-deref-methods` implied by `-D warnings`
 error: explicit `deref_mut` method call
   --> $DIR/explicit_deref_methods.rs:38:23
    |
    |
 LL |     let b: &mut str = a.deref_mut();
    |                       ^^^^^^^^^^^^^ help: try this: `&mut **a`
 error: explicit `deref` method call
   --> $DIR/explicit_deref_methods.rs:41:39
    |
    |
 LL |     let b: String = format!("{}, {}", a.deref(), a.deref());
    |                                       ^^^^^^^^^ help: try this: `&*a`
 error: explicit `deref` method call
   --> $DIR/explicit_deref_methods.rs:41:50
    |
    |
 LL |     let b: String = format!("{}, {}", a.deref(), a.deref());
    |                                                  ^^^^^^^^^ help: try this: `&*a`
 error: explicit `deref` method call
   --> $DIR/explicit_deref_methods.rs:43:20
    |
 LL |     println!("{}", a.deref());
 LL |     println!("{}", a.deref());
    |                    ^^^^^^^^^ help: try this: `&*a`
 error: explicit `deref` method call
   --> $DIR/explicit_deref_methods.rs:46:11
    |
    |
 LL |     match a.deref() {
    |           ^^^^^^^^^ help: try this: `&*a`
 error: explicit `deref` method call
   --> $DIR/explicit_deref_methods.rs:50:28
    |
    |
 LL |     let b: String = concat(a.deref());
    |                            ^^^^^^^^^ help: try this: `&*a`
 error: explicit `deref` method call
   --> $DIR/explicit_deref_methods.rs:52:13
    |
    |
 LL |     let b = just_return(a).deref();
    |             ^^^^^^^^^^^^^^^^^^^^^^ help: try this: `just_return(a)`
 error: explicit `deref` method call
   --> $DIR/explicit_deref_methods.rs:54:28
    |
    |
 LL |     let b: String = concat(just_return(a).deref());
    |                            ^^^^^^^^^^^^^^^^^^^^^^ help: try this: `just_return(a)`
 error: explicit `deref` method call
   --> $DIR/explicit_deref_methods.rs:56:19
    |
    |
 LL |     let b: &str = a.deref().deref();
    |                   ^^^^^^^^^^^^^^^^^ help: try this: `&**a`
 error: explicit `deref` method call
   --> $DIR/explicit_deref_methods.rs:59:13
    |
    |
 LL |     let b = opt_a.unwrap().deref();
    |             ^^^^^^^^^^^^^^^^^^^^^^ help: try this: `&*opt_a.unwrap()`
 error: explicit `deref` method call
   --> $DIR/explicit_deref_methods.rs:85:31
    |
    |
 LL |     let b: &str = expr_deref!(a.deref());
    |                               ^^^^^^^^^ help: try this: `&*a`
-error: aborting due to 12 previous errors
-error: aborting due to 12 previous errors
+error: call to `.deref()` on a reference in this situation does nothing
+   |
+   |
+LL |     let b = just_return(a).deref();
+   |                           ^^^^^^^^ unnecessary method call
error: test failed, to rerun pass `--test compile-test`
error: test failed, to rerun pass `--test compile-test`
+   = note: the type `str` does not implement `Deref`, so calling `deref` on `&str` copies the reference, which does not do anything and can be removed
+   = note: `-D noop-method-call` implied by `-D warnings`
+
+error: call to `.deref()` on a reference in this situation does nothing
+   |
+   |
+LL |     let b: String = concat(just_return(a).deref());
+   |                                          ^^^^^^^^ unnecessary method call
+   |
+   = note: the type `str` does not implement `Deref`, so calling `deref` on `&str` copies the reference, which does not do anything and can be removed
+error: aborting due to 14 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/explicit_deref_methods.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args explicit_deref_methods.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/explicit_deref_methods.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/explicit_deref_methods.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c7b10cf951dd44cc.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-3035b3cfcafc3b31.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-bd373ba7cdb9c07f.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-a36a3333b2a93763.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-2c5d99d6327d1048.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-426d0cca316c35ed.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-5a13c050c313baff.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-64c69a0c8d27494b.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-2d8078885e283441.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/explicit_deref_methods.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"explicit `deref` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":681,"byte_end":690,"line_start":36,"line_end":36,"column_start":19,"column_end":28,"is_primary":true,"text":[{"text":"    let b: &str = a.deref();","highlight_start":19,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::explicit-deref-methods` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":681,"byte_end":690,"line_start":36,"line_end":36,"column_start":19,"column_end":28,"is_primary":true,"text":[{"text":"    let b: &str = a.deref();","highlight_start":19,"highlight_end":28}],"label":null,"suggested_replacement":"&*a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref` method call\n  --> tests/ui/explicit_deref_methods.rs:36:19\n   |\nLL |     let b: &str = a.deref();\n   |                   ^^^^^^^^^ help: try this: `&*a`\n   |\n   = note: `-D clippy::explicit-deref-methods` implied by `-D warnings`\n\n"}
{"message":"explicit `deref_mut` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":715,"byte_end":728,"line_start":38,"line_end":38,"column_start":23,"column_end":36,"is_primary":true,"text":[{"text":"    let b: &mut str = a.deref_mut();","highlight_start":23,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":715,"byte_end":728,"line_start":38,"line_end":38,"column_start":23,"column_end":36,"is_primary":true,"text":[{"text":"    let b: &mut str = a.deref_mut();","highlight_start":23,"highlight_end":36}],"label":null,"suggested_replacement":"&mut **a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref_mut` method call\n  --> tests/ui/explicit_deref_methods.rs:38:23\n   |\nLL |     let b: &mut str = a.deref_mut();\n   |                       ^^^^^^^^^^^^^ help: try this: `&mut **a`\n\n"}
{"message":"explicit `deref` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":811,"byte_end":820,"line_start":41,"line_end":41,"column_start":39,"column_end":48,"is_primary":true,"text":[{"text":"    let b: String = format!(\"{}, {}\", a.deref(), a.deref());","highlight_start":39,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":811,"byte_end":820,"line_start":41,"line_end":41,"column_start":39,"column_end":48,"is_primary":true,"text":[{"text":"    let b: String = format!(\"{}, {}\", a.deref(), a.deref());","highlight_start":39,"highlight_end":48}],"label":null,"suggested_replacement":"&*a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref` method call\n  --> tests/ui/explicit_deref_methods.rs:41:39\n   |\nLL |     let b: String = format!(\"{}, {}\", a.deref(), a.deref());\n   |                                       ^^^^^^^^^ help: try this: `&*a`\n\n"}
{"message":"explicit `deref` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":822,"byte_end":831,"line_start":41,"line_end":41,"column_start":50,"column_end":59,"is_primary":true,"text":[{"text":"    let b: String = format!(\"{}, {}\", a.deref(), a.deref());","highlight_start":50,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":822,"byte_end":831,"line_start":41,"line_end":41,"column_start":50,"column_end":59,"is_primary":true,"text":[{"text":"    let b: String = format!(\"{}, {}\", a.deref(), a.deref());","highlight_start":50,"highlight_end":59}],"label":null,"suggested_replacement":"&*a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref` method call\n  --> tests/ui/explicit_deref_methods.rs:41:50\n   |\nLL |     let b: String = format!(\"{}, {}\", a.deref(), a.deref());\n   |                                                  ^^^^^^^^^ help: try this: `&*a`\n\n"}
{"message":"explicit `deref` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":854,"byte_end":863,"line_start":43,"line_end":43,"column_start":20,"column_end":29,"is_primary":true,"text":[{"text":"    println!(\"{}\", a.deref());","highlight_start":20,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":854,"byte_end":863,"line_start":43,"line_end":43,"column_start":20,"column_end":29,"is_primary":true,"text":[{"text":"    println!(\"{}\", a.deref());","highlight_start":20,"highlight_end":29}],"label":null,"suggested_replacement":"&*a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref` method call\n  --> tests/ui/explicit_deref_methods.rs:43:20\n   |\nLL |     println!(\"{}\", a.deref());\n   |                    ^^^^^^^^^ help: try this: `&*a`\n\n"}
{"message":"explicit `deref` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":920,"byte_end":929,"line_start":46,"line_end":46,"column_start":11,"column_end":20,"is_primary":true,"text":[{"text":"    match a.deref() {","highlight_start":11,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":920,"byte_end":929,"line_start":46,"line_end":46,"column_start":11,"column_end":20,"is_primary":true,"text":[{"text":"    match a.deref() {","highlight_start":11,"highlight_end":20}],"label":null,"suggested_replacement":"&*a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref` method call\n  --> tests/ui/explicit_deref_methods.rs:46:11\n   |\nLL |     match a.deref() {\n   |           ^^^^^^^^^ help: try this: `&*a`\n\n"}
{"message":"explicit `deref` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":983,"byte_end":992,"line_start":50,"line_end":50,"column_start":28,"column_end":37,"is_primary":true,"text":[{"text":"    let b: String = concat(a.deref());","highlight_start":28,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":983,"byte_end":992,"line_start":50,"line_end":50,"column_start":28,"column_end":37,"is_primary":true,"text":[{"text":"    let b: String = concat(a.deref());","highlight_start":28,"highlight_end":37}],"label":null,"suggested_replacement":"&*a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref` method call\n  --> tests/ui/explicit_deref_methods.rs:50:28\n   |\nLL |     let b: String = concat(a.deref());\n   |                            ^^^^^^^^^ help: try this: `&*a`\n\n"}
{"message":"explicit `deref` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1008,"byte_end":1030,"line_start":52,"line_end":52,"column_start":13,"column_end":35,"is_primary":true,"text":[{"text":"    let b = just_return(a).deref();","highlight_start":13,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1008,"byte_end":1030,"line_start":52,"line_end":52,"column_start":13,"column_end":35,"is_primary":true,"text":[{"text":"    let b = just_return(a).deref();","highlight_start":13,"highlight_end":35}],"label":null,"suggested_replacement":"just_return(a)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref` method call\n  --> tests/ui/explicit_deref_methods.rs:52:13\n   |\nLL |     let b = just_return(a).deref();\n   |             ^^^^^^^^^^^^^^^^^^^^^^ help: try this: `just_return(a)`\n\n"}
{"message":"explicit `deref` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1060,"byte_end":1082,"line_start":54,"line_end":54,"column_start":28,"column_end":50,"is_primary":true,"text":[{"text":"    let b: String = concat(just_return(a).deref());","highlight_start":28,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1060,"byte_end":1082,"line_start":54,"line_end":54,"column_start":28,"column_end":50,"is_primary":true,"text":[{"text":"    let b: String = concat(just_return(a).deref());","highlight_start":28,"highlight_end":50}],"label":null,"suggested_replacement":"just_return(a)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref` method call\n  --> tests/ui/explicit_deref_methods.rs:54:28\n   |\nLL |     let b: String = concat(just_return(a).deref());\n   |                            ^^^^^^^^^^^^^^^^^^^^^^ help: try this: `just_return(a)`\n\n"}
{"message":"explicit `deref` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1104,"byte_end":1121,"line_start":56,"line_end":56,"column_start":19,"column_end":36,"is_primary":true,"text":[{"text":"    let b: &str = a.deref().deref();","highlight_start":19,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1104,"byte_end":1121,"line_start":56,"line_end":56,"column_start":19,"column_end":36,"is_primary":true,"text":[{"text":"    let b: &str = a.deref().deref();","highlight_start":19,"highlight_end":36}],"label":null,"suggested_replacement":"&**a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref` method call\n  --> tests/ui/explicit_deref_methods.rs:56:19\n   |\nLL |     let b: &str = a.deref().deref();\n   |                   ^^^^^^^^^^^^^^^^^ help: try this: `&**a`\n\n"}
{"message":"explicit `deref` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1169,"byte_end":1191,"line_start":59,"line_end":59,"column_start":13,"column_end":35,"is_primary":true,"text":[{"text":"    let b = opt_a.unwrap().deref();","highlight_start":13,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1169,"byte_end":1191,"line_start":59,"line_end":59,"column_start":13,"column_end":35,"is_primary":true,"text":[{"text":"    let b = opt_a.unwrap().deref();","highlight_start":13,"highlight_end":35}],"label":null,"suggested_replacement":"&*opt_a.unwrap()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref` method call\n  --> tests/ui/explicit_deref_methods.rs:59:13\n   |\nLL |     let b = opt_a.unwrap().deref();\n   |             ^^^^^^^^^^^^^^^^^^^^^^ help: try this: `&*opt_a.unwrap()`\n\n"}
{"message":"explicit `deref` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1676,"byte_end":1685,"line_start":85,"line_end":85,"column_start":31,"column_end":40,"is_primary":true,"text":[{"text":"    let b: &str = expr_deref!(a.deref());","highlight_start":31,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1676,"byte_end":1685,"line_start":85,"line_end":85,"column_start":31,"column_end":40,"is_primary":true,"text":[{"text":"    let b: &str = expr_deref!(a.deref());","highlight_start":31,"highlight_end":40}],"label":null,"suggested_replacement":"&*a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref` method call\n  --> tests/ui/explicit_deref_methods.rs:85:31\n   |\nLL |     let b: &str = expr_deref!(a.deref());\n   |                               ^^^^^^^^^ help: try this: `&*a`\n\n"}
{"message":"call to `.deref()` on a reference in this situation does nothing","code":{"code":"noop_method_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1022,"byte_end":1030,"line_start":52,"line_end":52,"column_start":27,"column_end":35,"is_primary":true,"text":[{"text":"    let b = just_return(a).deref();","highlight_start":27,"highlight_end":35}],"label":"unnecessary method call","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the type `str` does not implement `Deref`, so calling `deref` on `&str` copies the reference, which does not do anything and can be removed","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"`-D noop-method-call` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: call to `.deref()` on a reference in this situation does nothing\n  --> tests/ui/explicit_deref_methods.rs:52:27\n   |\nLL |     let b = just_return(a).deref();\n   |                           ^^^^^^^^ unnecessary method call\n   |\n   = note: the type `str` does not implement `Deref`, so calling `deref` on `&str` copies the reference, which does not do anything and can be removed\n   = note: `-D noop-method-call` implied by `-D warnings`\n\n"}
{"message":"call to `.deref()` on a reference in this situation does nothing","code":{"code":"noop_method_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1074,"byte_end":1082,"line_start":54,"line_end":54,"column_start":42,"column_end":50,"is_primary":true,"text":[{"text":"    let b: String = concat(just_return(a).deref());","highlight_start":42,"highlight_end":50}],"label":"unnecessary method call","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the type `str` does not implement `Deref`, so calling `deref` on `&str` copies the reference, which does not do anything and can be removed","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: call to `.deref()` on a reference in this situation does nothing\n  --> tests/ui/explicit_deref_methods.rs:54:42\n   |\nLL |     let b: String = concat(just_return(a).deref());\n   |                                          ^^^^^^^^ unnecessary method call\n   |\n   = note: the type `str` does not implement `Deref`, so calling `deref` on `&str` copies the reference, which does not do anything and can be removed\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/index.crates.io-6f17d22bba15001f/compiletest_rs-0.10.0/src/lib.rs:111:22
