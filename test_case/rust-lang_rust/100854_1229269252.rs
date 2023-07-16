plain
........................................................................................ 2640/13440
........................................................................................ 2728/13440
........................................................................................ 2816/13440
........................................................................................ 2904/13440
...F.F.................................................................................. 2992/13440
........................................................................................ 3168/13440
........................................................................................ 3256/13440
.......................................................iiii.i........................... 3344/13440
........................................................................................ 3432/13440
---
........................................................................................ 4840/13440
........................................................................................ 4928/13440
........................................................................................ 5016/13440
........................................................................................ 5104/13440
...........................................................................i.F.......... 5192/13440
........................................................................................ 5368/13440
........................................................................................ 5456/13440
........................................................................................ 5544/13440
........................................................................................ 5632/13440
---

---- [ui] src/test/ui/consts/issue-73976-polymorphic.rs stdout ----
diff of stderr:

- error: constant pattern depends on a generic parameter
+ error: constant pattern depends on generic parameter
3    |
3    |
4 LL |     matches!(GetTypeId::<T>::VALUE, GetTypeId::<T>::VALUE)
5    |                                     ^^^^^^^^^^^^^^^^^^^^^
6 
- error: constant pattern depends on a generic parameter
- error: constant pattern depends on a generic parameter
+ error: constant pattern depends on generic parameter
9    |
9    |
10 LL |     matches!(GetTypeNameLen::<T>::VALUE, GetTypeNameLen::<T>::VALUE)
11    |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^
12 
- error: constant pattern depends on a generic parameter
- error: constant pattern depends on a generic parameter
+ error: constant pattern depends on generic parameter
15    |
15    |
16 LL |     matches!(GetTypeId::<T>::VALUE, GetTypeId::<T>::VALUE)
17    |                                     ^^^^^^^^^^^^^^^^^^^^^
18 
- error: constant pattern depends on a generic parameter
- error: constant pattern depends on a generic parameter
+ error: constant pattern depends on generic parameter
21    |
21    |
22 LL |     matches!(GetTypeNameLen::<T>::VALUE, GetTypeNameLen::<T>::VALUE)

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-73976-polymorphic/issue-73976-polymorphic.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-73976-polymorphic.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-73976-polymorphic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-73976-polymorphic" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-73976-polymorphic/auxiliary"
stdout: none
--- stderr -------------------------------
error: constant pattern depends on generic parameter
   |
   |
LL |     matches!(GetTypeId::<T>::VALUE, GetTypeId::<T>::VALUE)


error: constant pattern depends on generic parameter
   |
   |
LL |     matches!(GetTypeNameLen::<T>::VALUE, GetTypeNameLen::<T>::VALUE)
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


error: constant pattern depends on generic parameter
   |
   |
LL |     matches!(GetTypeId::<T>::VALUE, GetTypeId::<T>::VALUE)


error: constant pattern depends on generic parameter
   |
   |
LL |     matches!(GetTypeNameLen::<T>::VALUE, GetTypeNameLen::<T>::VALUE)

error: aborting due to 4 previous errors
------------------------------------------



---- [ui] src/test/ui/consts/issue-79137-toogeneric.rs stdout ----
diff of stderr:

- error: constant pattern depends on a generic parameter
+ error: constant pattern depends on generic parameter
3    |
3    |
4 LL |     matches!(GetVariantCount::<T>::VALUE, GetVariantCount::<T>::VALUE)
5    |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^
6 
- error: constant pattern depends on a generic parameter
- error: constant pattern depends on a generic parameter
+ error: constant pattern depends on generic parameter
9    |
9    |
10 LL |     matches!(GetVariantCount::<T>::VALUE, GetVariantCount::<T>::VALUE)

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-79137-toogeneric/issue-79137-toogeneric.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-79137-toogeneric.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-79137-toogeneric.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-79137-toogeneric" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-79137-toogeneric/auxiliary"
stdout: none
--- stderr -------------------------------
error: constant pattern depends on generic parameter
   |
   |
LL |     matches!(GetVariantCount::<T>::VALUE, GetVariantCount::<T>::VALUE)


error: constant pattern depends on generic parameter
   |
   |
LL |     matches!(GetVariantCount::<T>::VALUE, GetVariantCount::<T>::VALUE)

error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/inline-const/const-match-pat-generic.rs stdout ----
diff of stderr:

- error: constant pattern depends on a generic parameter
+ error: constant pattern depends on generic parameter
3    |
3    |
4 LL |         const { V } => {},
5    |         ^^^^^^^^^^^
6 
- error: constant pattern depends on a generic parameter
- error: constant pattern depends on a generic parameter
+ error: constant pattern depends on generic parameter
9    |
9    |
10 LL |         const { f(V) } => {},
11    |         ^^^^^^^^^^^^^^
12 
- error: constant pattern depends on a generic parameter
- error: constant pattern depends on a generic parameter
+ error: constant pattern depends on generic parameter
15    |
15    |
16 LL |         const { V } => {},
17    |         ^^^^^^^^^^^
18 
- error: constant pattern depends on a generic parameter
- error: constant pattern depends on a generic parameter
+ error: constant pattern depends on generic parameter
21    |
21    |
22 LL |         const { f(V) } => {},

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-const/const-match-pat-generic/const-match-pat-generic.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args inline-const/const-match-pat-generic.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inline-const/const-match-pat-generic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-const/const-match-pat-generic" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-const/const-match-pat-generic/auxiliary"
stdout: none
--- stderr -------------------------------
error: constant pattern depends on generic parameter
   |
   |
LL |         const { V } => {},


error: constant pattern depends on generic parameter
   |
   |
LL |         const { f(V) } => {},


error: constant pattern depends on generic parameter
   |
   |
LL |         const { V } => {},


error: constant pattern depends on generic parameter
   |
   |
LL |         const { f(V) } => {},

error: aborting due to 4 previous errors
------------------------------------------

