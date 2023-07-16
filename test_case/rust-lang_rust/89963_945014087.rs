plain

20    |              |
21    |              call expression requires function
22    |
- help: `E::Empty4` is a unit variant, you need to write it without the parentheses
+ help: `E::Empty4` is a unit variant, you need to write it without the parenthesis
24    |
25 LL |     let e4 = E::Empty4;

41    |               |
42    |               call expression requires function
43    |
43    |
- help: `XE::XEmpty4` is a unit variant, you need to write it without the parentheses
+ help: `XE::XEmpty4` is a unit variant, you need to write it without the parenthesis
45    |
46 LL |     let xe4 = XE::XEmpty4;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-unit-expr/empty-struct-unit-expr.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-unit-expr/empty-struct-unit-expr.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args empty/empty-struct-unit-expr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/empty/empty-struct-unit-expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-unit-expr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-unit-expr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0618]: expected function, found `Empty2`
   |
LL | struct Empty2;
LL | struct Empty2;
   | -------------- `Empty2` defined here
...
LL |     let e2 = Empty2(); //~ ERROR expected function, found `Empty2`
   |              |
   |              call expression requires function


error[E0618]: expected function, found enum variant `E::Empty4`
   |
LL |     Empty4
LL |     Empty4
   |     ------ `E::Empty4` defined here
...
LL |     let e4 = E::Empty4();
   |              |
   |              call expression requires function
   |
   |
help: `E::Empty4` is a unit variant, you need to write it without the parenthesis
   |
LL |     let e4 = E::Empty4;


error[E0618]: expected function, found `empty_struct::XEmpty2`
   |
   |
LL |     let xe2 = XEmpty2(); //~ ERROR expected function, found `empty_struct::XEmpty2`
   |               |
   |               call expression requires function


error[E0618]: expected function, found enum variant `XE::XEmpty4`
   |
   |
LL |     let xe4 = XE::XEmpty4();
   |               |
   |               call expression requires function
   |
   |
help: `XE::XEmpty4` is a unit variant, you need to write it without the parenthesis
   |
LL |     let xe4 = XE::XEmpty4;

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0618`.
---

9    |     |
10    |     call expression requires function
11    |
- help: `X::Entry` is a unit variant, you need to write it without the parentheses
+ help: `X::Entry` is a unit variant, you need to write it without the parenthesis
14 LL |     X::Entry;
15    |     ~~~~~~~~



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0618/E0618.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0618.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0618.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0618" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0618/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0618]: expected function, found enum variant `X::Entry`
   |
LL |     Entry,
LL |     Entry,
   |     ----- `X::Entry` defined here
LL |     X::Entry();
   |     ^^^^^^^^--
   |     |
   |     call expression requires function
   |     call expression requires function
   |
help: `X::Entry` is a unit variant, you need to write it without the parenthesis
LL |     X::Entry;
   |     ~~~~~~~~

error[E0618]: expected function, found `i32`
error[E0618]: expected function, found `i32`
  --> /checkout/src/test/ui/error-codes/E0618.rs:9:5
   |
LL |     let x = 0i32;
   |         - `x` has type `i32`
LL |     x();
   |     ^--
   |     call expression requires function

error: aborting due to 2 previous errors

---
---- [ui] ui/parser/recover-for-loop-parens-around-head.rs stdout ----
diff of stderr:

14    |
15 LL -     for ( elem in vec ) {
16 LL +     for  elem in vec  {
+    | 
18 
19 error[E0308]: mismatched types
20   --> $DIR/recover-for-loop-parens-around-head.rs:13:38
---
To only update this specific test, also pass `--test-args parser/recover-for-loop-parens-around-head.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/recover-for-loop-parens-around-head.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-for-loop-parens-around-head" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-for-loop-parens-around-head/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected one of `)`, `,`, `@`, or `|`, found keyword `in`
   |
   |
