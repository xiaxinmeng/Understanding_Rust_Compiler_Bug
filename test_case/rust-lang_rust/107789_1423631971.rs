plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:cb2b9920774a63bd54b3676f2b669ea1e777a91e)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
........................................................................................ 13288/14411
........................................................................................ 13376/14411
........................................................................................ 13464/14411
....................i................................................................... 13552/14411
..................................................................F.F................... 13640/14411
........................................................................................ 13816/14411
........................................................................................ 13904/14411
........................................................................................ 13992/14411
........................................................................................ 14080/14411
........................................................................................ 14080/14411
........................................................................................ 14168/14411
........................................................................................ 14256/14411
...........................................iii.......................................... 14344/14411
...................................................................
failures:

---- [ui] tests/ui/inference/need_type_info/issue-107745-avoid-expr-from-macro-expansion.rs stdout ----

1 error[E0282]: type annotations needed
1 error[E0282]: type annotations needed
-   --> $DIR/issue-107745-avoid-expr-from-macro-expansion.rs:15:22
+   --> $DIR/issue-107745-avoid-expr-from-macro-expansion.rs:17:22
4 LL |     println!("{:?}", []);
5    |                      ^^ cannot infer type



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/need_type_info/issue-107745-avoid-expr-from-macro-expansion/issue-107745-avoid-expr-from-macro-expansion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args inference/need_type_info/issue-107745-avoid-expr-from-macro-expansion.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/inference/need_type_info/issue-107745-avoid-expr-from-macro-expansion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/need_type_info/issue-107745-avoid-expr-from-macro-expansion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/need_type_info/issue-107745-avoid-expr-from-macro-expansion/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0282]: type annotations needed
  --> fake-test-src-base/inference/need_type_info/issue-107745-avoid-expr-from-macro-expansion.rs:17:22
LL |     println!("{:?}", []);
   |                      ^^ cannot infer type
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
---

---- [ui] tests/ui/parser/missing-closing-angle-bracket-eq-constraint.rs stdout ----
diff of stderr:

45 LL |   let v : Vec<(u32,_) = vec![];
47    |
47    |
- help: consider giving `v` an explicit type, where the type for type parameter `T` is specified
+ help: consider giving `v` an explicit type, where the placeholders `_` are specified
49    |
50 LL |   let v: Vec<T> : Vec<(u32,_) = vec![];


56 LL |   let v : Vec<'a = vec![];
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
58    |
58    |
- help: consider giving `v` an explicit type, where the type for type parameter `T` is specified
+ help: consider giving `v` an explicit type, where the placeholders `_` are specified
60    |
61 LL |   let v: Vec<T> : Vec<'a = vec![];


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/missing-closing-angle-bracket-eq-constraint/missing-closing-angle-bracket-eq-constraint.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/missing-closing-angle-bracket-eq-constraint/missing-closing-angle-bracket-eq-constraint.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/missing-closing-angle-bracket-eq-constraint.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/parser/missing-closing-angle-bracket-eq-constraint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/missing-closing-angle-bracket-eq-constraint" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/missing-closing-angle-bracket-eq-constraint/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected one of `,`, `:`, or `>`, found `=`
  --> fake-test-src-base/parser/missing-closing-angle-bracket-eq-constraint.rs:7:23
   |
LL |   let v : Vec<(u32,_) = vec![];
   |       -             - ^ expected one of `,`, `:`, or `>`
   |       |             |
   |       |             maybe try to close unmatched angle bracket
   |       while parsing the type for `v`
   |
help: you might have meant to end the type parameters here
   |
LL |   let v : Vec<(u32,_)> = vec![];


error: expected one of `!`, `(`, `+`, `,`, `::`, `<`, or `>`, found `{`
  --> fake-test-src-base/parser/missing-closing-angle-bracket-eq-constraint.rs:13:32
   |
LL |   let foo : Foo::<T1, T2 = Foo {_a : arg1, _b : arg2};
   |       ---                      ^ expected one of 7 possible tokens
   |       while parsing the type for `foo`
   |
   |
