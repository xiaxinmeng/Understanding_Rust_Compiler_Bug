plain
diff of stderr:

46   --> $DIR/2018-edition-error-in-non-macro-position.rs:13:14
47    |
48 LL | struct Foo { await: () }
-    |        |
-    |        while parsing this struct
+    |              ^^^^^ expected identifier, found keyword
52    |
52    |
+    = help: see <https://doc.rust-lang.org/book/ch05-01-defining-structs.html> for more information
53 help: escape `await` to use it as an identifier
54    |
55 LL | struct Foo { r#await: () }
56    |              ++
56    |              ++
+ help: remove the let, the `let` keyword is not allowed in struct field definitions
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ LL - struct Foo { await: () }
+ LL + struct Foo { : () }
57 
57 
+ error: expected `:`, found `(`
+   --> $DIR/2018-edition-error-in-non-macro-position.rs:13:21
+    |
+ LL | struct Foo { await: () }
+    |        ---          ^ expected `:`
+    |        while parsing this struct
+ 
58 error: expected identifier, found keyword `await`
59   --> $DIR/2018-edition-error-in-non-macro-position.rs:16:15
59   --> $DIR/2018-edition-error-in-non-macro-position.rs:16:15
60    |

77 LL | macro_rules! r#await {
79 
- error: aborting due to 7 previous errors
+ error: aborting due to 8 previous errors
81 
---
To only update this specific test, also pass `--test-args async-await/await-keyword/2018-edition-error-in-non-macro-position.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/await-keyword/2018-edition-error-in-non-macro-position.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/await-keyword/2018-edition-error-in-non-macro-position" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/await-keyword/2018-edition-error-in-non-macro-position/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected identifier, found keyword `await`
   |
   |
LL |     pub mod await { //~ ERROR expected identifier, found keyword `await`
   |
   |
help: escape `await` to use it as an identifier
   |
LL |     pub mod r#await { //~ ERROR expected identifier, found keyword `await`

error: expected identifier, found keyword `await`
  --> /checkout/src/test/ui/async-await/await-keyword/2018-edition-error-in-non-macro-position.rs:7:20
   |
   |
LL |         pub struct await; //~ ERROR expected identifier, found keyword `await`
   |
   |
help: escape `await` to use it as an identifier
   |
LL |         pub struct r#await; //~ ERROR expected identifier, found keyword `await`

error: expected identifier, found keyword `await`
  --> /checkout/src/test/ui/async-await/await-keyword/2018-edition-error-in-non-macro-position.rs:10:22
   |
   |
LL | use self::outer_mod::await::await; //~ ERROR expected identifier, found keyword `await`
   |
   |
help: escape `await` to use it as an identifier
   |
LL | use self::outer_mod::r#await::await; //~ ERROR expected identifier, found keyword `await`

error: expected identifier, found keyword `await`
  --> /checkout/src/test/ui/async-await/await-keyword/2018-edition-error-in-non-macro-position.rs:10:29
   |
   |
LL | use self::outer_mod::await::await; //~ ERROR expected identifier, found keyword `await`
   |
   |
help: escape `await` to use it as an identifier
   |
LL | use self::outer_mod::await::r#await; //~ ERROR expected identifier, found keyword `await`

error: expected identifier, found keyword `await`
  --> /checkout/src/test/ui/async-await/await-keyword/2018-edition-error-in-non-macro-position.rs:13:14
   |
   |
LL | struct Foo { await: () }
   |
   |
   = help: see <https://doc.rust-lang.org/book/ch05-01-defining-structs.html> for more information
help: escape `await` to use it as an identifier
   |
LL | struct Foo { r#await: () }
   |              ++
help: remove the let, the `let` keyword is not allowed in struct field definitions
   |
LL - struct Foo { await: () }
LL + struct Foo { : () }


error: expected `:`, found `(`
   |
   |
LL | struct Foo { await: () }
   |        ---          ^ expected `:`
   |        while parsing this struct

error: expected identifier, found keyword `await`
  --> /checkout/src/test/ui/async-await/await-keyword/2018-edition-error-in-non-macro-position.rs:16:15
  --> /checkout/src/test/ui/async-await/await-keyword/2018-edition-error-in-non-macro-position.rs:16:15
   |
LL | impl Foo { fn await() {} }
   |               ^^^^^ expected identifier, found keyword
   |
help: escape `await` to use it as an identifier
   |
LL | impl Foo { fn r#await() {} }

error: expected identifier, found keyword `await`
  --> /checkout/src/test/ui/async-await/await-keyword/2018-edition-error-in-non-macro-position.rs:19:14
   |
   |
LL | macro_rules! await {
   |              ^^^^^ expected identifier, found keyword
   |
help: escape `await` to use it as an identifier
   |
LL | macro_rules! r#await {

error: aborting due to 8 previous errors
------------------------------------------



---- [ui] src/test/ui/parser/mismatched-braces/missing-close-brace-in-struct.rs stdout ----
diff of stderr:

10 error: expected identifier, found keyword `trait`
11   --> $DIR/missing-close-brace-in-struct.rs:4:1
12    |
- LL | pub(crate) struct Bar<T> {
-    |                   --- while parsing this struct
16 LL | trait T {
17    | ^^^^^ expected identifier, found keyword
18    |


-    = help: the let keyword is not allowed in struct field definitions
-    = help: see https://doc.rust-lang.org/book/ch05-01-defining-structs.html for more information
+    = help: see <https://doc.rust-lang.org/book/ch05-01-defining-structs.html> for more information
+ help: remove the let, the `let` keyword is not allowed in struct field definitions
+    |
+ LL - trait T {
+ LL +  T {
21 
21 
- error: expected `:`, found `T`
-   --> $DIR/missing-close-brace-in-struct.rs:4:7
+ error: expected `:`, found `{`
24    |
24    |
+ LL | pub(crate) struct Bar<T> {
+    |                   --- while parsing this struct
25 LL | trait T {
-    |       ^ expected `:`
+    |         ^ expected `:`
27 
---
To only update this specific test, also pass `--test-args parser/mismatched-braces/missing-close-brace-in-struct.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/mismatched-braces/missing-close-brace-in-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mismatched-braces/missing-close-brace-in-struct" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mismatched-braces/missing-close-brace-in-struct/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/parser/mismatched-braces/missing-close-brace-in-struct.rs:13:65
   |
   |
LL | pub(crate) struct Bar<T> {
...
...
LL | fn main() {} //~ ERROR this file contains an unclosed delimiter

error: expected identifier, found keyword `trait`
  --> /checkout/src/test/ui/parser/mismatched-braces/missing-close-brace-in-struct.rs:4:1
   |
   |
LL | trait T { //~ ERROR expected identifier, found keyword `trait`
   |
   |
   = help: see <https://doc.rust-lang.org/book/ch05-01-defining-structs.html> for more information
help: remove the let, the `let` keyword is not allowed in struct field definitions
   |
LL - trait T { //~ ERROR expected identifier, found keyword `trait`
LL +  T { //~ ERROR expected identifier, found keyword `trait`


error: expected `:`, found `{`
   |
   |
LL | pub(crate) struct Bar<T> {
   |                   --- while parsing this struct
...
LL | trait T { //~ ERROR expected identifier, found keyword `trait`
   |         ^ expected `:`
error: aborting due to 3 previous errors
------------------------------------------


---

-    = help: the let keyword is not allowed in struct field definitions
-    = help: see https://doc.rust-lang.org/book/ch05-01-defining-structs.html for more information
- 
- error: expected `:`, found `foo`
-   --> $DIR/removed-syntax-field-let.rs:2:9
+    = help: see <https://doc.rust-lang.org/book/ch05-01-defining-structs.html> for more information
+ help: remove the let, the `let` keyword is not allowed in struct field definitions
- LL |     let foo: (),
-    |         ^^^ expected `:`
-    |         ^^^ expected `:`
+ LL -     let foo: (),
+ LL +      foo: (),
17 
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
19 
---
To only update this specific test, also pass `--test-args parser/removed-syntax-field-let.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/removed-syntax-field-let.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/removed-syntax-field-let" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/removed-syntax-field-let/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected identifier, found keyword `let`
   |
LL |     let foo: (),
   |     ^^^ expected identifier, found keyword
   |
   |
   = help: see <https://doc.rust-lang.org/book/ch05-01-defining-structs.html> for more information
help: remove the let, the `let` keyword is not allowed in struct field definitions
LL -     let foo: (),
LL +      foo: (),
   |

