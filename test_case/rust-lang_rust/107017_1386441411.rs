plain
---- [ui] checkout/tests/ui/proc-macro/allowed-signatures.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/proc-macro/allowed-signatures.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/allowed-signatures" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/allowed-signatures/auxiliary"
stdout: none
--- stderr -------------------------------
warning: dropping unsupported crate type `proc-macro` for target `wasm32-unknown-unknown`

error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type
   |
LL | #[proc_macro]
   | ^^^^^^^^^^^^^


error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type
   |
LL | #[proc_macro]
   | ^^^^^^^^^^^^^

---
diff of stderr:

+ warning: dropping unsupported crate type `proc-macro` for target `wasm32-unknown-unknown`
+ 
+ error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type
+    |
+    |
+ LL | #[proc_macro]
+ 
+ 
+ error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=wasm32-unknown-unknown
+    |
+    |
+ LL | #[proc_macro]
+ 
+ 
+ error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type
+    |
+    |
+ LL | #[proc_macro]
+ 
+ 
+ error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type
+    |
+    |
+ LL | #[proc_macro]
+ 
+ 
1 error: proc macro functions may not be `extern "C"`
3    |


16 LL | pub extern fn abi3(a: TokenStream) -> TokenStream {
18 
- error: aborting due to 3 previous errors
+ error: aborting due to 7 previous errors; 1 warning emitted
20 
---
To only update this specific test, also pass `--test-args proc-macro/proc-macro-abi.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/proc-macro/proc-macro-abi.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/proc-macro-abi" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/proc-macro-abi/auxiliary"
stdout: none
--- stderr -------------------------------
warning: dropping unsupported crate type `proc-macro` for target `wasm32-unknown-unknown`

error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type
   |
LL | #[proc_macro]
   | ^^^^^^^^^^^^^


error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type
   |
LL | #[proc_macro]
   | ^^^^^^^^^^^^^


error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type
   |
LL | #[proc_macro]
   | ^^^^^^^^^^^^^


error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type
   |
LL | #[proc_macro]
   | ^^^^^^^^^^^^^


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

error: aborting due to 7 previous errors; 1 warning emitted
------------------------------------------



---- [ui] checkout/tests/ui/proc-macro/signature-proc-macro-attribute.rs stdout ----
diff of stderr:

+ warning: dropping unsupported crate type `proc-macro` for target `wasm32-unknown-unknown`
+ 
+ error: the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type
+    |
+ LL | #[proc_macro_attribute]
+    | ^^^^^^^^^^^^^^^^^^^^^^^
+ 
+ 
+ error: the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type
+    |
+ LL | #[proc_macro_attribute]
+    | ^^^^^^^^^^^^^^^^^^^^^^^
+ 
+ 
+ error: the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type
+    |
+ LL | #[proc_macro_attribute]
+    | ^^^^^^^^^^^^^^^^^^^^^^^
+ 
+ 
+ error: the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type
+    |
+ LL | #[proc_macro_attribute]
+    | ^^^^^^^^^^^^^^^^^^^^^^^
+ 
+ 
1 error: mismatched attribute proc macro signature
2   --> $DIR/signature-proc-macro-attribute.rs:7:1
3    |

38 LL | pub fn too_many(a: TokenStream, b: TokenStream, c: String) -> TokenStream {
39    |                                                 ^^^^^^^^^ found unexpected argument
- error: aborting due to 6 previous errors
+ error: aborting due to 10 previous errors; 1 warning emitted
42 
43 
---
To only update this specific test, also pass `--test-args proc-macro/signature-proc-macro-attribute.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/proc-macro/signature-proc-macro-attribute.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/signature-proc-macro-attribute" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/signature-proc-macro-attribute/auxiliary"
stdout: none
--- stderr -------------------------------
warning: dropping unsupported crate type `proc-macro` for target `wasm32-unknown-unknown`

error: the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type
   |
LL | #[proc_macro_attribute]
   | ^^^^^^^^^^^^^^^^^^^^^^^


error: the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type
   |
LL | #[proc_macro_attribute]
   | ^^^^^^^^^^^^^^^^^^^^^^^


error: the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type
   |
LL | #[proc_macro_attribute]
   | ^^^^^^^^^^^^^^^^^^^^^^^


error: the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type
   |
LL | #[proc_macro_attribute]
   | ^^^^^^^^^^^^^^^^^^^^^^^


error: mismatched attribute proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro-attribute.rs:7:1
   |
LL | pub fn bad_input(input: String) -> TokenStream {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ attribute proc macro must have two arguments of type `proc_macro::TokenStream`
error: mismatched attribute proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro-attribute.rs:13:42
   |
   |
LL | pub fn bad_output(input: TokenStream) -> String {
   |                                          ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`
   |
   = note: attribute proc macros must have a signature of `fn(TokenStream, TokenStream) -> TokenStream`
error: mismatched attribute proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro-attribute.rs:13:1
   |
   |
LL | pub fn bad_output(input: TokenStream) -> String {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ attribute proc macro must have two arguments of type `proc_macro::TokenStream`
error: mismatched attribute proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro-attribute.rs:20:41
   |
   |
LL | pub fn bad_everything(input: String) -> String {
   |                                         ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`
   |
   = note: attribute proc macros must have a signature of `fn(TokenStream, TokenStream) -> TokenStream`
error: mismatched attribute proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro-attribute.rs:20:1
   |
   |
LL | pub fn bad_everything(input: String) -> String {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ attribute proc macro must have two arguments of type `proc_macro::TokenStream`
error: mismatched attribute proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro-attribute.rs:27:49
   |
   |
LL | pub fn too_many(a: TokenStream, b: TokenStream, c: String) -> TokenStream {
   |                                                 ^^^^^^^^^ found unexpected argument
error: aborting due to 10 previous errors; 1 warning emitted
------------------------------------------



---- [ui] checkout/tests/ui/proc-macro/signature-proc-macro-derive.rs stdout ----
diff of stderr:

+ warning: dropping unsupported crate type `proc-macro` for target `wasm32-unknown-unknown`
+ 
+ error: the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type
+    |
+    |
+ LL | #[proc_macro_derive(Blah)]
+ 
+ 
+ error: the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type
+    |
+    |
+ LL | #[proc_macro_derive(Bleh)]
+ 
+ 
+ error: the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type
+    |
+    |
+ LL | #[proc_macro_derive(Bluh)]
+ 
+ 
+ error: the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type
+    |
+    |
+ LL | #[proc_macro_derive(Blih)]
+ 
1 error: mismatched derive proc macro signature
2   --> $DIR/signature-proc-macro-derive.rs:7:25
3    |
3    |

36 LL | pub fn too_many(a: TokenStream, b: TokenStream, c: String) -> TokenStream {
37    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^ found unexpected arguments
- error: aborting due to 5 previous errors
+ error: aborting due to 9 previous errors; 1 warning emitted
40 
41 
---
To only update this specific test, also pass `--test-args proc-macro/signature-proc-macro-derive.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/proc-macro/signature-proc-macro-derive.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/signature-proc-macro-derive" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/signature-proc-macro-derive/auxiliary"
stdout: none
--- stderr -------------------------------
warning: dropping unsupported crate type `proc-macro` for target `wasm32-unknown-unknown`

error: the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type
   |
   |
LL | #[proc_macro_derive(Blah)]


error: the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type
   |
   |
LL | #[proc_macro_derive(Bleh)]


error: the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type
   |
   |
LL | #[proc_macro_derive(Bluh)]


error: the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type
   |
   |
LL | #[proc_macro_derive(Blih)]

error: mismatched derive proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro-derive.rs:7:25
   |
   |
LL | pub fn bad_input(input: String) -> TokenStream {
   |                         ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`
   |
   = note: derive proc macros must have a signature of `fn(TokenStream) -> TokenStream`
error: mismatched derive proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro-derive.rs:13:42
   |
   |
LL | pub fn bad_output(input: TokenStream) -> String {
   |                                          ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`
   |
   = note: derive proc macros must have a signature of `fn(TokenStream) -> TokenStream`
error: mismatched derive proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro-derive.rs:19:41
   |
   |
LL | pub fn bad_everything(input: String) -> String {
   |                                         ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`
   |
   = note: derive proc macros must have a signature of `fn(TokenStream) -> TokenStream`
error: mismatched derive proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro-derive.rs:19:30
   |
   |
LL | pub fn bad_everything(input: String) -> String {
   |                              ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`
   |
   = note: derive proc macros must have a signature of `fn(TokenStream) -> TokenStream`
error: mismatched derive proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro-derive.rs:26:33
   |
   |
LL | pub fn too_many(a: TokenStream, b: TokenStream, c: String) -> TokenStream {
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^ found unexpected arguments
error: aborting due to 9 previous errors; 1 warning emitted
------------------------------------------



---- [ui] checkout/tests/ui/proc-macro/signature-proc-macro.rs stdout ----
diff of stderr:

+ warning: dropping unsupported crate type `proc-macro` for target `wasm32-unknown-unknown`
+ 
+ error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type
+    |
+    |
+ LL | #[proc_macro]
+ 
+ 
+ error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type
+    |
+    |
+ LL | #[proc_macro]
+ 
+ 
+ error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type
+    |
+    |
+ LL | #[proc_macro]
+ 
+ 
+ error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type
+    |
+    |
+ LL | #[proc_macro]
+ 
1 error: mismatched function-like proc macro signature
2   --> $DIR/signature-proc-macro.rs:7:25
3    |
3    |

36 LL | pub fn too_many(a: TokenStream, b: TokenStream, c: String) -> TokenStream {
37    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^ found unexpected arguments
- error: aborting due to 5 previous errors
+ error: aborting due to 9 previous errors; 1 warning emitted
40 
41 
---
To only update this specific test, also pass `--test-args proc-macro/signature-proc-macro.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/proc-macro/signature-proc-macro.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/signature-proc-macro" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/signature-proc-macro/auxiliary"
stdout: none
--- stderr -------------------------------
warning: dropping unsupported crate type `proc-macro` for target `wasm32-unknown-unknown`

error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type
   |
LL | #[proc_macro]
   | ^^^^^^^^^^^^^


error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type
   |
LL | #[proc_macro]
   | ^^^^^^^^^^^^^


error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type
   |
LL | #[proc_macro]
   | ^^^^^^^^^^^^^


error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type
   |
LL | #[proc_macro]
   | ^^^^^^^^^^^^^


error: mismatched function-like proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro.rs:7:25
   |
LL | pub fn bad_input(input: String) -> TokenStream {
   |                         ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`
   |
   = note: function-like proc macros must have a signature of `fn(TokenStream) -> TokenStream`
error: mismatched function-like proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro.rs:13:42
   |
   |
LL | pub fn bad_output(input: TokenStream) -> String {
   |                                          ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`
   |
   = note: function-like proc macros must have a signature of `fn(TokenStream) -> TokenStream`
error: mismatched function-like proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro.rs:19:41
   |
   |
LL | pub fn bad_everything(input: String) -> String {
   |                                         ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`
   |
   = note: function-like proc macros must have a signature of `fn(TokenStream) -> TokenStream`
error: mismatched function-like proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro.rs:19:30
   |
   |
LL | pub fn bad_everything(input: String) -> String {
   |                              ^^^^^^ found std::string::String, expected type `proc_macro::TokenStream`
   |
   = note: function-like proc macros must have a signature of `fn(TokenStream) -> TokenStream`
error: mismatched function-like proc macro signature
  --> /checkout/tests/ui/proc-macro/signature-proc-macro.rs:26:33
   |
   |
LL | pub fn too_many(a: TokenStream, b: TokenStream, c: String) -> TokenStream {
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^ found unexpected arguments
error: aborting due to 9 previous errors; 1 warning emitted
------------------------------------------


