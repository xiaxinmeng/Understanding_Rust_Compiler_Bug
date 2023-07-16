plain
...i..ii..............................................................ii............................ 7800/11763
.............................................................................i.........i............ 7900/11763
.......................................................................i............................ 8000/11763
..................................................................i................................. 8100/11763
.........................................F....................................F..................... 8200/11763
..i.......................................................................F....F.................... 8300/11763
.................................................................................................... 8500/11763
.................................................................................................... 8600/11763
.................................................................................................... 8700/11763
.................................................................................................... 8800/11763
---
.................................................................................................... 10900/11763
.................................................................................................... 11000/11763
.....................................................................ii............................. 11100/11763
.................................................................................................... 11200/11763
....F..............FF............................................................................... 11300/11763
.................................................................................................... 11500/11763
.................................................................................................... 11600/11763
......................................................i.i........................................... 11700/11763
...............................................................
...............................................................
failures:

---- [ui] ui/infinite/infinite-autoderef.rs stdout ----
diff of stderr:

37    |
38    = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`infinite_autoderef`)
39 
+ error[E0055]: reached the recursion limit while auto-dereferencing `Foo`
+    |
+    |
+ LL |     Foo.bar();
+    |         ^^^ deref recursion limit reached
+    |
+    = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`infinite_autoderef`)
+ 
40 error[E0599]: no method named `bar` found for struct `Foo` in the current scope
42    |