LL |     for ( elem in vec ) {
   |                ^^ expected one of `)`, `,`, `@`, or `|`

error: unexpected parentheses surrounding `for` loop head
   |
   |
LL |     for ( elem in vec ) {
   |         ^             ^
help: remove parentheses in `for` loop
   |
   |
LL -     for ( elem in vec ) {
LL +     for  elem in vec  {

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/recover-for-loop-parens-around-head.rs:13:38
   |
   |
LL |         const RECOVERY_WITNESS: () = 0; //~ ERROR mismatched types
   |                                      ^ expected `()`, found integer
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.

---

338    |                 |
339    |                 call expression requires function
340    |
- help: `Z::Unit` is a unit variant, you need to write it without the parentheses
+ help: `Z::Unit` is a unit variant, you need to write it without the parenthesis
342    |
343 LL |         let _ = Z::Unit;

372    |                |
373    |                call expression requires function
374    |
374    |
- help: `m::E::Unit` is a unit variant, you need to write it without the parentheses
+ help: `m::E::Unit` is a unit variant, you need to write it without the parenthesis
376    |
377 LL |     let _: E = m::E::Unit;

406    |                |
407    |                call expression requires function
408    |
408    |
- help: `E::Unit` is a unit variant, you need to write it without the parentheses
+ help: `E::Unit` is a unit variant, you need to write it without the parenthesis
410    |
411 LL |     let _: E = E::Unit;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-enum-ctor/privacy-enum-ctor.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-enum-ctor/privacy-enum-ctor.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/privacy-enum-ctor.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/privacy-enum-ctor.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-enum-ctor" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-enum-ctor/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0423]: expected value, found enum `n::Z`
   |
LL |         n::Z;
   |         ^^^^
   |
   |
note: the enum is defined here
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:11:9
   |
LL | /         pub(in m) enum Z {
LL | |             Fn(u8),
LL | |             Struct {
LL | |                 s: u8,
LL | |             },
LL | |             Unit,
LL | |         }
   | |_________^
help: you might have meant to use the following enum variant
   |
LL |         m::Z::Unit;
help: the following enum variants are available
   |
   |
LL |         (m::Z::Fn(/* fields */));
   |         ~~~~~~~~~~~~~~~~~~~~~~~~
LL |         (m::Z::Struct { /* fields */ });

error[E0423]: expected value, found enum `Z`
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:25:9
   |
   |
LL |         Z;
   |         ^
   |
note: the enum is defined here
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:11:9
   |
LL | /         pub(in m) enum Z {
LL | |             Fn(u8),
LL | |             Struct {
LL | |                 s: u8,
LL | |             },
LL | |             Unit,
LL | |         }
   | |_________^
help: you might have meant to use the following enum variant
   |
LL |         m::Z::Unit;
help: the following enum variants are available
   |
   |
LL |         (m::Z::Fn(/* fields */));
   |         ~~~~~~~~~~~~~~~~~~~~~~~~
LL |         (m::Z::Struct { /* fields */ });


error[E0423]: expected value, found struct variant `Z::Struct`
   |
LL | /             Struct {
LL | /             Struct {
LL | |                 s: u8,
LL | |             },
   | |_____________- `Z::Struct` defined here
...
LL |           let _: Z = Z::Struct;
   |                      ^^^^^^^^^ help: use struct literal syntax instead: `Z::Struct { s: val }`

error[E0423]: expected value, found enum `m::E`
   |
LL |     fn f() {
LL |     fn f() {
   |     ------ similarly named function `f` defined here
...
LL |     let _: E = m::E;
   |
note: the enum is defined here
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:2:5
   |
   |
LL | /     pub enum E {
LL | |         Fn(u8),
LL | |         Struct {
LL | |             s: u8,
LL | |         },
LL | |         Unit,
LL | |     }
   | |_____^
help: you might have meant to use the following enum variant
   |
LL |     let _: E = E::Unit;
help: the following enum variants are available
   |
   |
LL |     let _: E = (E::Fn(/* fields */));
   |                ~~~~~~~~~~~~~~~~~~~~~
LL |     let _: E = (E::Struct { /* fields */ });
help: a function with a similar name exists
   |
   |
LL |     let _: E = m::f;
help: consider importing one of these items instead
   |
LL | use std::f32::consts::E;
   |
   |
LL | use std::f64::consts::E;
   |

error[E0423]: expected value, found struct variant `m::E::Struct`
   |
LL | /         Struct {
LL | /         Struct {
LL | |             s: u8,
LL | |         },
   | |_________- `m::E::Struct` defined here
...
LL |       let _: E = m::E::Struct;
   |                  ^^^^^^^^^^^^ help: use struct literal syntax instead: `m::E::Struct { s: val }`
error[E0423]: expected value, found enum `E`
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:49:16
   |
   |
LL |     let _: E = E;
   |
note: the enum is defined here
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:2:5
   |
   |
LL | /     pub enum E {
LL | |         Fn(u8),
LL | |         Struct {
LL | |             s: u8,
LL | |         },
LL | |         Unit,
LL | |     }
   | |_____^
help: you might have meant to use the following enum variant
   |
LL |     let _: E = E::Unit;
help: the following enum variants are available
   |
   |
LL |     let _: E = (E::Fn(/* fields */));
   |                ~~~~~~~~~~~~~~~~~~~~~
LL |     let _: E = (E::Struct { /* fields */ });
help: consider importing one of these items instead
   |
LL | use std::f32::consts::E;
   |
   |
LL | use std::f64::consts::E;
   |

error[E0423]: expected value, found struct variant `E::Struct`
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:53:16
   |
LL | /         Struct {
LL | |             s: u8,
LL | |         },
   | |_________- `E::Struct` defined here
...
LL |       let _: E = E::Struct;
   |                  ^^^^^^^^^ help: use struct literal syntax instead: `E::Struct { s: val }`
error[E0412]: cannot find type `Z` in this scope
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:57:12
   |
LL |     pub enum E {
LL |     pub enum E {
   |     ---------- similarly named enum `E` defined here
...
LL |     let _: Z = m::n::Z;
   |            ^ help: an enum with a similar name exists: `E`
   |
note: enum `m::Z` exists but is inaccessible
   |
   |
LL |         pub(in m) enum Z {
   |         ^^^^^^^^^^^^^^^^ not accessible

error[E0423]: expected value, found enum `m::n::Z`
   |
   |
LL |     let _: Z = m::n::Z;
   |
note: the enum is defined here
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:11:9
   |
   |
LL | /         pub(in m) enum Z {
LL | |             Fn(u8),
LL | |             Struct {
LL | |                 s: u8,
LL | |             },
LL | |             Unit,
LL | |         }
   | |_________^
help: you might have meant to use the following enum variant
   |
LL |     let _: Z = m::Z::Unit;
help: the following enum variants are available
   |
   |
LL |     let _: Z = (m::Z::Fn(/* fields */));
   |                ~~~~~~~~~~~~~~~~~~~~~~~~
LL |     let _: Z = (m::Z::Struct { /* fields */ });

error[E0412]: cannot find type `Z` in this scope
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:61:12
   |
   |
LL |     pub enum E {
   |     ---------- similarly named enum `E` defined here
...
LL |     let _: Z = m::n::Z::Fn;
   |            ^ help: an enum with a similar name exists: `E`
   |
note: enum `m::Z` exists but is inaccessible
   |
   |
LL |         pub(in m) enum Z {
   |         ^^^^^^^^^^^^^^^^ not accessible
error[E0412]: cannot find type `Z` in this scope
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:64:12
   |
LL |     pub enum E {
LL |     pub enum E {
   |     ---------- similarly named enum `E` defined here
...
LL |     let _: Z = m::n::Z::Struct;
   |            ^ help: an enum with a similar name exists: `E`
   |
note: enum `m::Z` exists but is inaccessible
   |
   |
LL |         pub(in m) enum Z {
   |         ^^^^^^^^^^^^^^^^ not accessible

error[E0423]: expected value, found struct variant `m::n::Z::Struct`
   |
LL | /             Struct {
LL | /             Struct {
LL | |                 s: u8,
LL | |             },
   | |_____________- `m::n::Z::Struct` defined here
...
LL |       let _: Z = m::n::Z::Struct;
   |                  ^^^^^^^^^^^^^^^ help: use struct literal syntax instead: `m::n::Z::Struct { s: val }`
error[E0412]: cannot find type `Z` in this scope
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:68:12
   |
LL |     pub enum E {
LL |     pub enum E {
   |     ---------- similarly named enum `E` defined here
...
LL |     let _: Z = m::n::Z::Unit {};
   |            ^ help: an enum with a similar name exists: `E`
   |
note: enum `m::Z` exists but is inaccessible
   |
   |
LL |         pub(in m) enum Z {
   |         ^^^^^^^^^^^^^^^^ not accessible

error[E0603]: enum `Z` is private
   |
   |
LL |     let _: Z = m::n::Z;
   |                      ^ private enum
   |
note: the enum `Z` is defined here
   |
   |
LL |         pub(in m) enum Z {


error[E0603]: enum `Z` is private
   |
   |
LL |     let _: Z = m::n::Z::Fn;
   |                      ^ private enum
   |
note: the enum `Z` is defined here
   |
   |
LL |         pub(in m) enum Z {


error[E0603]: enum `Z` is private
   |
   |
LL |     let _: Z = m::n::Z::Struct;
   |                      ^ private enum
   |
note: the enum `Z` is defined here
   |
   |
LL |         pub(in m) enum Z {


error[E0603]: enum `Z` is private
   |
   |
LL |     let _: Z = m::n::Z::Unit {};
   |                      ^ private enum
   |
note: the enum `Z` is defined here
   |
   |
LL |         pub(in m) enum Z {

error[E0308]: mismatched types
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:27:20
   |
   |
LL |             Fn(u8),
   |             ------ fn(u8) -> Z {Z::Fn} defined here
...
LL |         let _: Z = Z::Fn;
   |                -   ^^^^^ expected enum `Z`, found fn item
   |                expected due to this
   |
   = note: expected enum `Z`
   = note: expected enum `Z`
           found fn item `fn(u8) -> Z {Z::Fn}`
help: use parentheses to instantiate this tuple variant
   |
LL |         let _: Z = Z::Fn(_);


error[E0618]: expected function, found enum variant `Z::Unit`
   |
LL |             Unit,
LL |             Unit,
   |             ---- `Z::Unit` defined here
...
LL |         let _ = Z::Unit();
   |                 |
   |                 call expression requires function
   |
   |
help: `Z::Unit` is a unit variant, you need to write it without the parenthesis
   |
LL |         let _ = Z::Unit;

error[E0308]: mismatched types
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:43:16
   |
   |
LL |         Fn(u8),
   |         ------ fn(u8) -> E {E::Fn} defined here
...
LL |     let _: E = m::E::Fn;
   |            -   ^^^^^^^^ expected enum `E`, found fn item
   |            expected due to this
   |
   = note: expected enum `E`
   = note: expected enum `E`
           found fn item `fn(u8) -> E {E::Fn}`
help: use parentheses to instantiate this tuple variant
   |
LL |     let _: E = m::E::Fn(_);


error[E0618]: expected function, found enum variant `m::E::Unit`
   |
LL |         Unit,
LL |         Unit,
   |         ---- `m::E::Unit` defined here
...
LL |     let _: E = m::E::Unit();
   |                |
   |                call expression requires function
   |
   |
help: `m::E::Unit` is a unit variant, you need to write it without the parenthesis
   |
LL |     let _: E = m::E::Unit;

error[E0308]: mismatched types
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:51:16
   |
   |
LL |         Fn(u8),
   |         ------ fn(u8) -> E {E::Fn} defined here
...
LL |     let _: E = E::Fn;
   |            -   ^^^^^ expected enum `E`, found fn item
   |            expected due to this
   |
   = note: expected enum `E`
   = note: expected enum `E`
           found fn item `fn(u8) -> E {E::Fn}`
help: use parentheses to instantiate this tuple variant
   |
LL |     let _: E = E::Fn(_);


error[E0618]: expected function, found enum variant `E::Unit`
   |
LL |         Unit,
LL |         Unit,
   |         ---- `E::Unit` defined here
...
LL |     let _: E = E::Unit();
   |                |
   |                call expression requires function
   |
   |
help: `E::Unit` is a unit variant, you need to write it without the parenthesis
   |
LL |     let _: E = E::Unit;

error: aborting due to 23 previous errors

Some errors have detailed explanations: E0308, E0412, E0423, E0603, E0618.
---

27    |     |
28    |     call expression requires function
29    |
- help: `Alias::Unit` is a unit variant, you need to write it without the parentheses
+ help: `Alias::Unit` is a unit variant, you need to write it without the parenthesis
32 LL |     Alias::Unit;
33    |     ~~~~~~~~~~~



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-enum-variants/incorrect-variant-form-through-alias-caught/incorrect-variant-form-through-alias-caught.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-enum-variants/incorrect-variant-form-through-alias-caught.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-enum-variants/incorrect-variant-form-through-alias-caught.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-enum-variants/incorrect-variant-form-through-alias-caught" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-enum-variants/incorrect-variant-form-through-alias-caught/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0533]: expected unit struct, unit variant or constant, found struct variant `Alias::Braced`
   |
LL |     Alias::Braced;
   |     ^^^^^^^^^^^^^


error[E0533]: expected unit struct, unit variant or constant, found struct variant `Alias::Braced`
   |
LL |     let Alias::Braced = panic!();
   |         ^^^^^^^^^^^^^


error[E0164]: expected tuple struct or tuple variant, found struct variant `Alias::Braced`
   |
   |
LL |     let Alias::Braced(..) = panic!();
   |         ^^^^^^^^^^^^^^^^^ not a tuple variant or struct

error[E0618]: expected function, found enum variant `Alias::Unit`
   |
   |
LL | enum Enum { Braced {}, Unit, Tuple() }
   |                        ---- `Alias::Unit` defined here
LL |     Alias::Unit();
   |     ^^^^^^^^^^^--
   |     |
   |     call expression requires function
   |     call expression requires function
   |
help: `Alias::Unit` is a unit variant, you need to write it without the parenthesis
LL |     Alias::Unit;
   |     ~~~~~~~~~~~

error[E0164]: expected tuple struct or tuple variant, found unit variant `Alias::Unit`
---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:35
