\n\nThis error indicates that the compiler was unable to sensibly evaluate a\nconstant expression that had to be evaluated. Attempting to divide by 0\nor causing an integer overflow are two ways to induce this error.\n\nEnsure that the expressions given can be evaluated as the desired integer type.\n\nSee the [Custom Discriminants][custom-discriminants] section of the Reference\nfor more information about setting custom integer types on fieldless enums\nusing the [`repr` attribute][repr-attribute].\n\n[custom-discriminants]: https://doc.rust-lang.org/reference/items/enumerations.html#custom-discriminant-values-for-field-less-enumerations\n[repr-attribute]: https://doc.rust-lang.org/reference/type-layout.html#reprc-enums\n"},"level":"error","spans":[{"file_name":"tests/ui/indexing_slicing_index.rs","byte_start":1127,"byte_end":1149,"line_start":31,"line_end":31,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    const { &ARR[idx4()] }; // Ok, let rustc handle const contexts.","highlight_start":5,"highlight_end":27}],"label":"referenced constant has errors","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0080]: erroneous constant used\n  --> tests/ui/indexing_slicing_index.rs:31:5\n   |\nLL |     const { &ARR[idx4()] }; // Ok, let rustc handle const contexts.\n   |     ^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors\n\n"}
{"message":"this operation will panic at runtime","code":{"code":"unconditional_panic","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/indexing_slicing_index.rs","byte_start":1288,"byte_end":1292,"line_start":35,"line_end":35,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    y[4]; // Ok, rustc will handle references too.","highlight_start":5,"highlight_end":9}],"label":"index out of bounds: the length is 4 but the index is 4","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this operation will panic at runtime\n  --> tests/ui/indexing_slicing_index.rs:35:5\n   |\nLL |     y[4]; // Ok, rustc will handle references too.\n   |     ^^^^ index out of bounds: the length is 4 but the index is 4\n\n"}
{"message":"this operation will panic at runtime","code":{"code":"unconditional_panic","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/indexing_slicing_index.rs","byte_start":1480,"byte_end":1484,"line_start":44,"line_end":44,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    x[N]; // Ok, let rustc's `unconditional_panic` lint handle `usize` indexing on arrays.","highlight_start":5,"highlight_end":9}],"label":"index out of bounds: the length is 4 but the index is 15","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this operation will panic at runtime\n  --> tests/ui/indexing_slicing_index.rs:44:5\n   |\nLL |     x[N]; // Ok, let rustc's `unconditional_panic` lint handle `usize` indexing on arrays.\n   |     ^^^^ index out of bounds: the length is 4 but the index is 15\n\n"}
{"message":"indexing may panic","code":{"code":"clippy::indexing_slicing","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/indexing_slicing_index.rs","byte_start":612,"byte_end":620,"line_start":22,"line_end":22,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"    x[index];","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::indexing-slicing` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `.get(n)` or `.get_mut(n)` instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: indexing may panic\n  --> tests/ui/indexing_slicing_index.rs:22:5\n   |\nLL |     x[index];\n   |     ^^^^^^^^\n   |\n   = note: `-D clippy::indexing-slicing` implied by `-D warnings`\n   = help: consider using `.get(n)` or `.get_mut(n)` instead\n\n"}
{"message":"indexing may panic","code":{"code":"clippy::indexing_slicing","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/indexing_slicing_index.rs","byte_start":1364,"byte_end":1368,"line_start":38,"line_end":38,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    v[0];","highlight_start":5,"highlight_end":9}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.get(n)` or `.get_mut(n)` instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: indexing may panic\n  --> tests/ui/indexing_slicing_index.rs:38:5\n   |\nLL |     v[0];\n   |     ^^^^\n   |\n   = help: consider using `.get(n)` or `.get_mut(n)` instead\n\n"}
{"message":"indexing may panic","code":{"code":"clippy::indexing_slicing","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/indexing_slicing_index.rs","byte_start":1374,"byte_end":1379,"line_start":39,"line_end":39,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    v[10];","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.get(n)` or `.get_mut(n)` instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: indexing may panic\n  --> tests/ui/indexing_slicing_index.rs:39:5\n   |\nLL |     v[10];\n   |     ^^^^^\n   |\n   = help: consider using `.get(n)` or `.get_mut(n)` instead\n\n"}
{"message":"indexing may panic","code":{"code":"clippy::indexing_slicing","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/indexing_slicing_index.rs","byte_start":1385,"byte_end":1394,"line_start":40,"line_end":40,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    v[1 << 3];","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.get(n)` or `.get_mut(n)` instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: indexing may panic\n  --> tests/ui/indexing_slicing_index.rs:40:5\n   |\nLL |     v[1 << 3];\n   |     ^^^^^^^^^\n   |\n   = help: consider using `.get(n)` or `.get_mut(n)` instead\n\n"}
{"message":"indexing may panic","code":{"code":"clippy::indexing_slicing","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/indexing_slicing_index.rs","byte_start":1615,"byte_end":1619,"line_start":46,"line_end":46,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    v[N];","highlight_start":5,"highlight_end":9}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.get(n)` or `.get_mut(n)` instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: indexing may panic\n  --> tests/ui/indexing_slicing_index.rs:46:5\n   |\nLL |     v[N];\n   |     ^^^^\n   |\n   = help: consider using `.get(n)` or `.get_mut(n)` instead\n\n"}
{"message":"indexing may panic","code":{"code":"clippy::indexing_slicing","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/indexing_slicing_index.rs","byte_start":1625,"byte_end":1629,"line_start":47,"line_end":47,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    v[M];","highlight_start":5,"highlight_end":9}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.get(n)` or `.get_mut(n)` instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: indexing may panic\n  --> tests/ui/indexing_slicing_index.rs:47:5\n   |\nLL |     v[M];\n   |     ^^^^\n   |\n   = help: consider using `.get(n)` or `.get_mut(n)` instead\n\n"}
{"message":"For more information about this error, try `rustc --explain E0080`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0080`.\n"}

------------------------------------------


diff of stderr:

+error: this operation will panic at runtime
+  --> $DIR/self_assignment.rs:16:15
+   |
+LL |     s.b[10] = s.b[5 + 5];
+   |               ^^^^^^^^^^ index out of bounds: the length is 10 but the index is 10
+   = note: `#[deny(unconditional_panic)]` on by default
+
+error: this operation will panic at runtime
+  --> $DIR/self_assignment.rs:16:5
+  --> $DIR/self_assignment.rs:16:5
+   |
+LL |     s.b[10] = s.b[5 + 5];
+   |     ^^^^^^^ index out of bounds: the length is 10 but the index is 10
 error: self-assignment of `a` to `a`
   --> $DIR/self_assignment.rs:12:5
    |
 LL |     a = a;
 LL |     a = a;
    |     ^^^^^
    |
    = note: `-D clippy::self-assignment` implied by `-D warnings`
 
 error: self-assignment of `*b` to `*b`
    |
    |
 LL |     *b = *b;
 
 
 error: self-assignment of `s` to `s`
    |
 LL |     s = s;
    |     ^^^^^
 
 
 error: self-assignment of `s.a` to `s.a`
    |
    |
 LL |     s.a = s.a;
 
 
 error: self-assignment of `s.b[5 + 5]` to `s.b[10]`
    |
    |
 LL |     s.b[10] = s.b[5 + 5];
 
 
 error: self-assignment of `s.c[0][1]` to `s.c[0][1]`
    |
    |
 LL |     s.c[0][1] = s.c[0][1];
 
 
 error: self-assignment of `s.b[a]` to `s.b[a]`
    |
    |
 LL |     s.b[a] = s.b[a];
 
 
 error: self-assignment of `*s.e` to `*s.e`
    |
    |
 LL |     *s.e = *s.e;
 
 
 error: self-assignment of `s.b[10 + a]` to `s.b[a + 10]`
    |
    |
 LL |     s.b[a + 10] = s.b[10 + a];
 
 
 error: self-assignment of `t.1` to `t.1`
    |
    |
 LL |     t.1 = t.1;
 
 
 error: self-assignment of `(t.0)` to `t.0`
    |
    |
 LL |     t.0 = (t.0);
 
-error: aborting due to 11 previous errors
+error: aborting due to 13 previous errors
 
---
To only update this specific test, also pass `--test-args self_assignment.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/self_assignment.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/self_assignment.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-0c795f7a8756f15a.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-b0e96f2e9d30bd37.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-11c942eb60796e9d.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-3e103f3c7cb1e342.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-021aec868151835c.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-507b29393c1a728f.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-04f014bd62aa87c5.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-36709515b9cb16b6.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-cdd893c121eb00e4.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-7dc368fb32eb8aae.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-56bce9bcc023120a.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-71205fa4273edf27.so" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/self_assignment.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this operation will panic at runtime","code":{"code":"unconditional_panic","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/self_assignment.rs","byte_start":269,"byte_end":279,"line_start":16,"line_end":16,"column_start":15,"column_end":25,"is_primary":true,"text":[{"text":"    s.b[10] = s.b[5 + 5];","highlight_start":15,"highlight_end":25}],"label":"index out of bounds: the length is 10 but the index is 10","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[deny(unconditional_panic)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this operation will panic at runtime\n  --> tests/ui/self_assignment.rs:16:15\n   |\nLL |     s.b[10] = s.b[5 + 5];\n   |               ^^^^^^^^^^ index out of bounds: the length is 10 but the index is 10\n   |\n   = note: `#[deny(unconditional_panic)]` on by default\n\n"}
{"message":"this operation will panic at runtime","code":{"code":"unconditional_panic","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/self_assignment.rs","byte_start":259,"byte_end":266,"line_start":16,"line_end":16,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    s.b[10] = s.b[5 + 5];","highlight_start":5,"highlight_end":12}],"label":"index out of bounds: the length is 10 but the index is 10","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this operation will panic at runtime\n  --> tests/ui/self_assignment.rs:16:5\n   |\nLL |     s.b[10] = s.b[5 + 5];\n   |     ^^^^^^^ index out of bounds: the length is 10 but the index is 10\n\n"}
{"message":"self-assignment of `a` to `a`","code":{"code":"clippy::self_assignment","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/self_assignment.rs","byte_start":209,"byte_end":214,"line_start":12,"line_end":12,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    a = a;","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::self-assignment` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: self-assignment of `a` to `a`\n  --> tests/ui/self_assignment.rs:12:5\n   |\nLL |     a = a;\n   |     ^^^^^\n   |\n   = note: `-D clippy::self-assignment` implied by `-D warnings`\n\n"}
{"message":"self-assignment of `*b` to `*b`","code":{"code":"clippy::self_assignment","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/self_assignment.rs","byte_start":220,"byte_end":227,"line_start":13,"line_end":13,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    *b = *b;","highlight_start":5,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: self-assignment of `*b` to `*b`\n  --> tests/ui/self_assignment.rs:13:5\n   |\nLL |     *b = *b;\n   |     ^^^^^^^\n\n"}
{"message":"self-assignment of `s` to `s`","code":{"code":"clippy::self_assignment","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/self_assignment.rs","byte_start":233,"byte_end":238,"line_start":14,"line_end":14,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    s = s;","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: self-assignment of `s` to `s`\n  --> tests/ui/self_assignment.rs:14:5\n   |\nLL |     s = s;\n   |     ^^^^^\n\n"}
{"message":"self-assignment of `s.a` to `s.a`","code":{"code":"clippy::self_assignment","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/self_assignment.rs","byte_start":244,"byte_end":253,"line_start":15,"line_end":15,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    s.a = s.a;","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: self-assignment of `s.a` to `s.a`\n  --> tests/ui/self_assignment.rs:15:5\n   |\nLL |     s.a = s.a;\n   |     ^^^^^^^^^\n\n"}
{"message":"self-assignment of `s.b[5 + 5]` to `s.b[10]`","code":{"code":"clippy::self_assignment","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/self_assignment.rs","byte_start":259,"byte_end":279,"line_start":16,"line_end":16,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    s.b[10] = s.b[5 + 5];","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: self-assignment of `s.b[5 + 5]` to `s.b[10]`\n  --> tests/ui/self_assignment.rs:16:5\n   |\nLL |     s.b[10] = s.b[5 + 5];\n   |     ^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"self-assignment of `s.c[0][1]` to `s.c[0][1]`","code":{"code":"clippy::self_assignment","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/self_assignment.rs","byte_start":285,"byte_end":306,"line_start":17,"line_end":17,"column_start":5,"column_end":26,"is_primary":true,"text":[{"text":"    s.c[0][1] = s.c[0][1];","highlight_start":5,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: self-assignment of `s.c[0][1]` to `s.c[0][1]`\n  --> tests/ui/self_assignment.rs:17:5\n   |\nLL |     s.c[0][1] = s.c[0][1];\n   |     ^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"self-assignment of `s.b[a]` to `s.b[a]`","code":{"code":"clippy::self_assignment","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/self_assignment.rs","byte_start":312,"byte_end":327,"line_start":18,"line_end":18,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    s.b[a] = s.b[a];","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: self-assignment of `s.b[a]` to `s.b[a]`\n  --> tests/ui/self_assignment.rs:18:5\n   |\nLL |     s.b[a] = s.b[a];\n   |     ^^^^^^^^^^^^^^^\n\n"}
{"message":"self-assignment of `*s.e` to `*s.e`","code":{"code":"clippy::self_assignment","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/self_assignment.rs","byte_start":333,"byte_end":344,"line_start":19,"line_end":19,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    *s.e = *s.e;","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: self-assignment of `*s.e` to `*s.e`\n  --> tests/ui/self_assignment.rs:19:5\n   |\nLL |     *s.e = *s.e;\n   |     ^^^^^^^^^^^\n\n"}
{"message":"self-assignment of `s.b[10 + a]` to `s.b[a + 10]`","code":{"code":"clippy::self_assignment","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/self_assignment.rs","byte_start":350,"byte_end":375,"line_start":20,"line_end":20,"column_start":5,"column_end":30,"is_primary":true,"text":[{"text":"    s.b[a + 10] = s.b[10 + a];","highlight_start":5,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: self-assignment of `s.b[10 + a]` to `s.b[a + 10]`\n  --> tests/ui/self_assignment.rs:20:5\n   |\nLL |     s.b[a + 10] = s.b[10 + a];\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"self-assignment of `t.1` to `t.1`","code":{"code":"clippy::self_assignment","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/self_assignment.rs","byte_start":406,"byte_end":415,"line_start":23,"line_end":23,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    t.1 = t.1;","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: self-assignment of `t.1` to `t.1`\n  --> tests/ui/self_assignment.rs:23:5\n   |\nLL |     t.1 = t.1;\n   |     ^^^^^^^^^\n\n"}
{"message":"self-assignment of `(t.0)` to `t.0`","code":{"code":"clippy::self_assignment","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/self_assignment.rs","byte_start":421,"byte_end":432,"line_start":24,"line_end":24,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    t.0 = (t.0);","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: self-assignment of `(t.0)` to `t.0`\n  --> tests/ui/self_assignment.rs:24:5\n   |\nLL |     t.0 = (t.0);\n   |     ^^^^^^^^^^^\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.8.0/src/lib.rs:111:22
