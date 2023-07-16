\n\nThis error occurs when an expression was used in a place where the compiler\nexpected an expression of a different type. It can occur in several cases, the\nmost common being when calling a function and passing an argument which has a\ndifferent type than the matching type in the function declaration.\n"},"level":"error","spans":[{"file_name":"tests/ui/from_over_into.rs","byte_start":1461,"byte_end":1471,"line_start":85,"line_end":85,"column_start":15,"column_end":25,"is_primary":false,"text":[{"text":"type Opaque = impl Sized;","highlight_start":15,"highlight_end":25}],"label":"the expected opaque type","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/from_over_into.rs","byte_start":1461,"byte_end":1471,"line_start":85,"line_end":85,"column_start":15,"column_end":25,"is_primary":false,"text":[{"text":"type Opaque = impl Sized;","highlight_start":15,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `impl Trait`","def_site_span":{"file_name":"tests/ui/from_over_into.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":false,"text":[],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/ui/from_over_into.rs","byte_start":1548,"byte_end":1554,"line_start":88,"line_end":88,"column_start":22,"column_end":28,"is_primary":true,"text":[{"text":"    fn into(self) -> Opaque {}","highlight_start":22,"highlight_end":28}],"label":"expected opaque type, found `()`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/from_over_into.rs","byte_start":1534,"byte_end":1538,"line_start":88,"line_end":88,"column_start":8,"column_end":12,"is_primary":false,"text":[{"text":"    fn into(self) -> Opaque {}","highlight_start":8,"highlight_end":12}],"label":"implicitly returns `()` as its body has no tail or `return` expression","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected opaque type `Opaque`\n     found unit type `()`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this item cannot register hidden type without a `#[defines(Opaque)]` attribute","code":null,"level":"note","spans":[{"file_name":"tests/ui/from_over_into.rs","byte_start":1531,"byte_end":1554,"line_start":88,"line_end":88,"column_start":5,"column_end":28,"is_primary":true,"text":[{"text":"    fn into(self) -> Opaque {}","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> tests/ui/from_over_into.rs:88:22\n   |\nLL | type Opaque = impl Sized;\n   |               ---------- the expected opaque type\n...\nLL |     fn into(self) -> Opaque {}\n   |        ----          ^^^^^^ expected opaque type, found `()`\n   |        |\n   |        implicitly returns `()` as its body has no tail or `return` expression\n   |\n   = note: expected opaque type `Opaque`\n                found unit type `()`\nnote: this item cannot register hidden type without a `#[defines(Opaque)]` attribute\n  --> tests/ui/from_over_into.rs:88:5\n   |\nLL |     fn into(self) -> Opaque {}\n   |     ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"For more information about this error, try `rustc --explain E0308`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0308`.\n"}

------------------------------------------


diff of stderr:

-error: methods called `new` usually return `Self`
+error[E0308]: mismatched types
+  --> $DIR/new_ret_no_self.rs:422:13
    |
    |
-LL | /     pub fn new(_: String) -> impl R<Item = u32> {
-LL | |         S3
-LL | |     }
-   | |_____^
+LL |     type X = impl std::ops::Add<Output = X>;
+...
+LL |         pub fn new() -> X {
+LL |         pub fn new() -> X {
+   |                         - expected `issue10041::X` because of return type
+   |             ^^^^ expected opaque type, found `i32`
    |
    |
-   = note: `-D clippy::new-ret-no-self` implied by `-D warnings`
-
-error: methods called `new` usually return `Self`
-   |
-LL | /     pub fn new() -> u32 {
-LL | |         unimplemented!();
-LL | |     }
-LL | |     }
-   | |_____^
-
-error: methods called `new` usually return `Self`
-   |
-LL | /     pub fn new(_: String) -> u32 {
-LL | |         unimplemented!();
-LL | |     }
-LL | |     }
-   | |_____^
-
-error: methods called `new` usually return `Self`
-   |
-   |
-LL | /     pub fn new() -> (u32, u32) {
-LL | |         unimplemented!();
-LL | |     }
-
-
-error: methods called `new` usually return `Self`
-   |
-   |
-LL | /     pub fn new() -> *mut V {
-LL | |         unimplemented!();
-LL | |     }
-
-
-error: methods called `new` usually return `Self`
-   |
-LL | /     pub fn new() -> Option<u32> {
-LL | |         unimplemented!();
-LL | |     }
-LL | |     }
-   | |_____^
-
-error: methods called `new` usually return `Self`
-   |
-LL |         fn new() -> String;
-   |         ^^^^^^^^^^^^^^^^^^^
-
-
-error: methods called `new` usually return `Self`
-   |
-LL |         fn new(_: String) -> String;
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-
-
-error: methods called `new` usually return `Self`
-   |
-   |
-LL | /         fn new() -> (u32, u32) {
-LL | |             unimplemented!();
-LL | |         }
-
-
-error: methods called `new` usually return `Self`
-   |
-   |
-LL | /         fn new() -> *mut V {
-LL | |             unimplemented!();
-LL | |         }
-
-
-error: methods called `new` usually return `Self`
-   |
-   |
-LL | /         fn new(t: T) -> impl Into<i32> {
-LL | |             1
-LL | |         }
-
-
-error: methods called `new` usually return `Self`
-   |
-   |
-LL | /         fn new(t: T) -> impl Trait2<(), i32> {
-LL | |             unimplemented!()
-LL | |         }
-
-
-error: methods called `new` usually return `Self`
-   |
-   |
-LL | /         pub fn new() -> impl PartialOrd {
-LL | |             0i32
-LL | |         }
-
-
-error: methods called `new` usually return `Self`
+   = note: expected opaque type `issue10041::X`
+                     found type `i32`
+note: this item cannot register hidden type without a `#[defines(issue10041::X)]` attribute
    |
    |
-LL | /         pub fn new() -> X {
-LL | |             0i32
-LL | |         }
+LL |         pub fn new() -> X {
+   |         ^^^^^^^^^^^^^^^^^
 
-error: aborting due to 14 previous errors
---
To only update this specific test, also pass `--test-args new_ret_no_self.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/new_ret_no_self.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/new_ret_no_self.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-23c15a454288152e.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-fc796eef9a515a44.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-d8aee1681c496322.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-e5b19e58ae33983e.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-60efa73d6fad7e91.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-40ff94c070351d6c.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-e22c3fc74241d5e4.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/new_ret_no_self.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"mismatched types","code":{"code":"E0308","explanation":"Expected type did not match the received type.\n\nErroneous code examples:\n\n