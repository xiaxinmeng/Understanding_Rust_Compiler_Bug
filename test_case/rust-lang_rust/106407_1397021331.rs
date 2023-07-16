plain
........................................................................................ 10120/14212
........................................................................................ 10208/14212
........................................................................................ 10296/14212
........................................................................................ 10384/14212
...................................ii...............i.....iii.F...........F.F.F......... 10472/14212
........................................................................................ 10648/14212
........................................................................................ 10736/14212
........................................................................................ 10824/14212
........................................................................................ 10912/14212
---

---- [ui] checkout/tests/ui/proc-macro/proc-macro-abi.rs stdout ----
diff of stderr:

1 error: proc macro functions may not be `extern "C"`
-   --> $DIR/proc-macro-abi.rs:8:1
3    |
3    |
4 LL | pub extern "C" fn abi(a: TokenStream) -> TokenStream {

6 
6 
7 error: proc macro functions may not be `extern "system"`
-   --> $DIR/proc-macro-abi.rs:14:1
9    |
9    |
10 LL | pub extern "system" fn abi2(a: TokenStream) -> TokenStream {

12 
12 
13 error: proc macro functions may not be `extern "C"`
-   --> $DIR/proc-macro-abi.rs:20:1
15    |
15    |
16 LL | pub extern fn abi3(a: TokenStream) -> TokenStream {


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/proc-macro-abi/proc-macro-abi.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/proc-macro-abi/proc-macro-abi.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/proc-macro-abi.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/proc-macro/proc-macro-abi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/proc-macro-abi" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/proc-macro-abi/auxiliary"
stdout: none
--- stderr -------------------------------
error: proc macro functions may not be `extern "C"`
   |
   |
LL | pub extern "C" fn abi(a: TokenStream) -> TokenStream {


error: proc macro functions may not be `extern "system"`
   |
   |
LL | pub extern "system" fn abi2(a: TokenStream) -> TokenStream {


error: proc macro functions may not be `extern "C"`
   |
   |
LL | pub extern fn abi3(a: TokenStream) -> TokenStream {

error: aborting due to 3 previous errors
------------------------------------------

---
1 error: mismatched attribute proc macro signature
-   --> $DIR/signature-proc-macro-attribute.rs:7:1
+   --> $DIR/signature-proc-macro-attribute.rs:10:1
3    |
4 LL | pub fn bad_input(input: String) -> TokenStream {
5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ attribute proc macro must have two arguments of type `proc_macro::TokenStream`
6 
7 error: mismatched attribute proc macro signature
-   --> $DIR/signature-proc-macro-attribute.rs:13:42
+   --> $DIR/signature-proc-macro-attribute.rs:16:42
+   --> $DIR/signature-proc-macro-attribute.rs:16:42
9    |
10 LL | pub fn bad_output(input: TokenStream) -> String {
11    |                                          ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`

13    = note: attribute proc macros must have a signature of `fn(TokenStream, TokenStream) -> TokenStream`
15 error: mismatched attribute proc macro signature
-   --> $DIR/signature-proc-macro-attribute.rs:13:1
+   --> $DIR/signature-proc-macro-attribute.rs:16:1
17    |
17    |
18 LL | pub fn bad_output(input: TokenStream) -> String {
19    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ attribute proc macro must have two arguments of type `proc_macro::TokenStream`
20 
21 error: mismatched attribute proc macro signature
-   --> $DIR/signature-proc-macro-attribute.rs:20:41
+   --> $DIR/signature-proc-macro-attribute.rs:23:41
+   --> $DIR/signature-proc-macro-attribute.rs:23:41
23    |
24 LL | pub fn bad_everything(input: String) -> String {
25    |                                         ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`

27    = note: attribute proc macros must have a signature of `fn(TokenStream, TokenStream) -> TokenStream`
29 error: mismatched attribute proc macro signature
-   --> $DIR/signature-proc-macro-attribute.rs:20:1
+   --> $DIR/signature-proc-macro-attribute.rs:23:1
31    |
31    |
32 LL | pub fn bad_everything(input: String) -> String {
33    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ attribute proc macro must have two arguments of type `proc_macro::TokenStream`
34 
35 error: mismatched attribute proc macro signature
-   --> $DIR/signature-proc-macro-attribute.rs:27:49
+   --> $DIR/signature-proc-macro-attribute.rs:30:49
+   --> $DIR/signature-proc-macro-attribute.rs:30:49
37    |
38 LL | pub fn too_many(a: TokenStream, b: TokenStream, c: String) -> TokenStream {
39    |                                                 ^^^^^^^^^ found unexpected argument

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/signature-proc-macro-attribute/signature-proc-macro-attribute.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/signature-proc-macro-attribute.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/proc-macro/signature-proc-macro-attribute.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/signature-proc-macro-attribute" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/signature-proc-macro-attribute/auxiliary"
stdout: none
--- stderr -------------------------------
error: mismatched attribute proc macro signature
   |
   |
LL | pub fn bad_input(input: String) -> TokenStream {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ attribute proc macro must have two arguments of type `proc_macro::TokenStream`
error: mismatched attribute proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro-attribute.rs:16:42
   |
   |
LL | pub fn bad_output(input: TokenStream) -> String {
   |                                          ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`
   |
   = note: attribute proc macros must have a signature of `fn(TokenStream, TokenStream) -> TokenStream`
error: mismatched attribute proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro-attribute.rs:16:1
   |
   |
LL | pub fn bad_output(input: TokenStream) -> String {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ attribute proc macro must have two arguments of type `proc_macro::TokenStream`
error: mismatched attribute proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro-attribute.rs:23:41
   |
   |
LL | pub fn bad_everything(input: String) -> String {
   |                                         ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`
   |
   = note: attribute proc macros must have a signature of `fn(TokenStream, TokenStream) -> TokenStream`
error: mismatched attribute proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro-attribute.rs:23:1
   |
   |
LL | pub fn bad_everything(input: String) -> String {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ attribute proc macro must have two arguments of type `proc_macro::TokenStream`
error: mismatched attribute proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro-attribute.rs:30:49
   |
   |
LL | pub fn too_many(a: TokenStream, b: TokenStream, c: String) -> TokenStream {
   |                                                 ^^^^^^^^^ found unexpected argument
error: aborting due to 6 previous errors
------------------------------------------



---- [ui] checkout/tests/ui/proc-macro/signature-proc-macro-derive.rs stdout ----
diff of stderr:

1 error: mismatched derive proc macro signature
-   --> $DIR/signature-proc-macro-derive.rs:7:25
+   --> $DIR/signature-proc-macro-derive.rs:10:25
3    |
4 LL | pub fn bad_input(input: String) -> TokenStream {
5    |                         ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`

7    = note: derive proc macros must have a signature of `fn(TokenStream) -> TokenStream`
9 error: mismatched derive proc macro signature
-   --> $DIR/signature-proc-macro-derive.rs:13:42
+   --> $DIR/signature-proc-macro-derive.rs:16:42
11    |
11    |
12 LL | pub fn bad_output(input: TokenStream) -> String {
13    |                                          ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`

15    = note: derive proc macros must have a signature of `fn(TokenStream) -> TokenStream`
17 error: mismatched derive proc macro signature
-   --> $DIR/signature-proc-macro-derive.rs:19:41
+   --> $DIR/signature-proc-macro-derive.rs:22:41
19    |
19    |
20 LL | pub fn bad_everything(input: String) -> String {
21    |                                         ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`

23    = note: derive proc macros must have a signature of `fn(TokenStream) -> TokenStream`
25 error: mismatched derive proc macro signature
-   --> $DIR/signature-proc-macro-derive.rs:19:30
+   --> $DIR/signature-proc-macro-derive.rs:22:30
27    |
27    |
28 LL | pub fn bad_everything(input: String) -> String {
29    |                              ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`

31    = note: derive proc macros must have a signature of `fn(TokenStream) -> TokenStream`
33 error: mismatched derive proc macro signature
-   --> $DIR/signature-proc-macro-derive.rs:26:33
+   --> $DIR/signature-proc-macro-derive.rs:29:33
35    |
35    |
36 LL | pub fn too_many(a: TokenStream, b: TokenStream, c: String) -> TokenStream {
37    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^ found unexpected arguments

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/signature-proc-macro-derive/signature-proc-macro-derive.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/signature-proc-macro-derive.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/proc-macro/signature-proc-macro-derive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/signature-proc-macro-derive" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/signature-proc-macro-derive/auxiliary"
stdout: none
--- stderr -------------------------------
error: mismatched derive proc macro signature
   |
   |
LL | pub fn bad_input(input: String) -> TokenStream {
   |                         ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`
   |
   = note: derive proc macros must have a signature of `fn(TokenStream) -> TokenStream`
error: mismatched derive proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro-derive.rs:16:42
   |
   |
LL | pub fn bad_output(input: TokenStream) -> String {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |                                          ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`
   |
   = note: derive proc macros must have a signature of `fn(TokenStream) -> TokenStream`
error: mismatched derive proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro-derive.rs:22:41
   |
   |
LL | pub fn bad_everything(input: String) -> String {
   |                                         ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`
   |
   = note: derive proc macros must have a signature of `fn(TokenStream) -> TokenStream`
error: mismatched derive proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro-derive.rs:22:30
   |
   |
LL | pub fn bad_everything(input: String) -> String {
   |                              ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`
   |
   = note: derive proc macros must have a signature of `fn(TokenStream) -> TokenStream`
error: mismatched derive proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro-derive.rs:29:33
   |
   |
LL | pub fn too_many(a: TokenStream, b: TokenStream, c: String) -> TokenStream {
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^ found unexpected arguments
error: aborting due to 5 previous errors
------------------------------------------



---- [ui] checkout/tests/ui/proc-macro/signature-proc-macro.rs stdout ----
diff of stderr:

1 error: mismatched function-like proc macro signature
-   --> $DIR/signature-proc-macro.rs:7:25
+   --> $DIR/signature-proc-macro.rs:10:25
3    |
4 LL | pub fn bad_input(input: String) -> TokenStream {
5    |                         ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`

7    = note: function-like proc macros must have a signature of `fn(TokenStream) -> TokenStream`
9 error: mismatched function-like proc macro signature
-   --> $DIR/signature-proc-macro.rs:13:42
+   --> $DIR/signature-proc-macro.rs:16:42
11    |
11    |
12 LL | pub fn bad_output(input: TokenStream) -> String {
13    |                                          ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`

15    = note: function-like proc macros must have a signature of `fn(TokenStream) -> TokenStream`
17 error: mismatched function-like proc macro signature
-   --> $DIR/signature-proc-macro.rs:19:41
+   --> $DIR/signature-proc-macro.rs:22:41
19    |
19    |
20 LL | pub fn bad_everything(input: String) -> String {
21    |                                         ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`

23    = note: function-like proc macros must have a signature of `fn(TokenStream) -> TokenStream`
25 error: mismatched function-like proc macro signature
-   --> $DIR/signature-proc-macro.rs:19:30
+   --> $DIR/signature-proc-macro.rs:22:30
27    |
27    |
28 LL | pub fn bad_everything(input: String) -> String {
29    |                              ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`

31    = note: function-like proc macros must have a signature of `fn(TokenStream) -> TokenStream`
33 error: mismatched function-like proc macro signature
-   --> $DIR/signature-proc-macro.rs:26:33
+   --> $DIR/signature-proc-macro.rs:29:33
35    |
35    |
36 LL | pub fn too_many(a: TokenStream, b: TokenStream, c: String) -> TokenStream {
37    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^ found unexpected arguments

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/signature-proc-macro/signature-proc-macro.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/signature-proc-macro.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/proc-macro/signature-proc-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/signature-proc-macro" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/signature-proc-macro/auxiliary"
stdout: none
--- stderr -------------------------------
error: mismatched function-like proc macro signature
   |
   |
LL | pub fn bad_input(input: String) -> TokenStream {
   |                         ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`
   |
   = note: function-like proc macros must have a signature of `fn(TokenStream) -> TokenStream`
error: mismatched function-like proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro.rs:16:42
   |
   |
LL | pub fn bad_output(input: TokenStream) -> String {
   |                                          ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`
   |
   = note: function-like proc macros must have a signature of `fn(TokenStream) -> TokenStream`
error: mismatched function-like proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro.rs:22:41
   |
   |
LL | pub fn bad_everything(input: String) -> String {
   |                                         ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`
   |
   = note: function-like proc macros must have a signature of `fn(TokenStream) -> TokenStream`
error: mismatched function-like proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro.rs:22:30
   |
   |
LL | pub fn bad_everything(input: String) -> String {
   |                              ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`
   |
   = note: function-like proc macros must have a signature of `fn(TokenStream) -> TokenStream`
error: mismatched function-like proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro.rs:29:33
   |
   |
LL | pub fn too_many(a: TokenStream, b: TokenStream, c: String) -> TokenStream {
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^ found unexpected arguments
error: aborting due to 5 previous errors
------------------------------------------