help: you might have meant to end the type parameters here
   |
LL |   let foo : Foo::<T1>, T2 = Foo {_a : arg1, _b : arg2};


error: expected one of `,`, `:`, or `>`, found `=`
  --> fake-test-src-base/parser/missing-closing-angle-bracket-eq-constraint.rs:18:18
   |
LL |   let v : Vec<'a = vec![];
   |       -       -- ^ expected one of `,`, `:`, or `>`
   |       |       |
   |       |       maybe try to close unmatched angle bracket
   |       while parsing the type for `v`
   |
help: you might have meant to end the type parameters here
   |
LL |   let v : Vec<'a> = vec![];

error[E0282]: type annotations needed for `Vec<T>`
  --> fake-test-src-base/parser/missing-closing-angle-bracket-eq-constraint.rs:7:7
   |
   |
LL |   let v : Vec<(u32,_) = vec![];
   |
   |
help: consider giving `v` an explicit type, where the placeholders `_` are specified
   |
LL |   let v: Vec<T> : Vec<(u32,_) = vec![];

error[E0282]: type annotations needed for `Vec<T>`
  --> fake-test-src-base/parser/missing-closing-angle-bracket-eq-constraint.rs:18:7
   |
   |
LL |   let v : Vec<'a = vec![];
   |
   |
help: consider giving `v` an explicit type, where the placeholders `_` are specified
   |
LL |   let v: Vec<T> : Vec<'a = vec![];

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0282`.
For more information about this error, try `rustc --explain E0282`.
------------------------------------------


---- [ui] tests/ui/type/type-check/cannot_infer_local_or_vec.rs stdout ----
diff of stderr:

4 LL |     let x = vec![];
6    |
6    |
- help: consider giving `x` an explicit type, where the type for type parameter `T` is specified
+ help: consider giving `x` an explicit type, where the placeholders `_` are specified
8    |
9 LL |     let x: Vec<T> = vec![];


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/cannot_infer_local_or_vec/cannot_infer_local_or_vec.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/cannot_infer_local_or_vec/cannot_infer_local_or_vec.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type/type-check/cannot_infer_local_or_vec.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/type/type-check/cannot_infer_local_or_vec.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/cannot_infer_local_or_vec" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/cannot_infer_local_or_vec/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0282]: type annotations needed for `Vec<T>`
  --> fake-test-src-base/type/type-check/cannot_infer_local_or_vec.rs:2:9
   |
LL |     let x = vec![];
   |
   |
help: consider giving `x` an explicit type, where the placeholders `_` are specified
   |
LL |     let x: Vec<T> = vec![];

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
For more information about this error, try `rustc --explain E0282`.
------------------------------------------


---- [ui] tests/ui/type/type-check/cannot_infer_local_or_vec_in_tuples.rs stdout ----
diff of stderr:

4 LL |     let (x, ) = (vec![], );
5    |         ^^^^^   ---------- type must be known at this point
6    |
- help: consider giving this pattern a type, where the type for type parameter `T` is specified
+ help: consider giving this pattern a type, where the placeholders `_` are specified
8    |
9 LL |     let (x, ): (Vec<T>,) = (vec![], );


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/cannot_infer_local_or_vec_in_tuples/cannot_infer_local_or_vec_in_tuples.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/cannot_infer_local_or_vec_in_tuples/cannot_infer_local_or_vec_in_tuples.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type/type-check/cannot_infer_local_or_vec_in_tuples.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/type/type-check/cannot_infer_local_or_vec_in_tuples.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/cannot_infer_local_or_vec_in_tuples" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-check/cannot_infer_local_or_vec_in_tuples/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0282]: type annotations needed for `(Vec<T>,)`
  --> fake-test-src-base/type/type-check/cannot_infer_local_or_vec_in_tuples.rs:2:9
   |
LL |     let (x, ) = (vec![], );
   |         ^^^^^   ---------- type must be known at this point
   |
help: consider giving this pattern a type, where the placeholders `_` are specified
   |
LL |     let (x, ): (Vec<T>,) = (vec![], );

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
