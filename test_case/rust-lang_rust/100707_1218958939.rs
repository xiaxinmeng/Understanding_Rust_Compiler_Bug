plain
........................................................................................ 1144/13394
........................................................................................ 1232/13394
..................................................i..................................... 1320/13394
........................................................................................ 1408/13394
....................i....................................................F.F............ 1496/13394
........................................................................................ 1672/13394
..........................................................i......i.i.................... 1760/13394
........................................................................................ 1848/13394
........................................................................................ 1936/13394
---

---- [ui] src/test/ui/check-cfg/invalid-arguments.rs#names_simple_ident stdout ----
diff of stderr:

- error: invalid `--check-cfg` argument: `names("NOT_IDENT")` (`names()` arguments must be simple identifers)
+ error: invalid `--check-cfg` argument: `names("NOT_IDENT")` (`names()` arguments must be simple identifiers)
3 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/invalid-arguments.names_simple_ident/invalid-arguments.names_simple_ident.stderr
To only update this specific test, also pass `--test-args check-cfg/invalid-arguments.rs`


error in revision `names_simple_ident`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/check-cfg/invalid-arguments.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "names_simple_ident" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/invalid-arguments.names_simple_ident" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "--check-cfg=names(\"NOT_IDENT\")" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/invalid-arguments.names_simple_ident/auxiliary"
stdout: none
--- stderr -------------------------------
error: invalid `--check-cfg` argument: `names("NOT_IDENT")` (`names()` arguments must be simple identifiers)


---- [ui] src/test/ui/check-cfg/invalid-arguments.rs#values_simple_ident stdout ----
diff of stderr:
diff of stderr:

- error: invalid `--check-cfg` argument: `values("NOT_IDENT")` (`values()` first argument must be a simple identifer)
+ error: invalid `--check-cfg` argument: `values("NOT_IDENT")` (`values()` first argument must be a simple identifier)
3 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/invalid-arguments.values_simple_ident/invalid-arguments.values_simple_ident.stderr
To only update this specific test, also pass `--test-args check-cfg/invalid-arguments.rs`


error in revision `values_simple_ident`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/check-cfg/invalid-arguments.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "values_simple_ident" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/invalid-arguments.values_simple_ident" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "--check-cfg=values(\"NOT_IDENT\")" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-cfg/invalid-arguments.values_simple_ident/auxiliary"
stdout: none
--- stderr -------------------------------
error: invalid `--check-cfg` argument: `values("NOT_IDENT")` (`values()` first argument must be a simple identifier)


---- [ui] src/test/ui/proc-macro/derive-bad.rs stdout ----
diff of stderr:
diff of stderr:

6    |
7    = note: this error originates in the derive macro `A` (in Nightly builds, run with -Z macro-backtrace for more info)
8 
- error: proc-macro derive produced unparseable tokens
+ error: proc-macro derive produced unparsable tokens
11    |
12 LL | #[derive(A)]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/derive-bad/derive-bad.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/derive-bad.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/derive-bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/derive-bad" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/derive-bad/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected `:`, found `}`
   |
LL | #[derive(A)]
   |          ^ expected `:`
   |
---
   |
LL | #[derive(A)]
   |          - previous definition of the type `A` here
...
LL | struct A; //~ ERROR the name `A` is defined multiple times
   | ^^^^^^^^^ `A` redefined here
   |
   = note: `A` must be defined only once in the type namespace of this module
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0428`.
------------------------------------------
---

6    |
7    = note: this error originates in the derive macro `MyTrait` (in Nightly builds, run with -Z macro-backtrace for more info)
8 
- error: proc-macro derive produced unparseable tokens
+ error: proc-macro derive produced unparsable tokens
11    |
11    |
12 LL | #[derive(MyTrait)]

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-91800/issue-91800.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/issue-91800.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/issue-91800.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-91800" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-91800/auxiliary"
stdout: none
--- stderr -------------------------------
error: macros that expand to items must be delimited with braces or followed by a semicolon
   |
   |
LL | #[derive(MyTrait)]
   |
   = note: this error originates in the derive macro `MyTrait` (in Nightly builds, run with -Z macro-backtrace for more info)

error: proc-macro derive produced unparsable tokens
error: proc-macro derive produced unparsable tokens
  --> /checkout/src/test/ui/proc-macro/issue-91800.rs:6:10
   |
LL | #[derive(MyTrait)]

error: 
  --> /checkout/src/test/ui/proc-macro/issue-91800.rs:6:10
   |
   |
LL | #[derive(MyTrait)]
   |
   = note: this error originates in the derive macro `MyTrait` (in Nightly builds, run with -Z macro-backtrace for more info)


error: macros that expand to items must be delimited with braces or followed by a semicolon
   |
LL | #[attribute_macro]
   | ^^^^^^^^^^^^^^^^^^
   |
---
   | ^^^^^^^^^^^^^^^^^^
   |
   = note: this error originates in the attribute macro `attribute_macro` (in Nightly builds, run with -Z macro-backtrace for more info)

error: macros that expand to items must be delimited with braces or followed by a semicolon
   |
   |
LL | fn_macro! {}
   |
   = note: this error originates in the macro `fn_macro` (in Nightly builds, run with -Z macro-backtrace for more info)

error: 
error: 
  --> /checkout/src/test/ui/proc-macro/issue-91800.rs:13:1
   |
LL | fn_macro! {}
   |
   = note: this error originates in the macro `fn_macro` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 7 previous errors
