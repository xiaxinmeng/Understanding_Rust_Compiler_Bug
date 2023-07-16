plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 4a6547cca6d2b2f465f01331927855734687b527 and 30e044d2a4a55f87ba299f83bb50a64d79c131a9
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
 error: this could be a `const fn`
-  --> $DIR/could_be_const.rs:14:5
+  --> $DIR/could_be_const.rs:13:5
    |
 LL | /     pub fn new() -> Self {
 LL | |         Self { guess: 42 }
 LL | |     }
    |
    |
    = note: `-D clippy::missing-const-for-fn` implied by `-D warnings`
 error: this could be a `const fn`
-  --> $DIR/could_be_const.rs:18:5
+  --> $DIR/could_be_const.rs:17:5
    |
    |
 LL | /     fn const_generic_params<'a, T, const N: usize>(&self, b: &'a [T; N]) -> &'a [T; N] {
 LL | |         b
 LL | |     }
 
 error: this could be a `const fn`
-  --> $DIR/could_be_const.rs:24:1
+  --> $DIR/could_be_const.rs:23:1
+  --> $DIR/could_be_const.rs:23:1
    |
 LL | / fn one() -> i32 {
 LL | |     1
 LL | | }
 
 error: this could be a `const fn`
error: test failed, to rerun pass '--test compile-test'
-  --> $DIR/could_be_const.rs:29:1
-  --> $DIR/could_be_const.rs:29:1
+  --> $DIR/could_be_const.rs:28:1
    |
 LL | / fn two() -> i32 {
 LL | |     let abc = 2;
 LL | |     abc
 LL | | }
 
 error: this could be a `const fn`
-  --> $DIR/could_be_const.rs:35:1
+  --> $DIR/could_be_const.rs:34:1
+  --> $DIR/could_be_const.rs:34:1
    |
 LL | / fn string() -> String {
 LL | |     String::new()
 LL | | }
 
 error: this could be a `const fn`
-  --> $DIR/could_be_const.rs:40:1
+  --> $DIR/could_be_const.rs:39:1
+  --> $DIR/could_be_const.rs:39:1
    |
 LL | / unsafe fn four() -> i32 {
 LL | |     4
 LL | | }
 
 error: this could be a `const fn`
-  --> $DIR/could_be_const.rs:45:1
+  --> $DIR/could_be_const.rs:44:1
+  --> $DIR/could_be_const.rs:44:1
    |
 LL | / fn generic<T>(t: T) -> T {
 LL | |     t
 LL | | }
 
 error: this could be a `const fn`
-  --> $DIR/could_be_const.rs:68:9
+  --> $DIR/could_be_const.rs:67:9
+  --> $DIR/could_be_const.rs:67:9
    |
 LL | /         pub fn b(self, a: &A) -> B {
 LL | |             B
 LL | |         }
 
 error: this could be a `const fn`
-  --> $DIR/could_be_const.rs:78:5
+  --> $DIR/could_be_const.rs:77:5
+  --> $DIR/could_be_const.rs:77:5
    |
 LL | /     fn const_fn_stabilized_before_msrv(byte: u8) {
 LL | |         byte.is_ascii_digit();
 LL | |     }
 
 error: aborting due to 9 previous errors
 
 
