plain

 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:10:13
    |
 LL |     let _ = !true;
    |             ^^^^^ help: try: `false`
    |
    = note: `-D clippy::nonminimal-bool` implied by `-D warnings`
 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:11:13
    |
    |
 LL |     let _ = !false;
    |             ^^^^^^ help: try: `true`
 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:12:13
    |
 LL |     let _ = !!a;
 LL |     let _ = !!a;
    |             ^^^ help: try: `a`
 
 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:13:13
    |
 LL |     let _ = false || a;
    |             ^^^^^^^^^^ help: try: `a`
 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:17:13
    |
    |
 LL |     let _ = !(!a && b);
    |             ^^^^^^^^^^ help: try: `a || !b`
 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:18:13
    |
    |
 LL |     let _ = !(!a || b);
    |             ^^^^^^^^^^ help: try: `a && !b`
 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:19:13
    |
    |
 LL |     let _ = !a && !(b && c);
    |             ^^^^^^^^^^^^^^^ help: try: `!(a || b && c)`
 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:27:13
    |
    |
 LL |     let _ = a == b && c == 5 && a == b;
    |
 help: try
    |
    |
-LL |     let _ = a == b && c == 5;
-   |             ~~~~~~~~~~~~~~~~
 LL |     let _ = !(a != b || c != 5);
    |             ~~~~~~~~~~~~~~~~~~~
+LL |     let _ = a == b && c == 5;
 
 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:28:13
    |
    |
 LL |     let _ = a == b || c == 5 || a == b;
    |
 help: try
    |
    |
-LL |     let _ = a == b || c == 5;
-   |             ~~~~~~~~~~~~~~~~
 LL |     let _ = !(a != b && c != 5);
    |             ~~~~~~~~~~~~~~~~~~~
+LL |     let _ = a == b || c == 5;
 
 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:29:13
    |
    |
 LL |     let _ = a == b && c == 5 && b == a;
    |
 help: try
    |
    |
-LL |     let _ = a == b && c == 5;
-   |             ~~~~~~~~~~~~~~~~
 LL |     let _ = !(a != b || c != 5);
    |             ~~~~~~~~~~~~~~~~~~~
+LL |     let _ = a == b && c == 5;
 
 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:30:13
    |
    |
 LL |     let _ = a != b || !(a != b || c == d);
    |
 help: try
    |
    |
-LL |     let _ = a != b || c != d;
-   |             ~~~~~~~~~~~~~~~~
 LL |     let _ = !(a == b && c == d);
    |             ~~~~~~~~~~~~~~~~~~~
+LL |     let _ = a != b || c != d;
 
 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:31:13
    |
    |
 LL |     let _ = a != b && !(a != b && c == d);
    |
 help: try
    |
    |
-LL |     let _ = a != b && c != d;
-   |             ~~~~~~~~~~~~~~~~
 LL |     let _ = !(a == b || c == d);
    |             ~~~~~~~~~~~~~~~~~~~
+LL |     let _ = a != b && c != d;
 
 error: aborting due to 12 previous errors
 
 
