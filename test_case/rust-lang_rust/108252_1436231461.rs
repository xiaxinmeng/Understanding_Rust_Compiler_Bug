plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:75573f9759179a720f4c3af6c9fb518ac0061dca)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
............................................................................i........... 2200/14478
........................................................................................ 2288/14478
........................................................................................ 2376/14478
........................................................................................ 2464/14478
.............................F...F...................................................... 2552/14478
........................................................................................ 2728/14478
........................................................................................ 2816/14478
........................................................................................ 2904/14478
........................................................................................ 2992/14478
---
........................................................................................ 4488/14478
..........................................................i............................. 4576/14478
........................................................................................ 4664/14478
........................................................................................ 4752/14478
.............................................F.......F..................F............... 4840/14478
...............................................................i........................ 5016/14478
........................................................................................ 5104/14478
........................................................................................ 5192/14478
........................................................................................ 5280/14478
---
........................................................................................ 13112/14478
........................................................................................ 13200/14478
........................................................................................ 13288/14478
........................................................................................ 13376/14478
..............................................................F......................... 13464/14478
...................................F...........................................i........ 13552/14478
...........FF...F................................................F...................... 13640/14478
.........F.............................................................................. 13816/14478
........................................................................................ 13904/14478
........................................................................................ 13992/14478
........................................................................................ 14080/14478
---
---- [ui] tests/ui/async-await/issues/issue-78654.rs#min stdout ----
diff of stderr:

12    |
13    = note: expressions using a const parameter must map each value to a distinct output value
14    = note: proving the result of expressions other than the parameter are unique is not supported
+ help: Either remove the type parameter 'H' or make use of it, for example ` for S<H>`.
+    |
+ LL - impl<const H: feature> Foo {
+ LL + impl<> Foo {
15 
16 error: aborting due to 2 previous errors
17 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-78654.min/issue-78654.min.stderr
To only update this specific test, also pass `--test-args async-await/issues/issue-78654.rs`


error in revision `min`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/issues/issue-78654.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-78654.min" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-78654.min/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0573]: expected type, found built-in attribute `feature`
  --> fake-test-src-base/async-await/issues/issue-78654.rs:9:15
   |
LL | impl<const H: feature> Foo {


error[E0207]: the const parameter `H` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/async-await/issues/issue-78654.rs:9:6
   |
LL | impl<const H: feature> Foo {
   |
   |
   = note: expressions using a const parameter must map each value to a distinct output value
   = note: proving the result of expressions other than the parameter are unique is not supported
help: Either remove the type parameter 'H' or make use of it, for example ` for S<H>`.
   |
LL - impl<const H: feature> Foo {
LL + impl<> Foo {

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0207, E0573.
---
---- [ui] tests/ui/async-await/issues/issue-78654.rs#full stdout ----
diff of stderr:

12    |
13    = note: expressions using a const parameter must map each value to a distinct output value
14    = note: proving the result of expressions other than the parameter are unique is not supported
+ help: Either remove the type parameter 'H' or make use of it, for example ` for S<H>`.
+    |
+ LL - impl<const H: feature> Foo {
+ LL + impl<> Foo {
15 
16 error: aborting due to 2 previous errors
17 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-78654.full/issue-78654.full.stderr
To only update this specific test, also pass `--test-args async-await/issues/issue-78654.rs`


error in revision `full`: 1 errors occurred comparing output.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/issues/issue-78654.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-78654.full" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-78654.full/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0573]: expected type, found built-in attribute `feature`
  --> fake-test-src-base/async-await/issues/issue-78654.rs:9:15
   |
LL | impl<const H: feature> Foo {


error[E0207]: the const parameter `H` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/async-await/issues/issue-78654.rs:9:6
   |
LL | impl<const H: feature> Foo {
   |
   |
   = note: expressions using a const parameter must map each value to a distinct output value
   = note: proving the result of expressions other than the parameter are unique is not supported
help: Either remove the type parameter 'H' or make use of it, for example ` for S<H>`.
   |
LL - impl<const H: feature> Foo {
LL + impl<> Foo {

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0207, E0573.
---
---- [ui] tests/ui/const-generics/issues/issue-68366.rs#full stdout ----
diff of stderr:

6    |
7    = note: expressions using a const parameter must map each value to a distinct output value
8    = note: proving the result of expressions other than the parameter are unique is not supported
+ help: Either remove the type parameter 'N' or make use of it, for example ` for S<N>`.
+    |
+ LL - impl <const N: usize> Collatz<{Some(N)}> {}
+ LL + impl <> Collatz<{Some(N)}> {}
9 
9 
10 error[E0207]: the const parameter `N` is not constrained by the impl trait, self type, or predicates

15    |
15    |
16    = note: expressions using a const parameter must map each value to a distinct output value
17    = note: proving the result of expressions other than the parameter are unique is not supported
+ help: Either remove the type parameter 'N' or make use of it, for example ` for S<N>`.
+    |
+ LL - impl<const N: usize> Foo {}
+ LL + impl<> Foo {}
18 
19 error: aborting due to 2 previous errors
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-68366.full/issue-68366.full.stderr
To only update this specific test, also pass `--test-args const-generics/issues/issue-68366.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-generics/issues/issue-68366.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-68366.full" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-68366.full/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0207]: the const parameter `N` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/const-generics/issues/issue-68366.rs:11:7
   |
LL | impl <const N: usize> Collatz<{Some(N)}> {}
   |
   |
   = note: expressions using a const parameter must map each value to a distinct output value
   = note: proving the result of expressions other than the parameter are unique is not supported
help: Either remove the type parameter 'N' or make use of it, for example ` for S<N>`.
   |
LL - impl <const N: usize> Collatz<{Some(N)}> {}
LL + impl <> Collatz<{Some(N)}> {}


error[E0207]: the const parameter `N` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/const-generics/issues/issue-68366.rs:17:6
   |
LL | impl<const N: usize> Foo {}
   |
   |
   = note: expressions using a const parameter must map each value to a distinct output value
   = note: proving the result of expressions other than the parameter are unique is not supported
help: Either remove the type parameter 'N' or make use of it, for example ` for S<N>`.
   |
LL - impl<const N: usize> Foo {}
LL + impl<> Foo {}

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0207`.
For more information about this error, try `rustc --explain E0207`.
------------------------------------------


---- [ui] tests/ui/const-generics/issues/issue-68366.rs#min stdout ----
diff of stderr:

15    |
16    = note: expressions using a const parameter must map each value to a distinct output value
17    = note: proving the result of expressions other than the parameter are unique is not supported
+ help: Either remove the type parameter 'N' or make use of it, for example ` for S<N>`.
+    |
+ LL - impl <const N: usize> Collatz<{Some(N)}> {}
+ LL + impl <> Collatz<{Some(N)}> {}
18 
18 
19 error[E0207]: the const parameter `N` is not constrained by the impl trait, self type, or predicates

24    |
24    |
25    = note: expressions using a const parameter must map each value to a distinct output value
26    = note: proving the result of expressions other than the parameter are unique is not supported
+ help: Either remove the type parameter 'N' or make use of it, for example ` for S<N>`.
+    |
+ LL - impl<const N: usize> Foo {}
+ LL + impl<> Foo {}
27 
28 error: aborting due to 3 previous errors
29 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-68366.min/issue-68366.min.stderr
To only update this specific test, also pass `--test-args const-generics/issues/issue-68366.rs`


error in revision `min`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-generics/issues/issue-68366.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-68366.min" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-68366.min/auxiliary"
stdout: none
--- stderr -------------------------------
error: generic parameters may not be used in const operations
  --> fake-test-src-base/const-generics/issues/issue-68366.rs:11:37
   |
LL | impl <const N: usize> Collatz<{Some(N)}> {}
   |                                     ^ cannot perform const operation using `N`
   |
   = help: const parameters may only be used as standalone arguments, i.e. `N`
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error[E0207]: the const parameter `N` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/const-generics/issues/issue-68366.rs:11:7
   |
LL | impl <const N: usize> Collatz<{Some(N)}> {}
   |
   |
   = note: expressions using a const parameter must map each value to a distinct output value
   = note: proving the result of expressions other than the parameter are unique is not supported
help: Either remove the type parameter 'N' or make use of it, for example ` for S<N>`.
   |
LL - impl <const N: usize> Collatz<{Some(N)}> {}
LL + impl <> Collatz<{Some(N)}> {}


error[E0207]: the const parameter `N` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/const-generics/issues/issue-68366.rs:17:6
   |
LL | impl<const N: usize> Foo {}
   |
   |
   = note: expressions using a const parameter must map each value to a distinct output value
   = note: proving the result of expressions other than the parameter are unique is not supported
help: Either remove the type parameter 'N' or make use of it, for example ` for S<N>`.
   |
LL - impl<const N: usize> Foo {}
LL + impl<> Foo {}

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0207`.
For more information about this error, try `rustc --explain E0207`.
------------------------------------------


---- [ui] tests/ui/duplicate/duplicate-type-parameter.rs stdout ----
diff of stderr:

59    |
60 LL | impl<T,T> Qux<T,T> for Option<T> {}
+    |
+    |
+ help: Either remove the type parameter 'T' or make use of it, for example ` for S<T>`.
+    |
+ LL - impl<T,T> Qux<T,T> for Option<T> {}
+ LL + impl<T,> Qux<T,T> for Option<T> {}
62 
63 error: aborting due to 8 previous errors
64 

---
To only update this specific test, also pass `--test-args duplicate/duplicate-type-parameter.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/duplicate/duplicate-type-parameter.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate/duplicate-type-parameter" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/duplicate/duplicate-type-parameter/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0403]: the name `T` is already used for a generic parameter in this item's generic parameters
  --> fake-test-src-base/duplicate/duplicate-type-parameter.rs:1:12
   |
LL | type Foo<T,T> = Option<T>;
   |          - ^ already used
   |          first use of `T`


error[E0403]: the name `T` is already used for a generic parameter in this item's generic parameters
  --> fake-test-src-base/duplicate/duplicate-type-parameter.rs:4:14
   |
LL | struct Bar<T,T>(T);
   |            - ^ already used
   |            first use of `T`


error[E0403]: the name `T` is already used for a generic parameter in this item's generic parameters
  --> fake-test-src-base/duplicate/duplicate-type-parameter.rs:7:14
   |
LL | struct Baz<T,T> {
   |            - ^ already used
   |            first use of `T`


error[E0403]: the name `T` is already used for a generic parameter in this item's generic parameters
  --> fake-test-src-base/duplicate/duplicate-type-parameter.rs:12:12
   |
LL | enum Boo<T,T> {
   |          - ^ already used
   |          first use of `T`


error[E0403]: the name `T` is already used for a generic parameter in this item's generic parameters
  --> fake-test-src-base/duplicate/duplicate-type-parameter.rs:18:11
   |
LL | fn quux<T,T>(x: T) {}
   |         - ^ already used
   |         first use of `T`


error[E0403]: the name `T` is already used for a generic parameter in this item's generic parameters
  --> fake-test-src-base/duplicate/duplicate-type-parameter.rs:21:13
   |
LL | trait Qux<T,T> {}
   |           - ^ already used
   |           first use of `T`


error[E0403]: the name `T` is already used for a generic parameter in this item's generic parameters
  --> fake-test-src-base/duplicate/duplicate-type-parameter.rs:24:8
   |
LL | impl<T,T> Qux<T,T> for Option<T> {}
   |      - ^ already used
   |      first use of `T`

error[E0207]: the type parameter `T` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/duplicate/duplicate-type-parameter.rs:24:8
  --> fake-test-src-base/duplicate/duplicate-type-parameter.rs:24:8
   |
LL | impl<T,T> Qux<T,T> for Option<T> {}
   |
   |
help: Either remove the type parameter 'T' or make use of it, for example ` for S<T>`.
   |
LL - impl<T,T> Qux<T,T> for Option<T> {}
LL + impl<T,> Qux<T,T> for Option<T> {}

error: aborting due to 8 previous errors

Some errors have detailed explanations: E0207, E0403.
---
---- [ui] tests/ui/error-codes/E0207.rs stdout ----
diff of stderr:

3    |
4 LL | impl<T: Default> Foo {
+    |
+    |
+ help: Either remove the type parameter 'T' or make use of it, for example ` for S<T>`.
+    |
+ LL - impl<T: Default> Foo {
+ LL + impl<: Default> Foo {
6 
7 error: aborting due to previous error
8 

---
To only update this specific test, also pass `--test-args error-codes/E0207.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/error-codes/E0207.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0207" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0207/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0207]: the type parameter `T` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/error-codes/E0207.rs:3:6
   |
LL | impl<T: Default> Foo { //~ ERROR E0207
   |
   |
help: Either remove the type parameter 'T' or make use of it, for example ` for S<T>`.
   |
LL - impl<T: Default> Foo { //~ ERROR E0207
LL + impl<: Default> Foo { //~ ERROR E0207

error: aborting due to previous error

For more information about this error, try `rustc --explain E0207`.
For more information about this error, try `rustc --explain E0207`.
------------------------------------------


---- [ui] tests/ui/generic-associated-types/bugs/issue-87735.rs stdout ----
diff of stderr:

3    |
4 LL | impl<'b, T, U> AsRef2 for Foo<T>
+    |
+    |
+ help: Either remove the type parameter 'U' or make use of it, for example ` for S<U>`.
+    |
+ LL - impl<'b, T, U> AsRef2 for Foo<T>
+ LL + impl<'b, T, > AsRef2 for Foo<T>
6 
7 error: aborting due to previous error
8 

---
To only update this specific test, also pass `--test-args generic-associated-types/bugs/issue-87735.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/generic-associated-types/bugs/issue-87735.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-87735" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-87735/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0207]: the type parameter `U` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/generic-associated-types/bugs/issue-87735.rs:25:13
   |
LL | impl<'b, T, U> AsRef2 for Foo<T>
   |
   |
help: Either remove the type parameter 'U' or make use of it, for example ` for S<U>`.
   |
LL - impl<'b, T, U> AsRef2 for Foo<T>
LL + impl<'b, T, > AsRef2 for Foo<T>

error: aborting due to previous error

For more information about this error, try `rustc --explain E0207`.
For more information about this error, try `rustc --explain E0207`.
------------------------------------------


---- [ui] tests/ui/generic-associated-types/bugs/issue-88526.rs stdout ----
diff of stderr:

3    |
4 LL | impl<'q, Q, I, F> A for TestB<Q, F>
+    |
+    |
+ help: Either remove the type parameter 'I' or make use of it, for example ` for S<I>`.
+    |
+ LL - impl<'q, Q, I, F> A for TestB<Q, F>
+ LL + impl<'q, Q, , F> A for TestB<Q, F>
6 
7 error: aborting due to previous error
8 

---
To only update this specific test, also pass `--test-args generic-associated-types/bugs/issue-88526.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/generic-associated-types/bugs/issue-88526.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-88526" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-88526/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0207]: the type parameter `I` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/generic-associated-types/bugs/issue-88526.rs:25:13
   |
LL | impl<'q, Q, I, F> A for TestB<Q, F>
   |
   |
help: Either remove the type parameter 'I' or make use of it, for example ` for S<I>`.
   |
LL - impl<'q, Q, I, F> A for TestB<Q, F>
LL + impl<'q, Q, , F> A for TestB<Q, F>

error: aborting due to previous error

For more information about this error, try `rustc --explain E0207`.
For more information about this error, try `rustc --explain E0207`.
------------------------------------------


---- [ui] tests/ui/generic-associated-types/gat-trait-path-generic-type-arg.rs stdout ----
diff of stderr:

18    |
19 LL | impl <T, T1> Foo for T {
20    |          ^^ unconstrained type parameter
+    |
+ help: Either remove the type parameter 'T1' or make use of it, for example ` for S<T1>`.
+    |
+ LL - impl <T, T1> Foo for T {
+ LL + impl <T, > Foo for T {
21 
22 error: aborting due to 3 previous errors
23 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/gat-trait-path-generic-type-arg/gat-trait-path-generic-type-arg.stderr
To only update this specific test, also pass `--test-args generic-associated-types/gat-trait-path-generic-type-arg.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/generic-associated-types/gat-trait-path-generic-type-arg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/gat-trait-path-generic-type-arg" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/gat-trait-path-generic-type-arg/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0403]: the name `T1` is already used for a generic parameter in this item's generic parameters
  --> fake-test-src-base/generic-associated-types/gat-trait-path-generic-type-arg.rs:9:12
   |
LL | impl <T, T1> Foo for T {
   |          -- first use of `T1`
LL |     //~^ ERROR: the type parameter `T1` is not constrained
LL |     type F<T1> = &[u8];
   |            ^^ already used

error[E0637]: `&` without an explicit lifetime name cannot be used here
  --> fake-test-src-base/generic-associated-types/gat-trait-path-generic-type-arg.rs:9:18
   |
LL |     type F<T1> = &[u8];
   |                  ^ explicit lifetime name needed here
error[E0207]: the type parameter `T1` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/generic-associated-types/gat-trait-path-generic-type-arg.rs:7:10
   |
   |
LL | impl <T, T1> Foo for T {
   |          ^^ unconstrained type parameter
   |
help: Either remove the type parameter 'T1' or make use of it, for example ` for S<T1>`.
   |
LL - impl <T, T1> Foo for T {
LL + impl <T, > Foo for T {

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0207, E0403, E0637.
---
---- [ui] tests/ui/impl-trait/issues/issue-87340.rs stdout ----
diff of stderr:

3    |
4 LL | impl<T> X for () {
+    |
+    |
+ help: Either remove the type parameter 'T' or make use of it, for example ` for S<T>`.
+    |
+ LL - impl<T> X for () {
+ LL + impl<> X for () {
6 
7 error: aborting due to previous error
8 

---
To only update this specific test, also pass `--test-args impl-trait/issues/issue-87340.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/impl-trait/issues/issue-87340.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-87340" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-87340/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0207]: the type parameter `T` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/impl-trait/issues/issue-87340.rs:8:6
   |
LL | impl<T> X for () {
   |
   |
help: Either remove the type parameter 'T' or make use of it, for example ` for S<T>`.
   |
LL - impl<T> X for () {
LL + impl<> X for () {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0207`.
For more information about this error, try `rustc --explain E0207`.
------------------------------------------


---- [ui] tests/ui/impl-unused-tps-inherent.rs stdout ----
diff of stderr:

3    |
4 LL | impl<T> MyType {
+    |
+    |
+ help: Either remove the type parameter 'T' or make use of it, for example ` for S<T>`.
+    |
+ LL - impl<T> MyType {
+ LL + impl<> MyType {
6 
7 error[E0207]: the type parameter `U` is not constrained by the impl trait, self type, or predicates
8   --> $DIR/impl-unused-tps-inherent.rs:17:8


9    |
10 LL | impl<T,U> MyType1<T> {
+    |
+    |
+ help: Either remove the type parameter 'U' or make use of it, for example ` for S<U>`.
+    |
+ LL - impl<T,U> MyType1<T> {
+ LL + impl<T,> MyType1<T> {
12 
13 error: aborting due to 2 previous errors
14 

---
To only update this specific test, also pass `--test-args impl-unused-tps-inherent.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/impl-unused-tps-inherent.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-unused-tps-inherent" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-unused-tps-inherent/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0207]: the type parameter `T` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/impl-unused-tps-inherent.rs:9:6
   |
LL | impl<T> MyType {
   |
   |
help: Either remove the type parameter 'T' or make use of it, for example ` for S<T>`.
   |
LL - impl<T> MyType {
LL + impl<> MyType {

error[E0207]: the type parameter `U` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/impl-unused-tps-inherent.rs:17:8
   |
   |
LL | impl<T,U> MyType1<T> {
   |
   |
help: Either remove the type parameter 'U' or make use of it, for example ` for S<U>`.
   |
LL - impl<T,U> MyType1<T> {
LL + impl<T,> MyType1<T> {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0207`.
For more information about this error, try `rustc --explain E0207`.
------------------------------------------


---- [ui] tests/ui/impl-unused-rps-in-assoc-type.rs stdout ----
diff of stderr:

3    |
4 LL | impl<'a> Fun for Holder {
5    |      ^^ unconstrained lifetime parameter
+    |
+ help: Either remove the type parameter ''a' or make use of it, for example ` for S<'a>`.
+    |
+ LL - impl<'a> Fun for Holder {
+ LL + impl<> Fun for Holder {
6 
7 error: aborting due to previous error
8 

---
To only update this specific test, also pass `--test-args impl-unused-rps-in-assoc-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/impl-unused-rps-in-assoc-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-unused-rps-in-assoc-type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-unused-rps-in-assoc-type/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0207]: the lifetime parameter `'a` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/impl-unused-rps-in-assoc-type.rs:11:6
   |
LL | impl<'a> Fun for Holder { //~ ERROR E0207
   |      ^^ unconstrained lifetime parameter
   |
help: Either remove the type parameter ''a' or make use of it, for example ` for S<'a>`.
   |
LL - impl<'a> Fun for Holder { //~ ERROR E0207
LL + impl<> Fun for Holder { //~ ERROR E0207

error: aborting due to previous error

For more information about this error, try `rustc --explain E0207`.
For more information about this error, try `rustc --explain E0207`.
------------------------------------------


---- [ui] tests/ui/impl-unused-tps.rs stdout ----
diff of stderr:

3    |
4 LL | impl<T,U> Foo<T> for [isize;1] {
+    |
+    |
+ help: Either remove the type parameter 'U' or make use of it, for example ` for S<U>`.
+    |
+ LL - impl<T,U> Foo<T> for [isize;1] {
+ LL + impl<T,> Foo<T> for [isize;1] {
6 
7 error[E0207]: the type parameter `U` is not constrained by the impl trait, self type, or predicates
8   --> $DIR/impl-unused-tps.rs:30:8


9    |
10 LL | impl<T,U> Bar for T {
+    |
+    |
+ help: Either remove the type parameter 'U' or make use of it, for example ` for S<U>`.
+    |
+ LL - impl<T,U> Bar for T {
+ LL + impl<T,> Bar for T {
12 
13 error[E0207]: the type parameter `U` is not constrained by the impl trait, self type, or predicates
14   --> $DIR/impl-unused-tps.rs:38:8


15    |
16 LL | impl<T,U> Bar for T
+    |
+    |
+ help: Either remove the type parameter 'U' or make use of it, for example ` for S<U>`.
+    |
+ LL - impl<T,U> Bar for T
+ LL + impl<T,> Bar for T
18 
19 error[E0207]: the type parameter `U` is not constrained by the impl trait, self type, or predicates
20   --> $DIR/impl-unused-tps.rs:46:8


21    |
22 LL | impl<T,U,V> Foo<T> for T
+    |
+    |
+ help: Either remove the type parameter 'U' or make use of it, for example ` for S<U>`.
+    |
+ LL - impl<T,U,V> Foo<T> for T
+ LL + impl<T,,V> Foo<T> for T
24 
24 
25 error[E0207]: the type parameter `V` is not constrained by the impl trait, self type, or predicates

27    |
27    |
28 LL | impl<T,U,V> Foo<T> for T
+    |
+    |
+ help: Either remove the type parameter 'V' or make use of it, for example ` for S<V>`.
+    |
+ LL - impl<T,U,V> Foo<T> for T
+ LL + impl<T,U,> Foo<T> for T
30 
31 error: aborting due to 5 previous errors
32 

---
To only update this specific test, also pass `--test-args impl-unused-tps.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/impl-unused-tps.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-unused-tps" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-unused-tps/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0207]: the type parameter `U` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/impl-unused-tps.rs:13:8
   |
LL | impl<T,U> Foo<T> for [isize;1] {
   |
   |
help: Either remove the type parameter 'U' or make use of it, for example ` for S<U>`.
   |
LL - impl<T,U> Foo<T> for [isize;1] {
LL + impl<T,> Foo<T> for [isize;1] {

error[E0207]: the type parameter `U` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/impl-unused-tps.rs:30:8
   |
   |
LL | impl<T,U> Bar for T {
   |
   |
help: Either remove the type parameter 'U' or make use of it, for example ` for S<U>`.
   |
LL - impl<T,U> Bar for T {
LL + impl<T,> Bar for T {

error[E0207]: the type parameter `U` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/impl-unused-tps.rs:38:8
   |
   |
LL | impl<T,U> Bar for T
   |
   |
help: Either remove the type parameter 'U' or make use of it, for example ` for S<U>`.
   |
LL - impl<T,U> Bar for T
LL + impl<T,> Bar for T

error[E0207]: the type parameter `U` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/impl-unused-tps.rs:46:8
   |
   |
LL | impl<T,U,V> Foo<T> for T
   |
   |
help: Either remove the type parameter 'U' or make use of it, for example ` for S<U>`.
   |
LL - impl<T,U,V> Foo<T> for T
LL + impl<T,,V> Foo<T> for T


error[E0207]: the type parameter `V` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/impl-unused-tps.rs:46:10
   |
LL | impl<T,U,V> Foo<T> for T
   |
   |
help: Either remove the type parameter 'V' or make use of it, for example ` for S<V>`.
   |
LL - impl<T,U,V> Foo<T> for T
LL + impl<T,U,> Foo<T> for T

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0207`.
For more information about this error, try `rustc --explain E0207`.
------------------------------------------


---- [ui] tests/ui/issues/issue-16562.rs stdout ----
diff of stderr:

3    |
4 LL | impl<T, M: MatrixShape> Collection for Col<M, usize> {
+    |
+    |
+ help: Either remove the type parameter 'T' or make use of it, for example ` for S<T>`.
+    |
+ LL - impl<T, M: MatrixShape> Collection for Col<M, usize> {
+ LL + impl<, M: MatrixShape> Collection for Col<M, usize> {
6 
7 error: aborting due to previous error
8 

---
To only update this specific test, also pass `--test-args issues/issue-16562.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/issues/issue-16562.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16562" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16562/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0207]: the type parameter `T` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/issues/issue-16562.rs:10:6
   |
LL | impl<T, M: MatrixShape> Collection for Col<M, usize> {
   |
   |
help: Either remove the type parameter 'T' or make use of it, for example ` for S<T>`.
   |
LL - impl<T, M: MatrixShape> Collection for Col<M, usize> {
LL + impl<, M: MatrixShape> Collection for Col<M, usize> {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0207`.
For more information about this error, try `rustc --explain E0207`.
------------------------------------------


---- [ui] tests/ui/issues/issue-22886.rs stdout ----
diff of stderr:

3    |
4 LL | impl<'a> Iterator for Newtype {
5    |      ^^ unconstrained lifetime parameter
+    |
+ help: Either remove the type parameter ''a' or make use of it, for example ` for S<'a>`.
+    |
+ LL - impl<'a> Iterator for Newtype {
+ LL + impl<> Iterator for Newtype {
6 
7 error: aborting due to previous error
8 

---
To only update this specific test, also pass `--test-args issues/issue-22886.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/issues/issue-22886.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22886" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22886/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0207]: the lifetime parameter `'a` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/issues/issue-22886.rs:13:6
   |
LL | impl<'a> Iterator for Newtype { //~ ERROR E0207
   |      ^^ unconstrained lifetime parameter
   |
help: Either remove the type parameter ''a' or make use of it, for example ` for S<'a>`.
   |
LL - impl<'a> Iterator for Newtype { //~ ERROR E0207
LL + impl<> Iterator for Newtype { //~ ERROR E0207

error: aborting due to previous error

For more information about this error, try `rustc --explain E0207`.
For more information about this error, try `rustc --explain E0207`.
------------------------------------------


---- [ui] tests/ui/issues/issue-26262.rs stdout ----
diff of stderr:

3    |
4 LL | impl<T: Tr> S<T::Assoc> {
+    |
+    |
+ help: Either remove the type parameter 'T' or make use of it, for example ` for S<T>`.
+    |
+ LL - impl<T: Tr> S<T::Assoc> {
+ LL + impl<: Tr> S<T::Assoc> {
6 
6 
7 error[E0207]: the lifetime parameter `'a` is not constrained by the impl trait, self type, or predicates

9    |
9    |
10 LL | impl<'a,T: Trait2<'a>> Trait1<<T as Trait2<'a>>::Foo> for T {
11    |      ^^ unconstrained lifetime parameter
+    |
+ help: Either remove the type parameter ''a' or make use of it, for example ` for S<'a>`.
+    |
+ LL - impl<'a,T: Trait2<'a>> Trait1<<T as Trait2<'a>>::Foo> for T {
+ LL + impl<,T: Trait2<'a>> Trait1<<T as Trait2<'a>>::Foo> for T {
12 
13 error: aborting due to 2 previous errors
14 

---
To only update this specific test, also pass `--test-args issues/issue-26262.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/issues/issue-26262.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26262" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26262/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0207]: the type parameter `T` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/issues/issue-26262.rs:7:6
   |
LL | impl<T: Tr> S<T::Assoc> {
   |
   |
help: Either remove the type parameter 'T' or make use of it, for example ` for S<T>`.
   |
LL - impl<T: Tr> S<T::Assoc> {
LL + impl<: Tr> S<T::Assoc> {


error[E0207]: the lifetime parameter `'a` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/issues/issue-26262.rs:17:6
   |
LL | impl<'a,T: Trait2<'a>> Trait1<<T as Trait2<'a>>::Foo> for T {
   |      ^^ unconstrained lifetime parameter
   |
help: Either remove the type parameter ''a' or make use of it, for example ` for S<'a>`.
   |
LL - impl<'a,T: Trait2<'a>> Trait1<<T as Trait2<'a>>::Foo> for T {
LL + impl<,T: Trait2<'a>> Trait1<<T as Trait2<'a>>::Foo> for T {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0207`.
For more information about this error, try `rustc --explain E0207`.
------------------------------------------


---- [ui] tests/ui/issues/issue-29861.rs stdout ----
diff of stderr:

3    |
4 LL | impl<'a, T: 'a> MakeRef2 for T {
5    |      ^^ unconstrained lifetime parameter
+    |
+ help: Either remove the type parameter ''a' or make use of it, for example ` for S<'a>`.
+    |
+ LL - impl<'a, T: 'a> MakeRef2 for T {
+ LL + impl<, T: 'a> MakeRef2 for T {
6 
7 error: aborting due to previous error
8 

---
To only update this specific test, also pass `--test-args issues/issue-29861.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/issues/issue-29861.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-29861" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-29861/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0207]: the lifetime parameter `'a` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/issues/issue-29861.rs:11:6
   |
LL | impl<'a, T: 'a> MakeRef2 for T {
   |      ^^ unconstrained lifetime parameter
   |
help: Either remove the type parameter ''a' or make use of it, for example ` for S<'a>`.
   |
LL - impl<'a, T: 'a> MakeRef2 for T {
LL + impl<, T: 'a> MakeRef2 for T {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0207`.
For more information about this error, try `rustc --explain E0207`.
------------------------------------------


---- [ui] tests/ui/issues/issue-35139.rs stdout ----
diff of stderr:

3    |
4 LL | impl<'a> MethodType for MTFn {
5    |      ^^ unconstrained lifetime parameter
+    |
+ help: Either remove the type parameter ''a' or make use of it, for example ` for S<'a>`.
+    |
+ LL - impl<'a> MethodType for MTFn {
+ LL + impl<> MethodType for MTFn {
6 
7 error: aborting due to previous error
8 

---
To only update this specific test, also pass `--test-args issues/issue-35139.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/issues/issue-35139.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35139" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35139/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0207]: the lifetime parameter `'a` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/issues/issue-35139.rs:9:6
   |
LL | impl<'a> MethodType for MTFn { //~ ERROR E0207
   |      ^^ unconstrained lifetime parameter
   |
help: Either remove the type parameter ''a' or make use of it, for example ` for S<'a>`.
   |
LL - impl<'a> MethodType for MTFn { //~ ERROR E0207
LL + impl<> MethodType for MTFn { //~ ERROR E0207

error: aborting due to previous error

For more information about this error, try `rustc --explain E0207`.
For more information about this error, try `rustc --explain E0207`.
------------------------------------------


---- [ui] tests/ui/type-alias-impl-trait/assoc-type-lifetime-unconstrained.rs stdout ----
diff of stderr:

3    |
4 LL | impl<'a, I> UnwrapItemsExt for I {
5    |      ^^ unconstrained lifetime parameter
+    |
+ help: Either remove the type parameter ''a' or make use of it, for example ` for S<'a>`.
+    |
+ LL - impl<'a, I> UnwrapItemsExt for I {
+ LL + impl<, I> UnwrapItemsExt for I {
6 
7 error: aborting due to previous error
8 

---
To only update this specific test, also pass `--test-args type-alias-impl-trait/assoc-type-lifetime-unconstrained.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/type-alias-impl-trait/assoc-type-lifetime-unconstrained.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/assoc-type-lifetime-unconstrained" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/assoc-type-lifetime-unconstrained/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0207]: the lifetime parameter `'a` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/type-alias-impl-trait/assoc-type-lifetime-unconstrained.rs:17:6
   |
LL | impl<'a, I> UnwrapItemsExt for I {
   |      ^^ unconstrained lifetime parameter
   |
help: Either remove the type parameter ''a' or make use of it, for example ` for S<'a>`.
   |
LL - impl<'a, I> UnwrapItemsExt for I {
LL + impl<, I> UnwrapItemsExt for I {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0207`.
For more information about this error, try `rustc --explain E0207`.
------------------------------------------


---- [ui] tests/ui/type-alias-impl-trait/impl-with-unconstrained-param.rs stdout ----
diff of stderr:

3    |
4 LL | impl<T> X for () {
+    |
+    |
+ help: Either remove the type parameter 'T' or make use of it, for example ` for S<T>`.
+    |
+ LL - impl<T> X for () {
+ LL + impl<> X for () {
6 
7 error: aborting due to previous error
8 

---
To only update this specific test, also pass `--test-args type-alias-impl-trait/impl-with-unconstrained-param.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/type-alias-impl-trait/impl-with-unconstrained-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/impl-with-unconstrained-param" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/impl-with-unconstrained-param/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0207]: the type parameter `T` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/type-alias-impl-trait/impl-with-unconstrained-param.rs:11:6
   |
LL | impl<T> X for () {
   |
   |
help: Either remove the type parameter 'T' or make use of it, for example ` for S<T>`.
   |
LL - impl<T> X for () {
LL + impl<> X for () {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0207`.
---
3    |
4 LL | impl<T> Allocator for DefaultAllocator {
5    |      ^ unconstrained type parameter
+    |
+ help: Either remove the type parameter 'T' or make use of it, for example ` for S<T>`.
+    |
+ LL - impl<T> Allocator for DefaultAllocator {
+ LL + impl<> Allocator for DefaultAllocator {
6 
7 error: aborting due to previous error
8 

---
To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-74244.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/type-alias-impl-trait/issue-74244.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-74244" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-74244/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0207]: the type parameter `T` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/type-alias-impl-trait/issue-74244.rs:9:6
LL | impl<T> Allocator for DefaultAllocator {
   |      ^ unconstrained type parameter
   |
   |
help: Either remove the type parameter 'T' or make use of it, for example ` for S<T>`.
LL - impl<T> Allocator for DefaultAllocator {
LL + impl<> Allocator for DefaultAllocator {
   |

---
---- [ui] tests/ui/type-alias-impl-trait/issue-74761-2.rs stdout ----
diff of stderr:

3    |
4 LL | impl<'a, 'b> A for () {
5    |      ^^ unconstrained lifetime parameter
+    |
+ help: Either remove the type parameter ''a' or make use of it, for example ` for S<'a>`.
+    |
+ LL - impl<'a, 'b> A for () {
+ LL + impl<, 'b> A for () {
6 
6 
7 error[E0207]: the lifetime parameter `'b` is not constrained by the impl trait, self type, or predicates

9    |
9    |
10 LL | impl<'a, 'b> A for () {
11    |          ^^ unconstrained lifetime parameter
+    |
+ help: Either remove the type parameter ''b' or make use of it, for example ` for S<'b>`.
+    |
+ LL - impl<'a, 'b> A for () {
+ LL + impl<'a, > A for () {
12 
13 error: aborting due to 2 previous errors
14 

---
To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-74761-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/type-alias-impl-trait/issue-74761-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-74761-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-74761-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0207]: the lifetime parameter `'a` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/type-alias-impl-trait/issue-74761-2.rs:7:6
   |
LL | impl<'a, 'b> A for () {
   |      ^^ unconstrained lifetime parameter
   |
help: Either remove the type parameter ''a' or make use of it, for example ` for S<'a>`.
   |
LL - impl<'a, 'b> A for () {
LL + impl<, 'b> A for () {


error[E0207]: the lifetime parameter `'b` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/type-alias-impl-trait/issue-74761-2.rs:7:10
   |
LL | impl<'a, 'b> A for () {
   |          ^^ unconstrained lifetime parameter
   |
help: Either remove the type parameter ''b' or make use of it, for example ` for S<'b>`.
   |
LL - impl<'a, 'b> A for () {
LL + impl<'a, > A for () {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0207`.
For more information about this error, try `rustc --explain E0207`.
------------------------------------------


---- [ui] tests/ui/type-alias-impl-trait/issue-74761.rs stdout ----
diff of stderr:

3    |
4 LL | impl<'a, 'b> A for () {
5    |      ^^ unconstrained lifetime parameter
+    |
+ help: Either remove the type parameter ''a' or make use of it, for example ` for S<'a>`.
+    |
+ LL - impl<'a, 'b> A for () {
+ LL + impl<, 'b> A for () {
6 
6 
7 error[E0207]: the lifetime parameter `'b` is not constrained by the impl trait, self type, or predicates

9    |
9    |
10 LL | impl<'a, 'b> A for () {
11    |          ^^ unconstrained lifetime parameter
+    |
+ help: Either remove the type parameter ''b' or make use of it, for example ` for S<'b>`.
+    |
+ LL - impl<'a, 'b> A for () {
+ LL + impl<'a, > A for () {
12 
13 error: aborting due to 2 previous errors
14 

---
To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-74761.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/type-alias-impl-trait/issue-74761.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-74761" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-74761/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0207]: the lifetime parameter `'a` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/type-alias-impl-trait/issue-74761.rs:7:6
   |
LL | impl<'a, 'b> A for () {
   |      ^^ unconstrained lifetime parameter
   |
help: Either remove the type parameter ''a' or make use of it, for example ` for S<'a>`.
   |
LL - impl<'a, 'b> A for () {
LL + impl<, 'b> A for () {


error[E0207]: the lifetime parameter `'b` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/type-alias-impl-trait/issue-74761.rs:7:10
   |
LL | impl<'a, 'b> A for () {
   |          ^^ unconstrained lifetime parameter
   |
help: Either remove the type parameter ''b' or make use of it, for example ` for S<'b>`.
   |
LL - impl<'a, 'b> A for () {
LL + impl<'a, > A for () {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0207`.
For more information about this error, try `rustc --explain E0207`.
------------------------------------------


---- [ui] tests/ui/type-alias-impl-trait/type-alias-impl-trait-unconstrained-lifetime.rs stdout ----
diff of stderr:

3    |
4 LL | impl<'a, I: Iterator<Item = i32>> Trait for (i32, I) {
5    |      ^^ unconstrained lifetime parameter
+    |
+ help: Either remove the type parameter ''a' or make use of it, for example ` for S<'a>`.
+    |
+ LL - impl<'a, I: Iterator<Item = i32>> Trait for (i32, I) {
+ LL + impl<, I: Iterator<Item = i32>> Trait for (i32, I) {
6 
7 error: aborting due to previous error
8 

---
To only update this specific test, also pass `--test-args type-alias-impl-trait/type-alias-impl-trait-unconstrained-lifetime.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/type-alias-impl-trait/type-alias-impl-trait-unconstrained-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/type-alias-impl-trait-unconstrained-lifetime" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/type-alias-impl-trait-unconstrained-lifetime/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0207]: the lifetime parameter `'a` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/type-alias-impl-trait/type-alias-impl-trait-unconstrained-lifetime.rs:10:6
   |
LL | impl<'a, I: Iterator<Item = i32>> Trait for (i32, I) {
   |      ^^ unconstrained lifetime parameter
   |
help: Either remove the type parameter ''a' or make use of it, for example ` for S<'a>`.
   |
LL - impl<'a, I: Iterator<Item = i32>> Trait for (i32, I) {
LL + impl<, I: Iterator<Item = i32>> Trait for (i32, I) {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0207`.
For more information about this error, try `rustc --explain E0207`.
------------------------------------------


---- [ui] tests/ui/typeck/issue-13853-5.rs stdout ----
diff of stderr:

3    |
4 LL | impl<'a, T: Deserializable> Deserializable for &'a str {
+    |
+    |
+ help: Either remove the type parameter 'T' or make use of it, for example ` for S<T>`.
+    |
+ LL - impl<'a, T: Deserializable> Deserializable for &'a str {
+ LL + impl<'a, : Deserializable> Deserializable for &'a str {
6 
7 error: aborting due to previous error
8 

---
To only update this specific test, also pass `--test-args typeck/issue-13853-5.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/typeck/issue-13853-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-13853-5" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-13853-5/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0207]: the type parameter `T` is not constrained by the impl trait, self type, or predicates
  --> fake-test-src-base/typeck/issue-13853-5.rs:7:10
   |
LL | impl<'a, T: Deserializable> Deserializable for &'a str {
   |
   |
help: Either remove the type parameter 'T' or make use of it, for example ` for S<T>`.
   |
LL - impl<'a, T: Deserializable> Deserializable for &'a str {
LL + impl<'a, : Deserializable> Deserializable for &'a str {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0207`.