---
To only update this specific test, also pass `--test-args missing_const_for_fn/could_be_const.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/missing_const_for_fn/could_be_const.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/missing_const_for_fn/could_be_const.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-ff0575c5a2ca7cc8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7064d3f8532fb2a5.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-06539012b522c0cf.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8e5c09d8dc991112.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-90129c5dba212efa.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-17d0421109b348d4.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-54a7fffacad7d378.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/missing_const_for_fn/could_be_const.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this could be a `const fn`","code":{"code":"clippy::missing_const_for_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/could_be_const.rs","byte_start":229,"byte_end":284,"line_start":13,"line_end":15,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn new() -> Self {","highlight_start":5,"highlight_end":27},{"text":"        Self { guess: 42 }","highlight_start":1,"highlight_end":27},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::missing-const-for-fn` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this could be a `const fn`\n  --> tests/ui/missing_const_for_fn/could_be_const.rs:13:5\n   |\nLL | /     pub fn new() -> Self {\nLL | |         Self { guess: 42 }\nLL | |     }\n   | |_____^\n   |\n   = note: `-D clippy::missing-const-for-fn` implied by `-D warnings`\n\n"}
{"message":"this could be a `const fn`","code":{"code":"clippy::missing_const_for_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/could_be_const.rs","byte_start":290,"byte_end":390,"line_start":17,"line_end":19,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    fn const_generic_params<'a, T, const N: usize>(&self, b: &'a [T; N]) -> &'a [T; N] {","highlight_start":5,"highlight_end":89},{"text":"        b","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this could be a `const fn`\n  --> tests/ui/missing_const_for_fn/could_be_const.rs:17:5\n   |\nLL | /     fn const_generic_params<'a, T, const N: usize>(&self, b: &'a [T; N]) -> &'a [T; N] {\nLL | |         b\nLL | |     }\n   | |_____^\n\n"}
{"message":"this could be a `const fn`","code":{"code":"clippy::missing_const_for_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/could_be_const.rs","byte_start":412,"byte_end":437,"line_start":23,"line_end":25,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn one() -> i32 {","highlight_start":1,"highlight_end":18},{"text":"    1","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this could be a `const fn`\n  --> tests/ui/missing_const_for_fn/could_be_const.rs:23:1\n   |\nLL | / fn one() -> i32 {\nLL | |     1\nLL | | }\n   | |_^\n\n"}
{"message":"this could be a `const fn`","code":{"code":"clippy::missing_const_for_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/could_be_const.rs","byte_start":462,"byte_end":506,"line_start":28,"line_end":31,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn two() -> i32 {","highlight_start":1,"highlight_end":18},{"text":"    let abc = 2;","highlight_start":1,"highlight_end":17},{"text":"    abc","highlight_start":1,"highlight_end":8},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this could be a `const fn`\n  --> tests/ui/missing_const_for_fn/could_be_const.rs:28:1\n   |\nLL | / fn two() -> i32 {\nLL | |     let abc = 2;\nLL | |     abc\nLL | | }\n   | |_^\n\n"}
{"message":"this could be a `const fn`","code":{"code":"clippy::missing_const_for_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/could_be_const.rs","byte_start":544,"byte_end":587,"line_start":34,"line_end":36,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn string() -> String {","highlight_start":1,"highlight_end":24},{"text":"    String::new()","highlight_start":1,"highlight_end":18},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this could be a `const fn`\n  --> tests/ui/missing_const_for_fn/could_be_const.rs:34:1\n   |\nLL | / fn string() -> String {\nLL | |     String::new()\nLL | | }\n   | |_^\n\n"}
{"message":"this could be a `const fn`","code":{"code":"clippy::missing_const_for_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/could_be_const.rs","byte_start":607,"byte_end":640,"line_start":39,"line_end":41,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"unsafe fn four() -> i32 {","highlight_start":1,"highlight_end":26},{"text":"    4","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this could be a `const fn`\n  --> tests/ui/missing_const_for_fn/could_be_const.rs:39:1\n   |\nLL | / unsafe fn four() -> i32 {\nLL | |     4\nLL | | }\n   | |_^\n\n"}
{"message":"this could be a `const fn`","code":{"code":"clippy::missing_const_for_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/could_be_const.rs","byte_start":665,"byte_end":699,"line_start":44,"line_end":46,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn generic<T>(t: T) -> T {","highlight_start":1,"highlight_end":27},{"text":"    t","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this could be a `const fn`\n  --> tests/ui/missing_const_for_fn/could_be_const.rs:44:1\n   |\nLL | / fn generic<T>(t: T) -> T {\nLL | |     t\nLL | | }\n   | |_^\n\n"}
{"message":"this could be a `const fn`","code":{"code":"clippy::missing_const_for_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/could_be_const.rs","byte_start":1139,"byte_end":1191,"line_start":67,"line_end":69,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        pub fn b(self, a: &A) -> B {","highlight_start":9,"highlight_end":37},{"text":"            B","highlight_start":1,"highlight_end":14},{"text":"        }","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this could be a `const fn`\n  --> tests/ui/missing_const_for_fn/could_be_const.rs:67:9\n   |\nLL | /         pub fn b(self, a: &A) -> B {\nLL | |             B\nLL | |         }\n   | |_________^\n\n"}
{"message":"this could be a `const fn`","code":{"code":"clippy::missing_const_for_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/could_be_const.rs","byte_start":1368,"byte_end":1451,"line_start":77,"line_end":79,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    fn const_fn_stabilized_before_msrv(byte: u8) {","highlight_start":5,"highlight_end":51},{"text":"        byte.is_ascii_digit();","highlight_start":1,"highlight_end":31},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this could be a `const fn`\n  --> tests/ui/missing_const_for_fn/could_be_const.rs:77:5\n   |\nLL | /     fn const_fn_stabilized_before_msrv(byte: u8) {\nLL | |         byte.is_ascii_digit();\nLL | |     }\n   | |_____^\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