---
To only update this specific test, also pass `--test-args nonminimal_bool.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/nonminimal_bool.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/nonminimal_bool.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-738a932898af8104.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-432c9b3871214358.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-bbe41a872bc8e443.so" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-dead5f2b179ae6e1.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-0285a3716fdfa0ac.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-31d96b843c92ffee.so" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-b0d9270169d1e3a9.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/nonminimal_bool.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":291,"byte_end":296,"line_start":10,"line_end":10,"column_start":13,"column_end":18,"is_primary":true,"text":[{"text":"    let _ = !true;","highlight_start":13,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::nonminimal-bool` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":291,"byte_end":296,"line_start":10,"line_end":10,"column_start":13,"column_end":18,"is_primary":true,"text":[{"text":"    let _ = !true;","highlight_start":13,"highlight_end":18}],"label":null,"suggested_replacement":"false","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:10:13\n   |\nLL |     let _ = !true;\n   |             ^^^^^ help: try: `false`\n   |\n   = note: `-D clippy::nonminimal-bool` implied by `-D warnings`\n\n"}
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":310,"byte_end":316,"line_start":11,"line_end":11,"column_start":13,"column_end":19,"is_primary":true,"text":[{"text":"    let _ = !false;","highlight_start":13,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":310,"byte_end":316,"line_start":11,"line_end":11,"column_start":13,"column_end":19,"is_primary":true,"text":[{"text":"    let _ = !false;","highlight_start":13,"highlight_end":19}],"label":null,"suggested_replacement":"true","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:11:13\n   |\nLL |     let _ = !false;\n   |             ^^^^^^ help: try: `true`\n\n"}
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":330,"byte_end":333,"line_start":12,"line_end":12,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"    let _ = !!a;","highlight_start":13,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":330,"byte_end":333,"line_start":12,"line_end":12,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"    let _ = !!a;","highlight_start":13,"highlight_end":16}],"label":null,"suggested_replacement":"a","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:12:13\n   |\nLL |     let _ = !!a;\n   |             ^^^ help: try: `a`\n\n"}
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":347,"byte_end":357,"line_start":13,"line_end":13,"column_start":13,"column_end":23,"is_primary":true,"text":[{"text":"    let _ = false || a;","highlight_start":13,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":347,"byte_end":357,"line_start":13,"line_end":13,"column_start":13,"column_end":23,"is_primary":true,"text":[{"text":"    let _ = false || a;","highlight_start":13,"highlight_end":23}],"label":null,"suggested_replacement":"a","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:13:13\n   |\nLL |     let _ = false || a;\n   |             ^^^^^^^^^^ help: try: `a`\n\n"}
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":483,"byte_end":493,"line_start":17,"line_end":17,"column_start":13,"column_end":23,"is_primary":true,"text":[{"text":"    let _ = !(!a && b);","highlight_start":13,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":483,"byte_end":493,"line_start":17,"line_end":17,"column_start":13,"column_end":23,"is_primary":true,"text":[{"text":"    let _ = !(!a && b);","highlight_start":13,"highlight_end":23}],"label":null,"suggested_replacement":"a || !b","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:17:13\n   |\nLL |     let _ = !(!a && b);\n   |             ^^^^^^^^^^ help: try: `a || !b`\n\n"}
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":507,"byte_end":517,"line_start":18,"line_end":18,"column_start":13,"column_end":23,"is_primary":true,"text":[{"text":"    let _ = !(!a || b);","highlight_start":13,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":507,"byte_end":517,"line_start":18,"line_end":18,"column_start":13,"column_end":23,"is_primary":true,"text":[{"text":"    let _ = !(!a || b);","highlight_start":13,"highlight_end":23}],"label":null,"suggested_replacement":"a && !b","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:18:13\n   |\nLL |     let _ = !(!a || b);\n   |             ^^^^^^^^^^ help: try: `a && !b`\n\n"}
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":531,"byte_end":546,"line_start":19,"line_end":19,"column_start":13,"column_end":28,"is_primary":true,"text":[{"text":"    let _ = !a && !(b && c);","highlight_start":13,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":531,"byte_end":546,"line_start":19,"line_end":19,"column_start":13,"column_end":28,"is_primary":true,"text":[{"text":"    let _ = !a && !(b && c);","highlight_start":13,"highlight_end":28}],"label":null,"suggested_replacement":"!(a || b && c)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:19:13\n   |\nLL |     let _ = !a && !(b && c);\n   |             ^^^^^^^^^^^^^^^ help: try: `!(a || b && c)`\n\n"}
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":725,"byte_end":751,"line_start":27,"line_end":27,"column_start":13,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = a == b && c == 5 && a == b;","highlight_start":13,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":725,"byte_end":751,"line_start":27,"line_end":27,"column_start":13,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = a == b && c == 5 && a == b;","highlight_start":13,"highlight_end":39}],"label":null,"suggested_replacement":"!(a != b || c != 5)","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":725,"byte_end":751,"line_start":27,"line_end":27,"column_start":13,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = a == b && c == 5 && a == b;","highlight_start":13,"highlight_end":39}],"label":null,"suggested_replacement":"a == b && c == 5","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:27:13\n   |\nLL |     let _ = a == b && c == 5 && a == b;\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nhelp: try\n   |\nLL |     let _ = !(a != b || c != 5);\n   |             ~~~~~~~~~~~~~~~~~~~\nLL |     let _ = a == b && c == 5;\n   |             ~~~~~~~~~~~~~~~~\n\n"}
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":765,"byte_end":791,"line_start":28,"line_end":28,"column_start":13,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = a == b || c == 5 || a == b;","highlight_start":13,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":765,"byte_end":791,"line_start":28,"line_end":28,"column_start":13,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = a == b || c == 5 || a == b;","highlight_start":13,"highlight_end":39}],"label":null,"suggested_replacement":"!(a != b && c != 5)","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":765,"byte_end":791,"line_start":28,"line_end":28,"column_start":13,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = a == b || c == 5 || a == b;","highlight_start":13,"highlight_end":39}],"label":null,"suggested_replacement":"a == b || c == 5","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:28:13\n   |\nLL |     let _ = a == b || c == 5 || a == b;\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nhelp: try\n   |\nLL |     let _ = !(a != b && c != 5);\n   |             ~~~~~~~~~~~~~~~~~~~\nLL |     let _ = a == b || c == 5;\n   |             ~~~~~~~~~~~~~~~~\n\n"}
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":805,"byte_end":831,"line_start":29,"line_end":29,"column_start":13,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = a == b && c == 5 && b == a;","highlight_start":13,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":805,"byte_end":831,"line_start":29,"line_end":29,"column_start":13,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = a == b && c == 5 && b == a;","highlight_start":13,"highlight_end":39}],"label":null,"suggested_replacement":"!(a != b || c != 5)","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":805,"byte_end":831,"line_start":29,"line_end":29,"column_start":13,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = a == b && c == 5 && b == a;","highlight_start":13,"highlight_end":39}],"label":null,"suggested_replacement":"a == b && c == 5","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:29:13\n   |\nLL |     let _ = a == b && c == 5 && b == a;\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nhelp: try\n   |\nLL |     let _ = !(a != b || c != 5);\n   |             ~~~~~~~~~~~~~~~~~~~\nLL |     let _ = a == b && c == 5;\n   |             ~~~~~~~~~~~~~~~~\n\n"}
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":845,"byte_end":874,"line_start":30,"line_end":30,"column_start":13,"column_end":42,"is_primary":true,"text":[{"text":"    let _ = a != b || !(a != b || c == d);","highlight_start":13,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":845,"byte_end":874,"line_start":30,"line_end":30,"column_start":13,"column_end":42,"is_primary":true,"text":[{"text":"    let _ = a != b || !(a != b || c == d);","highlight_start":13,"highlight_end":42}],"label":null,"suggested_replacement":"!(a == b && c == d)","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":845,"byte_end":874,"line_start":30,"line_end":30,"column_start":13,"column_end":42,"is_primary":true,"text":[{"text":"    let _ = a != b || !(a != b || c == d);","highlight_start":13,"highlight_end":42}],"label":null,"suggested_replacement":"a != b || c != d","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:30:13\n   |\nLL |     let _ = a != b || !(a != b || c == d);\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nhelp: try\n   |\nLL |     let _ = !(a == b && c == d);\n   |             ~~~~~~~~~~~~~~~~~~~\nLL |     let _ = a != b || c != d;\n   |             ~~~~~~~~~~~~~~~~\n\n"}
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":888,"byte_end":917,"line_start":31,"line_end":31,"column_start":13,"column_end":42,"is_primary":true,"text":[{"text":"    let _ = a != b && !(a != b && c == d);","highlight_start":13,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":888,"byte_end":917,"line_start":31,"line_end":31,"column_start":13,"column_end":42,"is_primary":true,"text":[{"text":"    let _ = a != b && !(a != b && c == d);","highlight_start":13,"highlight_end":42}],"label":null,"suggested_replacement":"!(a == b || c == d)","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":888,"byte_end":917,"line_start":31,"line_end":31,"column_start":13,"column_end":42,"is_primary":true,"text":[{"text":"    let _ = a != b && !(a != b && c == d);","highlight_start":13,"highlight_end":42}],"label":null,"suggested_replacement":"a != b && c != d","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:31:13\n   |\nLL |     let _ = a != b && !(a != b && c == d);\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nhelp: try\n   |\nLL |     let _ = !(a == b || c == d);\n   |             ~~~~~~~~~~~~~~~~~~~\nLL |     let _ = a != b && c != d;\n   |             ~~~~~~~~~~~~~~~~\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.0/src/lib.rs:105:22
