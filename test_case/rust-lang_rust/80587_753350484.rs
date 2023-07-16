plain
......................................i............................................................. 7800/11239
.............................i...................................................................... 7900/11239
........................................FF.......................................................... 8000/11239
................................................................................i................... 8100/11239
..........FF..F.F................................................................................... 8200/11239
.................................................................................................... 8400/11239
.................................................................................................... 8500/11239
..............................................iiii.iiii............................................. 8600/11239
......i.............i............................................................................... 8700/11239
......i.............i............................................................................... 8700/11239
.................................................................................................... 8800/11239
.................................................................................................... 8900/11239
.................................................................................................... 9000/11239
.................................................................................................... 9100/11239
.................................................................................................... 9200/11239
...................................i......i......................................................... 9300/11239
..........................................................................iiiiii..iiiiii.i.......... 9400/11239
.................................................................................................... 9600/11239
............................F....................................................................... 9700/11239
.................................................................................................... 9800/11239
.................................................................................................... 9900/11239
---

---- [ui] ui/parser/issue-22712.rs stdout ----
diff of stderr:

- error: expected one of `:`, `;`, `=`, `@`, or `|`, found `<`
-   --> $DIR/issue-22712.rs:6:12
+ error: expected `;`, found `}`
3    |
3    |
4 LL |     let Foo<Vec<u8>>
-    |            ^ expected one of `:`, `;`, `=`, `@`, or `|`
+    |                     ^ help: add `;` here
+ LL | }
+    | - unexpected token
- error: aborting due to previous error
- error: aborting due to previous error
+ error[E0532]: expected unit struct, unit variant or constant, found struct `Foo`
+    |
+    |
+ LL | / struct Foo<B> {
+ LL | |     buffer: B
+ LL | | }
+    | |_- `Foo` defined here
+ ...
+ LL |       let Foo<Vec<u8>>
+    |           ^^^^^^^^^^^^ help: use struct pattern syntax instead: `Foo { buffer }`
+ error: aborting due to 2 previous errors
+ 
+ For more information about this error, try `rustc --explain E0532`.
9 
9 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-22712/issue-22712.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issue-22712.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-22712.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-22712" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-22712/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected `;`, found `}`
   |
   |
LL |     let Foo<Vec<u8>>  //~ ERROR expected one of `:`, `;`, `=`, `@`, or `|`, found `<`
   |                     ^ help: add `;` here
LL | }
   | - unexpected token

error[E0532]: expected unit struct, unit variant or constant, found struct `Foo`
   |
   |
LL | / struct Foo<B> {
LL | |     buffer: B
LL | | }
   | |_- `Foo` defined here
