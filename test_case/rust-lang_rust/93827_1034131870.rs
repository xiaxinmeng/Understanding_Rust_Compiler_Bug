plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 9747ee4755e6e3c4b4b4c7a0587df29123835cda and dc56722c1814d79d60d1d1d274a33d8f32c82b13
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

 error: this could be a `const fn`
   --> $DIR/could_be_const.rs:13:5
    |
 LL | /     pub fn new() -> Self {
 LL | |         Self { guess: 42 }
 LL | |     }
    |
    |
    = note: `-D clippy::missing-const-for-fn` implied by `-D warnings`
 error: this could be a `const fn`
   --> $DIR/could_be_const.rs:17:5
    |
    |
 LL | /     fn const_generic_params<'a, T, const N: usize>(&self, b: &'a [T; N]) -> &'a [T; N] {
 LL | |         b
 LL | |     }
 
 error: this could be a `const fn`
   --> $DIR/could_be_const.rs:23:1
    |
    |
 LL | / fn one() -> i32 {
 LL | |     1
 LL | | }
 
 error: this could be a `const fn`
   --> $DIR/could_be_const.rs:28:1
    |
    |
 LL | / fn two() -> i32 {
 LL | |     let abc = 2;
 LL | |     abc
 LL | | }
 
 error: this could be a `const fn`
   --> $DIR/could_be_const.rs:34:1
    |
    |
 LL | / fn string() -> String {
 LL | |     String::new()
 LL | | }
    | |_^
 
 error: this could be a `const fn`
 error: this could be a `const fn`
   --> $DIR/could_be_const.rs:39:1
    |
 LL | / unsafe fn four() -> i32 {
 LL | |     4
 LL | | }
 
 error: this could be a `const fn`
   --> $DIR/could_be_const.rs:44:1
    |
    |
 LL | / fn generic<T>(t: T) -> T {
 LL | |     t
 LL | | }
 
 error: this could be a `const fn`
+  --> $DIR/could_be_const.rs:54:1
+   |
+   |
+LL | / fn generic_arr<T: Copy>(t: [T; 1]) -> T {
+LL | |     t[0]
+LL | | }
+
+error: this could be a `const fn`
   --> $DIR/could_be_const.rs:67:9
    |
    |
 LL | /         pub fn b(self, a: &A) -> B {
 LL | |             B
 LL | |         }
 
 error: this could be a `const fn`
   --> $DIR/could_be_const.rs:77:5
    |
    |
 LL | /     fn const_fn_stabilized_before_msrv(byte: u8) {
 LL | |         byte.is_ascii_digit();
 LL | |     }
 
-error: aborting due to 9 previous errors
+error: aborting due to 10 previous errors
 
---
To only update this specific test, also pass `--test-args missing_const_for_fn/could_be_const.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/missing_const_for_fn/could_be_const.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/missing_const_for_fn/could_be_const.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-e6439823b6d16f2c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-6688bfb3a0699fc9.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-b682a5a8a9c64b20.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-84536a848ae0c873.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/missing_const_for_fn/could_be_const.stage-id.aux"
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
{"message":"this could be a `const fn`","code":{"code":"clippy::missing_const_for_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/could_be_const.rs","byte_start":888,"byte_end":940,"line_start":54,"line_end":56,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn generic_arr<T: Copy>(t: [T; 1]) -> T {","highlight_start":1,"highlight_end":42},{"text":"    t[0]","highlight_start":1,"highlight_end":9},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this could be a `const fn`\n  --> tests/ui/missing_const_for_fn/could_be_const.rs:54:1\n   |\nLL | / fn generic_arr<T: Copy>(t: [T; 1]) -> T {\nLL | |     t[0]\nLL | | }\n   | |_^\n\n"}
{"message":"this could be a `const fn`","code":{"code":"clippy::missing_const_for_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/could_be_const.rs","byte_start":1139,"byte_end":1191,"line_start":67,"line_end":69,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        pub fn b(self, a: &A) -> B {","highlight_start":9,"highlight_end":37},{"text":"            B","highlight_start":1,"highlight_end":14},{"text":"        }","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this could be a `const fn`\n  --> tests/ui/missing_const_for_fn/could_be_const.rs:67:9\n   |\nLL | /         pub fn b(self, a: &A) -> B {\nLL | |             B\nLL | |         }\n   | |_________^\n\n"}
{"message":"this could be a `const fn`","code":{"code":"clippy::missing_const_for_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/could_be_const.rs","byte_start":1368,"byte_end":1451,"line_start":77,"line_end":79,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    fn const_fn_stabilized_before_msrv(byte: u8) {","highlight_start":5,"highlight_end":51},{"text":"        byte.is_ascii_digit();","highlight_start":1,"highlight_end":31},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this could be a `const fn`\n  --> tests/ui/missing_const_for_fn/could_be_const.rs:77:5\n   |\nLL | /     fn const_fn_stabilized_before_msrv(byte: u8) {\nLL | |         byte.is_ascii_digit();\nLL | |     }\n   | |_____^\n\n"}

------------------------------------------

normalized stderr:
normalized stderr:
error: this could be a `const fn`
  --> $DIR/cant_be_const.rs:59:5
   |
LL | /     fn g() -> u32 {
LL | |         33
LL | |     }
   |
   |
   = note: `-D clippy::missing-const-for-fn` implied by `-D warnings`
error: aborting due to previous error



---
To only update this specific test, also pass `--test-args missing_const_for_fn/cant_be_const.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/missing_const_for_fn/cant_be_const.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/missing_const_for_fn/cant_be_const.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-e6439823b6d16f2c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-6688bfb3a0699fc9.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-b682a5a8a9c64b20.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-84536a848ae0c873.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/missing_const_for_fn/cant_be_const.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this could be a `const fn`","code":{"code":"clippy::missing_const_for_fn","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/cant_be_const.rs","byte_start":1390,"byte_end":1422,"line_start":59,"line_end":61,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    fn g() -> u32 {","highlight_start":5,"highlight_end":20},{"text":"        33","highlight_start":1,"highlight_end":11},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::missing-const-for-fn` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this could be a `const fn`\n  --> tests/ui/missing_const_for_fn/cant_be_const.rs:59:5\n   |\nLL | /     fn g() -> u32 {\nLL | |         33\nLL | |     }\n   | |_____^\n   |\n   = note: `-D clippy::missing-const-for-fn` implied by `-D warnings`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/lib.rs:105:22