46 LL |     Foo.bar();
47    |         ^^^ method not found in `Foo`
- error: aborting due to 6 previous errors
+ error: aborting due to 7 previous errors
50 
51 Some errors have detailed explanations: E0055, E0308, E0599, E0609.
---
To only update this specific test, also pass `--test-args infinite/infinite-autoderef.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/infinite/infinite-autoderef.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-autoderef" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-autoderef/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/infinite/infinite-autoderef.rs:20:13
   |
LL |         x = box x;
   |             ^^^^^
   |             |
   |             cyclic type of infinite size
   |             help: try using a conversion method: `(box x).to_string()`

error[E0055]: reached the recursion limit while auto-dereferencing `Foo`
   |
   |
LL |     Foo.foo;
   |     ^^^^^^^ deref recursion limit reached
   |
   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`infinite_autoderef`)

error[E0055]: reached the recursion limit while auto-dereferencing `Foo`
   |
   |
LL |     Foo.foo;
   |         ^^^ deref recursion limit reached
   |
   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`infinite_autoderef`)
error[E0609]: no field `foo` on type `Foo`
  --> /checkout/src/test/ui/infinite/infinite-autoderef.rs:25:9
   |
   |
LL |     Foo.foo;


error[E0055]: reached the recursion limit while auto-dereferencing `Foo`
   |
   |
LL |     Foo.bar();
   |         ^^^ deref recursion limit reached
   |
   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`infinite_autoderef`)

error[E0055]: reached the recursion limit while auto-dereferencing `Foo`
   |
   |
LL |     Foo.bar();
   |         ^^^ deref recursion limit reached
   |
   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`infinite_autoderef`)

error[E0599]: no method named `bar` found for struct `Foo` in the current scope
   |
LL | struct Foo;
LL | struct Foo;
   | ----------- method `bar` not found for this
...
LL |     Foo.bar();
   |         ^^^ method not found in `Foo`
error: aborting due to 7 previous errors

Some errors have detailed explanations: E0055, E0308, E0599, E0609.
For more information about an error, try `rustc --explain E0055`.
For more information about an error, try `rustc --explain E0055`.

------------------------------------------


---- [ui] ui/issues/issue-69396-const-no-type-in-macro.rs stdout ----
diff of stderr:

16    = note: `A` must be defined only once in the value namespace of this module
18 
- error: missing type for `const` item
- error: missing type for `const` item
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures
21    |
21    |
22 LL |               const A = "A".$fn();

-    |                     ^ help: provide a type for the item: `A: usize`
+    |                     |
+    |                     not allowed in type signatures
+    |                     not allowed in type signatures
+    |                     help: replace `_` with the correct type: `usize`
24 ...
25 LL | / suite! {
26 LL | |     len;
47    |
48    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
49 
- error: aborting due to 3 previous errors
- error: aborting due to 3 previous errors
+ error: missing type for `const` item
+   --> $DIR/issue-69396-const-no-type-in-macro.rs:4:19
+    |
+ LL |               const A = "A".$fn();
+    |                     ^ help: provide a type for the item: `A: <type>`
+ ...
+ LL | / suite! {
+ LL | |     len;
+ LL | |     is_empty;
+ LL | | }
+    |
+    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
+ error: aborting due to 4 previous errors
+ error: aborting due to 4 previous errors
51 
52 Some errors have detailed explanations: E0121, E0428.
53 For more information about an error, try `rustc --explain E0121`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69396-const-no-type-in-macro/issue-69396-const-no-type-in-macro.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-69396-const-no-type-in-macro.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-69396-const-no-type-in-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69396-const-no-type-in-macro" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69396-const-no-type-in-macro/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0428]: the name `A` is defined multiple times
   |
   |
LL |               const A = "A".$fn();
   |               |
   |               |
   |               `A` redefined here
   |               previous definition of the value `A` here
...
LL | / suite! {
LL | |     len;
LL | |     is_empty;
LL | | }
   |
   |
   = note: `A` must be defined only once in the value namespace of this module


error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |               const A = "A".$fn();
   |                     |
   |                     not allowed in type signatures
   |                     not allowed in type signatures
   |                     help: replace `_` with the correct type: `usize`
...
LL | / suite! {
LL | |     len;
LL | |     is_empty;
LL | | }
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |               const A = "A".$fn();
   |                     |
   |                     not allowed in type signatures
   |                     not allowed in type signatures
   |                     help: replace `_` with the correct type: `bool`
...
LL | / suite! {
LL | |     len;
LL | |     is_empty;
LL | | }
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: missing type for `const` item
error: missing type for `const` item
  --> /checkout/src/test/ui/issues/issue-69396-const-no-type-in-macro.rs:4:19
   |
LL |               const A = "A".$fn();
   |                     ^ help: provide a type for the item: `A: <type>`
...
LL | / suite! {
LL | |     len;
LL | |     is_empty;
LL | | }
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 4 previous errors
---
---- [ui] ui/parser/expr-as-stmt-2.rs stdout ----
diff of stderr:

31    |
32 LL |   fn foo(a: Option<u32>, b: Option<u32>) -> bool {
33    |                                             ---- expected `bool` because of return type
- LL |       if let Some(x) = a { true } else { false }
-    |       ------------------------------------------ help: parentheses are required to parse this as an expression: `(if let Some(x) = a { true } else { false })`
37 LL | /     &&
37 LL | /     &&
38 LL | |     if let Some(y) = a { true } else { false }

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/expr-as-stmt-2/expr-as-stmt-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/expr-as-stmt-2.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/expr-as-stmt-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/expr-as-stmt-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/expr-as-stmt-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/expr-as-stmt-2.rs:3:26
   |
LL |     if let Some(x) = a { true } else { false }
   |     |                    |
   |     |                    |
   |     |                    expected `()`, found `bool`
   |     expected this to be `()`
   |
help: you might have meant to return this value
   |
LL |     if let Some(x) = a { return true; } else { false }
   |                          ^^^^^^     ^
error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/expr-as-stmt-2.rs:3:40
   |
   |
LL |     if let Some(x) = a { true } else { false }
   |     |                                  |
   |     |                                  |
   |     |                                  expected `()`, found `bool`
   |     expected this to be `()`
   |
help: you might have meant to return this value
   |
LL |     if let Some(x) = a { true } else { return false; }
   |                                        ^^^^^^      ^
error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/expr-as-stmt-2.rs:6:5
   |
   |
LL |   fn foo(a: Option<u32>, b: Option<u32>) -> bool {
   |                                             ---- expected `bool` because of return type
...
LL | /     && //~ ERROR mismatched types
LL | |     if let Some(y) = a { true } else { false }
   | |______________________________________________^ expected `bool`, found `&&bool`
error: aborting due to 3 previous errors

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
For more information about this error, try `rustc --explain E0308`.
---
diff of stderr:

83   --> $DIR/expr-as-stmt.rs:24:11
84    |
85 LL |     { 3 } * 3
-    |     ----- ^^^
-    |     |
-    |     help: parentheses are required to parse this as an expression: `({ 3 })`
89 
90 error: aborting due to 9 previous errors
91 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/expr-as-stmt/expr-as-stmt.stderr
diff of fixed:

21 }
22 
23 fn baz() -> i32 {
-     ({ 3 }) * 3 //~ ERROR type `{integer}` cannot be dereferenced
+     { 3 } * 3 //~ ERROR type `{integer}` cannot be dereferenced
25     //~^ ERROR mismatched types
27 


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/expr-as-stmt/expr-as-stmt.fixed
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/expr-as-stmt.rs`
error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/expr-as-stmt.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/expr-as-stmt" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/expr-as-stmt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected expression, found `+`
  --> /checkout/src/test/ui/parser/expr-as-stmt.rs:7:9
   |
