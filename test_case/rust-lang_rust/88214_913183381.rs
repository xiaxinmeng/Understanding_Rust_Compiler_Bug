plain

---- compile_test stdout ----
diff of stderr:

 error: you seem to want to iterate on a map's values
+  --> $DIR/for_kv_map.rs:9:5
    |
    |
-LL |     for (_, v) in &m {
-   |                   ^^
+LL | /     for (_, v) in &m {
+LL | |         let _v = v;
+LL | |     }
    |
    |
    = note: `-D clippy::for-kv-map` implied by `-D warnings`
 help: use the corresponding method
    |
 LL |     for v in m.values() {
 
 
 error: you seem to want to iterate on a map's values
+  --> $DIR/for_kv_map.rs:14:5
    |
    |
-LL |     for (_, v) in &*m {
-   |                   ^^^
+LL | /     for (_, v) in &*m {
+LL | |         let _v = v;
+LL | |         // Here the `*` is not actually necessary, but the test tests that we don't
+LL | |         // suggest
+LL | |         // `in *m.values()` as we used to
+LL | |     }
    |
 help: use the corresponding method
    |
    |
 LL |     for v in (*m).values() {
 
 
 error: you seem to want to iterate on a map's values
+  --> $DIR/for_kv_map.rs:22:5
    |
    |
-LL |     for (_, v) in &mut m {
-   |                   ^^^^^^
+LL | /     for (_, v) in &mut m {
+LL | |         let _v = v;
+LL | |     }
    |
 help: use the corresponding method
    |
    |
 LL |     for v in m.values_mut() {
 
 
 error: you seem to want to iterate on a map's values
+  --> $DIR/for_kv_map.rs:27:5
    |
    |
-LL |     for (_, v) in &mut *m {
-   |                   ^^^^^^^
+LL | /     for (_, v) in &mut *m {
+LL | |         let _v = v;
+LL | |     }
    |
 help: use the corresponding method
    |
    |
 LL |     for v in (*m).values_mut() {
 
 
 error: you seem to want to iterate on a map's keys
+  --> $DIR/for_kv_map.rs:33:5
    |
    |
-LL |     for (k, _value) in rm {
-   |                        ^^
+LL | /     for (k, _value) in rm {
+LL | |         let _k = k;
+LL | |     }
    |
 help: use the corresponding method
    |
    |
 LL |     for k in rm.keys() {
 
 error: aborting due to 5 previous errors
 
 
---
To only update this specific test, also pass `--test-args for_kv_map.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/for_kv_map.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/for_kv_map.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-ff0575c5a2ca7cc8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7064d3f8532fb2a5.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-06539012b522c0cf.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8e5c09d8dc991112.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-90129c5dba212efa.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-17d0421109b348d4.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-54a7fffacad7d378.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/for_kv_map.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"you seem to want to iterate on a map's values","code":{"code":"clippy::for_kv_map","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_kv_map.rs","byte_start":179,"byte_end":223,"line_start":9,"line_end":11,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for (_, v) in &m {","highlight_start":5,"highlight_end":23},{"text":"        let _v = v;","highlight_start":1,"highlight_end":20},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_kv_map.rs","byte_start":179,"byte_end":223,"line_start":9,"line_end":11,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for (_, v) in &m {","highlight_start":5,"highlight_end":23},{"text":"        let _v = v;","highlight_start":1,"highlight_end":20},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_kv_map.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::for-kv-map` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use the corresponding method","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_kv_map.rs","byte_start":183,"byte_end":189,"line_start":9,"line_end":9,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"    for (_, v) in &m {","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":"v","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/for_kv_map.rs","byte_start":193,"byte_end":195,"line_start":9,"line_end":9,"column_start":19,"column_end":21,"is_primary":true,"text":[{"text":"    for (_, v) in &m {","highlight_start":19,"highlight_end":21}],"label":null,"suggested_replacement":"m.values()","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/for_kv_map.rs","byte_start":193,"byte_end":195,"line_start":9,"line_end":9,"column_start":19,"column_end":21,"is_primary":false,"text":[{"text":"    for (_, v) in &m {","highlight_start":19,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_kv_map.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: you seem to want to iterate on a map's values\n  --> tests/ui/for_kv_map.rs:9:5\n   |\nLL | /     for (_, v) in &m {\nLL | |         let _v = v;\nLL | |     }\n   | |_____^\n   |\n   = note: `-D clippy::for-kv-map` implied by `-D warnings`\nhelp: use the corresponding method\n   |\nLL |     for v in m.values() {\n   |         ~    ~~~~~~~~~~\n\n"}
{"message":"you seem to want to iterate on a map's values","code":{"code":"clippy::for_kv_map","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_kv_map.rs","byte_start":289,"byte_end":479,"line_start":14,"line_end":19,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for (_, v) in &*m {","highlight_start":5,"highlight_end":24},{"text":"        let _v = v;","highlight_start":1,"highlight_end":20},{"text":"        // Here the `*` is not actually necessary, but the test tests that we don't","highlight_start":1,"highlight_end":84},{"text":"        // suggest","highlight_start":1,"highlight_end":19},{"text":"        // `in *m.values()` as we used to","highlight_start":1,"highlight_end":42},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_kv_map.rs","byte_start":289,"byte_end":479,"line_start":14,"line_end":19,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for (_, v) in &*m {","highlight_start":5,"highlight_end":24},{"text":"        let _v = v;","highlight_start":1,"highlight_end":20},{"text":"        // Here the `*` is not actually necessary, but the test tests that we don't","highlight_start":1,"highlight_end":84},{"text":"        // suggest","highlight_start":1,"highlight_end":19},{"text":"        // `in *m.values()` as we used to","highlight_start":1,"highlight_end":42},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_kv_map.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"use the corresponding method","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_kv_map.rs","byte_start":293,"byte_end":299,"line_start":14,"line_end":14,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"    for (_, v) in &*m {","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":"v","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/for_kv_map.rs","byte_start":303,"byte_end":306,"line_start":14,"line_end":14,"column_start":19,"column_end":22,"is_primary":true,"text":[{"text":"    for (_, v) in &*m {","highlight_start":19,"highlight_end":22}],"label":null,"suggested_replacement":"(*m).values()","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/for_kv_map.rs","byte_start":303,"byte_end":306,"line_start":14,"line_end":14,"column_start":19,"column_end":22,"is_primary":false,"text":[{"text":"    for (_, v) in &*m {","highlight_start":19,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_kv_map.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: you seem to want to iterate on a map's values\n  --> tests/ui/for_kv_map.rs:14:5\n   |\nLL | /     for (_, v) in &*m {\nLL | |         let _v = v;\nLL | |         // Here the `*` is not actually necessary, but the test tests that we don't\nLL | |         // suggest\nLL | |         // `in *m.values()` as we used to\nLL | |     }\n   | |_____^\n   |\nhelp: use the corresponding method\n   |\nLL |     for v in (*m).values() {\n   |         ~    ~~~~~~~~~~~~~\n\n"}
{"message":"you seem to want to iterate on a map's values","code":{"code":"clippy::for_kv_map","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_kv_map.rs","byte_start":536,"byte_end":584,"line_start":22,"line_end":24,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for (_, v) in &mut m {","highlight_start":5,"highlight_end":27},{"text":"        let _v = v;","highlight_start":1,"highlight_end":20},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_kv_map.rs","byte_start":536,"byte_end":584,"line_start":22,"line_end":24,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for (_, v) in &mut m {","highlight_start":5,"highlight_end":27},{"text":"        let _v = v;","highlight_start":1,"highlight_end":20},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_kv_map.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"use the corresponding method","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_kv_map.rs","byte_start":540,"byte_end":546,"line_start":22,"line_end":22,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"    for (_, v) in &mut m {","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":"v","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/for_kv_map.rs","byte_start":550,"byte_end":556,"line_start":22,"line_end":22,"column_start":19,"column_end":25,"is_primary":true,"text":[{"text":"    for (_, v) in &mut m {","highlight_start":19,"highlight_end":25}],"label":null,"suggested_replacement":"m.values_mut()","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/for_kv_map.rs","byte_start":550,"byte_end":556,"line_start":22,"line_end":22,"column_start":19,"column_end":25,"is_primary":false,"text":[{"text":"    for (_, v) in &mut m {","highlight_start":19,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_kv_map.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: you seem to want to iterate on a map's values\n  --> tests/ui/for_kv_map.rs:22:5\n   |\nLL | /     for (_, v) in &mut m {\nLL | |         let _v = v;\nLL | |     }\n   | |_____^\n   |\nhelp: use the corresponding method\n   |\nLL |     for v in m.values_mut() {\n   |         ~    ~~~~~~~~~~~~~~\n\n"}
{"message":"you seem to want to iterate on a map's values","code":{"code":"clippy::for_kv_map","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_kv_map.rs","byte_start":647,"byte_end":696,"line_start":27,"line_end":29,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for (_, v) in &mut *m {","highlight_start":5,"highlight_end":28},{"text":"        let _v = v;","highlight_start":1,"highlight_end":20},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_kv_map.rs","byte_start":647,"byte_end":696,"line_start":27,"line_end":29,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for (_, v) in &mut *m {","highlight_start":5,"highlight_end":28},{"text":"        let _v = v;","highlight_start":1,"highlight_end":20},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_kv_map.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"use the corresponding method","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_kv_map.rs","byte_start":651,"byte_end":657,"line_start":27,"line_end":27,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"    for (_, v) in &mut *m {","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":"v","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/for_kv_map.rs","byte_start":661,"byte_end":668,"line_start":27,"line_end":27,"column_start":19,"column_end":26,"is_primary":true,"text":[{"text":"    for (_, v) in &mut *m {","highlight_start":19,"highlight_end":26}],"label":null,"suggested_replacement":"(*m).values_mut()","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/for_kv_map.rs","byte_start":661,"byte_end":668,"line_start":27,"line_end":27,"column_start":19,"column_end":26,"is_primary":false,"text":[{"text":"    for (_, v) in &mut *m {","highlight_start":19,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_kv_map.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: you seem to want to iterate on a map's values\n  --> tests/ui/for_kv_map.rs:27:5\n   |\nLL | /     for (_, v) in &mut *m {\nLL | |         let _v = v;\nLL | |     }\n   | |_____^\n   |\nhelp: use the corresponding method\n   |\nLL |     for v in (*m).values_mut() {\n   |         ~    ~~~~~~~~~~~~~~~~~\n\n"}
{"message":"you seem to want to iterate on a map's keys","code":{"code":"clippy::for_kv_map","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_kv_map.rs","byte_start":766,"byte_end":815,"line_start":33,"line_end":35,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for (k, _value) in rm {","highlight_start":5,"highlight_end":28},{"text":"        let _k = k;","highlight_start":1,"highlight_end":20},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_kv_map.rs","byte_start":766,"byte_end":815,"line_start":33,"line_end":35,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for (k, _value) in rm {","highlight_start":5,"highlight_end":28},{"text":"        let _k = k;","highlight_start":1,"highlight_end":20},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_kv_map.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"use the corresponding method","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_kv_map.rs","byte_start":770,"byte_end":781,"line_start":33,"line_end":33,"column_start":9,"column_end":20,"is_primary":true,"text":[{"text":"    for (k, _value) in rm {","highlight_start":9,"highlight_end":20}],"label":null,"suggested_replacement":"k","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/for_kv_map.rs","byte_start":785,"byte_end":787,"line_start":33,"line_end":33,"column_start":24,"column_end":26,"is_primary":true,"text":[{"text":"    for (k, _value) in rm {","highlight_start":24,"highlight_end":26}],"label":null,"suggested_replacement":"rm.keys()","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/for_kv_map.rs","byte_start":785,"byte_end":787,"line_start":33,"line_end":33,"column_start":24,"column_end":26,"is_primary":false,"text":[{"text":"    for (k, _value) in rm {","highlight_start":24,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_kv_map.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: you seem to want to iterate on a map's keys\n  --> tests/ui/for_kv_map.rs:33:5\n   |\nLL | /     for (k, _value) in rm {\nLL | |         let _k = k;\nLL | |     }\n   | |_____^\n   |\nhelp: use the corresponding method\n   |\nLL |     for k in rm.keys() {\n   |         ~    ~~~~~~~~~\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: you are iterating over `Iterator::next()` which is an Option; this will compile but is probably not what you want
+  --> $DIR/for_loop_unfixable.rs:21:5
    |
    |
 LL |     for _v in vec.iter().next() {}
-   |               ^^^^^^^^^^^^^^^^^
    |
    |
    = note: `-D clippy::iter-next-loop` implied by `-D warnings`
 error: aborting due to previous error
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/for_loop_unfixable.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args for_loop_unfixable.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/for_loop_unfixable.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/for_loop_unfixable.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-ff0575c5a2ca7cc8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7064d3f8532fb2a5.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-06539012b522c0cf.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8e5c09d8dc991112.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-90129c5dba212efa.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-17d0421109b348d4.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-54a7fffacad7d378.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/for_loop_unfixable.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"you are iterating over `Iterator::next()` which is an Option; this will compile but is probably not what you want","code":{"code":"clippy::iter_next_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop_unfixable.rs","byte_start":423,"byte_end":453,"line_start":21,"line_end":21,"column_start":5,"column_end":35,"is_primary":true,"text":[{"text":"    for _v in vec.iter().next() {}","highlight_start":5,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_loop_unfixable.rs","byte_start":423,"byte_end":453,"line_start":21,"line_end":21,"column_start":5,"column_end":35,"is_primary":false,"text":[{"text":"    for _v in vec.iter().next() {}","highlight_start":5,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_loop_unfixable.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::iter-next-loop` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: you are iterating over `Iterator::next()` which is an Option; this will compile but is probably not what you want\n  --> tests/ui/for_loop_unfixable.rs:21:5\n   |\nLL |     for _v in vec.iter().next() {}\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::iter-next-loop` implied by `-D warnings`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: for loop over `option`, which is an `Option`. This is more readably written as an `if let` statement
    |
 LL |     for x in option {
    |              ^^^^^^
    |
    |
    = note: `-D clippy::for-loops-over-fallibles` implied by `-D warnings`
    = help: consider replacing `for x in option` with `if let Some(x) = option`
 
 error: for loop over `result`, which is a `Result`. This is more readably written as an `if let` statement
    |
 LL |     for x in result {
    |              ^^^^^^
    |
    |
    = help: consider replacing `for x in result` with `if let Ok(x) = result`
 
 error: for loop over `option.ok_or("x not found")`, which is a `Result`. This is more readably written as an `if let` statement
    |
    |
 LL |     for x in option.ok_or("x not found") {
    |
    |
    = help: consider replacing `for x in option.ok_or("x not found")` with `if let Ok(x) = option.ok_or("x not found")`
 
 error: you are iterating over `Iterator::next()` which is an Option; this will compile but is probably not what you want
+  --> $DIR/for_loops_over_fallibles.rs:24:5
    |
    |
-LL |     for x in v.iter().next() {
-   |              ^^^^^^^^^^^^^^^
+LL | /     for x in v.iter().next() {
+LL | |         println!("{}", x);
+LL | |     }
    |
    |
    = note: `#[deny(clippy::iter_next_loop)]` on by default
 
 error: for loop over `v.iter().next().and(Some(0))`, which is an `Option`. This is more readably written as an `if let` statement
    |
    |
 LL |     for x in v.iter().next().and(Some(0)) {
    |
    |
    = help: consider replacing `for x in v.iter().next().and(Some(0))` with `if let Some(x) = v.iter().next().and(Some(0))`
 
 error: for loop over `v.iter().next().ok_or("x not found")`, which is a `Result`. This is more readably written as an `if let` statement
    |
    |
 LL |     for x in v.iter().next().ok_or("x not found") {
    |
    |
    = help: consider replacing `for x in v.iter().next().ok_or("x not found")` with `if let Ok(x) = v.iter().next().ok_or("x not found")`
 
 error: this loop never actually loops
    |
    |
 LL | /     while let Some(x) = option {
 LL | |         println!("{}", x);
 LL | |         break;
 LL | |     }
    |
    |
    = note: `#[deny(clippy::never_loop)]` on by default
 
 error: this loop never actually loops
    |
    |
 LL | /     while let Ok(x) = result {
 LL | |         println!("{}", x);
 LL | |         break;
 LL | |     }
 
 error: aborting due to 8 previous errors
 
 
---
To only update this specific test, also pass `--test-args for_loops_over_fallibles.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/for_loops_over_fallibles.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/for_loops_over_fallibles.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-ff0575c5a2ca7cc8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7064d3f8532fb2a5.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-06539012b522c0cf.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8e5c09d8dc991112.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-90129c5dba212efa.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-17d0421109b348d4.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-54a7fffacad7d378.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/for_loops_over_fallibles.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"for loop over `option`, which is an `Option`. This is more readably written as an `if let` statement","code":{"code":"clippy::for_loops_over_fallibles","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":219,"byte_end":225,"line_start":9,"line_end":9,"column_start":14,"column_end":20,"is_primary":true,"text":[{"text":"    for x in option {","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":219,"byte_end":225,"line_start":9,"line_end":9,"column_start":14,"column_end":20,"is_primary":false,"text":[{"text":"    for x in option {","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::for-loops-over-fallibles` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider replacing `for x in option` with `if let Some(x) = option`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: for loop over `option`, which is an `Option`. This is more readably written as an `if let` statement\n  --> tests/ui/for_loops_over_fallibles.rs:9:14\n   |\nLL |     for x in option {\n   |              ^^^^^^\n   |\n   = note: `-D clippy::for-loops-over-fallibles` implied by `-D warnings`\n   = help: consider replacing `for x in option` with `if let Some(x) = option`\n\n"}
{"message":"for loop over `result`, which is a `Result`. This is more readably written as an `if let` statement","code":{"code":"clippy::for_loops_over_fallibles","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":304,"byte_end":310,"line_start":14,"line_end":14,"column_start":14,"column_end":20,"is_primary":true,"text":[{"text":"    for x in result {","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":304,"byte_end":310,"line_start":14,"line_end":14,"column_start":14,"column_end":20,"is_primary":false,"text":[{"text":"    for x in result {","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider replacing `for x in result` with `if let Ok(x) = result`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: for loop over `result`, which is a `Result`. This is more readably written as an `if let` statement\n  --> tests/ui/for_loops_over_fallibles.rs:14:14\n   |\nLL |     for x in result {\n   |              ^^^^^^\n   |\n   = help: consider replacing `for x in result` with `if let Ok(x) = result`\n\n"}
{"message":"for loop over `option.ok_or(\"x not found\")`, which is a `Result`. This is more readably written as an `if let` statement","code":{"code":"clippy::for_loops_over_fallibles","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":360,"byte_end":387,"line_start":18,"line_end":18,"column_start":14,"column_end":41,"is_primary":true,"text":[{"text":"    for x in option.ok_or(\"x not found\") {","highlight_start":14,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":360,"byte_end":387,"line_start":18,"line_end":18,"column_start":14,"column_end":41,"is_primary":false,"text":[{"text":"    for x in option.ok_or(\"x not found\") {","highlight_start":14,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider replacing `for x in option.ok_or(\"x not found\")` with `if let Ok(x) = option.ok_or(\"x not found\")`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: for loop over `option.ok_or(\"x not found\")`, which is a `Result`. This is more readably written as an `if let` statement\n  --> tests/ui/for_loops_over_fallibles.rs:18:14\n   |\nLL |     for x in option.ok_or(\"x not found\") {\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider replacing `for x in option.ok_or(\"x not found\")` with `if let Ok(x) = option.ok_or(\"x not found\")`\n\n"}
{"message":"you are iterating over `Iterator::next()` which is an Option; this will compile but is probably not what you want","code":{"code":"clippy::iter_next_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":539,"byte_end":598,"line_start":24,"line_end":26,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for x in v.iter().next() {","highlight_start":5,"highlight_end":31},{"text":"        println!(\"{}\", x);","highlight_start":1,"highlight_end":27},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":539,"byte_end":598,"line_start":24,"line_end":26,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for x in v.iter().next() {","highlight_start":5,"highlight_end":31},{"text":"        println!(\"{}\", x);","highlight_start":1,"highlight_end":27},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`#[deny(clippy::iter_next_loop)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: you are iterating over `Iterator::next()` which is an Option; this will compile but is probably not what you want\n  --> tests/ui/for_loops_over_fallibles.rs:24:5\n   |\nLL | /     for x in v.iter().next() {\nLL | |         println!(\"{}\", x);\nLL | |     }\n   | |_____^\n   |\n   = note: `#[deny(clippy::iter_next_loop)]` on by default\n\n"}
{"message":"for loop over `v.iter().next().and(Some(0))`, which is an `Option`. This is more readably written as an `if let` statement","code":{"code":"clippy::for_loops_over_fallibles","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":684,"byte_end":712,"line_start":29,"line_end":29,"column_start":14,"column_end":42,"is_primary":true,"text":[{"text":"    for x in v.iter().next().and(Some(0)) {","highlight_start":14,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":684,"byte_end":712,"line_start":29,"line_end":29,"column_start":14,"column_end":42,"is_primary":false,"text":[{"text":"    for x in v.iter().next().and(Some(0)) {","highlight_start":14,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider replacing `for x in v.iter().next().and(Some(0))` with `if let Some(x) = v.iter().next().and(Some(0))`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: for loop over `v.iter().next().and(Some(0))`, which is an `Option`. This is more readably written as an `if let` statement\n  --> tests/ui/for_loops_over_fallibles.rs:29:14\n   |\nLL |     for x in v.iter().next().and(Some(0)) {\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider replacing `for x in v.iter().next().and(Some(0))` with `if let Some(x) = v.iter().next().and(Some(0))`\n\n"}
{"message":"for loop over `v.iter().next().ok_or(\"x not found\")`, which is a `Result`. This is more readably written as an `if let` statement","code":{"code":"clippy::for_loops_over_fallibles","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":762,"byte_end":798,"line_start":33,"line_end":33,"column_start":14,"column_end":50,"is_primary":true,"text":[{"text":"    for x in v.iter().next().ok_or(\"x not found\") {","highlight_start":14,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":762,"byte_end":798,"line_start":33,"line_end":33,"column_start":14,"column_end":50,"is_primary":false,"text":[{"text":"    for x in v.iter().next().ok_or(\"x not found\") {","highlight_start":14,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider replacing `for x in v.iter().next().ok_or(\"x not found\")` with `if let Ok(x) = v.iter().next().ok_or(\"x not found\")`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: for loop over `v.iter().next().ok_or(\"x not found\")`, which is a `Result`. This is more readably written as an `if let` statement\n  --> tests/ui/for_loops_over_fallibles.rs:33:14\n   |\nLL |     for x in v.iter().next().ok_or(\"x not found\") {\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider replacing `for x in v.iter().next().ok_or(\"x not found\")` with `if let Ok(x) = v.iter().next().ok_or(\"x not found\")`\n\n"}
{"message":"this loop never actually loops","code":{"code":"clippy::never_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":998,"byte_end":1074,"line_start":45,"line_end":48,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    while let Some(x) = option {","highlight_start":5,"highlight_end":33},{"text":"        println!(\"{}\", x);","highlight_start":1,"highlight_end":27},{"text":"        break;","highlight_start":1,"highlight_end":15},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[deny(clippy::never_loop)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this loop never actually loops\n  --> tests/ui/for_loops_over_fallibles.rs:45:5\n   |\nLL | /     while let Some(x) = option {\nLL | |         println!(\"{}\", x);\nLL | |         break;\nLL | |     }\n   | |_____^\n   |\n   = note: `#[deny(clippy::never_loop)]` on by default\n\n"}
{"message":"this loop never actually loops","code":{"code":"clippy::never_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loops_over_fallibles.rs","byte_start":1123,"byte_end":1197,"line_start":51,"line_end":54,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    while let Ok(x) = result {","highlight_start":5,"highlight_end":31},{"text":"        println!(\"{}\", x);","highlight_start":1,"highlight_end":27},{"text":"        break;","highlight_start":1,"highlight_end":15},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this loop never actually loops\n  --> tests/ui/for_loops_over_fallibles.rs:51:5\n   |\nLL | /     while let Ok(x) = result {\nLL | |         println!(\"{}\", x);\nLL | |         break;\nLL | |     }\n   | |_____^\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: the loop variable `i` is only used to index `vec`
-  --> $DIR/needless_range_loop.rs:10:14
+  --> $DIR/needless_range_loop.rs:10:5
    |
-LL |     for i in 0..vec.len() {
-   |              ^^^^^^^^^^^^
+LL | /     for i in 0..vec.len() {
+LL | |         println!("{}", vec[i]);
+LL | |     }
    |
    |
    = note: `-D clippy::needless-range-loop` implied by `-D warnings`
    |
    |
 LL |     for <item> in &vec {
 
 
 error: the loop variable `i` is only used to index `vec`
-  --> $DIR/needless_range_loop.rs:19:14
+  --> $DIR/needless_range_loop.rs:19:5
    |
-LL |     for i in 0..vec.len() {
-   |              ^^^^^^^^^^^^
+LL | /     for i in 0..vec.len() {
+LL | |         let _ = vec[i];
+LL | |     }
    |
 help: consider using an iterator
    |
    |
 LL |     for <item> in &vec {
 
 
 error: the loop variable `j` is only used to index `STATIC`
-  --> $DIR/needless_range_loop.rs:24:14
+  --> $DIR/needless_range_loop.rs:24:5
    |
-LL |     for j in 0..4 {
-   |              ^^^^
+LL | /     for j in 0..4 {
+LL | |         println!("{:?}", STATIC[j]);
+LL | |     }
    |
 help: consider using an iterator
    |
    |
 LL |     for <item> in &STATIC {
 
 
 error: the loop variable `j` is only used to index `CONST`
-  --> $DIR/needless_range_loop.rs:28:14
+  --> $DIR/needless_range_loop.rs:28:5
    |
-LL |     for j in 0..4 {
-   |              ^^^^
+LL | /     for j in 0..4 {
+LL | |         println!("{:?}", CONST[j]);
+LL | |     }
    |
 help: consider using an iterator
    |
    |
 LL |     for <item> in &CONST {
 
 
 error: the loop variable `i` is used to index `vec`
-  --> $DIR/needless_range_loop.rs:32:14
+  --> $DIR/needless_range_loop.rs:32:5
    |
-LL |     for i in 0..vec.len() {
-   |              ^^^^^^^^^^^^
+LL | /     for i in 0..vec.len() {
+LL | |         println!("{} {}", vec[i], i);
+LL | |     }
    |
 help: consider using an iterator
    |
    |
 LL |     for (i, <item>) in vec.iter().enumerate() {
 
 
 error: the loop variable `i` is only used to index `vec2`
-  --> $DIR/needless_range_loop.rs:40:14
+  --> $DIR/needless_range_loop.rs:40:5
    |
-LL |     for i in 0..vec.len() {
-   |              ^^^^^^^^^^^^
+LL | /     for i in 0..vec.len() {
+LL | |         println!("{}", vec2[i]);
+LL | |     }
    |
 help: consider using an iterator
    |
    |
 LL |     for <item> in vec2.iter().take(vec.len()) {
 
 
 error: the loop variable `i` is only used to index `vec`
-  --> $DIR/needless_range_loop.rs:44:14
+  --> $DIR/needless_range_loop.rs:44:5
    |
-LL |     for i in 5..vec.len() {
-   |              ^^^^^^^^^^^^
+LL | /     for i in 5..vec.len() {
+LL | |         println!("{}", vec[i]);
+LL | |     }
    |
 help: consider using an iterator
    |
    |
 LL |     for <item> in vec.iter().skip(5) {
 
 
 error: the loop variable `i` is only used to index `vec`
-  --> $DIR/needless_range_loop.rs:48:14
+  --> $DIR/needless_range_loop.rs:48:5
    |
-LL |     for i in 0..MAX_LEN {
-   |              ^^^^^^^^^^
+LL | /     for i in 0..MAX_LEN {
+LL | |         println!("{}", vec[i]);
+LL | |     }
    |
 help: consider using an iterator
    |
    |
 LL |     for <item> in vec.iter().take(MAX_LEN) {
 
 
 error: the loop variable `i` is only used to index `vec`
-  --> $DIR/needless_range_loop.rs:52:14
+  --> $DIR/needless_range_loop.rs:52:5
    |
-LL |     for i in 0..=MAX_LEN {
-   |              ^^^^^^^^^^^
+LL | /     for i in 0..=MAX_LEN {
+LL | |         println!("{}", vec[i]);
+LL | |     }
    |
 help: consider using an iterator
    |
    |
 LL |     for <item> in vec.iter().take(MAX_LEN + 1) {
 
 
 error: the loop variable `i` is only used to index `vec`
-  --> $DIR/needless_range_loop.rs:56:14
+  --> $DIR/needless_range_loop.rs:56:5
-LL |     for i in 5..10 {
-   |              ^^^^^
+LL | /     for i in 5..10 {
+LL | /     for i in 5..10 {
+LL | |         println!("{}", vec[i]);
+LL | |     }
    |
 help: consider using an iterator
    |
    |
 LL |     for <item> in vec.iter().take(10).skip(5) {
 
 
 error: the loop variable `i` is only used to index `vec`
-  --> $DIR/needless_range_loop.rs:60:14
+  --> $DIR/needless_range_loop.rs:60:5
-LL |     for i in 5..=10 {
-   |              ^^^^^^
-   |              ^^^^^^
+LL | /     for i in 5..=10 {
+LL | |         println!("{}", vec[i]);
+LL | |     }
    |
 help: consider using an iterator
    |
    |
 LL |     for <item> in vec.iter().take(10 + 1).skip(5) {
 
 
 error: the loop variable `i` is used to index `vec`
-  --> $DIR/needless_range_loop.rs:64:14
+  --> $DIR/needless_range_loop.rs:64:5
    |
-LL |     for i in 5..vec.len() {
-   |              ^^^^^^^^^^^^
+LL | /     for i in 5..vec.len() {
+LL | |         println!("{} {}", vec[i], i);
+LL | |     }
    |
 help: consider using an iterator
    |
    |
 LL |     for (i, <item>) in vec.iter().enumerate().skip(5) {
 
 
 error: the loop variable `i` is used to index `vec`
-  --> $DIR/needless_range_loop.rs:68:14
+  --> $DIR/needless_range_loop.rs:68:5
-LL |     for i in 5..10 {
-   |              ^^^^^
+LL | /     for i in 5..10 {
+LL | /     for i in 5..10 {
+LL | |         println!("{} {}", vec[i], i);
+LL | |     }
    |
 help: consider using an iterator
    |
    |
 LL |     for (i, <item>) in vec.iter().enumerate().take(10).skip(5) {
 
 
 error: the loop variable `i` is used to index `vec`
-  --> $DIR/needless_range_loop.rs:73:14
+  --> $DIR/needless_range_loop.rs:73:5
    |
-LL |     for i in 0..vec.len() {
-   |              ^^^^^^^^^^^^
+LL | /     for i in 0..vec.len() {
+LL | |         vec[i] = Some(1).unwrap_or_else(|| panic!("error on {}", i));
+LL | |     }
    |
 help: consider using an iterator
    |
    |
 LL |     for (i, <item>) in vec.iter_mut().enumerate() {
 
 error: aborting due to 14 previous errors
 
 
---
To only update this specific test, also pass `--test-args needless_range_loop.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/needless_range_loop.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/needless_range_loop.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-ff0575c5a2ca7cc8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7064d3f8532fb2a5.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-06539012b522c0cf.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8e5c09d8dc991112.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-90129c5dba212efa.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-17d0421109b348d4.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-54a7fffacad7d378.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/needless_range_loop.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"the loop variable `i` is only used to index `vec`","code":{"code":"clippy::needless_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":236,"byte_end":297,"line_start":10,"line_end":12,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":5,"highlight_end":28},{"text":"        println!(\"{}\", vec[i]);","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":236,"byte_end":297,"line_start":10,"line_end":12,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":5,"highlight_end":28},{"text":"        println!(\"{}\", vec[i]);","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::needless-range-loop` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using an iterator","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":240,"byte_end":241,"line_start":10,"line_end":10,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"<item>","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/needless_range_loop.rs","byte_start":245,"byte_end":257,"line_start":10,"line_end":10,"column_start":14,"column_end":26,"is_primary":true,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":14,"highlight_end":26}],"label":null,"suggested_replacement":"&vec","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":245,"byte_end":257,"line_start":10,"line_end":10,"column_start":14,"column_end":26,"is_primary":false,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":14,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: the loop variable `i` is only used to index `vec`\n  --> tests/ui/needless_range_loop.rs:10:5\n   |\nLL | /     for i in 0..vec.len() {\nLL | |         println!(\"{}\", vec[i]);\nLL | |     }\n   | |_____^\n   |\n   = note: `-D clippy::needless-range-loop` implied by `-D warnings`\nhelp: consider using an iterator\n   |\nLL |     for <item> in &vec {\n   |         ~~~~~~    ~~~~\n\n"}
{"message":"the loop variable `i` is only used to index `vec`","code":{"code":"clippy::needless_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":449,"byte_end":502,"line_start":19,"line_end":21,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":5,"highlight_end":28},{"text":"        let _ = vec[i];","highlight_start":1,"highlight_end":24},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":449,"byte_end":502,"line_start":19,"line_end":21,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":5,"highlight_end":28},{"text":"        let _ = vec[i];","highlight_start":1,"highlight_end":24},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider using an iterator","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":453,"byte_end":454,"line_start":19,"line_end":19,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"<item>","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/needless_range_loop.rs","byte_start":458,"byte_end":470,"line_start":19,"line_end":19,"column_start":14,"column_end":26,"is_primary":true,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":14,"highlight_end":26}],"label":null,"suggested_replacement":"&vec","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":458,"byte_end":470,"line_start":19,"line_end":19,"column_start":14,"column_end":26,"is_primary":false,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":14,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: the loop variable `i` is only used to index `vec`\n  --> tests/ui/needless_range_loop.rs:19:5\n   |\nLL | /     for i in 0..vec.len() {\nLL | |         let _ = vec[i];\nLL | |     }\n   | |_____^\n   |\nhelp: consider using an iterator\n   |\nLL |     for <item> in &vec {\n   |         ~~~~~~    ~~~~\n\n"}
{"message":"the loop variable `j` is only used to index `STATIC`","code":{"code":"clippy::needless_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":524,"byte_end":582,"line_start":24,"line_end":26,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for j in 0..4 {","highlight_start":5,"highlight_end":20},{"text":"        println!(\"{:?}\", STATIC[j]);","highlight_start":1,"highlight_end":37},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":524,"byte_end":582,"line_start":24,"line_end":26,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for j in 0..4 {","highlight_start":5,"highlight_end":20},{"text":"        println!(\"{:?}\", STATIC[j]);","highlight_start":1,"highlight_end":37},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider using an iterator","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":528,"byte_end":529,"line_start":24,"line_end":24,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    for j in 0..4 {","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"<item>","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/needless_range_loop.rs","byte_start":533,"byte_end":537,"line_start":24,"line_end":24,"column_start":14,"column_end":18,"is_primary":true,"text":[{"text":"    for j in 0..4 {","highlight_start":14,"highlight_end":18}],"label":null,"suggested_replacement":"&STATIC","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":533,"byte_end":537,"line_start":24,"line_end":24,"column_start":14,"column_end":18,"is_primary":false,"text":[{"text":"    for j in 0..4 {","highlight_start":14,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: the loop variable `j` is only used to index `STATIC`\n  --> tests/ui/needless_range_loop.rs:24:5\n   |\nLL | /     for j in 0..4 {\nLL | |         println!(\"{:?}\", STATIC[j]);\nLL | |     }\n   | |_____^\n   |\nhelp: consider using an iterator\n   |\nLL |     for <item> in &STATIC {\n   |         ~~~~~~    ~~~~~~~\n\n"}
{"message":"the loop variable `j` is only used to index `CONST`","code":{"code":"clippy::needless_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":588,"byte_end":645,"line_start":28,"line_end":30,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for j in 0..4 {","highlight_start":5,"highlight_end":20},{"text":"        println!(\"{:?}\", CONST[j]);","highlight_start":1,"highlight_end":36},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":588,"byte_end":645,"line_start":28,"line_end":30,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for j in 0..4 {","highlight_start":5,"highlight_end":20},{"text":"        println!(\"{:?}\", CONST[j]);","highlight_start":1,"highlight_end":36},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider using an iterator","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":592,"byte_end":593,"line_start":28,"line_end":28,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    for j in 0..4 {","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"<item>","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/needless_range_loop.rs","byte_start":597,"byte_end":601,"line_start":28,"line_end":28,"column_start":14,"column_end":18,"is_primary":true,"text":[{"text":"    for j in 0..4 {","highlight_start":14,"highlight_end":18}],"label":null,"suggested_replacement":"&CONST","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":597,"byte_end":601,"line_start":28,"line_end":28,"column_start":14,"column_end":18,"is_primary":false,"text":[{"text":"    for j in 0..4 {","highlight_start":14,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: the loop variable `j` is only used to index `CONST`\n  --> tests/ui/needless_range_loop.rs:28:5\n   |\nLL | /     for j in 0..4 {\nLL | |         println!(\"{:?}\", CONST[j]);\nLL | |     }\n   | |_____^\n   |\nhelp: consider using an iterator\n   |\nLL |     for <item> in &CONST {\n   |         ~~~~~~    ~~~~~~\n\n"}
{"message":"the loop variable `i` is used to index `vec`","code":{"code":"clippy::needless_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":651,"byte_end":718,"line_start":32,"line_end":34,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":5,"highlight_end":28},{"text":"        println!(\"{} {}\", vec[i], i);","highlight_start":1,"highlight_end":38},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":651,"byte_end":718,"line_start":32,"line_end":34,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":5,"highlight_end":28},{"text":"        println!(\"{} {}\", vec[i], i);","highlight_start":1,"highlight_end":38},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider using an iterator","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":655,"byte_end":656,"line_start":32,"line_end":32,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"(i, <item>)","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/needless_range_loop.rs","byte_start":660,"byte_end":672,"line_start":32,"line_end":32,"column_start":14,"column_end":26,"is_primary":true,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":14,"highlight_end":26}],"label":null,"suggested_replacement":"vec.iter().enumerate()","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":660,"byte_end":672,"line_start":32,"line_end":32,"column_start":14,"column_end":26,"is_primary":false,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":14,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: the loop variable `i` is used to index `vec`\n  --> tests/ui/needless_range_loop.rs:32:5\n   |\nLL | /     for i in 0..vec.len() {\nLL | |         println!(\"{} {}\", vec[i], i);\nLL | |     }\n   | |_____^\n   |\nhelp: consider using an iterator\n   |\nLL |     for (i, <item>) in vec.iter().enumerate() {\n   |         ~~~~~~~~~~~    ~~~~~~~~~~~~~~~~~~~~~~\n\n"}
{"message":"the loop variable `i` is only used to index `vec2`","code":{"code":"clippy::needless_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":859,"byte_end":921,"line_start":40,"line_end":42,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":5,"highlight_end":28},{"text":"        println!(\"{}\", vec2[i]);","highlight_start":1,"highlight_end":33},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":859,"byte_end":921,"line_start":40,"line_end":42,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":5,"highlight_end":28},{"text":"        println!(\"{}\", vec2[i]);","highlight_start":1,"highlight_end":33},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider using an iterator","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":863,"byte_end":864,"line_start":40,"line_end":40,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"<item>","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/needless_range_loop.rs","byte_start":868,"byte_end":880,"line_start":40,"line_end":40,"column_start":14,"column_end":26,"is_primary":true,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":14,"highlight_end":26}],"label":null,"suggested_replacement":"vec2.iter().take(vec.len())","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":868,"byte_end":880,"line_start":40,"line_end":40,"column_start":14,"column_end":26,"is_primary":false,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":14,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: the loop variable `i` is only used to index `vec2`\n  --> tests/ui/needless_range_loop.rs:40:5\n   |\nLL | /     for i in 0..vec.len() {\nLL | |         println!(\"{}\", vec2[i]);\nLL | |     }\n   | |_____^\n   |\nhelp: consider using an iterator\n   |\nLL |     for <item> in vec2.iter().take(vec.len()) {\n   |         ~~~~~~    ~~~~~~~~~~~~~~~~~~~~~~~~~~~\n\n"}
{"message":"the loop variable `i` is only used to index `vec`","code":{"code":"clippy::needless_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":927,"byte_end":988,"line_start":44,"line_end":46,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for i in 5..vec.len() {","highlight_start":5,"highlight_end":28},{"text":"        println!(\"{}\", vec[i]);","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":927,"byte_end":988,"line_start":44,"line_end":46,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for i in 5..vec.len() {","highlight_start":5,"highlight_end":28},{"text":"        println!(\"{}\", vec[i]);","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider using an iterator","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":931,"byte_end":932,"line_start":44,"line_end":44,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    for i in 5..vec.len() {","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"<item>","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/needless_range_loop.rs","byte_start":936,"byte_end":948,"line_start":44,"line_end":44,"column_start":14,"column_end":26,"is_primary":true,"text":[{"text":"    for i in 5..vec.len() {","highlight_start":14,"highlight_end":26}],"label":null,"suggested_replacement":"vec.iter().skip(5)","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":936,"byte_end":948,"line_start":44,"line_end":44,"column_start":14,"column_end":26,"is_primary":false,"text":[{"text":"    for i in 5..vec.len() {","highlight_start":14,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: the loop variable `i` is only used to index `vec`\n  --> tests/ui/needless_range_loop.rs:44:5\n   |\nLL | /     for i in 5..vec.len() {\nLL | |         println!(\"{}\", vec[i]);\nLL | |     }\n   | |_____^\n   |\nhelp: consider using an iterator\n   |\nLL |     for <item> in vec.iter().skip(5) {\n   |         ~~~~~~    ~~~~~~~~~~~~~~~~~~\n\n"}
{"message":"the loop variable `i` is only used to index `vec`","code":{"code":"clippy::needless_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":994,"byte_end":1053,"line_start":48,"line_end":50,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for i in 0..MAX_LEN {","highlight_start":5,"highlight_end":26},{"text":"        println!(\"{}\", vec[i]);","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":994,"byte_end":1053,"line_start":48,"line_end":50,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for i in 0..MAX_LEN {","highlight_start":5,"highlight_end":26},{"text":"        println!(\"{}\", vec[i]);","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider using an iterator","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":998,"byte_end":999,"line_start":48,"line_end":48,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    for i in 0..MAX_LEN {","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"<item>","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1003,"byte_end":1013,"line_start":48,"line_end":48,"column_start":14,"column_end":24,"is_primary":true,"text":[{"text":"    for i in 0..MAX_LEN {","highlight_start":14,"highlight_end":24}],"label":null,"suggested_replacement":"vec.iter().take(MAX_LEN)","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1003,"byte_end":1013,"line_start":48,"line_end":48,"column_start":14,"column_end":24,"is_primary":false,"text":[{"text":"    for i in 0..MAX_LEN {","highlight_start":14,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: the loop variable `i` is only used to index `vec`\n  --> tests/ui/needless_range_loop.rs:48:5\n   |\nLL | /     for i in 0..MAX_LEN {\nLL | |         println!(\"{}\", vec[i]);\nLL | |     }\n   | |_____^\n   |\nhelp: consider using an iterator\n   |\nLL |     for <item> in vec.iter().take(MAX_LEN) {\n   |         ~~~~~~    ~~~~~~~~~~~~~~~~~~~~~~~~\n\n"}
{"message":"the loop variable `i` is only used to index `vec`","code":{"code":"clippy::needless_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1059,"byte_end":1119,"line_start":52,"line_end":54,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for i in 0..=MAX_LEN {","highlight_start":5,"highlight_end":27},{"text":"        println!(\"{}\", vec[i]);","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1059,"byte_end":1119,"line_start":52,"line_end":54,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for i in 0..=MAX_LEN {","highlight_start":5,"highlight_end":27},{"text":"        println!(\"{}\", vec[i]);","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider using an iterator","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1063,"byte_end":1064,"line_start":52,"line_end":52,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    for i in 0..=MAX_LEN {","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"<item>","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1068,"byte_end":1079,"line_start":52,"line_end":52,"column_start":14,"column_end":25,"is_primary":true,"text":[{"text":"    for i in 0..=MAX_LEN {","highlight_start":14,"highlight_end":25}],"label":null,"suggested_replacement":"vec.iter().take(MAX_LEN + 1)","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1068,"byte_end":1079,"line_start":52,"line_end":52,"column_start":14,"column_end":25,"is_primary":false,"text":[{"text":"    for i in 0..=MAX_LEN {","highlight_start":14,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: the loop variable `i` is only used to index `vec`\n  --> tests/ui/needless_range_loop.rs:52:5\n   |\nLL | /     for i in 0..=MAX_LEN {\nLL | |         println!(\"{}\", vec[i]);\nLL | |     }\n   | |_____^\n   |\nhelp: consider using an iterator\n   |\nLL |     for <item> in vec.iter().take(MAX_LEN + 1) {\n   |         ~~~~~~    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n\n"}
{"message":"the loop variable `i` is only used to index `vec`","code":{"code":"clippy::needless_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1125,"byte_end":1179,"line_start":56,"line_end":58,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for i in 5..10 {","highlight_start":5,"highlight_end":21},{"text":"        println!(\"{}\", vec[i]);","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1125,"byte_end":1179,"line_start":56,"line_end":58,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for i in 5..10 {","highlight_start":5,"highlight_end":21},{"text":"        println!(\"{}\", vec[i]);","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider using an iterator","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1129,"byte_end":1130,"line_start":56,"line_end":56,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    for i in 5..10 {","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"<item>","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1134,"byte_end":1139,"line_start":56,"line_end":56,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"    for i in 5..10 {","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":"vec.iter().take(10).skip(5)","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1134,"byte_end":1139,"line_start":56,"line_end":56,"column_start":14,"column_end":19,"is_primary":false,"text":[{"text":"    for i in 5..10 {","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: the loop variable `i` is only used to index `vec`\n  --> tests/ui/needless_range_loop.rs:56:5\n   |\nLL | /     for i in 5..10 {\nLL | |         println!(\"{}\", vec[i]);\nLL | |     }\n   | |_____^\n   |\nhelp: consider using an iterator\n   |\nLL |     for <item> in vec.iter().take(10).skip(5) {\n   |         ~~~~~~    ~~~~~~~~~~~~~~~~~~~~~~~~~~~\n\n"}
{"message":"the loop variable `i` is only used to index `vec`","code":{"code":"clippy::needless_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1185,"byte_end":1240,"line_start":60,"line_end":62,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for i in 5..=10 {","highlight_start":5,"highlight_end":22},{"text":"        println!(\"{}\", vec[i]);","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1185,"byte_end":1240,"line_start":60,"line_end":62,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for i in 5..=10 {","highlight_start":5,"highlight_end":22},{"text":"        println!(\"{}\", vec[i]);","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider using an iterator","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1189,"byte_end":1190,"line_start":60,"line_end":60,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    for i in 5..=10 {","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"<item>","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1194,"byte_end":1200,"line_start":60,"line_end":60,"column_start":14,"column_end":20,"is_primary":true,"text":[{"text":"    for i in 5..=10 {","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":"vec.iter().take(10 + 1).skip(5)","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1194,"byte_end":1200,"line_start":60,"line_end":60,"column_start":14,"column_end":20,"is_primary":false,"text":[{"text":"    for i in 5..=10 {","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: the loop variable `i` is only used to index `vec`\n  --> tests/ui/needless_range_loop.rs:60:5\n   |\nLL | /     for i in 5..=10 {\nLL | |         println!(\"{}\", vec[i]);\nLL | |     }\n   | |_____^\n   |\nhelp: consider using an iterator\n   |\nLL |     for <item> in vec.iter().take(10 + 1).skip(5) {\n   |         ~~~~~~    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n\n"}
{"message":"the loop variable `i` is used to index `vec`","code":{"code":"clippy::needless_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1246,"byte_end":1313,"line_start":64,"line_end":66,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for i in 5..vec.len() {","highlight_start":5,"highlight_end":28},{"text":"        println!(\"{} {}\", vec[i], i);","highlight_start":1,"highlight_end":38},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1246,"byte_end":1313,"line_start":64,"line_end":66,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for i in 5..vec.len() {","highlight_start":5,"highlight_end":28},{"text":"        println!(\"{} {}\", vec[i], i);","highlight_start":1,"highlight_end":38},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider using an iterator","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1250,"byte_end":1251,"line_start":64,"line_end":64,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    for i in 5..vec.len() {","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"(i, <item>)","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1255,"byte_end":1267,"line_start":64,"line_end":64,"column_start":14,"column_end":26,"is_primary":true,"text":[{"text":"    for i in 5..vec.len() {","highlight_start":14,"highlight_end":26}],"label":null,"suggested_replacement":"vec.iter().enumerate().skip(5)","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1255,"byte_end":1267,"line_start":64,"line_end":64,"column_start":14,"column_end":26,"is_primary":false,"text":[{"text":"    for i in 5..vec.len() {","highlight_start":14,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: the loop variable `i` is used to index `vec`\n  --> tests/ui/needless_range_loop.rs:64:5\n   |\nLL | /     for i in 5..vec.len() {\nLL | |         println!(\"{} {}\", vec[i], i);\nLL | |     }\n   | |_____^\n   |\nhelp: consider using an iterator\n   |\nLL |     for (i, <item>) in vec.iter().enumerate().skip(5) {\n   |         ~~~~~~~~~~~    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n\n"}
{"message":"the loop variable `i` is used to index `vec`","code":{"code":"clippy::needless_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1319,"byte_end":1379,"line_start":68,"line_end":70,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for i in 5..10 {","highlight_start":5,"highlight_end":21},{"text":"        println!(\"{} {}\", vec[i], i);","highlight_start":1,"highlight_end":38},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1319,"byte_end":1379,"line_start":68,"line_end":70,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for i in 5..10 {","highlight_start":5,"highlight_end":21},{"text":"        println!(\"{} {}\", vec[i], i);","highlight_start":1,"highlight_end":38},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider using an iterator","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1323,"byte_end":1324,"line_start":68,"line_end":68,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    for i in 5..10 {","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"(i, <item>)","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1328,"byte_end":1333,"line_start":68,"line_end":68,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"    for i in 5..10 {","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":"vec.iter().enumerate().take(10).skip(5)","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1328,"byte_end":1333,"line_start":68,"line_end":68,"column_start":14,"column_end":19,"is_primary":false,"text":[{"text":"    for i in 5..10 {","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: the loop variable `i` is used to index `vec`\n  --> tests/ui/needless_range_loop.rs:68:5\n   |\nLL | /     for i in 5..10 {\nLL | |         println!(\"{} {}\", vec[i], i);\nLL | |     }\n   | |_____^\n   |\nhelp: consider using an iterator\n   |\nLL |     for (i, <item>) in vec.iter().enumerate().take(10).skip(5) {\n   |         ~~~~~~~~~~~    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n\n"}
error: test failed, to rerun pass '--test compile-test'
{"message":"the loop variable `i` is used to index `vec`","code":{"code":"clippy::needless_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1398,"byte_end":1497,"line_start":73,"line_end":75,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":5,"highlight_end":28},{"text":"        vec[i] = Some(1).unwrap_or_else(|| panic!(\"error on {}\", i));","highlight_start":1,"highlight_end":70},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1398,"byte_end":1497,"line_start":73,"line_end":75,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":5,"highlight_end":28},{"text":"        vec[i] = Some(1).unwrap_or_else(|| panic!(\"error on {}\", i));","highlight_start":1,"highlight_end":70},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider using an iterator","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1402,"byte_end":1403,"line_start":73,"line_end":73,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"(i, <item>)","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1407,"byte_end":1419,"line_start":73,"line_end":73,"column_start":14,"column_end":26,"is_primary":true,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":14,"highlight_end":26}],"label":null,"suggested_replacement":"vec.iter_mut().enumerate()","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":1407,"byte_end":1419,"line_start":73,"line_end":73,"column_start":14,"column_end":26,"is_primary":false,"text":[{"text":"    for i in 0..vec.len() {","highlight_start":14,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: the loop variable `i` is used to index `vec`\n  --> tests/ui/needless_range_loop.rs:73:5\n   |\nLL | /     for i in 0..vec.len() {\nLL | |         vec[i] = Some(1).unwrap_or_else(|| panic!(\"error on {}\", i));\nLL | |     }\n   | |_____^\n   |\nhelp: consider using an iterator\n   |\nLL |     for (i, <item>) in vec.iter_mut().enumerate() {\n   |         ~~~~~~~~~~~    ~~~~~~~~~~~~~~~~~~~~~~~~~~\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: the loop variable `i` is only used to index `ns`
-  --> $DIR/needless_range_loop2.rs:10:14
+  --> $DIR/needless_range_loop2.rs:10:5
-LL |     for i in 3..10 {
-   |              ^^^^^
+LL | /     for i in 3..10 {
+LL | /     for i in 3..10 {
+LL | |         println!("{}", ns[i]);
+LL | |     }
    |
    |
    = note: `-D clippy::needless-range-loop` implied by `-D warnings`
    |
    |
 LL |     for <item> in ns.iter().take(10).skip(3) {
 
 
 error: the loop variable `i` is only used to index `ms`
-  --> $DIR/needless_range_loop2.rs:31:14
+  --> $DIR/needless_range_loop2.rs:31:5
    |
-LL |     for i in 0..ms.len() {
-   |              ^^^^^^^^^^^
+LL | /     for i in 0..ms.len() {
+LL | |         ms[i] *= 2;
+LL | |     }
    |
 help: consider using an iterator
    |
    |
 LL |     for <item> in &mut ms {
 
 
 error: the loop variable `i` is only used to index `ms`
-  --> $DIR/needless_range_loop2.rs:37:14
+  --> $DIR/needless_range_loop2.rs:37:5
    |
-LL |     for i in 0..ms.len() {
-   |              ^^^^^^^^^^^
+LL | /     for i in 0..ms.len() {
+LL | |         let x = &mut ms[i];
+LL | |         *x *= 2;
+LL | |     }
    |
 help: consider using an iterator
    |
    |
 LL |     for <item> in &mut ms {
 
 
 error: the loop variable `i` is only used to index `vec`
-  --> $DIR/needless_range_loop2.rs:61:14
+  --> $DIR/needless_range_loop2.rs:61:5
    |
-LL |     for i in x..x + 4 {
-   |              ^^^^^^^^
+LL | /     for i in x..x + 4 {
+LL | |         vec[i] += 1;
+LL | |     }
    |
 help: consider using an iterator
    |
    |
 LL |     for <item> in vec.iter_mut().skip(x).take(4) {
 
 
 error: the loop variable `i` is only used to index `vec`
-  --> $DIR/needless_range_loop2.rs:68:14
+  --> $DIR/needless_range_loop2.rs:68:5
    |
-LL |     for i in x..=x + 4 {
-   |              ^^^^^^^^^
+LL | /     for i in x..=x + 4 {
+LL | |         vec[i] += 1;
+LL | |     }
    |
 help: consider using an iterator
    |
    |
 LL |     for <item> in vec.iter_mut().skip(x).take(4 + 1) {
 
 
 error: the loop variable `i` is only used to index `arr`
-  --> $DIR/needless_range_loop2.rs:74:14
+  --> $DIR/needless_range_loop2.rs:74:5
-LL |     for i in 0..3 {
-   |              ^^^^
+LL | /     for i in 0..3 {
+LL | /     for i in 0..3 {
+LL | |         println!("{}", arr[i]);
+LL | |     }
    |
 help: consider using an iterator
    |
    |
 LL |     for <item> in &arr {
 
 
 error: the loop variable `i` is only used to index `arr`
-  --> $DIR/needless_range_loop2.rs:78:14
+  --> $DIR/needless_range_loop2.rs:78:5
-LL |     for i in 0..2 {
-   |              ^^^^
+LL | /     for i in 0..2 {
+LL | /     for i in 0..2 {
+LL | |         println!("{}", arr[i]);
+LL | |     }
    |
 help: consider using an iterator
    |
    |
 LL |     for <item> in arr.iter().take(2) {
 
 
 error: the loop variable `i` is only used to index `arr`
-  --> $DIR/needless_range_loop2.rs:82:14
+  --> $DIR/needless_range_loop2.rs:82:5
-LL |     for i in 1..3 {
-   |              ^^^^
+LL | /     for i in 1..3 {
+LL | /     for i in 1..3 {
+LL | |         println!("{}", arr[i]);
+LL | |     }
    |
 help: consider using an iterator
    |
    |
 LL |     for <item> in arr.iter().skip(1) {
 
 error: aborting due to 8 previous errors
 
 
---
To only update this specific test, also pass `--test-args needless_range_loop2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/needless_range_loop2.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/needless_range_loop2.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-ff0575c5a2ca7cc8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7064d3f8532fb2a5.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-06539012b522c0cf.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8e5c09d8dc991112.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-90129c5dba212efa.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-17d0421109b348d4.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-54a7fffacad7d378.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/needless_range_loop2.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"the loop variable `i` is only used to index `ns`","code":{"code":"clippy::needless_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":144,"byte_end":197,"line_start":10,"line_end":12,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for i in 3..10 {","highlight_start":5,"highlight_end":21},{"text":"        println!(\"{}\", ns[i]);","highlight_start":1,"highlight_end":31},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":144,"byte_end":197,"line_start":10,"line_end":12,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for i in 3..10 {","highlight_start":5,"highlight_end":21},{"text":"        println!(\"{}\", ns[i]);","highlight_start":1,"highlight_end":31},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::needless-range-loop` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using an iterator","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":148,"byte_end":149,"line_start":10,"line_end":10,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    for i in 3..10 {","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"<item>","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":153,"byte_end":158,"line_start":10,"line_end":10,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"    for i in 3..10 {","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":"ns.iter().take(10).skip(3)","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":153,"byte_end":158,"line_start":10,"line_end":10,"column_start":14,"column_end":19,"is_primary":false,"text":[{"text":"    for i in 3..10 {","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: the loop variable `i` is only used to index `ns`\n  --> tests/ui/needless_range_loop2.rs:10:5\n   |\nLL | /     for i in 3..10 {\nLL | |         println!(\"{}\", ns[i]);\nLL | |     }\n   | |_____^\n   |\n   = note: `-D clippy::needless-range-loop` implied by `-D warnings`\nhelp: consider using an iterator\n   |\nLL |     for <item> in ns.iter().take(10).skip(3) {\n   |         ~~~~~~    ~~~~~~~~~~~~~~~~~~~~~~~~~~\n\n"}
{"message":"the loop variable `i` is only used to index `ms`","code":{"code":"clippy::needless_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":519,"byte_end":567,"line_start":31,"line_end":33,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for i in 0..ms.len() {","highlight_start":5,"highlight_end":27},{"text":"        ms[i] *= 2;","highlight_start":1,"highlight_end":20},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":519,"byte_end":567,"line_start":31,"line_end":33,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for i in 0..ms.len() {","highlight_start":5,"highlight_end":27},{"text":"        ms[i] *= 2;","highlight_start":1,"highlight_end":20},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider using an iterator","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":523,"byte_end":524,"line_start":31,"line_end":31,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    for i in 0..ms.len() {","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"<item>","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":528,"byte_end":539,"line_start":31,"line_end":31,"column_start":14,"column_end":25,"is_primary":true,"text":[{"text":"    for i in 0..ms.len() {","highlight_start":14,"highlight_end":25}],"label":null,"suggested_replacement":"&mut ms","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":528,"byte_end":539,"line_start":31,"line_end":31,"column_start":14,"column_end":25,"is_primary":false,"text":[{"text":"    for i in 0..ms.len() {","highlight_start":14,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: the loop variable `i` is only used to index `ms`\n  --> tests/ui/needless_range_loop2.rs:31:5\n   |\nLL | /     for i in 0..ms.len() {\nLL | |         ms[i] *= 2;\nLL | |     }\n   | |_____^\n   |\nhelp: consider using an iterator\n   |\nLL |     for <item> in &mut ms {\n   |         ~~~~~~    ~~~~~~~\n\n"}
{"message":"the loop variable `i` is only used to index `ms`","code":{"code":"clippy::needless_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":660,"byte_end":733,"line_start":37,"line_end":40,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for i in 0..ms.len() {","highlight_start":5,"highlight_end":27},{"text":"        let x = &mut ms[i];","highlight_start":1,"highlight_end":28},{"text":"        *x *= 2;","highlight_start":1,"highlight_end":17},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":660,"byte_end":733,"line_start":37,"line_end":40,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for i in 0..ms.len() {","highlight_start":5,"highlight_end":27},{"text":"        let x = &mut ms[i];","highlight_start":1,"highlight_end":28},{"text":"        *x *= 2;","highlight_start":1,"highlight_end":17},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider using an iterator","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":664,"byte_end":665,"line_start":37,"line_end":37,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    for i in 0..ms.len() {","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"<item>","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":669,"byte_end":680,"line_start":37,"line_end":37,"column_start":14,"column_end":25,"is_primary":true,"text":[{"text":"    for i in 0..ms.len() {","highlight_start":14,"highlight_end":25}],"label":null,"suggested_replacement":"&mut ms","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":669,"byte_end":680,"line_start":37,"line_end":37,"column_start":14,"column_end":25,"is_primary":false,"text":[{"text":"    for i in 0..ms.len() {","highlight_start":14,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: the loop variable `i` is only used to index `ms`\n  --> tests/ui/needless_range_loop2.rs:37:5\n   |\nLL | /     for i in 0..ms.len() {\nLL | |         let x = &mut ms[i];\nLL | |         *x *= 2;\nLL | |     }\n   | |_____^\n   |\nhelp: consider using an iterator\n   |\nLL |     for <item> in &mut ms {\n   |         ~~~~~~    ~~~~~~~\n\n"}
{"message":"the loop variable `i` is only used to index `vec`","code":{"code":"clippy::needless_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1229,"byte_end":1275,"line_start":61,"line_end":63,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for i in x..x + 4 {","highlight_start":5,"highlight_end":24},{"text":"        vec[i] += 1;","highlight_start":1,"highlight_end":21},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1229,"byte_end":1275,"line_start":61,"line_end":63,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for i in x..x + 4 {","highlight_start":5,"highlight_end":24},{"text":"        vec[i] += 1;","highlight_start":1,"highlight_end":21},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider using an iterator","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1233,"byte_end":1234,"line_start":61,"line_end":61,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    for i in x..x + 4 {","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"<item>","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1238,"byte_end":1246,"line_start":61,"line_end":61,"column_start":14,"column_end":22,"is_primary":true,"text":[{"text":"    for i in x..x + 4 {","highlight_start":14,"highlight_end":22}],"label":null,"suggested_replacement":"vec.iter_mut().skip(x).take(4)","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1238,"byte_end":1246,"line_start":61,"line_end":61,"column_start":14,"column_end":22,"is_primary":false,"text":[{"text":"    for i in x..x + 4 {","highlight_start":14,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: the loop variable `i` is only used to index `vec`\n  --> tests/ui/needless_range_loop2.rs:61:5\n   |\nLL | /     for i in x..x + 4 {\nLL | |         vec[i] += 1;\nLL | |     }\n   | |_____^\n   |\nhelp: consider using an iterator\n   |\nLL |     for <item> in vec.iter_mut().skip(x).take(4) {\n   |         ~~~~~~    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n\n"}
{"message":"the loop variable `i` is only used to index `vec`","code":{"code":"clippy::needless_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1328,"byte_end":1375,"line_start":68,"line_end":70,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for i in x..=x + 4 {","highlight_start":5,"highlight_end":25},{"text":"        vec[i] += 1;","highlight_start":1,"highlight_end":21},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1328,"byte_end":1375,"line_start":68,"line_end":70,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for i in x..=x + 4 {","highlight_start":5,"highlight_end":25},{"text":"        vec[i] += 1;","highlight_start":1,"highlight_end":21},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider using an iterator","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1332,"byte_end":1333,"line_start":68,"line_end":68,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    for i in x..=x + 4 {","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"<item>","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1337,"byte_end":1346,"line_start":68,"line_end":68,"column_start":14,"column_end":23,"is_primary":true,"text":[{"text":"    for i in x..=x + 4 {","highlight_start":14,"highlight_end":23}],"label":null,"suggested_replacement":"vec.iter_mut().skip(x).take(4 + 1)","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1337,"byte_end":1346,"line_start":68,"line_end":68,"column_start":14,"column_end":23,"is_primary":false,"text":[{"text":"    for i in x..=x + 4 {","highlight_start":14,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: the loop variable `i` is only used to index `vec`\n  --> tests/ui/needless_range_loop2.rs:68:5\n   |\nLL | /     for i in x..=x + 4 {\nLL | |         vec[i] += 1;\nLL | |     }\n   | |_____^\n   |\nhelp: consider using an iterator\n   |\nLL |     for <item> in vec.iter_mut().skip(x).take(4 + 1) {\n   |         ~~~~~~    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n\n"}
{"message":"the loop variable `i` is only used to index `arr`","code":{"code":"clippy::needless_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1407,"byte_end":1460,"line_start":74,"line_end":76,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for i in 0..3 {","highlight_start":5,"highlight_end":20},{"text":"        println!(\"{}\", arr[i]);","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1407,"byte_end":1460,"line_start":74,"line_end":76,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for i in 0..3 {","highlight_start":5,"highlight_end":20},{"text":"        println!(\"{}\", arr[i]);","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider using an iterator","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1411,"byte_end":1412,"line_start":74,"line_end":74,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    for i in 0..3 {","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"<item>","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1416,"byte_end":1420,"line_start":74,"line_end":74,"column_start":14,"column_end":18,"is_primary":true,"text":[{"text":"    for i in 0..3 {","highlight_start":14,"highlight_end":18}],"label":null,"suggested_replacement":"&arr","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1416,"byte_end":1420,"line_start":74,"line_end":74,"column_start":14,"column_end":18,"is_primary":false,"text":[{"text":"    for i in 0..3 {","highlight_start":14,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: the loop variable `i` is only used to index `arr`\n  --> tests/ui/needless_range_loop2.rs:74:5\n   |\nLL | /     for i in 0..3 {\nLL | |         println!(\"{}\", arr[i]);\nLL | |     }\n   | |_____^\n   |\nhelp: consider using an iterator\n   |\nLL |     for <item> in &arr {\n   |         ~~~~~~    ~~~~\n\n"}
{"message":"the loop variable `i` is only used to index `arr`","code":{"code":"clippy::needless_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1466,"byte_end":1519,"line_start":78,"line_end":80,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for i in 0..2 {","highlight_start":5,"highlight_end":20},{"text":"        println!(\"{}\", arr[i]);","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1466,"byte_end":1519,"line_start":78,"line_end":80,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for i in 0..2 {","highlight_start":5,"highlight_end":20},{"text":"        println!(\"{}\", arr[i]);","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider using an iterator","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1470,"byte_end":1471,"line_start":78,"line_end":78,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    for i in 0..2 {","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"<item>","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1475,"byte_end":1479,"line_start":78,"line_end":78,"column_start":14,"column_end":18,"is_primary":true,"text":[{"text":"    for i in 0..2 {","highlight_start":14,"highlight_end":18}],"label":null,"suggested_replacement":"arr.iter().take(2)","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1475,"byte_end":1479,"line_start":78,"line_end":78,"column_start":14,"column_end":18,"is_primary":false,"text":[{"text":"    for i in 0..2 {","highlight_start":14,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: the loop variable `i` is only used to index `arr`\n  --> tests/ui/needless_range_loop2.rs:78:5\n   |\nLL | /     for i in 0..2 {\nLL | |         println!(\"{}\", arr[i]);\nLL | |     }\n   | |_____^\n   |\nhelp: consider using an iterator\n   |\nLL |     for <item> in arr.iter().take(2) {\n   |         ~~~~~~    ~~~~~~~~~~~~~~~~~~\n\n"}
{"message":"the loop variable `i` is only used to index `arr`","code":{"code":"clippy::needless_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1525,"byte_end":1578,"line_start":82,"line_end":84,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    for i in 1..3 {","highlight_start":5,"highlight_end":20},{"text":"        println!(\"{}\", arr[i]);","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1525,"byte_end":1578,"line_start":82,"line_end":84,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    for i in 1..3 {","highlight_start":5,"highlight_end":20},{"text":"        println!(\"{}\", arr[i]);","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"consider using an iterator","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1529,"byte_end":1530,"line_start":82,"line_end":82,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    for i in 1..3 {","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"<item>","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1534,"byte_end":1538,"line_start":82,"line_end":82,"column_start":14,"column_end":18,"is_primary":true,"text":[{"text":"    for i in 1..3 {","highlight_start":14,"highlight_end":18}],"label":null,"suggested_replacement":"arr.iter().skip(1)","suggestion_applicability":"Unspecified","expansion":{"span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":1534,"byte_end":1538,"line_start":82,"line_end":82,"column_start":14,"column_end":18,"is_primary":false,"text":[{"text":"    for i in 1..3 {","highlight_start":14,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for` loop","def_site_span":{"file_name":"tests/ui/needless_range_loop2.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: the loop variable `i` is only used to index `arr`\n  --> tests/ui/needless_range_loop2.rs:82:5\n   |\nLL | /     for i in 1..3 {\nLL | |         println!(\"{}\", arr[i]);\nLL | |     }\n   | |_____^\n   |\nhelp: consider using an iterator\n   |\nLL |     for <item> in arr.iter().skip(1) {\n   |         ~~~~~~    ~~~~~~~~~~~~~~~~~~\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
