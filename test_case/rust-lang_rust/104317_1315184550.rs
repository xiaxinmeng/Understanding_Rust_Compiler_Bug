plain
........................................................................................ 1056/13844
........................................................................................ 1144/13844
........................................................................................ 1232/13844
........................................................................................ 1320/13844
......................F..i.....FF....................................................... 1408/13844
i....................................................................................... 1584/13844
........................................................................................ 1672/13844
........................................................................................ 1760/13844
...................................................i...........ii....................... 1848/13844
---
- error[E0080]: evaluation of constant value failed
+ note: erroneous constant used
16   --> $DIR/issue-81899.rs:4:23
17    |
18 LL | const _CONST: &[u8] = &f(&[], |_| {});
-    |                       ^^^^^^^^^^^^^^^ referenced constant has errors
+    |                       ^^^^^^^^^^^^^^^
20 
- error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args borrowck/issue-81899.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-81899.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-81899" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-81899/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/borrowck/issue-81899.rs:11:5
   |
   |
LL | const _CONST: &[u8] = &f(&[], |_| {});
   |                        -------------- inside `_CONST` at /checkout/src/test/ui/borrowck/issue-81899.rs:4:24
...
LL |     panic!() //~ ERROR: evaluation of constant value failed
   |     |
   |     the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/borrowck/issue-81899.rs:11:5
   |     the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/borrowck/issue-81899.rs:11:5
   |     inside `f::<[closure@/checkout/src/test/ui/borrowck/issue-81899.rs:4:31: 4:34]>` at /checkout/library/std/src/panic.rs:19:9
   = note: this error originates in the macro `$crate::panic::panic_2015` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)

note: erroneous constant used
  --> /checkout/src/test/ui/borrowck/issue-81899.rs:4:23
  --> /checkout/src/test/ui/borrowck/issue-81899.rs:4:23
   |
LL | const _CONST: &[u8] = &f(&[], |_| {});

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
---
- error[E0080]: evaluation of constant value failed
+ note: erroneous constant used
16   --> $DIR/issue-88434-removal-index-should-be-less.rs:3:23
17    |
18 LL | const _CONST: &[u8] = &f(&[], |_| {});
-    |                       ^^^^^^^^^^^^^^^ referenced constant has errors
+    |                       ^^^^^^^^^^^^^^^
20 
- error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args borrowck/issue-88434-removal-index-should-be-less.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-88434-removal-index-should-be-less" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-88434-removal-index-should-be-less/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:10:5
   |
   |
LL | const _CONST: &[u8] = &f(&[], |_| {});
   |                        -------------- inside `_CONST` at /checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:3:24
...
LL |     panic!() //~ ERROR evaluation of constant value failed
   |     |
   |     the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:10:5
   |     the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:10:5
   |     inside `f::<[closure@/checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:3:31: 3:34]>` at /checkout/library/std/src/panic.rs:19:9
   = note: this error originates in the macro `$crate::panic::panic_2015` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)

note: erroneous constant used
  --> /checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:3:23
  --> /checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:3:23
   |
LL | const _CONST: &[u8] = &f(&[], |_| {});

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
---
- error[E0080]: evaluation of constant value failed
+ note: erroneous constant used
16   --> $DIR/issue-88434-minimal-example.rs:3:21
17    |
18 LL | const _CONST: &() = &f(&|_| {});
-    |                     ^^^^^^^^^^^ referenced constant has errors
+    |                     ^^^^^^^^^^^
20 
- error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args borrowck/issue-88434-minimal-example.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-88434-minimal-example" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-88434-minimal-example/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs:10:5
   |
   |
LL | const _CONST: &() = &f(&|_| {});
   |                      ---------- inside `_CONST` at /checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs:3:22
...
LL |     panic!() //~ ERROR evaluation of constant value failed
   |     |
   |     the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs:10:5
   |     the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs:10:5
   |     inside `f::<[closure@/checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs:3:25: 3:28]>` at /checkout/library/std/src/panic.rs:19:9
   = note: this error originates in the macro `$crate::panic::panic_2015` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)