LL |     {2} + {2} //~ ERROR expected expression, found `+`
   |     --- ^ expected expression
   |     |
   |     help: parentheses are required to parse this as an expression: `({2})`
error: expected expression, found `+`
  --> /checkout/src/test/ui/parser/expr-as-stmt.rs:12:9
   |
   |
LL |     {2} + 2 //~ ERROR expected expression, found `+`
   |     --- ^ expected expression
   |     |
   |     help: parentheses are required to parse this as an expression: `({2})`
error: expected expression, found `+`
  --> /checkout/src/test/ui/parser/expr-as-stmt.rs:18:12
   |
   |
LL |     { 42 } + foo; //~ ERROR expected expression, found `+`
   |     ------ ^ expected expression
   |     |
   |     help: parentheses are required to parse this as an expression: `({ 42 })`
error: expected expression, found `>`
  --> /checkout/src/test/ui/parser/expr-as-stmt.rs:31:7
   |
   |
LL |     } > 0 //~ ERROR expected expression
   |       ^ expected expression
help: parentheses are required to parse this as an expression
   |
LL |     (match x {
LL |         _ => 1,
LL |         _ => 1,
LL |     }) > 0 //~ ERROR expected expression

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/expr-as-stmt.rs:7:6
   |
   |
LL |     {2} + {2} //~ ERROR expected expression, found `+`
   |      ^ expected `()`, found integer
   |
help: you might have meant to return this value
   |
LL |     {return 2;} + {2} //~ ERROR expected expression, found `+`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/expr-as-stmt.rs:12:6
   |
   |
LL |     {2} + 2 //~ ERROR expected expression, found `+`
   |      ^ expected `()`, found integer
   |
help: you might have meant to return this value
   |
LL |     {return 2;} + 2 //~ ERROR expected expression, found `+`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/expr-as-stmt.rs:18:7
   |
   |
LL |     { 42 } + foo; //~ ERROR expected expression, found `+`
   |       ^^ expected `()`, found integer
   |
help: you might have meant to return this value
   |
LL |     { return 42; } + foo; //~ ERROR expected expression, found `+`
   |       ^^^^^^   ^
error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/expr-as-stmt.rs:24:7
   |
   |
LL |     { 3 } * 3 //~ ERROR type `{integer}` cannot be dereferenced
   |       ^ expected `()`, found integer
   |
help: you might have meant to return this value
   |
LL |     { return 3; } * 3 //~ ERROR type `{integer}` cannot be dereferenced


