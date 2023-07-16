plain
---- [ui] src/test/ui/associated-consts/issue-93835.rs stdout ----
diff of stderr:

27    |
28 LL |     p::a<p:p<e=6>>
- help: maybe you meant to write an assignment here
-    |
-    |
- LL |     let p:a<p:p<e=6>>
34 
35 error[E0658]: associated const equality is incomplete
36   --> $DIR/issue-93835.rs:2:13

---
To only update this specific test, also pass `--test-args associated-consts/issue-93835.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/issue-93835.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-93835" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-93835/auxiliary"
stdout: none
--- stderr -------------------------------
error: comparison operators cannot be chained
   |
LL | fn e() {
   |        - while parsing this struct
   |        - while parsing this struct
LL |     p:a<p:p<e=6>>
   |        ^        ^
   |
   = help: use `::<...>` instead of `<...>` to specify lifetime, type, or const arguments
   = help: or use `(...)` if you meant to specify fn arguments
error[E0425]: cannot find value `p` in this scope
  --> /checkout/src/test/ui/associated-consts/issue-93835.rs:2:5
   |
   |
LL |     p:a<p:p<e=6>>
   |
   |
help: you might have meant to write a `struct` literal
   |
LL ~ fn e() { SomeStruct {
LL |     p:a<p:p<e=6>>
 ...
LL |     //~| ERROR associated type bounds
LL ~ }}
   |
help: maybe you meant to write a path separator here
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
LL |     p::a<p:p<e=6>>

error[E0658]: associated const equality is incomplete
  --> /checkout/src/test/ui/associated-consts/issue-93835.rs:2:13
   |
   |
LL |     p:a<p:p<e=6>>
   |
   = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
   = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
   = help: add `#![feature(associated_const_equality)]` to the crate attributes to enable
error[E0658]: associated const equality is incomplete
  --> /checkout/src/test/ui/associated-consts/issue-93835.rs:2:13
   |
   |
LL |     p:a<p:p<e=6>>
   |
   = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
   = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
   = help: add `#![feature(associated_const_equality)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/associated-consts/issue-93835.rs:2:9
   |
   |
LL |     p:a<p:p<e=6>>
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0425, E0658.
For more information about an error, try `rustc --explain E0425`.
---
diff of stderr:

30   --> $DIR/E0423.rs:14:8
31    |
32 LL |     if T {} == T {} { println!("Ok"); }
-    |
-    |
- help: surround the struct literal with parentheses
-    |
- LL |     if (T {}) == T {} { println!("Ok"); }
-    |        +    +
+    |        ^ you might want to surround a struct literal with parentheses: `(T { /* fields */ })`?
40 error[E0423]: expected function, tuple struct or tuple variant, found struct `Foo`
41   --> $DIR/E0423.rs:4:13



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0423/E0423.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0423.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0423.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0423" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0423/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/error-codes/E0423.rs:12:32
   |
   |
LL |     if let S { x: _x, y: 2 } = S { x: 1, y: 2 } { println!("Ok"); }
   |
   |
help: surround the struct literal with parentheses
   |
LL |     if let S { x: _x, y: 2 } = (S { x: 1, y: 2 }) { println!("Ok"); }

error: expected expression, found `==`
  --> /checkout/src/test/ui/error-codes/E0423.rs:14:13
   |
   |
LL |     if T {} == T {} { println!("Ok"); }

error: struct literals are not allowed here
  --> /checkout/src/test/ui/error-codes/E0423.rs:20:14
   |
   |
LL |     for _ in std::ops::Range { start: 0, end: 10 } {}
   |
   |
help: surround the struct literal with parentheses
   |
LL |     for _ in (std::ops::Range { start: 0, end: 10 }) {}

error[E0423]: expected value, found struct `T`
  --> /checkout/src/test/ui/error-codes/E0423.rs:14:8
   |
   |
LL |     if T {} == T {} { println!("Ok"); }
   |        ^ you might want to surround a struct literal with parentheses: `(T { /* fields */ })`?
error[E0423]: expected function, tuple struct or tuple variant, found struct `Foo`
  --> /checkout/src/test/ui/error-codes/E0423.rs:4:13
   |
   |
LL |     struct Foo { a: bool };
   |     ---------------------- `Foo` defined here
LL |
LL |     let f = Foo(); //~ ERROR E0423
...
LL | fn foo() {
   | -------- similarly named function `foo` defined here
   |
   |
help: use struct literal syntax instead
   |
LL |     let f = Foo { a: val }; //~ ERROR E0423
help: a function with a similar name exists
   |
   |
LL |     let f = foo(); //~ ERROR E0423

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0423`.
---
diff of stderr:

46   --> $DIR/struct-literal-variant-in-if.rs:10:13
47    |
48 LL |     if x == E::V { field } {}
-    |
-    |
- help: surround the struct literal with parentheses
-    |
- LL |     if x == (E::V { field }) {}
-    |             +              +
+    |             ^^^^ you might want to surround a struct literal with parentheses: `(E::V { /* fields */ })`?
56 error[E0308]: mismatched types
57   --> $DIR/struct-literal-variant-in-if.rs:10:20



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/struct-literal-variant-in-if/struct-literal-variant-in-if.stderr
To only update this specific test, also pass `--test-args parser/struct-literal-variant-in-if.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/struct-literal-variant-in-if.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/struct-literal-variant-in-if" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/struct-literal-variant-in-if/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/parser/struct-literal-variant-in-if.rs:13:13
   |
   |
LL |     if x == E::I { field1: true, field2: 42 } {}
   |
   |
help: surround the struct literal with parentheses
   |
LL |     if x == (E::I { field1: true, field2: 42 }) {}

error: struct literals are not allowed here
  --> /checkout/src/test/ui/parser/struct-literal-variant-in-if.rs:15:13
   |
   |
LL |     if x == E::V { field: false } {}
   |
   |
help: surround the struct literal with parentheses
   |
LL |     if x == (E::V { field: false }) {}

error: struct literals are not allowed here
  --> /checkout/src/test/ui/parser/struct-literal-variant-in-if.rs:17:13
   |
   |
LL |     if x == E::J { field: -42 } {}
   |
   |
help: surround the struct literal with parentheses
   |
LL |     if x == (E::J { field: -42 }) {}

error: struct literals are not allowed here
  --> /checkout/src/test/ui/parser/struct-literal-variant-in-if.rs:19:13
   |
   |
LL |     if x == E::K { field: "" } {}
   |
   |
help: surround the struct literal with parentheses
   |
LL |     if x == (E::K { field: "" }) {}

error[E0423]: expected value, found struct variant `E::V`
  --> /checkout/src/test/ui/parser/struct-literal-variant-in-if.rs:10:13
   |
   |
LL |     if x == E::V { field } {}
   |             ^^^^ you might want to surround a struct literal with parentheses: `(E::V { /* fields */ })`?
error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/struct-literal-variant-in-if.rs:10:20
   |
   |
LL |     if x == E::V { field } {}
   |     |              |
   |     |              expected `()`, found `bool`
   |     expected this to be `()`


error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/struct-literal-variant-in-if.rs:21:20
   |
LL |     let y: usize = ();
   |            |
   |            expected due to this

error: aborting due to 7 previous errors
---
diff of stderr:

2   --> $DIR/type-ascription-instead-of-let.rs:5:9
3    |
4 LL |         temp: i32 = fun(5i32);
-    |         |
-    |         not found in this scope
-    |         not found in this scope
-    |         help: maybe you meant to write an assignment here: `let temp`
+    |         ^^^^ expecting a type here because of type ascription
10 error[E0425]: cannot find value `temp` in this scope
11   --> $DIR/type-ascription-instead-of-let.rs:7:9



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/type-ascription-instead-of-let/type-ascription-instead-of-let.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/type-ascription-instead-of-let.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/type-ascription-instead-of-let.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/type-ascription-instead-of-let" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/type-ascription-instead-of-let/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find value `temp` in this scope
   |
   |
LL |         temp: i32 = fun(5i32);
   |         ^^^^ expecting a type here because of type ascription
error[E0425]: cannot find value `temp` in this scope
  --> /checkout/src/test/ui/suggestions/type-ascription-instead-of-let.rs:7:9
   |
   |
LL |         temp + value + 1

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0425`.
---
diff of stderr:

2   --> $DIR/issue-91267.rs:2:16
3    |
4 LL |     0: u8<e<5>=e>
-    |                |
-    |                not found in this scope
-    |                not found in this scope
-    |                help: maybe you meant to write an assignment here: `let e`
+    |                ^ expecting a type here because of type ascription
10 error[E0229]: associated type bindings are not allowed here
11   --> $DIR/issue-91267.rs:2:11



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-91267/issue-91267.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args typeck/issue-91267.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/issue-91267.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-91267" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-91267/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0412]: cannot find type `e` in this scope
   |
   |
LL |     0: u8<e<5>=e>
   |                ^ expecting a type here because of type ascription
error[E0229]: associated type bindings are not allowed here
  --> /checkout/src/test/ui/typeck/issue-91267.rs:2:11
   |
   |
LL |     0: u8<e<5>=e>
   |           ^^^^^^ associated type not allowed here
error[E0308]: mismatched types
  --> /checkout/src/test/ui/typeck/issue-91267.rs:2:5
   |
LL | fn main() {
LL | fn main() {
   |           - expected `()` because of default return type
LL |     0: u8<e<5>=e>
   |     ^^^^^^^^^^^^^ expected `()`, found `u8`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0229, E0308, E0412.
For more information about an error, try `rustc --explain E0229`.