note: erroneous constant used
  --> /checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs:3:21
  --> /checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs:3:21
   |
LL | const _CONST: &() = &f(&|_| {});

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-17252.rs stdout ----
diff of stderr:

1 error[E0391]: cycle detected when const-evaluating + checking `FOO`
-   --> $DIR/issue-17252.rs:1:1
3    |
4 LL | const FOO: usize = FOO;
-    | ^^^^^^^^^^^^^^^^^^^^^^^
+    |                    ^^^
+    |                    ^^^
6    |
7    = note: ...which immediately requires const-evaluating + checking `FOO` again
8 note: cycle used when const-evaluating + checking `main::{constant#0}`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17252/issue-17252.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-17252.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-17252.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17252" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17252/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when const-evaluating + checking `FOO`
   |
LL | const FOO: usize = FOO; //~ ERROR E0391
   |                    ^^^
   |
   |
   = note: ...which immediately requires const-evaluating + checking `FOO` again
note: cycle used when const-evaluating + checking `main::{constant#0}`
   |
   |
LL |     let _x: [u8; FOO]; // caused stack overflow prior to fix

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-23302-3.rs stdout ----
diff of stderr:

1 error[E0391]: cycle detected when const-evaluating + checking `A`
-   --> $DIR/issue-23302-3.rs:1:1
3    |
3    |
4 LL | const A: i32 = B;
+    |                ^
6    |
6    |
7 note: ...which requires const-evaluating + checking `B`...
-   --> $DIR/issue-23302-3.rs:3:1
9    |
9    |
10 LL | const B: i32 = A;
+    |                ^
+    |                ^
12    = note: ...which again requires const-evaluating + checking `A`, completing the cycle
13 note: cycle used when simplifying constant for the type system `A`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-3/issue-23302-3.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-3/issue-23302-3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-23302-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23302-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23302-3/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when const-evaluating + checking `A`
   |
   |
LL | const A: i32 = B; //~ ERROR cycle detected
   |
   |
note: ...which requires const-evaluating + checking `B`...
   |
   |
LL | const B: i32 = A;
   |                ^
   = note: ...which again requires const-evaluating + checking `A`, completing the cycle
note: cycle used when simplifying constant for the type system `A`
   |
   |
LL | const A: i32 = B; //~ ERROR cycle detected

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
---
- error[E0080]: evaluation of constant value failed
+ note: erroneous constant used
10   --> $DIR/issue-41394.rs:7:9
11    |
12 LL |     A = Foo::A as isize
-    |         ^^^^^^^^^^^^^^^ referenced constant has errors
+    |         ^^^^^^^^^^^^^^^
14 
- error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args issues/issue-41394.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-41394.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41394" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41394/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: cannot add `{integer}` to `&str`
   |
   |
LL |     A = "" + 1
   |         -- ^ - {integer}
   |         &str

note: erroneous constant used
  --> /checkout/src/test/ui/issues/issue-41394.rs:7:9
  --> /checkout/src/test/ui/issues/issue-41394.rs:7:9
   |
LL |     A = Foo::A as isize

error: aborting due to previous error

For more information about this error, try `rustc --explain E0369`.
For more information about this error, try `rustc --explain E0369`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-69602-type-err-during-codegen-ice.rs stdout ----
diff of stderr:

13 LL | impl TraitB for B {
14    | ^^^^^^^^^^^^^^^^^ missing `MyA` in implementation
- error[E0080]: evaluation of constant value failed
+ note: erroneous constant used
17   --> $DIR/issue-69602-type-err-during-codegen-ice.rs:21:17
18    |
18    |
19 LL |     let _ = [0; B::VALUE];
-    |                 ^^^^^^^^ referenced constant has errors
+    |                 ^^^^^^^^
21 
- error: aborting due to 3 previous errors
---
To only update this specific test, also pass `--test-args issues/issue-69602-type-err-during-codegen-ice.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-69602-type-err-during-codegen-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69602-type-err-during-codegen-ice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69602-type-err-during-codegen-ice/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0437]: type `M` is not a member of trait `TraitB`
   |
   |
LL |     type M   = A; //~ ERROR type `M` is not a member of trait `TraitB`
   |     ^^^^^^^^^^^^^ not a member of trait `TraitB`
error[E0046]: not all trait items implemented, missing: `MyA`
  --> /checkout/src/test/ui/issues/issue-69602-type-err-during-codegen-ice.rs:16:1
   |
   |
LL |     type MyA: TraitA;
   |     ---------------- `MyA` from trait
...
LL | impl TraitB for B { //~ ERROR not all trait items implemented, missing: `MyA`
   | ^^^^^^^^^^^^^^^^^ missing `MyA` in implementation
note: erroneous constant used
  --> /checkout/src/test/ui/issues/issue-69602-type-err-during-codegen-ice.rs:21:17
   |
   |
LL |     let _ = [0; B::VALUE];

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0046, E0437.
Some errors have detailed explanations: E0046, E0437.
For more information about an error, try `rustc --explain E0046`.
------------------------------------------


---- [ui] src/test/ui/resolve/issue-50599.rs stdout ----
diff of stderr:

16 LL +     const M: usize = (f64::from(N) * LOG10_2) as usize;
18 
- error[E0080]: evaluation of constant value failed
+ note: erroneous constant used
20   --> $DIR/issue-50599.rs:4:29
20   --> $DIR/issue-50599.rs:4:29
21    |
22 LL |     let mut digits = [0u32; M];
-    |                             ^ referenced constant has errors
+    |                             ^
24 
- error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args resolve/issue-50599.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/issue-50599.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-50599" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-50599/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find value `LOG10_2` in module `std::f64`
   |
   |
LL |     const M: usize = (f64::from(N) * std::f64::LOG10_2) as usize; //~ ERROR cannot find value
   |                                                ^^^^^^^ not found in `std::f64`
help: consider importing one of these items
   |
LL | use std::f32::consts::LOG10_2;
   |
   |
LL | use std::f64::consts::LOG10_2;
   |
help: if you import `LOG10_2`, refer to it directly
   |
LL -     const M: usize = (f64::from(N) * std::f64::LOG10_2) as usize; //~ ERROR cannot find value
LL +     const M: usize = (f64::from(N) * LOG10_2) as usize; //~ ERROR cannot find value

note: erroneous constant used
  --> /checkout/src/test/ui/resolve/issue-50599.rs:4:29
   |
   |
LL |     let mut digits = [0u32; M];

error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
For more information about this error, try `rustc --explain E0425`.
------------------------------------------


---- [ui] src/test/ui/type/type-dependent-def-issue-49241.rs stdout ----
diff of stderr:

6    |     |
7    |     help: consider using `let` instead of `const`: `let l`
- error[E0080]: evaluation of constant value failed
+ note: erroneous constant used
10   --> $DIR/type-dependent-def-issue-49241.rs:4:18
11    |
11    |
12 LL |     let s: [u32; l] = v.into_iter().collect();
-    |                  ^ referenced constant has errors
+    |                  ^
14 
- error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args type/type-dependent-def-issue-49241.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-dependent-def-issue-49241.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-dependent-def-issue-49241" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-dependent-def-issue-49241/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/type/type-dependent-def-issue-49241.rs:3:22
   |
   |
LL |     const l: usize = v.count(); //~ ERROR attempt to use a non-constant value in a constant
   |     -------          ^ non-constant value
   |     |
   |     help: consider using `let` instead of `const`: `let l`
note: erroneous constant used
  --> /checkout/src/test/ui/type/type-dependent-def-issue-49241.rs:4:18
   |
   |
LL |     let s: [u32; l] = v.into_iter().collect();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0435`.