error[E0614]: type `{integer}` cannot be dereferenced
   |
   |
LL |     { 3 } * 3 //~ ERROR type `{integer}` cannot be dereferenced

error: aborting due to 9 previous errors

Some errors have detailed explanations: E0308, E0614.
---
---- [ui] ui/parser/item-free-const-no-body-semantic-fail.rs stdout ----
diff of stderr:

14    |        |
15    |        help: provide a definition for the constant: `= <expr>;`
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures
+    |
+ LL | const B;
+    |       ^ not allowed in type signatures
18 
18 
+ error: missing type for `const` item
+   --> $DIR/item-free-const-no-body-semantic-fail.rs:6:7
+    |
+ LL | const B;
+    |       ^ help: provide a type for the item: `B: <type>`
+ error: aborting due to 4 previous errors
+ 
+ For more information about this error, try `rustc --explain E0121`.
19 
19 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/item-free-const-no-body-semantic-fail/item-free-const-no-body-semantic-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/item-free-const-no-body-semantic-fail.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/item-free-const-no-body-semantic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/item-free-const-no-body-semantic-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/item-free-const-no-body-semantic-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: free constant item without body
   |
   |
LL | const A: u8; //~ ERROR free constant item without body
   |            |
   |            |
   |            help: provide a definition for the constant: `= <expr>;`

error: free constant item without body
   |
   |
LL | const B; //~ ERROR free constant item without body
   |        |
   |        |
   |        help: provide a definition for the constant: `= <expr>;`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | const B; //~ ERROR free constant item without body
   |       ^ not allowed in type signatures
error: missing type for `const` item
  --> /checkout/src/test/ui/parser/item-free-const-no-body-semantic-fail.rs:6:7
   |
   |
LL | const B; //~ ERROR free constant item without body
   |       ^ help: provide a type for the item: `B: <type>`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0121`.


------------------------------------------


---- [ui] ui/parser/item-free-static-no-body-semantic-fail.rs stdout ----
diff of stderr:

30    |             |
31    |             help: provide a definition for the static: `= <expr>;`
- error: aborting due to 4 previous errors
- error: aborting due to 4 previous errors
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures
+    |
+ LL | static B;
+    |        ^ not allowed in type signatures
34 
34 
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures
+    |
+    |
+ LL | static mut D;
+    |            ^ not allowed in type signatures
+ 
+ error: missing type for `static` item
+    |
+ LL | static B;
+ LL | static B;
+    |        ^ help: provide a type for the item: `B: <type>`
+ 
+ error: missing type for `static mut` item
+    |
+    |
+ LL | static mut D;
+    |            ^ help: provide a type for the item: `D: <type>`
+ error: aborting due to 8 previous errors
+ 
+ For more information about this error, try `rustc --explain E0121`.
35 
35 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/item-free-static-no-body-semantic-fail/item-free-static-no-body-semantic-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/item-free-static-no-body-semantic-fail.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/item-free-static-no-body-semantic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/item-free-static-no-body-semantic-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/item-free-static-no-body-semantic-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: free static item without body
   |
   |
LL | static A: u8; //~ ERROR free static item without body
   |             |
   |             |
   |             help: provide a definition for the static: `= <expr>;`

error: free static item without body
   |
   |
LL | static B; //~ ERROR free static item without body
   |         |
   |         |
   |         help: provide a definition for the static: `= <expr>;`

error: free static item without body
   |
   |
LL | static mut C: u8; //~ ERROR free static item without body
   |                 |
   |                 |
   |                 help: provide a definition for the static: `= <expr>;`

error: free static item without body
   |
   |
LL | static mut D; //~ ERROR free static item without body
   |             |
   |             |
   |             help: provide a definition for the static: `= <expr>;`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | static B; //~ ERROR free static item without body
   |        ^ not allowed in type signatures

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | static mut D; //~ ERROR free static item without body
   |            ^ not allowed in type signatures

error: missing type for `static` item
   |
   |
LL | static B; //~ ERROR free static item without body
   |        ^ help: provide a type for the item: `B: <type>`

error: missing type for `static mut` item
   |
   |
LL | static mut D; //~ ERROR free static item without body
   |            ^ help: provide a type for the item: `D: <type>`
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0121`.