...
LL |       let Foo<Vec<u8>>  //~ ERROR expected one of `:`, `;`, `=`, `@`, or `|`, found `<`
   |           ^^^^^^^^^^^^ help: use struct pattern syntax instead: `Foo { buffer }`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0532`.


------------------------------------------


---- [ui] ui/parser/issue-22647.rs stdout ----
diff of stderr:

- error: expected one of `:`, `;`, `=`, `@`, or `|`, found `<`
-   --> $DIR/issue-22647.rs:2:15
+ error: expected expression, found keyword `where`
3    |
3    |
- LL |     let caller<F> = |f: F|
-    |               ^ expected one of `:`, `;`, `=`, `@`, or `|`
+ LL |     where F: Fn() -> i32
6 
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-22647/issue-22647.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issue-22647.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-22647.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-22647" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-22647/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected expression, found keyword `where`
   |
LL |     where F: Fn() -> i32
   |     ^^^^^ expected expression

---

---- [ui] ui/parser/pat-lt-bracket-1.rs stdout ----
diff of stderr:

- error: expected one of `=>`, `@`, `if`, or `|`, found `<`
-   --> $DIR/pat-lt-bracket-1.rs:3:7
+ error: expected one of `,`, `.`, `:`, `=`, `>`, `?`, or an operator, found `=>`
3    |
3    |
4 LL |     x < 7 => (),
-    |       ^ expected one of `=>`, `@`, `if`, or `|`
+    |           ^^ expected one of 7 possible tokens
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-lt-bracket-1/pat-lt-bracket-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/pat-lt-bracket-1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/pat-lt-bracket-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-lt-bracket-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-lt-bracket-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected one of `,`, `.`, `:`, `=`, `>`, `?`, or an operator, found `=>`
   |
   |
LL |     x < 7 => (),
   |           ^^ expected one of 7 possible tokens
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/parser/pat-lt-bracket-2.rs stdout ----
diff of stderr:

- error: expected one of `:`, `@`, or `|`, found `<`
-   --> $DIR/pat-lt-bracket-2.rs:1:7
+ error: expected one of `>`, a const expression, lifetime, or type, found `)`
3    |
3    |
4 LL | fn a(B<) {}
-    |       ^ expected one of `:`, `@`, or `|`
-    |
-    = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
- help: if this is a `self` type, give it a parameter name
-    |
- LL | fn a(self: B<) {}
-    |      ^^^^^^^
- help: if this is a type, explicitly ignore the parameter name
-    |
- LL | fn a(_: B<) {}
-    |      ^^^^
+    |        ^ expected one of `>`, a const expression, lifetime, or type
17 error: aborting due to previous error
18 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-lt-bracket-2/pat-lt-bracket-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/pat-lt-bracket-2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/pat-lt-bracket-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-lt-bracket-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-lt-bracket-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected one of `>`, a const expression, lifetime, or type, found `)`
   |
   |
LL | fn a(B<) {}
   |        ^ expected one of `>`, a const expression, lifetime, or type
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/parser/pat-lt-bracket-3.rs stdout ----
diff of stderr:

- error: expected one of `=>`, `@`, `if`, or `|`, found `<`
-   --> $DIR/pat-lt-bracket-3.rs:6:16
+ error[E0507]: cannot move out of `self.0` which is behind a shared reference
3    |
3    |
+ LL |         match *self {
+    |               ^^^^^ help: consider borrowing here: `&*self`
4 LL |             Foo<T>(x, y) => {
-    |                ^ expected one of `=>`, `@`, `if`, or `|`
+    |                    -  - ...and here
+    |                    data moved here
+    |
+    |
+    = note: move occurs because these variables have types that don't implement the `Copy` trait
7 error: aborting due to previous error
8 

+ For more information about this error, try `rustc --explain E0507`.
+ For more information about this error, try `rustc --explain E0507`.
9 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-lt-bracket-3/pat-lt-bracket-3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/pat-lt-bracket-3.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/pat-lt-bracket-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-lt-bracket-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-lt-bracket-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0507]: cannot move out of `self.0` which is behind a shared reference
   |
LL |         match *self {
LL |         match *self {
   |               ^^^^^ help: consider borrowing here: `&*self`
LL |             Foo<T>(x, y) => {
   |                    -  - ...and here
   |                    data moved here
   |
   |
   = note: move occurs because these variables have types that don't implement the `Copy` trait
error: aborting due to previous error

For more information about this error, try `rustc --explain E0507`.


------------------------------------------


---- [ui] ui/parser/pat-lt-bracket-4.rs stdout ----
diff of stderr:

- error: expected one of `=>`, `@`, `if`, or `|`, found `<`
-   --> $DIR/pat-lt-bracket-4.rs:8:12
+ error[E0433]: failed to resolve: use of undeclared type `Foo`
3    |
3    |
4 LL |         Foo<T>::A(value) => value,
-    |            ^ expected one of `=>`, `@`, `if`, or `|`
+    |         ^^^ use of undeclared type `Foo`
- error: aborting due to previous error
+ error[E0433]: failed to resolve: use of undeclared type `Foo`
+   --> $DIR/pat-lt-bracket-4.rs:9:9
+    |
+    |
+ LL |         Foo<T>::B => 7,
+    |         ^^^ use of undeclared type `Foo`
+ error[E0412]: cannot find type `T` in this scope
+   --> $DIR/pat-lt-bracket-4.rs:8:13
+    |
+    |
+ LL |         Foo<T>::A(value) => value,
+    |             ^ not found in this scope
+ error[E0412]: cannot find type `T` in this scope
+   --> $DIR/pat-lt-bracket-4.rs:9:13
+    |
+    |
+ LL |         Foo<T>::B => 7,
+    |             ^ not found in this scope
+ error: aborting due to 4 previous errors
+ 
+ Some errors have detailed explanations: E0412, E0433.
+ For more information about an error, try `rustc --explain E0412`.
+ For more information about an error, try `rustc --explain E0412`.
9 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-lt-bracket-4/pat-lt-bracket-4.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/pat-lt-bracket-4.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/pat-lt-bracket-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-lt-bracket-4" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/pat-lt-bracket-4/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0433]: failed to resolve: use of undeclared type `Foo`
  --> /checkout/src/test/ui/parser/pat-lt-bracket-4.rs:8:9
   |
LL |         Foo<T>::A(value) => value, //~ error: expected one of `=>`, `@`, `if`, or `|`, found `<`
   |         ^^^ use of undeclared type `Foo`
error[E0433]: failed to resolve: use of undeclared type `Foo`
  --> /checkout/src/test/ui/parser/pat-lt-bracket-4.rs:9:9
   |
   |
LL |         Foo<T>::B => 7,
   |         ^^^ use of undeclared type `Foo`
error[E0412]: cannot find type `T` in this scope
  --> /checkout/src/test/ui/parser/pat-lt-bracket-4.rs:8:13
   |
   |
LL |         Foo<T>::A(value) => value, //~ error: expected one of `=>`, `@`, `if`, or `|`, found `<`
   |             ^ not found in this scope
error[E0412]: cannot find type `T` in this scope
  --> /checkout/src/test/ui/parser/pat-lt-bracket-4.rs:9:13
   |
   |
LL |         Foo<T>::B => 7,
   |             ^ not found in this scope
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0412, E0433.
For more information about an error, try `rustc --explain E0412`.
For more information about an error, try `rustc --explain E0412`.

------------------------------------------


---- [ui] ui/span/issue-34264.rs stdout ----
diff of stderr:

- error: expected one of `:`, `@`, or `|`, found `<`
-   --> $DIR/issue-34264.rs:1:14
+ error: expected one of `!`, `(`, `...`, `..=`, `..`, `::`, `:`, `{`, or `|`, found `,`
3    |
3    |
4 LL | fn foo(Option<i32>, String) {}
-    |              ^ expected one of `:`, `@`, or `|`
-    |
-    = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
- help: if this is a `self` type, give it a parameter name
-    |
- LL | fn foo(self: Option<i32>, String) {}
-    |        ^^^^^^^^^^^^
- help: if this is a type, explicitly ignore the parameter name
-    |
- LL | fn foo(_: Option<i32>, String) {}
+    |                   ^ expected one of 9 possible tokens
16 
16 
17 error: expected one of `:`, `@`, or `|`, found `)`

62   --> $DIR/issue-34264.rs:1:4
63    |
63    |
64 LL | fn foo(Option<i32>, String) {}
+    |    ^^^           -  ------
66 
67 error[E0308]: mismatched types
68   --> $DIR/issue-34264.rs:8:13
68   --> $DIR/issue-34264.rs:8:13


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-34264/issue-34264.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args span/issue-34264.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-34264.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-34264" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-34264/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected one of `!`, `(`, `...`, `..=`, `..`, `::`, `:`, `{`, or `|`, found `,`
   |
   |
LL | fn foo(Option<i32>, String) {} //~ ERROR expected one of
   |                   ^ expected one of 9 possible tokens

error: expected one of `:`, `@`, or `|`, found `)`
   |
   |
LL | fn foo(Option<i32>, String) {} //~ ERROR expected one of
   |                           ^ expected one of `:`, `@`, or `|`
   |
   = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
help: if this is a parameter name, give it a type
   |
LL | fn foo(Option<i32>, String: TypeName) {} //~ ERROR expected one of
   |                     ^^^^^^^^^^^^^^^^
help: if this is a type, explicitly ignore the parameter name
   |
LL | fn foo(Option<i32>, _: String) {} //~ ERROR expected one of


error: expected one of `:`, `@`, or `|`, found `,`
   |
   |
LL | fn bar(x, y: usize) {} //~ ERROR expected one of
   |         ^ expected one of `:`, `@`, or `|`
   |
   = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
help: if this is a `self` type, give it a parameter name
   |
LL | fn bar(self: x, y: usize) {} //~ ERROR expected one of
   |        ^^^^^^^
help: if this is a parameter name, give it a type
   |
LL | fn bar(x: TypeName, y: usize) {} //~ ERROR expected one of
   |        ^^^^^^^^^^^
help: if this is a type, explicitly ignore the parameter name
   |
LL | fn bar(_: x, y: usize) {} //~ ERROR expected one of

error[E0061]: this function takes 2 arguments but 3 arguments were supplied
  --> /checkout/src/test/ui/span/issue-34264.rs:7:5
   |
   |
LL |     foo(Some(42), 2, ""); //~ ERROR this function takes
   |     ^^^ --------  -  -- supplied 3 arguments
   |     expected 2 arguments
   |
note: function defined here
  --> /checkout/src/test/ui/span/issue-34264.rs:1:4
  --> /checkout/src/test/ui/span/issue-34264.rs:1:4
   |
LL | fn foo(Option<i32>, String) {} //~ ERROR expected one of
   |    ^^^           -  ------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/span/issue-34264.rs:8:13
   |
   |
LL |     bar("", ""); //~ ERROR mismatched types
   |             ^^ expected `usize`, found `&str`
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
  --> /checkout/src/test/ui/span/issue-34264.rs:10:5
   |
   |
LL |     bar(1, 2, 3); //~ ERROR this function takes
   |     ^^^ -  -  - supplied 3 arguments
   |     expected 2 arguments
   |
note: function defined here
  --> /checkout/src/test/ui/span/issue-34264.rs:3:4
  --> /checkout/src/test/ui/span/issue-34264.rs:3:4
   |
LL | fn bar(x, y: usize) {} //~ ERROR expected one of
   |    ^^^ -  --------
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0061, E0308.
For more information about an error, try `rustc --explain E0061`.
For more information about an error, try `rustc --explain E0061`.

------------------------------------------


---- [ui] ui/suggestions/issue-64252-self-type.rs stdout ----
diff of stderr:

- error: expected one of `:`, `@`, or `|`, found `<`
-   --> $DIR/issue-64252-self-type.rs:4:15
+ error: expected one of `!`, `(`, `...`, `..=`, `..`, `::`, `:`, `{`, or `|`, found `)`
3    |
3    |
4 LL | pub fn foo(Box<Self>) { }
-    |               ^ expected one of `:`, `@`, or `|`
-    |
-    = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
- help: if this is a `self` type, give it a parameter name
-    |
- LL | pub fn foo(self: Box<Self>) { }
-    |            ^^^^^^^^^
- help: if this is a type, explicitly ignore the parameter name
-    |
- LL | pub fn foo(_: Box<Self>) { }
+    |                     ^ expected one of 9 possible tokens
16 
16 
- error: expected one of `:`, `@`, or `|`, found `<`
-   --> $DIR/issue-64252-self-type.rs:10:15
+ error: expected one of `!`, `(`, `...`, `..=`, `..`, `::`, `:`, `{`, or `|`, found `)`
19    |
19    |
20 LL |     fn bar(Box<Self>) { }
-    |               ^ expected one of `:`, `@`, or `|`
-    |
-    = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
- help: if this is a `self` type, give it a parameter name
-    |
- LL |     fn bar(self: Box<Self>) { }
-    |            ^^^^^^^^^
- help: if this is a type, explicitly ignore the parameter name
-    |
- LL |     fn bar(_: Box<Self>) { }
+    |                     ^ expected one of 9 possible tokens
32 
33 error: aborting due to 2 previous errors
34 
34 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-64252-self-type/issue-64252-self-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/issue-64252-self-type.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-64252-self-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-64252-self-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-64252-self-type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected one of `!`, `(`, `...`, `..=`, `..`, `::`, `:`, `{`, or `|`, found `)`
   |
   |
LL | pub fn foo(Box<Self>) { }
   |                     ^ expected one of 9 possible tokens

error: expected one of `!`, `(`, `...`, `..=`, `..`, `::`, `:`, `{`, or `|`, found `)`
   |
   |
LL |     fn bar(Box<Self>) { }
   |                     ^ expected one of 9 possible tokens
error: aborting due to 2 previous errors


------------------------------------------
---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:30
