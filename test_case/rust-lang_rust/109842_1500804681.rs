plain
+   |
+LL |     clone_double_ref,
+   |     ^^^^^^^^^^^^^^^^
+   |
+   = note: `-D unknown-lints` implied by `-D warnings`
 error: explicit `deref` method call
   --> $DIR/explicit_deref_methods.rs:36:19
    |
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
+   |
+   = note: the type `&str` which `deref` is being called on is the same as the type returned from `deref`, so the method call does not do anything and can be removed
+   = note: `-D noop-method-call` implied by `-D warnings`
+
+error: call to `.deref()` on a reference in this situation does nothing
+   |
+   |
+LL |     let b: String = concat(just_return(a).deref());
+   |                                          ^^^^^^^^ unnecessary method call
+   |
+   |
+   = note: the type `&str` which `deref` is being called on is the same as the type returned from `deref`, so the method call does not do anything and can be removed
+error: aborting due to 15 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/explicit_deref_methods.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args explicit_deref_methods.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/explicit_deref_methods.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/explicit_deref_methods.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-d8aee1681c496322.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-60efa73d6fad7e91.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-e5b19e58ae33983e.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-40ff94c070351d6c.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-23c15a454288152e.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-e22c3fc74241d5e4.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-fc796eef9a515a44.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/explicit_deref_methods.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"unknown lint: `clone_double_ref`","code":{"code":"unknown_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":128,"byte_end":144,"line_start":6,"line_end":6,"column_start":5,"column_end":21,"is_primary":true,"text":[{"text":"    clone_double_ref,","highlight_start":5,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D unknown-lints` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: unknown lint: `clone_double_ref`\n  --> tests/ui/explicit_deref_methods.rs:6:5\n   |\nLL |     clone_double_ref,\n   |     ^^^^^^^^^^^^^^^^\n   |\n   = note: `-D unknown-lints` implied by `-D warnings`\n\n"}
{"message":"explicit `deref` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":673,"byte_end":682,"line_start":36,"line_end":36,"column_start":19,"column_end":28,"is_primary":true,"text":[{"text":"    let b: &str = a.deref();","highlight_start":19,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::explicit-deref-methods` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":673,"byte_end":682,"line_start":36,"line_end":36,"column_start":19,"column_end":28,"is_primary":true,"text":[{"text":"    let b: &str = a.deref();","highlight_start":19,"highlight_end":28}],"label":null,"suggested_replacement":"&*a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref` method call\n  --> tests/ui/explicit_deref_methods.rs:36:19\n   |\nLL |     let b: &str = a.deref();\n   |                   ^^^^^^^^^ help: try this: `&*a`\n   |\n   = note: `-D clippy::explicit-deref-methods` implied by `-D warnings`\n\n"}
{"message":"explicit `deref_mut` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":707,"byte_end":720,"line_start":38,"line_end":38,"column_start":23,"column_end":36,"is_primary":true,"text":[{"text":"    let b: &mut str = a.deref_mut();","highlight_start":23,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":707,"byte_end":720,"line_start":38,"line_end":38,"column_start":23,"column_end":36,"is_primary":true,"text":[{"text":"    let b: &mut str = a.deref_mut();","highlight_start":23,"highlight_end":36}],"label":null,"suggested_replacement":"&mut **a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref_mut` method call\n  --> tests/ui/explicit_deref_methods.rs:38:23\n   |\nLL |     let b: &mut str = a.deref_mut();\n   |                       ^^^^^^^^^^^^^ help: try this: `&mut **a`\n\n"}
{"message":"explicit `deref` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":803,"byte_end":812,"line_start":41,"line_end":41,"column_start":39,"column_end":48,"is_primary":true,"text":[{"text":"    let b: String = format!(\"{}, {}\", a.deref(), a.deref());","highlight_start":39,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":803,"byte_end":812,"line_start":41,"line_end":41,"column_start":39,"column_end":48,"is_primary":true,"text":[{"text":"    let b: String = format!(\"{}, {}\", a.deref(), a.deref());","highlight_start":39,"highlight_end":48}],"label":null,"suggested_replacement":"&*a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref` method call\n  --> tests/ui/explicit_deref_methods.rs:41:39\n   |\nLL |     let b: String = format!(\"{}, {}\", a.deref(), a.deref());\n   |                                       ^^^^^^^^^ help: try this: `&*a`\n\n"}
{"message":"explicit `deref` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":814,"byte_end":823,"line_start":41,"line_end":41,"column_start":50,"column_end":59,"is_primary":true,"text":[{"text":"    let b: String = format!(\"{}, {}\", a.deref(), a.deref());","highlight_start":50,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":814,"byte_end":823,"line_start":41,"line_end":41,"column_start":50,"column_end":59,"is_primary":true,"text":[{"text":"    let b: String = format!(\"{}, {}\", a.deref(), a.deref());","highlight_start":50,"highlight_end":59}],"label":null,"suggested_replacement":"&*a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref` method call\n  --> tests/ui/explicit_deref_methods.rs:41:50\n   |\nLL |     let b: String = format!(\"{}, {}\", a.deref(), a.deref());\n   |                                                  ^^^^^^^^^ help: try this: `&*a`\n\n"}
{"message":"explicit `deref` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":846,"byte_end":855,"line_start":43,"line_end":43,"column_start":20,"column_end":29,"is_primary":true,"text":[{"text":"    println!(\"{}\", a.deref());","highlight_start":20,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":846,"byte_end":855,"line_start":43,"line_end":43,"column_start":20,"column_end":29,"is_primary":true,"text":[{"text":"    println!(\"{}\", a.deref());","highlight_start":20,"highlight_end":29}],"label":null,"suggested_replacement":"&*a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref` method call\n  --> tests/ui/explicit_deref_methods.rs:43:20\n   |\nLL |     println!(\"{}\", a.deref());\n   |                    ^^^^^^^^^ help: try this: `&*a`\n\n"}
{"message":"explicit `deref` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":912,"byte_end":921,"line_start":46,"line_end":46,"column_start":11,"column_end":20,"is_primary":true,"text":[{"text":"    match a.deref() {","highlight_start":11,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":912,"byte_end":921,"line_start":46,"line_end":46,"column_start":11,"column_end":20,"is_primary":true,"text":[{"text":"    match a.deref() {","highlight_start":11,"highlight_end":20}],"label":null,"suggested_replacement":"&*a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref` method call\n  --> tests/ui/explicit_deref_methods.rs:46:11\n   |\nLL |     match a.deref() {\n   |           ^^^^^^^^^ help: try this: `&*a`\n\n"}
{"message":"explicit `deref` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":975,"byte_end":984,"line_start":50,"line_end":50,"column_start":28,"column_end":37,"is_primary":true,"text":[{"text":"    let b: String = concat(a.deref());","highlight_start":28,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":975,"byte_end":984,"line_start":50,"line_end":50,"column_start":28,"column_end":37,"is_primary":true,"text":[{"text":"    let b: String = concat(a.deref());","highlight_start":28,"highlight_end":37}],"label":null,"suggested_replacement":"&*a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref` method call\n  --> tests/ui/explicit_deref_methods.rs:50:28\n   |\nLL |     let b: String = concat(a.deref());\n   |                            ^^^^^^^^^ help: try this: `&*a`\n\n"}
{"message":"explicit `deref` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1000,"byte_end":1022,"line_start":52,"line_end":52,"column_start":13,"column_end":35,"is_primary":true,"text":[{"text":"    let b = just_return(a).deref();","highlight_start":13,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1000,"byte_end":1022,"line_start":52,"line_end":52,"column_start":13,"column_end":35,"is_primary":true,"text":[{"text":"    let b = just_return(a).deref();","highlight_start":13,"highlight_end":35}],"label":null,"suggested_replacement":"just_return(a)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref` method call\n  --> tests/ui/explicit_deref_methods.rs:52:13\n   |\nLL |     let b = just_return(a).deref();\n   |             ^^^^^^^^^^^^^^^^^^^^^^ help: try this: `just_return(a)`\n\n"}
{"message":"explicit `deref` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1052,"byte_end":1074,"line_start":54,"line_end":54,"column_start":28,"column_end":50,"is_primary":true,"text":[{"text":"    let b: String = concat(just_return(a).deref());","highlight_start":28,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1052,"byte_end":1074,"line_start":54,"line_end":54,"column_start":28,"column_end":50,"is_primary":true,"text":[{"text":"    let b: String = concat(just_return(a).deref());","highlight_start":28,"highlight_end":50}],"label":null,"suggested_replacement":"just_return(a)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref` method call\n  --> tests/ui/explicit_deref_methods.rs:54:28\n   |\nLL |     let b: String = concat(just_return(a).deref());\n   |                            ^^^^^^^^^^^^^^^^^^^^^^ help: try this: `just_return(a)`\n\n"}
{"message":"explicit `deref` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1096,"byte_end":1113,"line_start":56,"line_end":56,"column_start":19,"column_end":36,"is_primary":true,"text":[{"text":"    let b: &str = a.deref().deref();","highlight_start":19,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1096,"byte_end":1113,"line_start":56,"line_end":56,"column_start":19,"column_end":36,"is_primary":true,"text":[{"text":"    let b: &str = a.deref().deref();","highlight_start":19,"highlight_end":36}],"label":null,"suggested_replacement":"&**a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref` method call\n  --> tests/ui/explicit_deref_methods.rs:56:19\n   |\nLL |     let b: &str = a.deref().deref();\n   |                   ^^^^^^^^^^^^^^^^^ help: try this: `&**a`\n\n"}
{"message":"explicit `deref` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1161,"byte_end":1183,"line_start":59,"line_end":59,"column_start":13,"column_end":35,"is_primary":true,"text":[{"text":"    let b = opt_a.unwrap().deref();","highlight_start":13,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1161,"byte_end":1183,"line_start":59,"line_end":59,"column_start":13,"column_end":35,"is_primary":true,"text":[{"text":"    let b = opt_a.unwrap().deref();","highlight_start":13,"highlight_end":35}],"label":null,"suggested_replacement":"&*opt_a.unwrap()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref` method call\n  --> tests/ui/explicit_deref_methods.rs:59:13\n   |\nLL |     let b = opt_a.unwrap().deref();\n   |             ^^^^^^^^^^^^^^^^^^^^^^ help: try this: `&*opt_a.unwrap()`\n\n"}
{"message":"explicit `deref` method call","code":{"code":"clippy::explicit_deref_methods","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1668,"byte_end":1677,"line_start":85,"line_end":85,"column_start":31,"column_end":40,"is_primary":true,"text":[{"text":"    let b: &str = expr_deref!(a.deref());","highlight_start":31,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1668,"byte_end":1677,"line_start":85,"line_end":85,"column_start":31,"column_end":40,"is_primary":true,"text":[{"text":"    let b: &str = expr_deref!(a.deref());","highlight_start":31,"highlight_end":40}],"label":null,"suggested_replacement":"&*a","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: explicit `deref` method call\n  --> tests/ui/explicit_deref_methods.rs:85:31\n   |\nLL |     let b: &str = expr_deref!(a.deref());\n   |                               ^^^^^^^^^ help: try this: `&*a`\n\n"}
{"message":"call to `.deref()` on a reference in this situation does nothing","code":{"code":"noop_method_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1014,"byte_end":1022,"line_start":52,"line_end":52,"column_start":27,"column_end":35,"is_primary":true,"text":[{"text":"    let b = just_return(a).deref();","highlight_start":27,"highlight_end":35}],"label":"unnecessary method call","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the type `&str` which `deref` is being called on is the same as the type returned from `deref`, so the method call does not do anything and can be removed","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"`-D noop-method-call` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: call to `.deref()` on a reference in this situation does nothing\n  --> tests/ui/explicit_deref_methods.rs:52:27\n   |\nLL |     let b = just_return(a).deref();\n   |                           ^^^^^^^^ unnecessary method call\n   |\n   = note: the type `&str` which `deref` is being called on is the same as the type returned from `deref`, so the method call does not do anything and can be removed\n   = note: `-D noop-method-call` implied by `-D warnings`\n\n"}
{"message":"call to `.deref()` on a reference in this situation does nothing","code":{"code":"noop_method_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_deref_methods.rs","byte_start":1066,"byte_end":1074,"line_start":54,"line_end":54,"column_start":42,"column_end":50,"is_primary":true,"text":[{"text":"    let b: String = concat(just_return(a).deref());","highlight_start":42,"highlight_end":50}],"label":"unnecessary method call","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the type `&str` which `deref` is being called on is the same as the type returned from `deref`, so the method call does not do anything and can be removed","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: call to `.deref()` on a reference in this situation does nothing\n  --> tests/ui/explicit_deref_methods.rs:54:42\n   |\nLL |     let b: String = concat(just_return(a).deref());\n   |                                          ^^^^^^^^ unnecessary method call\n   |\n   = note: the type `&str` which `deref` is being called on is the same as the type returned from `deref`, so the method call does not do anything and can be removed\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

+error: unknown lint: `clone_double_ref`
+  --> $DIR/rename.rs:30:10
+   |
+LL | #![allow(clone_double_ref)]
+   |          ^^^^^^^^^^^^^^^^
+   |
+   = note: `-D unknown-lints` implied by `-D warnings`
+
 error: lint `clippy::almost_complete_letter_range` has been renamed to `clippy::almost_complete_range`
    |
    |
 LL | #![warn(clippy::almost_complete_letter_range)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::almost_complete_range`
    |
    = note: `-D renamed-and-removed-lints` implied by `-D warnings`
 
 error: lint `clippy::blacklisted_name` has been renamed to `clippy::disallowed_names`
    |
    |
 LL | #![warn(clippy::blacklisted_name)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::disallowed_names`
 
 error: lint `clippy::block_in_if_condition_expr` has been renamed to `clippy::blocks_in_if_conditions`
    |
    |
 LL | #![warn(clippy::block_in_if_condition_expr)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::blocks_in_if_conditions`
 
 error: lint `clippy::block_in_if_condition_stmt` has been renamed to `clippy::blocks_in_if_conditions`
    |
    |
 LL | #![warn(clippy::block_in_if_condition_stmt)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::blocks_in_if_conditions`
 
 error: lint `clippy::box_vec` has been renamed to `clippy::box_collection`
    |
    |
 LL | #![warn(clippy::box_vec)]
    |         ^^^^^^^^^^^^^^^ help: use the new name: `clippy::box_collection`
 
 error: lint `clippy::const_static_lifetime` has been renamed to `clippy::redundant_static_lifetimes`
    |
    |
 LL | #![warn(clippy::const_static_lifetime)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::redundant_static_lifetimes`
 
 error: lint `clippy::cyclomatic_complexity` has been renamed to `clippy::cognitive_complexity`
    |
    |
 LL | #![warn(clippy::cyclomatic_complexity)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::cognitive_complexity`
 
 error: lint `clippy::derive_hash_xor_eq` has been renamed to `clippy::derived_hash_with_manual_eq`
    |
    |
 LL | #![warn(clippy::derive_hash_xor_eq)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::derived_hash_with_manual_eq`
 
 error: lint `clippy::disallowed_method` has been renamed to `clippy::disallowed_methods`
    |
    |
 LL | #![warn(clippy::disallowed_method)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::disallowed_methods`
 
 error: lint `clippy::disallowed_type` has been renamed to `clippy::disallowed_types`
    |
    |
 LL | #![warn(clippy::disallowed_type)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::disallowed_types`
 
 error: lint `clippy::eval_order_dependence` has been renamed to `clippy::mixed_read_write_in_expression`
    |
    |
 LL | #![warn(clippy::eval_order_dependence)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::mixed_read_write_in_expression`
 
 error: lint `clippy::identity_conversion` has been renamed to `clippy::useless_conversion`
    |
    |
 LL | #![warn(clippy::identity_conversion)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::useless_conversion`
 
 error: lint `clippy::if_let_some_result` has been renamed to `clippy::match_result_ok`
    |
    |
 LL | #![warn(clippy::if_let_some_result)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::match_result_ok`
 
 error: lint `clippy::logic_bug` has been renamed to `clippy::overly_complex_bool_expr`
    |
    |
 LL | #![warn(clippy::logic_bug)]
    |         ^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::overly_complex_bool_expr`
 
 error: lint `clippy::new_without_default_derive` has been renamed to `clippy::new_without_default`
    |
    |
 LL | #![warn(clippy::new_without_default_derive)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::new_without_default`
 
 error: lint `clippy::option_and_then_some` has been renamed to `clippy::bind_instead_of_map`
    |
    |
 LL | #![warn(clippy::option_and_then_some)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::bind_instead_of_map`
 
 error: lint `clippy::option_expect_used` has been renamed to `clippy::expect_used`
    |
    |
 LL | #![warn(clippy::option_expect_used)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::expect_used`
 
 error: lint `clippy::option_map_unwrap_or` has been renamed to `clippy::map_unwrap_or`
    |
    |
 LL | #![warn(clippy::option_map_unwrap_or)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::map_unwrap_or`
 
 error: lint `clippy::option_map_unwrap_or_else` has been renamed to `clippy::map_unwrap_or`
    |
    |
 LL | #![warn(clippy::option_map_unwrap_or_else)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::map_unwrap_or`
 
 error: lint `clippy::option_unwrap_used` has been renamed to `clippy::unwrap_used`
    |
    |
 LL | #![warn(clippy::option_unwrap_used)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::unwrap_used`
 
 error: lint `clippy::ref_in_deref` has been renamed to `clippy::needless_borrow`
    |
    |
 LL | #![warn(clippy::ref_in_deref)]
    |         ^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::needless_borrow`
 
 error: lint `clippy::result_expect_used` has been renamed to `clippy::expect_used`
    |
    |
 LL | #![warn(clippy::result_expect_used)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::expect_used`
 
 error: lint `clippy::result_map_unwrap_or_else` has been renamed to `clippy::map_unwrap_or`
    |
    |
 LL | #![warn(clippy::result_map_unwrap_or_else)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::map_unwrap_or`
 
 error: lint `clippy::result_unwrap_used` has been renamed to `clippy::unwrap_used`
    |
    |
 LL | #![warn(clippy::result_unwrap_used)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::unwrap_used`
 
 error: lint `clippy::single_char_push_str` has been renamed to `clippy::single_char_add_str`
    |
    |
 LL | #![warn(clippy::single_char_push_str)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::single_char_add_str`
 
 error: lint `clippy::stutter` has been renamed to `clippy::module_name_repetitions`
    |
    |
 LL | #![warn(clippy::stutter)]
    |         ^^^^^^^^^^^^^^^ help: use the new name: `clippy::module_name_repetitions`
 
 error: lint `clippy::to_string_in_display` has been renamed to `clippy::recursive_format_impl`
    |
    |
 LL | #![warn(clippy::to_string_in_display)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::recursive_format_impl`
 
 error: lint `clippy::zero_width_space` has been renamed to `clippy::invisible_characters`
    |
    |
 LL | #![warn(clippy::zero_width_space)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::invisible_characters`
 
 error: lint `clippy::clone_double_ref` has been renamed to `suspicious_double_ref_op`
    |
    |
 LL | #![warn(clippy::clone_double_ref)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `suspicious_double_ref_op`
 error: lint `clippy::drop_bounds` has been renamed to `drop_bounds`
   --> $DIR/rename.rs:72:9
    |
    |
 LL | #![warn(clippy::drop_bounds)]
    |         ^^^^^^^^^^^^^^^^^^^ help: use the new name: `drop_bounds`
 
 error: lint `clippy::for_loop_over_option` has been renamed to `for_loops_over_fallibles`
    |
    |
 LL | #![warn(clippy::for_loop_over_option)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `for_loops_over_fallibles`
 
 error: lint `clippy::for_loop_over_result` has been renamed to `for_loops_over_fallibles`
    |
    |
 LL | #![warn(clippy::for_loop_over_result)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `for_loops_over_fallibles`
 
 error: lint `clippy::for_loops_over_fallibles` has been renamed to `for_loops_over_fallibles`
    |
    |
 LL | #![warn(clippy::for_loops_over_fallibles)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `for_loops_over_fallibles`
 
 error: lint `clippy::into_iter_on_array` has been renamed to `array_into_iter`
    |
    |
 LL | #![warn(clippy::into_iter_on_array)]
 
 
 error: lint `clippy::invalid_atomic_ordering` has been renamed to `invalid_atomic_ordering`
    |
    |
 LL | #![warn(clippy::invalid_atomic_ordering)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `invalid_atomic_ordering`
 
 error: lint `clippy::invalid_ref` has been renamed to `invalid_value`
    |
    |
 LL | #![warn(clippy::invalid_ref)]
    |         ^^^^^^^^^^^^^^^^^^^ help: use the new name: `invalid_value`
 
 error: lint `clippy::let_underscore_drop` has been renamed to `let_underscore_drop`
    |
    |
 LL | #![warn(clippy::let_underscore_drop)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `let_underscore_drop`
 
 error: lint `clippy::mem_discriminant_non_enum` has been renamed to `enum_intrinsics_non_enums`
    |
    |
 LL | #![warn(clippy::mem_discriminant_non_enum)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `enum_intrinsics_non_enums`
 
 error: lint `clippy::panic_params` has been renamed to `non_fmt_panics`
    |
    |
 LL | #![warn(clippy::panic_params)]
 
 
 error: lint `clippy::positional_named_format_parameters` has been renamed to `named_arguments_used_positionally`
    |
    |
 LL | #![warn(clippy::positional_named_format_parameters)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `named_arguments_used_positionally`
 error: lint `clippy::temporary_cstring_as_ptr` has been renamed to `temporary_cstring_as_ptr`
   --> $DIR/rename.rs:83:9
    |
    |
 LL | #![warn(clippy::temporary_cstring_as_ptr)]
 
 
 error: lint `clippy::unknown_clippy_lints` has been renamed to `unknown_lints`
    |
    |
 LL | #![warn(clippy::unknown_clippy_lints)]
 
 
 error: lint `clippy::unused_label` has been renamed to `unused_labels`
    |
    |
 LL | #![warn(clippy::unused_label)]
    |         ^^^^^^^^^^^^^^^^^^^^ help: use the new name: `unused_labels`
-error: aborting due to 43 previous errors
+error: aborting due to 44 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/rename.stage-id.stderr
diff of fixed:

 // This file was generated by `cargo dev update_lints`.
 // Use that command to update this file and do not edit by hand.
 // Manual edits will be overwritten.
 // run-rustfix
 
 
 #![allow(clippy::almost_complete_range)]
 #![allow(clippy::disallowed_names)]
 #![allow(clippy::blocks_in_if_conditions)]
 #![allow(clippy::box_collection)]
 #![allow(clippy::redundant_static_lifetimes)]
 #![allow(clippy::cognitive_complexity)]
 #![allow(clippy::derived_hash_with_manual_eq)]
 #![allow(clippy::disallowed_methods)]
 #![allow(clippy::disallowed_types)]
 #![allow(clippy::mixed_read_write_in_expression)]
 #![allow(clippy::useless_conversion)]
 #![allow(clippy::match_result_ok)]
 #![allow(clippy::overly_complex_bool_expr)]
 #![allow(clippy::new_without_default)]
 #![allow(clippy::bind_instead_of_map)]
 #![allow(clippy::expect_used)]
 #![allow(clippy::map_unwrap_or)]
 #![allow(clippy::unwrap_used)]
 #![allow(clippy::needless_borrow)]
 #![allow(clippy::single_char_add_str)]
 #![allow(clippy::module_name_repetitions)]
 #![allow(clippy::recursive_format_impl)]
 #![allow(clippy::invisible_characters)]
 #![allow(clone_double_ref)]
 #![allow(drop_bounds)]
 #![allow(for_loops_over_fallibles)]
 #![allow(array_into_iter)]
 #![allow(invalid_atomic_ordering)]
 #![allow(invalid_value)]
 #![allow(let_underscore_drop)]
 #![allow(enum_intrinsics_non_enums)]
 #![allow(non_fmt_panics)]
 #![allow(named_arguments_used_positionally)]
 #![allow(temporary_cstring_as_ptr)]
 #![allow(unknown_lints)]
 #![allow(unused_labels)]
 #![warn(clippy::almost_complete_range)]
 #![warn(clippy::disallowed_names)]
 #![warn(clippy::blocks_in_if_conditions)]
 #![warn(clippy::blocks_in_if_conditions)]
 #![warn(clippy::box_collection)]
 #![warn(clippy::redundant_static_lifetimes)]
 #![warn(clippy::cognitive_complexity)]
 #![warn(clippy::derived_hash_with_manual_eq)]
 #![warn(clippy::disallowed_methods)]
 #![warn(clippy::disallowed_types)]
 #![warn(clippy::mixed_read_write_in_expression)]
 #![warn(clippy::useless_conversion)]
 #![warn(clippy::match_result_ok)]
 #![warn(clippy::overly_complex_bool_expr)]
 #![warn(clippy::new_without_default)]
 #![warn(clippy::bind_instead_of_map)]
 #![warn(clippy::expect_used)]
 #![warn(clippy::map_unwrap_or)]
 #![warn(clippy::map_unwrap_or)]
 #![warn(clippy::unwrap_used)]
 #![warn(clippy::needless_borrow)]
 #![warn(clippy::expect_used)]
 #![warn(clippy::map_unwrap_or)]
 #![warn(clippy::unwrap_used)]
 #![warn(clippy::single_char_add_str)]
 #![warn(clippy::module_name_repetitions)]
 #![warn(clippy::recursive_format_impl)]
 #![warn(clippy::invisible_characters)]
-#![warn(clone_double_ref)]
+#![warn(suspicious_double_ref_op)]
 #![warn(drop_bounds)]
 #![warn(for_loops_over_fallibles)]
 #![warn(for_loops_over_fallibles)]
 #![warn(array_into_iter)]
 #![warn(invalid_atomic_ordering)]
---
 
 fn main() {}
 

The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/rename.stage-id.fixed
To only update this specific test, also pass `--test-args rename.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/rename.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/rename.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-d8aee1681c496322.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-60efa73d6fad7e91.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-e5b19e58ae33983e.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-40ff94c070351d6c.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-23c15a454288152e.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-e22c3fc74241d5e4.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-fc796eef9a515a44.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/rename.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"unknown lint: `clone_double_ref`","code":{"code":"unknown_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1083,"byte_end":1099,"line_start":30,"line_end":30,"column_start":10,"column_end":26,"is_primary":true,"text":[{"text":"#![allow(clone_double_ref)]","highlight_start":10,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D unknown-lints` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: unknown lint: `clone_double_ref`\n  --> tests/ui/rename.rs:30:10\n   |\nLL | #![allow(clone_double_ref)]\n   |          ^^^^^^^^^^^^^^^^\n   |\n   = note: `-D unknown-lints` implied by `-D warnings`\n\n"}
{"message":"lint `clippy::almost_complete_letter_range` has been renamed to `clippy::almost_complete_range`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1481,"byte_end":1517,"line_start":43,"line_end":43,"column_start":9,"column_end":45,"is_primary":true,"text":[{"text":"#![warn(clippy::almost_complete_letter_range)]","highlight_start":9,"highlight_end":45}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D renamed-and-removed-lints` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1481,"byte_end":1517,"line_start":43,"line_end":43,"column_start":9,"column_end":45,"is_primary":true,"text":[{"text":"#![warn(clippy::almost_complete_letter_range)]","highlight_start":9,"highlight_end":45}],"label":null,"suggested_replacement":"clippy::almost_complete_range","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::almost_complete_letter_range` has been renamed to `clippy::almost_complete_range`\n  --> tests/ui/rename.rs:43:9\n   |\nLL | #![warn(clippy::almost_complete_letter_range)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::almost_complete_range`\n   |\n   = note: `-D renamed-and-removed-lints` implied by `-D warnings`\n\n"}
{"message":"lint `clippy::blacklisted_name` has been renamed to `clippy::disallowed_names`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1528,"byte_end":1552,"line_start":44,"line_end":44,"column_start":9,"column_end":33,"is_primary":true,"text":[{"text":"#![warn(clippy::blacklisted_name)]","highlight_start":9,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1528,"byte_end":1552,"line_start":44,"line_end":44,"column_start":9,"column_end":33,"is_primary":true,"text":[{"text":"#![warn(clippy::blacklisted_name)]","highlight_start":9,"highlight_end":33}],"label":null,"suggested_replacement":"clippy::disallowed_names","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::blacklisted_name` has been renamed to `clippy::disallowed_names`\n  --> tests/ui/rename.rs:44:9\n   |\nLL | #![warn(clippy::blacklisted_name)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::disallowed_names`\n\n"}
{"message":"lint `clippy::block_in_if_condition_expr` has been renamed to `clippy::blocks_in_if_conditions`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1563,"byte_end":1597,"line_start":45,"line_end":45,"column_start":9,"column_end":43,"is_primary":true,"text":[{"text":"#![warn(clippy::block_in_if_condition_expr)]","highlight_start":9,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1563,"byte_end":1597,"line_start":45,"line_end":45,"column_start":9,"column_end":43,"is_primary":true,"text":[{"text":"#![warn(clippy::block_in_if_condition_expr)]","highlight_start":9,"highlight_end":43}],"label":null,"suggested_replacement":"clippy::blocks_in_if_conditions","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::block_in_if_condition_expr` has been renamed to `clippy::blocks_in_if_conditions`\n  --> tests/ui/rename.rs:45:9\n   |\nLL | #![warn(clippy::block_in_if_condition_expr)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::blocks_in_if_conditions`\n\n"}
{"message":"lint `clippy::block_in_if_condition_stmt` has been renamed to `clippy::blocks_in_if_conditions`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1608,"byte_end":1642,"line_start":46,"line_end":46,"column_start":9,"column_end":43,"is_primary":true,"text":[{"text":"#![warn(clippy::block_in_if_condition_stmt)]","highlight_start":9,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1608,"byte_end":1642,"line_start":46,"line_end":46,"column_start":9,"column_end":43,"is_primary":true,"text":[{"text":"#![warn(clippy::block_in_if_condition_stmt)]","highlight_start":9,"highlight_end":43}],"label":null,"suggested_replacement":"clippy::blocks_in_if_conditions","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::block_in_if_condition_stmt` has been renamed to `clippy::blocks_in_if_conditions`\n  --> tests/ui/rename.rs:46:9\n   |\nLL | #![warn(clippy::block_in_if_condition_stmt)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::blocks_in_if_conditions`\n\n"}
{"message":"lint `clippy::box_vec` has been renamed to `clippy::box_collection`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1653,"byte_end":1668,"line_start":47,"line_end":47,"column_start":9,"column_end":24,"is_primary":true,"text":[{"text":"#![warn(clippy::box_vec)]","highlight_start":9,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1653,"byte_end":1668,"line_start":47,"line_end":47,"column_start":9,"column_end":24,"is_primary":true,"text":[{"text":"#![warn(clippy::box_vec)]","highlight_start":9,"highlight_end":24}],"label":null,"suggested_replacement":"clippy::box_collection","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::box_vec` has been renamed to `clippy::box_collection`\n  --> tests/ui/rename.rs:47:9\n   |\nLL | #![warn(clippy::box_vec)]\n   |         ^^^^^^^^^^^^^^^ help: use the new name: `clippy::box_collection`\n\n"}
{"message":"lint `clippy::const_static_lifetime` has been renamed to `clippy::redundant_static_lifetimes`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1679,"byte_end":1708,"line_start":48,"line_end":48,"column_start":9,"column_end":38,"is_primary":true,"text":[{"text":"#![warn(clippy::const_static_lifetime)]","highlight_start":9,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1679,"byte_end":1708,"line_start":48,"line_end":48,"column_start":9,"column_end":38,"is_primary":true,"text":[{"text":"#![warn(clippy::const_static_lifetime)]","highlight_start":9,"highlight_end":38}],"label":null,"suggested_replacement":"clippy::redundant_static_lifetimes","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::const_static_lifetime` has been renamed to `clippy::redundant_static_lifetimes`\n  --> tests/ui/rename.rs:48:9\n   |\nLL | #![warn(clippy::const_static_lifetime)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::redundant_static_lifetimes`\n\n"}
{"message":"lint `clippy::cyclomatic_complexity` has been renamed to `clippy::cognitive_complexity`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1719,"byte_end":1748,"line_start":49,"line_end":49,"column_start":9,"column_end":38,"is_primary":true,"text":[{"text":"#![warn(clippy::cyclomatic_complexity)]","highlight_start":9,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1719,"byte_end":1748,"line_start":49,"line_end":49,"column_start":9,"column_end":38,"is_primary":true,"text":[{"text":"#![warn(clippy::cyclomatic_complexity)]","highlight_start":9,"highlight_end":38}],"label":null,"suggested_replacement":"clippy::cognitive_complexity","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::cyclomatic_complexity` has been renamed to `clippy::cognitive_complexity`\n  --> tests/ui/rename.rs:49:9\n   |\nLL | #![warn(clippy::cyclomatic_complexity)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::cognitive_complexity`\n\n"}
{"message":"lint `clippy::derive_hash_xor_eq` has been renamed to `clippy::derived_hash_with_manual_eq`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1759,"byte_end":1785,"line_start":50,"line_end":50,"column_start":9,"column_end":35,"is_primary":true,"text":[{"text":"#![warn(clippy::derive_hash_xor_eq)]","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1759,"byte_end":1785,"line_start":50,"line_end":50,"column_start":9,"column_end":35,"is_primary":true,"text":[{"text":"#![warn(clippy::derive_hash_xor_eq)]","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":"clippy::derived_hash_with_manual_eq","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::derive_hash_xor_eq` has been renamed to `clippy::derived_hash_with_manual_eq`\n  --> tests/ui/rename.rs:50:9\n   |\nLL | #![warn(clippy::derive_hash_xor_eq)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::derived_hash_with_manual_eq`\n\n"}
{"message":"lint `clippy::disallowed_method` has been renamed to `clippy::disallowed_methods`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1796,"byte_end":1821,"line_start":51,"line_end":51,"column_start":9,"column_end":34,"is_primary":true,"text":[{"text":"#![warn(clippy::disallowed_method)]","highlight_start":9,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1796,"byte_end":1821,"line_start":51,"line_end":51,"column_start":9,"column_end":34,"is_primary":true,"text":[{"text":"#![warn(clippy::disallowed_method)]","highlight_start":9,"highlight_end":34}],"label":null,"suggested_replacement":"clippy::disallowed_methods","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::disallowed_method` has been renamed to `clippy::disallowed_methods`\n  --> tests/ui/rename.rs:51:9\n   |\nLL | #![warn(clippy::disallowed_method)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::disallowed_methods`\n\n"}
{"message":"lint `clippy::disallowed_type` has been renamed to `clippy::disallowed_types`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1832,"byte_end":1855,"line_start":52,"line_end":52,"column_start":9,"column_end":32,"is_primary":true,"text":[{"text":"#![warn(clippy::disallowed_type)]","highlight_start":9,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1832,"byte_end":1855,"line_start":52,"line_end":52,"column_start":9,"column_end":32,"is_primary":true,"text":[{"text":"#![warn(clippy::disallowed_type)]","highlight_start":9,"highlight_end":32}],"label":null,"suggested_replacement":"clippy::disallowed_types","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::disallowed_type` has been renamed to `clippy::disallowed_types`\n  --> tests/ui/rename.rs:52:9\n   |\nLL | #![warn(clippy::disallowed_type)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::disallowed_types`\n\n"}
{"message":"lint `clippy::eval_order_dependence` has been renamed to `clippy::mixed_read_write_in_expression`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1866,"byte_end":1895,"line_start":53,"line_end":53,"column_start":9,"column_end":38,"is_primary":true,"text":[{"text":"#![warn(clippy::eval_order_dependence)]","highlight_start":9,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1866,"byte_end":1895,"line_start":53,"line_end":53,"column_start":9,"column_end":38,"is_primary":true,"text":[{"text":"#![warn(clippy::eval_order_dependence)]","highlight_start":9,"highlight_end":38}],"label":null,"suggested_replacement":"clippy::mixed_read_write_in_expression","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::eval_order_dependence` has been renamed to `clippy::mixed_read_write_in_expression`\n  --> tests/ui/rename.rs:53:9\n   |\nLL | #![warn(clippy::eval_order_dependence)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::mixed_read_write_in_expression`\n\n"}
{"message":"lint `clippy::identity_conversion` has been renamed to `clippy::useless_conversion`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1906,"byte_end":1933,"line_start":54,"line_end":54,"column_start":9,"column_end":36,"is_primary":true,"text":[{"text":"#![warn(clippy::identity_conversion)]","highlight_start":9,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1906,"byte_end":1933,"line_start":54,"line_end":54,"column_start":9,"column_end":36,"is_primary":true,"text":[{"text":"#![warn(clippy::identity_conversion)]","highlight_start":9,"highlight_end":36}],"label":null,"suggested_replacement":"clippy::useless_conversion","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::identity_conversion` has been renamed to `clippy::useless_conversion`\n  --> tests/ui/rename.rs:54:9\n   |\nLL | #![warn(clippy::identity_conversion)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::useless_conversion`\n\n"}
{"message":"lint `clippy::if_let_some_result` has been renamed to `clippy::match_result_ok`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1944,"byte_end":1970,"line_start":55,"line_end":55,"column_start":9,"column_end":35,"is_primary":true,"text":[{"text":"#![warn(clippy::if_let_some_result)]","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1944,"byte_end":1970,"line_start":55,"line_end":55,"column_start":9,"column_end":35,"is_primary":true,"text":[{"text":"#![warn(clippy::if_let_some_result)]","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":"clippy::match_result_ok","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::if_let_some_result` has been renamed to `clippy::match_result_ok`\n  --> tests/ui/rename.rs:55:9\n   |\nLL | #![warn(clippy::if_let_some_result)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::match_result_ok`\n\n"}
{"message":"lint `clippy::logic_bug` has been renamed to `clippy::overly_complex_bool_expr`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1981,"byte_end":1998,"line_start":56,"line_end":56,"column_start":9,"column_end":26,"is_primary":true,"text":[{"text":"#![warn(clippy::logic_bug)]","highlight_start":9,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":1981,"byte_end":1998,"line_start":56,"line_end":56,"column_start":9,"column_end":26,"is_primary":true,"text":[{"text":"#![warn(clippy::logic_bug)]","highlight_start":9,"highlight_end":26}],"label":null,"suggested_replacement":"clippy::overly_complex_bool_expr","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::logic_bug` has been renamed to `clippy::overly_complex_bool_expr`\n  --> tests/ui/rename.rs:56:9\n   |\nLL | #![warn(clippy::logic_bug)]\n   |         ^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::overly_complex_bool_expr`\n\n"}
{"message":"lint `clippy::new_without_default_derive` has been renamed to `clippy::new_without_default`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2009,"byte_end":2043,"line_start":57,"line_end":57,"column_start":9,"column_end":43,"is_primary":true,"text":[{"text":"#![warn(clippy::new_without_default_derive)]","highlight_start":9,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2009,"byte_end":2043,"line_start":57,"line_end":57,"column_start":9,"column_end":43,"is_primary":true,"text":[{"text":"#![warn(clippy::new_without_default_derive)]","highlight_start":9,"highlight_end":43}],"label":null,"suggested_replacement":"clippy::new_without_default","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::new_without_default_derive` has been renamed to `clippy::new_without_default`\n  --> tests/ui/rename.rs:57:9\n   |\nLL | #![warn(clippy::new_without_default_derive)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::new_without_default`\n\n"}
{"message":"lint `clippy::option_and_then_some` has been renamed to `clippy::bind_instead_of_map`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2054,"byte_end":2082,"line_start":58,"line_end":58,"column_start":9,"column_end":37,"is_primary":true,"text":[{"text":"#![warn(clippy::option_and_then_some)]","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2054,"byte_end":2082,"line_start":58,"line_end":58,"column_start":9,"column_end":37,"is_primary":true,"text":[{"text":"#![warn(clippy::option_and_then_some)]","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":"clippy::bind_instead_of_map","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::option_and_then_some` has been renamed to `clippy::bind_instead_of_map`\n  --> tests/ui/rename.rs:58:9\n   |\nLL | #![warn(clippy::option_and_then_some)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::bind_instead_of_map`\n\n"}
{"message":"lint `clippy::option_expect_used` has been renamed to `clippy::expect_used`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2093,"byte_end":2119,"line_start":59,"line_end":59,"column_start":9,"column_end":35,"is_primary":true,"text":[{"text":"#![warn(clippy::option_expect_used)]","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2093,"byte_end":2119,"line_start":59,"line_end":59,"column_start":9,"column_end":35,"is_primary":true,"text":[{"text":"#![warn(clippy::option_expect_used)]","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":"clippy::expect_used","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::option_expect_used` has been renamed to `clippy::expect_used`\n  --> tests/ui/rename.rs:59:9\n   |\nLL | #![warn(clippy::option_expect_used)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::expect_used`\n\n"}
{"message":"lint `clippy::option_map_unwrap_or` has been renamed to `clippy::map_unwrap_or`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2130,"byte_end":2158,"line_start":60,"line_end":60,"column_start":9,"column_end":37,"is_primary":true,"text":[{"text":"#![warn(clippy::option_map_unwrap_or)]","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2130,"byte_end":2158,"line_start":60,"line_end":60,"column_start":9,"column_end":37,"is_primary":true,"text":[{"text":"#![warn(clippy::option_map_unwrap_or)]","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":"clippy::map_unwrap_or","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::option_map_unwrap_or` has been renamed to `clippy::map_unwrap_or`\n  --> tests/ui/rename.rs:60:9\n   |\nLL | #![warn(clippy::option_map_unwrap_or)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::map_unwrap_or`\n\n"}
{"message":"lint `clippy::option_map_unwrap_or_else` has been renamed to `clippy::map_unwrap_or`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2169,"byte_end":2202,"line_start":61,"line_end":61,"column_start":9,"column_end":42,"is_primary":true,"text":[{"text":"#![warn(clippy::option_map_unwrap_or_else)]","highlight_start":9,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2169,"byte_end":2202,"line_start":61,"line_end":61,"column_start":9,"column_end":42,"is_primary":true,"text":[{"text":"#![warn(clippy::option_map_unwrap_or_else)]","highlight_start":9,"highlight_end":42}],"label":null,"suggested_replacement":"clippy::map_unwrap_or","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::option_map_unwrap_or_else` has been renamed to `clippy::map_unwrap_or`\n  --> tests/ui/rename.rs:61:9\n   |\nLL | #![warn(clippy::option_map_unwrap_or_else)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::map_unwrap_or`\n\n"}
{"message":"lint `clippy::option_unwrap_used` has been renamed to `clippy::unwrap_used`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2213,"byte_end":2239,"line_start":62,"line_end":62,"column_start":9,"column_end":35,"is_primary":true,"text":[{"text":"#![warn(clippy::option_unwrap_used)]","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2213,"byte_end":2239,"line_start":62,"line_end":62,"column_start":9,"column_end":35,"is_primary":true,"text":[{"text":"#![warn(clippy::option_unwrap_used)]","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":"clippy::unwrap_used","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::option_unwrap_used` has been renamed to `clippy::unwrap_used`\n  --> tests/ui/rename.rs:62:9\n   |\nLL | #![warn(clippy::option_unwrap_used)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::unwrap_used`\n\n"}
{"message":"lint `clippy::ref_in_deref` has been renamed to `clippy::needless_borrow`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2250,"byte_end":2270,"line_start":63,"line_end":63,"column_start":9,"column_end":29,"is_primary":true,"text":[{"text":"#![warn(clippy::ref_in_deref)]","highlight_start":9,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2250,"byte_end":2270,"line_start":63,"line_end":63,"column_start":9,"column_end":29,"is_primary":true,"text":[{"text":"#![warn(clippy::ref_in_deref)]","highlight_start":9,"highlight_end":29}],"label":null,"suggested_replacement":"clippy::needless_borrow","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::ref_in_deref` has been renamed to `clippy::needless_borrow`\n  --> tests/ui/rename.rs:63:9\n   |\nLL | #![warn(clippy::ref_in_deref)]\n   |         ^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::needless_borrow`\n\n"}
{"message":"lint `clippy::result_expect_used` has been renamed to `clippy::expect_used`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2281,"byte_end":2307,"line_start":64,"line_end":64,"column_start":9,"column_end":35,"is_primary":true,"text":[{"text":"#![warn(clippy::result_expect_used)]","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2281,"byte_end":2307,"line_start":64,"line_end":64,"column_start":9,"column_end":35,"is_primary":true,"text":[{"text":"#![warn(clippy::result_expect_used)]","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":"clippy::expect_used","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::result_expect_used` has been renamed to `clippy::expect_used`\n  --> tests/ui/rename.rs:64:9\n   |\nLL | #![warn(clippy::result_expect_used)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::expect_used`\n\n"}
{"message":"lint `clippy::result_map_unwrap_or_else` has been renamed to `clippy::map_unwrap_or`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2318,"byte_end":2351,"line_start":65,"line_end":65,"column_start":9,"column_end":42,"is_primary":true,"text":[{"text":"#![warn(clippy::result_map_unwrap_or_else)]","highlight_start":9,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2318,"byte_end":2351,"line_start":65,"line_end":65,"column_start":9,"column_end":42,"is_primary":true,"text":[{"text":"#![warn(clippy::result_map_unwrap_or_else)]","highlight_start":9,"highlight_end":42}],"label":null,"suggested_replacement":"clippy::map_unwrap_or","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::result_map_unwrap_or_else` has been renamed to `clippy::map_unwrap_or`\n  --> tests/ui/rename.rs:65:9\n   |\nLL | #![warn(clippy::result_map_unwrap_or_else)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::map_unwrap_or`\n\n"}
{"message":"lint `clippy::result_unwrap_used` has been renamed to `clippy::unwrap_used`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2362,"byte_end":2388,"line_start":66,"line_end":66,"column_start":9,"column_end":35,"is_primary":true,"text":[{"text":"#![warn(clippy::result_unwrap_used)]","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2362,"byte_end":2388,"line_start":66,"line_end":66,"column_start":9,"column_end":35,"is_primary":true,"text":[{"text":"#![warn(clippy::result_unwrap_used)]","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":"clippy::unwrap_used","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::result_unwrap_used` has been renamed to `clippy::unwrap_used`\n  --> tests/ui/rename.rs:66:9\n   |\nLL | #![warn(clippy::result_unwrap_used)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::unwrap_used`\n\n"}
{"message":"lint `clippy::single_char_push_str` has been renamed to `clippy::single_char_add_str`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2399,"byte_end":2427,"line_start":67,"line_end":67,"column_start":9,"column_end":37,"is_primary":true,"text":[{"text":"#![warn(clippy::single_char_push_str)]","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2399,"byte_end":2427,"line_start":67,"line_end":67,"column_start":9,"column_end":37,"is_primary":true,"text":[{"text":"#![warn(clippy::single_char_push_str)]","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":"clippy::single_char_add_str","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::single_char_push_str` has been renamed to `clippy::single_char_add_str`\n  --> tests/ui/rename.rs:67:9\n   |\nLL | #![warn(clippy::single_char_push_str)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::single_char_add_str`\n\n"}
{"message":"lint `clippy::stutter` has been renamed to `clippy::module_name_repetitions`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2438,"byte_end":2453,"line_start":68,"line_end":68,"column_start":9,"column_end":24,"is_primary":true,"text":[{"text":"#![warn(clippy::stutter)]","highlight_start":9,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2438,"byte_end":2453,"line_start":68,"line_end":68,"column_start":9,"column_end":24,"is_primary":true,"text":[{"text":"#![warn(clippy::stutter)]","highlight_start":9,"highlight_end":24}],"label":null,"suggested_replacement":"clippy::module_name_repetitions","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::stutter` has been renamed to `clippy::module_name_repetitions`\n  --> tests/ui/rename.rs:68:9\n   |\nLL | #![warn(clippy::stutter)]\n   |         ^^^^^^^^^^^^^^^ help: use the new name: `clippy::module_name_repetitions`\n\n"}
{"message":"lint `clippy::to_string_in_display` has been renamed to `clippy::recursive_format_impl`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2464,"byte_end":2492,"line_start":69,"line_end":69,"column_start":9,"column_end":37,"is_primary":true,"text":[{"text":"#![warn(clippy::to_string_in_display)]","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2464,"byte_end":2492,"line_start":69,"line_end":69,"column_start":9,"column_end":37,"is_primary":true,"text":[{"text":"#![warn(clippy::to_string_in_display)]","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":"clippy::recursive_format_impl","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::to_string_in_display` has been renamed to `clippy::recursive_format_impl`\n  --> tests/ui/rename.rs:69:9\n   |\nLL | #![warn(clippy::to_string_in_display)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::recursive_format_impl`\n\n"}
{"message":"lint `clippy::zero_width_space` has been renamed to `clippy::invisible_characters`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2503,"byte_end":2527,"line_start":70,"line_end":70,"column_start":9,"column_end":33,"is_primary":true,"text":[{"text":"#![warn(clippy::zero_width_space)]","highlight_start":9,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2503,"byte_end":2527,"line_start":70,"line_end":70,"column_start":9,"column_end":33,"is_primary":true,"text":[{"text":"#![warn(clippy::zero_width_space)]","highlight_start":9,"highlight_end":33}],"label":null,"suggested_replacement":"clippy::invisible_characters","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::zero_width_space` has been renamed to `clippy::invisible_characters`\n  --> tests/ui/rename.rs:70:9\n   |\nLL | #![warn(clippy::zero_width_space)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `clippy::invisible_characters`\n\n"}
{"message":"lint `clippy::clone_double_ref` has been renamed to `suspicious_double_ref_op`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2538,"byte_end":2562,"line_start":71,"line_end":71,"column_start":9,"column_end":33,"is_primary":true,"text":[{"text":"#![warn(clippy::clone_double_ref)]","highlight_start":9,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2538,"byte_end":2562,"line_start":71,"line_end":71,"column_start":9,"column_end":33,"is_primary":true,"text":[{"text":"#![warn(clippy::clone_double_ref)]","highlight_start":9,"highlight_end":33}],"label":null,"suggested_replacement":"suspicious_double_ref_op","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::clone_double_ref` has been renamed to `suspicious_double_ref_op`\n  --> tests/ui/rename.rs:71:9\n   |\nLL | #![warn(clippy::clone_double_ref)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `suspicious_double_ref_op`\n\n"}
{"message":"lint `clippy::drop_bounds` has been renamed to `drop_bounds`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2573,"byte_end":2592,"line_start":72,"line_end":72,"column_start":9,"column_end":28,"is_primary":true,"text":[{"text":"#![warn(clippy::drop_bounds)]","highlight_start":9,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2573,"byte_end":2592,"line_start":72,"line_end":72,"column_start":9,"column_end":28,"is_primary":true,"text":[{"text":"#![warn(clippy::drop_bounds)]","highlight_start":9,"highlight_end":28}],"label":null,"suggested_replacement":"drop_bounds","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::drop_bounds` has been renamed to `drop_bounds`\n  --> tests/ui/rename.rs:72:9\n   |\nLL | #![warn(clippy::drop_bounds)]\n   |         ^^^^^^^^^^^^^^^^^^^ help: use the new name: `drop_bounds`\n\n"}
{"message":"lint `clippy::for_loop_over_option` has been renamed to `for_loops_over_fallibles`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2603,"byte_end":2631,"line_start":73,"line_end":73,"column_start":9,"column_end":37,"is_primary":true,"text":[{"text":"#![warn(clippy::for_loop_over_option)]","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2603,"byte_end":2631,"line_start":73,"line_end":73,"column_start":9,"column_end":37,"is_primary":true,"text":[{"text":"#![warn(clippy::for_loop_over_option)]","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":"for_loops_over_fallibles","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::for_loop_over_option` has been renamed to `for_loops_over_fallibles`\n  --> tests/ui/rename.rs:73:9\n   |\nLL | #![warn(clippy::for_loop_over_option)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `for_loops_over_fallibles`\n\n"}
{"message":"lint `clippy::for_loop_over_result` has been renamed to `for_loops_over_fallibles`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2642,"byte_end":2670,"line_start":74,"line_end":74,"column_start":9,"column_end":37,"is_primary":true,"text":[{"text":"#![warn(clippy::for_loop_over_result)]","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2642,"byte_end":2670,"line_start":74,"line_end":74,"column_start":9,"column_end":37,"is_primary":true,"text":[{"text":"#![warn(clippy::for_loop_over_result)]","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":"for_loops_over_fallibles","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::for_loop_over_result` has been renamed to `for_loops_over_fallibles`\n  --> tests/ui/rename.rs:74:9\n   |\nLL | #![warn(clippy::for_loop_over_result)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `for_loops_over_fallibles`\n\n"}
{"message":"lint `clippy::for_loops_over_fallibles` has been renamed to `for_loops_over_fallibles`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2681,"byte_end":2713,"line_start":75,"line_end":75,"column_start":9,"column_end":41,"is_primary":true,"text":[{"text":"#![warn(clippy::for_loops_over_fallibles)]","highlight_start":9,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2681,"byte_end":2713,"line_start":75,"line_end":75,"column_start":9,"column_end":41,"is_primary":true,"text":[{"text":"#![warn(clippy::for_loops_over_fallibles)]","highlight_start":9,"highlight_end":41}],"label":null,"suggested_replacement":"for_loops_over_fallibles","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::for_loops_over_fallibles` has been renamed to `for_loops_over_fallibles`\n  --> tests/ui/rename.rs:75:9\n   |\nLL | #![warn(clippy::for_loops_over_fallibles)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `for_loops_over_fallibles`\n\n"}
{"message":"lint `clippy::into_iter_on_array` has been renamed to `array_into_iter`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2724,"byte_end":2750,"line_start":76,"line_end":76,"column_start":9,"column_end":35,"is_primary":true,"text":[{"text":"#![warn(clippy::into_iter_on_array)]","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2724,"byte_end":2750,"line_start":76,"line_end":76,"column_start":9,"column_end":35,"is_primary":true,"text":[{"text":"#![warn(clippy::into_iter_on_array)]","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":"array_into_iter","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::into_iter_on_array` has been renamed to `array_into_iter`\n  --> tests/ui/rename.rs:76:9\n   |\nLL | #![warn(clippy::into_iter_on_array)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `array_into_iter`\n\n"}
{"message":"lint `clippy::invalid_atomic_ordering` has been renamed to `invalid_atomic_ordering`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2761,"byte_end":2792,"line_start":77,"line_end":77,"column_start":9,"column_end":40,"is_primary":true,"text":[{"text":"#![warn(clippy::invalid_atomic_ordering)]","highlight_start":9,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2761,"byte_end":2792,"line_start":77,"line_end":77,"column_start":9,"column_end":40,"is_primary":true,"text":[{"text":"#![warn(clippy::invalid_atomic_ordering)]","highlight_start":9,"highlight_end":40}],"label":null,"suggested_replacement":"invalid_atomic_ordering","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::invalid_atomic_ordering` has been renamed to `invalid_atomic_ordering`\n  --> tests/ui/rename.rs:77:9\n   |\nLL | #![warn(clippy::invalid_atomic_ordering)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `invalid_atomic_ordering`\n\n"}
{"message":"lint `clippy::invalid_ref` has been renamed to `invalid_value`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2803,"byte_end":2822,"line_start":78,"line_end":78,"column_start":9,"column_end":28,"is_primary":true,"text":[{"text":"#![warn(clippy::invalid_ref)]","highlight_start":9,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2803,"byte_end":2822,"line_start":78,"line_end":78,"column_start":9,"column_end":28,"is_primary":true,"text":[{"text":"#![warn(clippy::invalid_ref)]","highlight_start":9,"highlight_end":28}],"label":null,"suggested_replacement":"invalid_value","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::invalid_ref` has been renamed to `invalid_value`\n  --> tests/ui/rename.rs:78:9\n   |\nLL | #![warn(clippy::invalid_ref)]\n   |         ^^^^^^^^^^^^^^^^^^^ help: use the new name: `invalid_value`\n\n"}
{"message":"lint `clippy::let_underscore_drop` has been renamed to `let_underscore_drop`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2833,"byte_end":2860,"line_start":79,"line_end":79,"column_start":9,"column_end":36,"is_primary":true,"text":[{"text":"#![warn(clippy::let_underscore_drop)]","highlight_start":9,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2833,"byte_end":2860,"line_start":79,"line_end":79,"column_start":9,"column_end":36,"is_primary":true,"text":[{"text":"#![warn(clippy::let_underscore_drop)]","highlight_start":9,"highlight_end":36}],"label":null,"suggested_replacement":"let_underscore_drop","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::let_underscore_drop` has been renamed to `let_underscore_drop`\n  --> tests/ui/rename.rs:79:9\n   |\nLL | #![warn(clippy::let_underscore_drop)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `let_underscore_drop`\n\n"}
{"message":"lint `clippy::mem_discriminant_non_enum` has been renamed to `enum_intrinsics_non_enums`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2871,"byte_end":2904,"line_start":80,"line_end":80,"column_start":9,"column_end":42,"is_primary":true,"text":[{"text":"#![warn(clippy::mem_discriminant_non_enum)]","highlight_start":9,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2871,"byte_end":2904,"line_start":80,"line_end":80,"column_start":9,"column_end":42,"is_primary":true,"text":[{"text":"#![warn(clippy::mem_discriminant_non_enum)]","highlight_start":9,"highlight_end":42}],"label":null,"suggested_replacement":"enum_intrinsics_non_enums","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::mem_discriminant_non_enum` has been renamed to `enum_intrinsics_non_enums`\n  --> tests/ui/rename.rs:80:9\n   |\nLL | #![warn(clippy::mem_discriminant_non_enum)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `enum_intrinsics_non_enums`\n\n"}
{"message":"lint `clippy::panic_params` has been renamed to `non_fmt_panics`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2915,"byte_end":2935,"line_start":81,"line_end":81,"column_start":9,"column_end":29,"is_primary":true,"text":[{"text":"#![warn(clippy::panic_params)]","highlight_start":9,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2915,"byte_end":2935,"line_start":81,"line_end":81,"column_start":9,"column_end":29,"is_primary":true,"text":[{"text":"#![warn(clippy::panic_params)]","highlight_start":9,"highlight_end":29}],"label":null,"suggested_replacement":"non_fmt_panics","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::panic_params` has been renamed to `non_fmt_panics`\n  --> tests/ui/rename.rs:81:9\n   |\nLL | #![warn(clippy::panic_params)]\n   |         ^^^^^^^^^^^^^^^^^^^^ help: use the new name: `non_fmt_panics`\n\n"}
{"message":"lint `clippy::positional_named_format_parameters` has been renamed to `named_arguments_used_positionally`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2946,"byte_end":2988,"line_start":82,"line_end":82,"column_start":9,"column_end":51,"is_primary":true,"text":[{"text":"#![warn(clippy::positional_named_format_parameters)]","highlight_start":9,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2946,"byte_end":2988,"line_start":82,"line_end":82,"column_start":9,"column_end":51,"is_primary":true,"text":[{"text":"#![warn(clippy::positional_named_format_parameters)]","highlight_start":9,"highlight_end":51}],"label":null,"suggested_replacement":"named_arguments_used_positionally","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::positional_named_format_parameters` has been renamed to `named_arguments_used_positionally`\n  --> tests/ui/rename.rs:82:9\n   |\nLL | #![warn(clippy::positional_named_format_parameters)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `named_arguments_used_positionally`\n\n"}
{"message":"lint `clippy::temporary_cstring_as_ptr` has been renamed to `temporary_cstring_as_ptr`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2999,"byte_end":3031,"line_start":83,"line_end":83,"column_start":9,"column_end":41,"is_primary":true,"text":[{"text":"#![warn(clippy::temporary_cstring_as_ptr)]","highlight_start":9,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":2999,"byte_end":3031,"line_start":83,"line_end":83,"column_start":9,"column_end":41,"is_primary":true,"text":[{"text":"#![warn(clippy::temporary_cstring_as_ptr)]","highlight_start":9,"highlight_end":41}],"label":null,"suggested_replacement":"temporary_cstring_as_ptr","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::temporary_cstring_as_ptr` has been renamed to `temporary_cstring_as_ptr`\n  --> tests/ui/rename.rs:83:9\n   |\nLL | #![warn(clippy::temporary_cstring_as_ptr)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `temporary_cstring_as_ptr`\n\n"}
{"message":"lint `clippy::unknown_clippy_lints` has been renamed to `unknown_lints`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":3042,"byte_end":3070,"line_start":84,"line_end":84,"column_start":9,"column_end":37,"is_primary":true,"text":[{"text":"#![warn(clippy::unknown_clippy_lints)]","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":3042,"byte_end":3070,"line_start":84,"line_end":84,"column_start":9,"column_end":37,"is_primary":true,"text":[{"text":"#![warn(clippy::unknown_clippy_lints)]","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":"unknown_lints","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::unknown_clippy_lints` has been renamed to `unknown_lints`\n  --> tests/ui/rename.rs:84:9\n   |\nLL | #![warn(clippy::unknown_clippy_lints)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `unknown_lints`\n\n"}
{"message":"lint `clippy::unused_label` has been renamed to `unused_labels`","code":{"code":"renamed_and_removed_lints","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/rename.rs","byte_start":3081,"byte_end":3101,"line_start":85,"line_end":85,"column_start":9,"column_end":29,"is_primary":true,"text":[{"text":"#![warn(clippy::unused_label)]","highlight_start":9,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use the new name","code":null,"level":"help","spans":[{"file_name":"tests/ui/rename.rs","byte_start":3081,"byte_end":3101,"line_start":85,"line_end":85,"column_start":9,"column_end":29,"is_primary":true,"text":[{"text":"#![warn(clippy::unused_label)]","highlight_start":9,"highlight_end":29}],"label":null,"suggested_replacement":"unused_labels","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: lint `clippy::unused_label` has been renamed to `unused_labels`\n  --> tests/ui/rename.rs:85:9\n   |\nLL | #![warn(clippy::unused_label)]\n   |         ^^^^^^^^^^^^^^^^^^^^ help: use the new name: `unused_labels`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/index.crates.io-6f17d22bba15001f/compiletest_rs-0.9.0/src/lib.rs:111:22