------------------------------------------


---- [ui] ui/suggestions/const-no-type.rs stdout ----
diff of stderr:

- error: missing type for `const` item
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures
3    |
4 LL | const C = 42;


-    |       ^ help: provide a type for the item: `C: i32`
+    |       |
+    |       not allowed in type signatures
+    |       not allowed in type signatures
+    |       help: replace `_` with the correct type: `i32`
- error: missing type for `const` item
- error: missing type for `const` item
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures
9    |
9    |
10 LL | const D = &&42;

-    |       ^ help: provide a type for the item: `D: &&i32`
+    |       |
+    |       not allowed in type signatures
+    |       not allowed in type signatures
+    |       help: replace `_` with the correct type: `&&i32`
12 
- error: missing type for `static` item
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures
15    |
15    |
16 LL | static S = Vec::<String>::new();

-    |        ^ help: provide a type for the item: `S: Vec<String>`
+    |        |
+    |        not allowed in type signatures
+    |        not allowed in type signatures
+    |        help: replace `_` with the correct type: `Vec<String>`
18 
- error: missing type for `static mut` item
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures
21    |
21    |
22 LL | static mut SM = "abc";

-    |            ^^ help: provide a type for the item: `SM: &str`
+    |            |
+    |            not allowed in type signatures
+    |            not allowed in type signatures
+    |            help: replace `_` with the correct type: `&str`
25 error: missing type for `const` item
26   --> $DIR/const-no-type.rs:14:7


40 LL | static mut SM2 = "abc";
41    |            ^^^ help: provide a type for the item: `SM2: <type>`
- error: aborting due to 7 previous errors
+ error: missing type for `const` item
+   --> $DIR/const-no-type.rs:33:7
+    |
+    |
+ LL | const C = 42;
+    |       ^ help: provide a type for the item: `C: <type>`
+ error: missing type for `const` item
+   --> $DIR/const-no-type.rs:38:7
+    |
+    |
+ LL | const D = &&42;
+    |       ^ help: provide a type for the item: `D: <type>`
+ 
+ error: missing type for `static` item
+    |
+    |
+ LL | static S = Vec::<String>::new();
+    |        ^ help: provide a type for the item: `S: <type>`
+ 
+ error: missing type for `static mut` item
+    |
+    |
+ LL | static mut SM = "abc";
+    |            ^^ help: provide a type for the item: `SM: <type>`
+ error: aborting due to 11 previous errors
+ 
+ For more information about this error, try `rustc --explain E0121`.
45 
45 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/const-no-type/const-no-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/const-no-type.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/const-no-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/const-no-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/const-no-type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
LL | const C = 42;
   |       ^
   |       |
   |       |
   |       not allowed in type signatures
   |       help: replace `_` with the correct type: `i32`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | const D = &&42;
   |       |
   |       not allowed in type signatures
   |       not allowed in type signatures
   |       help: replace `_` with the correct type: `&&i32`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | static S = Vec::<String>::new();
   |        |
   |        not allowed in type signatures
   |        not allowed in type signatures
   |        help: replace `_` with the correct type: `Vec<String>`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | static mut SM = "abc";
   |            |
   |            not allowed in type signatures
   |            not allowed in type signatures
   |            help: replace `_` with the correct type: `&str`
error: missing type for `const` item
  --> /checkout/src/test/ui/suggestions/const-no-type.rs:14:7
   |
LL | const C2 = 42;
LL | const C2 = 42;
   |       ^^ help: provide a type for the item: `C2: <type>`

error: missing type for `static` item
   |
   |
LL | static S2 = "abc";
   |        ^^ help: provide a type for the item: `S2: <type>`

