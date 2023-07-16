plain
---- [ui] src/test/ui/type-alias-enum-variants/self-in-enum-definition.rs stdout ----
diff of stderr:

14    |
15 LL |     V3 = Self::V1 {} as u8 + 2,
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
16    |          ^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires caching mir of `Alpha::V3::{constant#0}` for CTFE...
+    |
+    |
+ LL |     V3 = Self::V1 {} as u8 + 2,
+    |          ^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires elaborating drops for `Alpha::V3::{constant#0}`...
+    |
+    |
+ LL |     V3 = Self::V1 {} as u8 + 2,
+    |          ^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires borrow-checking `Alpha::V3::{constant#0}`...
+    |
+    |
+ LL |     V3 = Self::V1 {} as u8 + 2,
+    |          ^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires processing MIR for `Alpha::V3::{constant#0}`...
+    |
+    |
+ LL |     V3 = Self::V1 {} as u8 + 2,
+    |          ^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires const checking `Alpha::V3::{constant#0}`...
+    |
+    |
+ LL |     V3 = Self::V1 {} as u8 + 2,
+    |          ^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires preparing `Alpha::V3::{constant#0}` for borrow checking...
+    |
+    |
+ LL |     V3 = Self::V1 {} as u8 + 2,
+    |          ^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires unsafety-checking `Alpha::V3::{constant#0}`...
+    |
+    |
+ LL |     V3 = Self::V1 {} as u8 + 2,
+    |          ^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires building MIR for `Alpha::V3::{constant#0}`...
+    |
+    |
+ LL |     V3 = Self::V1 {} as u8 + 2,
+    |          ^^^^^^^^^^^^^^^^^^^^^
17    = note: ...which requires computing layout of `Alpha`...
18    = note: ...which again requires simplifying constant for the type system `Alpha::V3::{constant#0}`, completing the cycle
19 note: cycle used when collecting item types in top-level module

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-enum-variants/self-in-enum-definition/self-in-enum-definition.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-enum-variants/self-in-enum-definition.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-enum-variants/self-in-enum-definition.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-enum-variants/self-in-enum-definition" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-enum-variants/self-in-enum-definition/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when simplifying constant for the type system `Alpha::V3::{constant#0}`
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |
   |
note: ...which requires simplifying constant for the type system `Alpha::V3::{constant#0}`...
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |          ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `Alpha::V3::{constant#0}`...
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |          ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires caching mir of `Alpha::V3::{constant#0}` for CTFE...
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |          ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires elaborating drops for `Alpha::V3::{constant#0}`...
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |          ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires borrow-checking `Alpha::V3::{constant#0}`...
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |          ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `Alpha::V3::{constant#0}`...
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |          ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const checking `Alpha::V3::{constant#0}`...
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |          ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires preparing `Alpha::V3::{constant#0}` for borrow checking...
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |          ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires unsafety-checking `Alpha::V3::{constant#0}`...
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |          ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `Alpha::V3::{constant#0}`...
   |
   |
LL |     V3 = Self::V1 {} as u8 + 2, //~ ERROR cycle detected when simplifying constant
   |          ^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires computing layout of `Alpha`...
   = note: ...which again requires simplifying constant for the type system `Alpha::V3::{constant#0}`, completing the cycle
note: cycle used when collecting item types in top-level module
   |
LL | / #[repr(u8)]
LL | / #[repr(u8)]
LL | | enum Alpha {
LL | |     V1 = 41,
LL | |     V2 = Self::V1 as u8 + 1,    // OK; See #50072.
LL | |
LL | | fn main() {}
   | |____________^