error: missing type for `static mut` item
   |
   |
LL | static mut SM2 = "abc";
   |            ^^^ help: provide a type for the item: `SM2: <type>`
error: missing type for `const` item
  --> /checkout/src/test/ui/suggestions/const-no-type.rs:33:7
   |
LL | const C = 42;
LL | const C = 42;
   |       ^ help: provide a type for the item: `C: <type>`
error: missing type for `const` item
  --> /checkout/src/test/ui/suggestions/const-no-type.rs:38:7
   |
   |
LL | const D = &&42;
   |       ^ help: provide a type for the item: `D: <type>`

error: missing type for `static` item
   |
   |
LL | static S = Vec::<String>::new();
   |        ^ help: provide a type for the item: `S: <type>`

error: missing type for `static mut` item
   |
   |
LL | static mut SM = "abc";
   |            ^^ help: provide a type for the item: `SM: <type>`
error: aborting due to 11 previous errors

For more information about this error, try `rustc --explain E0121`.

---

6    |                 |
7    |                 &str
8 
- error[E0369]: cannot add `{integer}` to `&str`
-   --> $DIR/issue-79040.rs:2:25
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures
11    |
11    |
12 LL |     const FOO = "hello" + 1;
-    |                 ------- ^ - {integer}
-    |                 &str
+    |           ^^^ not allowed in type signatures
16 
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error: missing type for `const` item
+   --> $DIR/issue-79040.rs:2:11
+    |
+ LL |     const FOO = "hello" + 1;
+    |           ^^^ help: provide a type for the item: `FOO: <type>`
- For more information about this error, try `rustc --explain E0369`.
+ error: aborting due to 3 previous errors
+ 
+ Some errors have detailed explanations: E0121, E0369.
---
To only update this specific test, also pass `--test-args typeck/issue-79040.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/issue-79040.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-79040" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-79040/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0369]: cannot add `{integer}` to `&str`
   |
   |
LL |     const FOO = "hello" + 1; //~ ERROR cannot add `{integer}` to `&str`
   |                 ------- ^ - {integer}
   |                 &str


error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     const FOO = "hello" + 1; //~ ERROR cannot add `{integer}` to `&str`
   |           ^^^ not allowed in type signatures
error: missing type for `const` item
  --> /checkout/src/test/ui/typeck/issue-79040.rs:2:11
   |
   |
LL |     const FOO = "hello" + 1; //~ ERROR cannot add `{integer}` to `&str`
   |           ^^^ help: provide a type for the item: `FOO: <type>`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0121, E0369.
For more information about an error, try `rustc --explain E0121`.
For more information about an error, try `rustc --explain E0121`.

------------------------------------------


---- [ui] ui/typeck/typeck_type_placeholder_item.rs#min_tait stdout ----
diff of stderr:

188 LL |     b: (T, T),
190 
190 
- error: missing type for `static` item
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures
193    |
194 LL |     static A = 42;


-    |            ^ help: provide a type for the item: `A: i32`
+    |            |
+    |            not allowed in type signatures
+    |            not allowed in type signatures
+    |            help: replace `_` with the correct type: `i32`
196 
197 error[E0121]: the type placeholder `_` is not allowed within types on item signatures

632    |              not allowed in type signatures
632    |              not allowed in type signatures
633    |              help: replace `_` with the correct type: `i32`
- error: aborting due to 69 previous errors
- error: aborting due to 69 previous errors
+ error: missing type for `static` item
+    |
+    |
+ LL |     static A = 42;
+    |            ^ help: provide a type for the item: `A: <type>`
+ error: aborting due to 70 previous errors
636 
637 Some errors have detailed explanations: E0121, E0282, E0403.
638 For more information about an error, try `rustc --explain E0121`.
638 For more information about an error, try `rustc --explain E0121`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item.min_tait/typeck_type_placeholder_item.min_tait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args typeck/typeck_type_placeholder_item.rs`

error in revision `min_tait`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item.min_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item.min_tait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:158:18
   |
LL | struct BadStruct<_>(_);
   |                  ^ expected identifier, found reserved identifier
error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:161:16
   |
   |
LL | trait BadTrait<_> {}
   |                ^ expected identifier, found reserved identifier
error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:171:19
   |
   |
LL | struct BadStruct1<_, _>(_);
   |                   ^ expected identifier, found reserved identifier
error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:171:22
   |
   |
LL | struct BadStruct1<_, _>(_);
   |                      ^ expected identifier, found reserved identifier
error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:176:19
   |
   |
LL | struct BadStruct2<_, T>(_, T);
   |                   ^ expected identifier, found reserved identifier

error: associated constant in `impl` without body
   |
LL |     const C: _;
   |     ^^^^^^^^^^-
   |               |
   |               |
   |               help: provide a definition for the constant: `= <expr>;`

error[E0403]: the name `_` is already used for a generic parameter in this item's generic parameters
   |
   |
LL | struct BadStruct1<_, _>(_);
   |                   -  ^ already used
   |                   first use of `_`


error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test() -> _ { 5 }
   |              |
   |              not allowed in type signatures
   |              help: replace with the correct return type: `i32`


error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test2() -> (_, _) { (5, 5) }
   |               -^--^-
   |               ||  |
   |               ||  not allowed in type signatures
   |               |not allowed in type signatures
   |               help: replace with the correct return type: `(i32, i32)`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | static TEST3: _ = "test";
   |               |
   |               not allowed in type signatures
   |               not allowed in type signatures
   |               help: replace `_` with the correct type: `&str`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | static TEST4: _ = 145;
   |               |
   |               not allowed in type signatures
   |               not allowed in type signatures
   |               help: replace `_` with the correct type: `i32`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | static TEST5: (_, _) = (1, 2);
   |               ^^^^^^ not allowed in type signatures

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test6(_: _) { }
   |             ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn test6<T>(_: T) { }
   |         ^^^    ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test6_b<T>(_: _, _: T) { }
   |                  ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn test6_b<T, U>(_: U, _: T) { }
   |             ^^^     ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test6_c<T, K, L, A, B>(_: _, _: (T, K, L, A, B)) { }
   |                              ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn test6_c<T, K, L, A, B, U>(_: U, _: (T, K, L, A, B)) { }
   |                         ^^^     ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test7(x: _) { let _x: usize = x; }
   |             ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn test7<T>(x: T) { let _x: usize = x; }
   |         ^^^    ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test8(_f: fn() -> _) { }
   |                      |
   |                      not allowed in type signatures
   |                      help: use type parameters instead: `T`


error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test8(_f: fn() -> _) { }
   |                      ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn test8<T>(_f: fn() -> T) { }
   |         ^^^             ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test11(x: &usize) -> &_ {
   |                         -^
   |                         ||
   |                         |not allowed in type signatures
   |                         help: replace with the correct return type: `&'static &'static usize`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | unsafe fn test12(x: *const usize) -> *const *const _ {
   |                                      |             |
   |                                      |             not allowed in type signatures
   |                                      |             not allowed in type signatures
   |                                      help: replace with the correct return type: `*const *const usize`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
LL |     a: _,
   |        ^ not allowed in type signatures
   |        ^ not allowed in type signatures
LL |     //~^ ERROR the type placeholder `_` is not allowed within types on item signatures
LL |     b: (_, _),
   |         ^  ^ not allowed in type signatures
   |         not allowed in type signatures
   |
help: use type parameters instead
   |
   |
LL | struct Test10<T> {
LL |     a: T,
LL |     //~^ ERROR the type placeholder `_` is not allowed within types on item signatures
LL |     b: (T, T),


error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
LL |     static A = 42;
   |            ^
   |            |
   |            |
   |            not allowed in type signatures
   |            help: replace `_` with the correct type: `i32`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     static B: _ = 42;
   |               |
   |               not allowed in type signatures
   |               not allowed in type signatures
   |               help: replace `_` with the correct type: `i32`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     static C: Option<_> = Some(42);
   |               ^^^^^^^^^ not allowed in type signatures

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test() -> _ { 5 }
   |                     |
   |                     not allowed in type signatures
   |                     help: replace with the correct return type: `i32`


error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test2() -> (_, _) { (5, 5) }
   |                      -^--^-
   |                      ||  |
   |                      ||  not allowed in type signatures
   |                      |not allowed in type signatures
   |                      help: replace with the correct return type: `(i32, i32)`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     static FN_TEST3: _ = "test";
   |                      |
   |                      not allowed in type signatures
   |                      not allowed in type signatures
   |                      help: replace `_` with the correct type: `&str`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     static FN_TEST4: _ = 145;
   |                      |
   |                      not allowed in type signatures
   |                      not allowed in type signatures
   |                      help: replace `_` with the correct type: `i32`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     static FN_TEST5: (_, _) = (1, 2);
   |                      ^^^^^^ not allowed in type signatures

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test6(_: _) { }
   |                    ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL |     fn fn_test6<T>(_: T) { }
   |                ^^^    ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test7(x: _) { let _x: usize = x; }
   |                    ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL |     fn fn_test7<T>(x: T) { let _x: usize = x; }
   |                ^^^    ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test8(_f: fn() -> _) { }
   |                             |
   |                             not allowed in type signatures
   |                             help: use type parameters instead: `T`


error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test8(_f: fn() -> _) { }
   |                             ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL |     fn fn_test8<T>(_f: fn() -> T) { }
   |                ^^^             ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
LL |         a: _,
   |            ^ not allowed in type signatures
   |            ^ not allowed in type signatures
LL |         //~^ ERROR the type placeholder `_` is not allowed within types on item signatures
LL |         b: (_, _),
   |             ^  ^ not allowed in type signatures
   |             not allowed in type signatures
   |
help: use type parameters instead
   |
   |
LL |     struct FnTest10<T> {
LL |         a: T,
LL |         //~^ ERROR the type placeholder `_` is not allowed within types on item signatures
LL |         b: (T, T),

error[E0282]: type annotations needed
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:132:18
   |
   |
LL |     fn fn_test11(_: _) -> (_, _) { panic!() }
   |                  ^ cannot infer type

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test11(_: _) -> (_, _) { panic!() }
   |                            ^  ^ not allowed in type signatures
   |                            not allowed in type signatures


error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test12(x: i32) -> (_, _) { (x, x) }
   |                             -^--^-
   |                             ||  |
   |                             ||  not allowed in type signatures
   |                             |not allowed in type signatures
   |                             help: replace with the correct return type: `(i32, i32)`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test13(x: _) -> (i32, _) { (x, x) }
   |                           ------^-
   |                           |     not allowed in type signatures
   |                           |     not allowed in type signatures
   |                           help: replace with the correct return type: `(i32, i32)`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | struct BadStruct<_>(_);
   |                     ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | struct BadStruct<T>(T);
   |                  ^  ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | impl BadTrait<_> for BadStruct<_> {}
   |               ^                ^ not allowed in type signatures
   |               not allowed in type signatures
   |
help: use type parameters instead
   |
   |
LL | impl<T> BadTrait<T> for BadStruct<T> {}
   |     ^^^          ^                ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn impl_trait() -> impl BadTrait<_> {
   |                                  ^ not allowed in type signatures

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | struct BadStruct1<_, _>(_);
   |                         ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | struct BadStruct1<T, _>(T);
   |                   ^     ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | struct BadStruct2<_, T>(_, T);
   |                         ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | struct BadStruct2<U, T>(U, T);
   |                   ^     ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | type X = Box<_>;
   |              ^ not allowed in type signatures

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
---
test result: FAILED. 11656 passed; 10 failed; 97 ignored; 0 measured; 0 filtered out; finished in 130.52s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:06
