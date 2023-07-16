plain
...............i.i.................ii...i............................................... 264/13234
........................................................................................ 352/13234
........................................................................................ 440/13234
........................................................................................ 528/13234
.......................................F.....F.......................................... 616/13234
.......................................F.................................F.............. 704/13234
.........................................................i....F......................... 792/13234
........................................................................................ 968/13234
........................................................................................ 1056/13234
........................................................................................ 1144/13234
........................................................................................ 1232/13234
---
......................................i..........i..........i........................... 3872/13234
........................................................................................ 3960/13234
....................................i................................................... 4048/13234
................................................................................i....... 4136/13234
....................F....F.............................................................. 4224/13234
........F............................................................................... 4400/13234
........................................................F............................... 4488/13234
........................................................................................ 4576/13234
.............................................................................F.......... 4664/13234
.............................................................................F.......... 4664/13234
........................................................................................ 4752/13234
........................................................................................ 4840/13234
...................................F.F.................................................. 4928/13234
........................................................................................ 5104/13234
.........................i.............................................................. 5192/13234
....i................................................................................... 5280/13234
........................................................................................ 5368/13234
---
........................................................................................ 6424/13234
.................................................................F...........i.......... 6512/13234
........................................................................................ 6600/13234
......................................................i................................. 6688/13234
.....................ii.iiF..F....i....i.F.............................................. 6776/13234
..............F.....................................................i....i.............. 6952/13234
...........................i..................i.............i........................... 7040/13234
.............................i.......................................................... 7128/13234
..............................................i......................................... 7216/13234
---
........................................................................................ 7656/13234
.............................................................ii......................... 7744/13234
........................................................................................ 7832/13234
........................................................................................ 7920/13234
.........................F....F......................................ii................i 8008/13234
........................................................................................ 8184/13234
........................................................................................ 8272/13234
........................................................................................ 8360/13234
........................................................................................ 8448/13234
---
.............................................iiiiii.i..iiiiii.i......................... 10824/13234
........................................................................................ 10912/13234
........................................................................................ 11000/13234
........................................................................................ 11088/13234
...........................F...F....F.F.............F................................... 11176/13234
........................................................................................ 11352/13234
........................................................................................ 11440/13234
....................................................................F................... 11528/13234
....................................................................F................... 11528/13234
...................F..................................................................F. 11616/13234
....................F..F..F.........................................................i... 11704/13234
........................................................................................ 11880/13234
........................................................................................ 11880/13234
.......................F...........................F.................................... 11968/13234
..........F...F.F....................................................................... 12056/13234
.........................F.............................................................. 12232/13234
........................................................................................ 12320/13234
..............................................................i......................... 12408/13234
................F....................................................................... 12496/13234
---
failures:

---- [ui] src/test/ui/associated-types/substs-ppaux.rs#verbose stdout ----

error in revision `verbose`: /checkout/src/test/ui/associated-types/substs-ppaux.rs:49: unexpected error: '49:5: 49:26: the trait bound `str: Foo<'_#0r, '_#1r, u8>` is not satisfied [E0277]'

error in revision `verbose`: /checkout/src/test/ui/associated-types/substs-ppaux.rs:49: expected error not found: the size for values of type

error in revision `verbose`: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/substs-ppaux.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "verbose" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/substs-ppaux.verbose" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "verbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/substs-ppaux.verbose/auxiliary"
    Error {
        line_num: 49,
        kind: Some(
            Error,
            Error,
        ),
        msg: "49:5: 49:26: the trait bound `str: Foo<'_#0r, '_#1r, u8>` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
---
        msg: "the size for values of type",
    },
]

thread '[ui] src/test/ui/associated-types/substs-ppaux.rs#verbose' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13

---- [ui] src/test/ui/associated-types/substs-ppaux.rs#normal stdout ----


error in revision `normal`: /checkout/src/test/ui/associated-types/substs-ppaux.rs:49: unexpected error: '49:5: 49:26: the trait bound `str: Foo<'_, '_, u8>` is not satisfied [E0277]'

error in revision `normal`: /checkout/src/test/ui/associated-types/substs-ppaux.rs:49: expected error not found: the size for values of type

error in revision `normal`: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/substs-ppaux.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "normal" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/substs-ppaux.normal" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/substs-ppaux.normal/auxiliary"
    Error {
        line_num: 49,
        kind: Some(
            Error,
            Error,
        ),
        msg: "49:5: 49:26: the trait bound `str: Foo<'_, '_, u8>` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
---
        msg: "the size for values of type",
    },
]

thread '[ui] src/test/ui/associated-types/substs-ppaux.rs#normal' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13
---- [ui] src/test/ui/async-await/issue-70594.rs stdout ----


error: /checkout/src/test/ui/async-await/issue-70594.rs:4: unexpected error: '4:11: 4:17: `()` cannot be converted to a future [E0277]'
error: /checkout/src/test/ui/async-await/issue-70594.rs:4: expected error not found: `()` is not a future

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-70594.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-70594" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-70594/auxiliary"
    Error {
        line_num: 4,
        kind: Some(
            Error,
            Error,
        ),
        msg: "4:11: 4:17: `()` cannot be converted to a future [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 4,
        kind: Some(
            Error,
        ),
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
        msg: "`()` is not a future",
]

thread '[ui] src/test/ui/async-await/issue-70594.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/async-await/issues/issue-62009-1.rs stdout ----

error: /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:12: unexpected error: '12:15: 12:21: `[closure@/checkout/src/test/ui/async-await/issues/issue-62009-1.rs:12:6: 12:9]` cannot be converted to a future [E0277]'
error: /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:12: expected error not found: is not a future

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-62009-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1/auxiliary"
    Error {
        line_num: 12,
        kind: Some(
            Error,
            Error,
        ),
        msg: "12:15: 12:21: `[closure@/checkout/src/test/ui/async-await/issues/issue-62009-1.rs:12:6: 12:9]` cannot be converted to a future [E0277]",
]

not found errors (from test file): [
    Error {
---
thread '[ui] src/test/ui/async-await/issues/issue-62009-1.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13

---- [ui] src/test/ui/async-await/unnecessary-await.rs stdout ----

error: /checkout/src/test/ui/async-await/unnecessary-await.rs:9: unexpected error: '9:10: 9:16: `()` cannot be converted to a future [E0277]'
error: /checkout/src/test/ui/async-await/unnecessary-await.rs:9: expected error not found: `()` is not a future

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/unnecessary-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/unnecessary-await" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/unnecessary-await/auxiliary"
    Error {
        line_num: 9,
        kind: Some(
            Error,
            Error,
        ),
        msg: "9:10: 9:16: `()` cannot be converted to a future [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 9,
        kind: Some(
            Error,
        ),
        msg: "`()` is not a future",
]

thread '[ui] src/test/ui/async-await/unnecessary-await.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-85848.rs stdout ----

error: /checkout/src/test/ui/const-generics/generic_const_exprs/issue-85848.rs:24: unexpected error: '24:29: 24:33: the trait bound `&C: Delegates<()>` is not satisfied [E0277]'

error: /checkout/src/test/ui/const-generics/generic_const_exprs/issue-85848.rs:24: expected error not found: the trait bound `(): _Contains<&C>` is not satisfied [E0277]
error: /checkout/src/test/ui/const-generics/generic_const_exprs/issue-85848.rs:24: expected error not found: unconstrained generic constant

error: /checkout/src/test/ui/const-generics/generic_const_exprs/issue-85848.rs:24: expected error not found: mismatched types [E0308]


error: 1 unexpected errors found, 3 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-85848.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-85848" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-85848/auxiliary"
    Error {
        line_num: 24,
        kind: Some(
            Error,
            Error,
        ),
        msg: "24:29: 24:33: the trait bound `&C: Delegates<()>` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 24,
        kind: Some(
            Error,
        ),
        msg: "the trait bound `(): _Contains<&C>` is not satisfied [E0277]",
    Error {
        line_num: 24,
        kind: Some(
            Error,
---
thread '[ui] src/test/ui/const-generics/generic_const_exprs/issue-85848.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13

---- [ui] src/test/ui/for/for-c-in-str.rs stdout ----

error: /checkout/src/test/ui/for/for-c-in-str.rs:4: unexpected error: '4:14: 4:20: `&str` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/for/for-c-in-str.rs:4: unexpected note: 'cannot implicitly convert `&str` to an iterator; try calling `.chars()` or `.bytes()`'

error: /checkout/src/test/ui/for/for-c-in-str.rs:4: unexpected help message: '4:14: 4:20: the trait `IntoIterator` is not implemented for `&str`'
error: /checkout/src/test/ui/for/for-c-in-str.rs:4: expected error not found: `&str` is not an iterator

error: /checkout/src/test/ui/for/for-c-in-str.rs:4: expected note not found: `&str` is not an iterator


error: /checkout/src/test/ui/for/for-c-in-str.rs:4: expected help message not found: the trait `Iterator` is not implemented for `&str`
error: /checkout/src/test/ui/for/for-c-in-str.rs:4: expected note not found: required because of the requirements on the impl of `IntoIterator` for `&str`

error: /checkout/src/test/ui/for/for-c-in-str.rs:4: expected note not found: in this expansion of desugaring of `for` loop


error: 3 unexpected errors found, 5 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/for/for-c-in-str.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for/for-c-in-str" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for/for-c-in-str/auxiliary"
    Error {
        line_num: 4,
        kind: Some(
            Error,
            Error,
        ),
        msg: "4:14: 4:20: `&str` cannot be converted to an iterator [E0277]",
    Error {
        line_num: 4,
        kind: Some(
            Note,
            Note,
        ),
        msg: "cannot implicitly convert `&str` to an iterator; try calling `.chars()` or `.bytes()`",
    Error {
        line_num: 4,
        kind: Some(
            Help,
            Help,
        ),
        msg: "4:14: 4:20: the trait `IntoIterator` is not implemented for `&str`",
]

not found errors (from test file): [
    Error {
---
        line_num: 4,
        kind: Some(
            Help,
        ),
        msg: "the trait `Iterator` is not implemented for `&str`",
    Error {
        line_num: 4,
        kind: Some(
            Note,
---
        line_num: 4,
        kind: Some(
            Note,
        ),
        msg: "in this expansion of desugaring of `for` loop",
]

thread '[ui] src/test/ui/for/for-c-in-str.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/for/for-loop-bogosity.rs stdout ----

error: /checkout/src/test/ui/for/for-loop-bogosity.rs:17: unexpected error: '17:14: 17:19: `MyStruct` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/for/for-loop-bogosity.rs:17: expected error not found: `MyStruct` is not an iterator
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/for/for-loop-bogosity.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for/for-loop-bogosity" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/for/for-loop-bogosity/auxiliary"
    Error {
        line_num: 17,
        kind: Some(
            Error,
            Error,
        ),
        msg: "17:14: 17:19: `MyStruct` cannot be converted to an iterator [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 17,
        kind: Some(
            Error,
        ),
        msg: "`MyStruct` is not an iterator",
]

thread '[ui] src/test/ui/for/for-loop-bogosity.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/generator/yield-outside-generator-issue-78653.rs stdout ----

error: /checkout/src/test/ui/generator/yield-outside-generator-issue-78653.rs:4: unexpected error: '4:23: 4:24: `{integer}` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/generator/yield-outside-generator-issue-78653.rs:4: expected error not found: `{integer}` is not an iterator
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/yield-outside-generator-issue-78653.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-outside-generator-issue-78653" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-outside-generator-issue-78653/auxiliary"
    Error {
        line_num: 4,
        kind: Some(
            Error,
            Error,
        ),
        msg: "4:23: 4:24: `{integer}` cannot be converted to an iterator [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 4,
        kind: Some(
            Error,
        ),
        msg: "`{integer}` is not an iterator",
]

thread '[ui] src/test/ui/generator/yield-outside-generator-issue-78653.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate.rs stdout ----

error: /checkout/src/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate.rs:30: unexpected error: '30:7: 30:8: no method named `f` found for struct `S` in the current scope [E0599]'

error: /checkout/src/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate.rs:30: unexpected note: 'method not found in `S`'

error: /checkout/src/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate.rs:10: unexpected note: '10:1: 10:8: `M` defines an item `f`, perhaps you need to implement it'

error: /checkout/src/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate.rs:14: expected note not found: trait bound `<S as X>::Y<i32> = i32` was not satisfied
error: /checkout/src/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate.rs:14: expected note not found: unsatisfied trait bound introduced here

error: /checkout/src/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate.rs:14: expected note not found: 


error: /checkout/src/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate.rs:14: expected note not found: 

error: /checkout/src/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate.rs:20: expected note not found: doesn't satisfy `<S as X>::Y<i32> = i32`

error: /checkout/src/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate.rs:20: expected note not found: doesn't satisfy `S: M`

error: /checkout/src/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate.rs:30: expected error not found: the method `f` exists for struct `S`, but its trait bounds were not satisfied
error: /checkout/src/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate.rs:30: expected note not found: method cannot be called on `S` due to unsatisfied trait bounds

error: 3 unexpected errors found, 8 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate/auxiliary"
    Error {
        line_num: 30,
        kind: Some(
            Error,
            Error,
        ),
        msg: "30:7: 30:8: no method named `f` found for struct `S` in the current scope [E0599]",
    Error {
        line_num: 30,
        kind: Some(
            Note,
---
        line_num: 10,
        kind: Some(
            Note,
        ),
        msg: "10:1: 10:8: `M` defines an item `f`, perhaps you need to implement it",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 14,
        kind: Some(
            Note,
        ),
        msg: "trait bound `<S as X>::Y<i32> = i32` was not satisfied",
    Error {
        line_num: 14,
        kind: Some(
            Note,
---
        line_num: 20,
        kind: Some(
            Note,
        ),
        msg: "doesn't satisfy `<S as X>::Y<i32> = i32`",
    Error {
        line_num: 20,
        kind: Some(
            Note,
            Note,
        ),
        msg: "doesn't satisfy `S: M`",
    Error {
        line_num: 30,
        kind: Some(
            Error,
            Error,
        ),
        msg: "the method `f` exists for struct `S`, but its trait bounds were not satisfied",
    Error {
        line_num: 30,
        kind: Some(
            Note,
            Note,
        ),
        msg: "method cannot be called on `S` due to unsatisfied trait bounds",
]

thread '[ui] src/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/hrtb/issue-30786.rs stdout ----

error: /checkout/src/test/ui/hrtb/issue-30786.rs:118: unexpected error: '118:22: 118:29: no method named `filterx` found for struct `Map` in the current scope [E0599]'

error: /checkout/src/test/ui/hrtb/issue-30786.rs:130: unexpected error: '130:24: 130:30: no method named `countx` found for struct `Filter` in the current scope [E0599]'
error: /checkout/src/test/ui/hrtb/issue-30786.rs:118: expected error not found: the method

error: /checkout/src/test/ui/hrtb/issue-30786.rs:130: expected error not found: the method


error: 2 unexpected errors found, 2 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hrtb/issue-30786.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-30786" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-30786/auxiliary"
    Error {
        line_num: 118,
        kind: Some(
            Error,
            Error,
        ),
        msg: "118:22: 118:29: no method named `filterx` found for struct `Map` in the current scope [E0599]",
    Error {
        line_num: 130,
        kind: Some(
            Error,
            Error,
        ),
        msg: "130:24: 130:30: no method named `countx` found for struct `Filter` in the current scope [E0599]",
]

not found errors (from test file): [
    Error {
---
thread '[ui] src/test/ui/hrtb/issue-30786.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13

---- [ui] src/test/ui/impl-trait/nested_impl_trait.rs stdout ----

error: /checkout/src/test/ui/impl-trait/nested_impl_trait.rs:5: unexpected error: '5:46: 5:67: the trait bound `impl Into<u32>: Into<impl Debug>` is not satisfied [E0277]'

error: /checkout/src/test/ui/impl-trait/nested_impl_trait.rs:18: unexpected error: '18:34: 18:55: the trait bound `impl Into<u32>: Into<impl Debug>` is not satisfied [E0277]'

error: /checkout/src/test/ui/impl-trait/nested_impl_trait.rs:5: expected error not found: the trait bound `impl Debug: From<impl Into<u32>>` is not satisfied

error: /checkout/src/test/ui/impl-trait/nested_impl_trait.rs:18: expected error not found: the trait bound `impl Debug: From<impl Into<u32>>` is not satisfied
error: 2 unexpected errors found, 2 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/nested_impl_trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested_impl_trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested_impl_trait/auxiliary"
    Error {
        line_num: 5,
        kind: Some(
            Error,
            Error,
        ),
        msg: "5:46: 5:67: the trait bound `impl Into<u32>: Into<impl Debug>` is not satisfied [E0277]",
    Error {
        line_num: 18,
        kind: Some(
            Error,
            Error,
        ),
        msg: "18:34: 18:55: the trait bound `impl Into<u32>: Into<impl Debug>` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 5,
        kind: Some(
            Error,
        ),
        msg: "the trait bound `impl Debug: From<impl Into<u32>>` is not satisfied",
    Error {
        line_num: 18,
        kind: Some(
            Error,
            Error,
        ),
        msg: "the trait bound `impl Debug: From<impl Into<u32>>` is not satisfied",
]

thread '[ui] src/test/ui/impl-trait/nested_impl_trait.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/impl-trait/projection-mismatch-in-impl-where-clause.rs stdout ----

error: /checkout/src/test/ui/impl-trait/projection-mismatch-in-impl-where-clause.rs:13: unexpected error: '13:14: 13:23: the trait bound `(): Test` is not satisfied [E0277]'

error: /checkout/src/test/ui/impl-trait/projection-mismatch-in-impl-where-clause.rs:13: expected error not found: type mismatch resolving `<() as Super>::Assoc == ()`
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/projection-mismatch-in-impl-where-clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/projection-mismatch-in-impl-where-clause" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/projection-mismatch-in-impl-where-clause/auxiliary"
    Error {
        line_num: 13,
        kind: Some(
            Error,
            Error,
        ),
        msg: "13:14: 13:23: the trait bound `(): Test` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 13,
        kind: Some(
            Error,
        ),
        msg: "type mismatch resolving `<() as Super>::Assoc == ()`",
]

thread '[ui] src/test/ui/impl-trait/projection-mismatch-in-impl-where-clause.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/issues/issue-20605.rs stdout ----

error: /checkout/src/test/ui/issues/issue-20605.rs:2: unexpected error: '2:17: 2:24: `dyn Iterator<Item = &'a mut u8>` cannot be converted to an iterator [E0277]'
error: /checkout/src/test/ui/issues/issue-20605.rs:2: expected error not found: the size for values of type

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-20605.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20605" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20605/auxiliary"
    Error {
        line_num: 2,
        kind: Some(
            Error,
            Error,
        ),
        msg: "2:17: 2:24: `dyn Iterator<Item = &'a mut u8>` cannot be converted to an iterator [E0277]",
]

not found errors (from test file): [
    Error {
---
thread '[ui] src/test/ui/issues/issue-20605.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13

---- [ui] src/test/ui/issues/issue-33941.rs stdout ----

error: /checkout/src/test/ui/issues/issue-33941.rs:6: unexpected error: '6:14: 6:44: `Cloned<std::collections::hash_map::Iter<'_, _, _>>` cannot be converted to an iterator [E0277]'
error: /checkout/src/test/ui/issues/issue-33941.rs:6: expected error not found: type mismatch

error: /checkout/src/test/ui/issues/issue-33941.rs:6: expected error not found: type mismatch


error: 1 unexpected errors found, 2 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-33941.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-33941" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zdeduplicate-diagnostics=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-33941/auxiliary"
    Error {
        line_num: 6,
        kind: Some(
            Error,
            Error,
        ),
        msg: "6:14: 6:44: `Cloned<std::collections::hash_map::Iter<'_, _, _>>` cannot be converted to an iterator [E0277]",
]

not found errors (from test file): [
    Error {
---
thread '[ui] src/test/ui/issues/issue-33941.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13

---- [ui] src/test/ui/issues/issue-38821.rs stdout ----

error: /checkout/src/test/ui/issues/issue-38821.rs:23: unexpected error: '23:17: 23:21: the trait bound `<Col as Expression>::SqlType: IntoNullable` is not satisfied [E0277]'

error: /checkout/src/test/ui/issues/issue-38821.rs:23: expected error not found: the trait bound `<Col as Expression>::SqlType: NotNull` is not satisfied
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-38821.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-38821" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-38821/auxiliary"
    Error {
        line_num: 23,
        kind: Some(
            Error,
            Error,
        ),
        msg: "23:17: 23:21: the trait bound `<Col as Expression>::SqlType: IntoNullable` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 23,
        kind: Some(
            Error,
        ),
        msg: "the trait bound `<Col as Expression>::SqlType: NotNull` is not satisfied",
]

thread '[ui] src/test/ui/issues/issue-38821.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/issues/issue-57843.rs stdout ----

error: /checkout/src/test/ui/issues/issue-57843.rs:25: unexpected error: '25:9: 25:25: the trait bound `for<'a> [closure@/checkout/src/test/ui/issues/issue-57843.rs:25:18: 25:21]: ClonableFn<&'a bool>` is not satisfied [E0277]'

error: /checkout/src/test/ui/issues/issue-57843.rs:25: expected error not found: implementation of `FnOnce` is not general enough
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-57843.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-57843" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-57843/auxiliary"
    Error {
        line_num: 25,
        kind: Some(
            Error,
            Error,
        ),
        msg: "25:9: 25:25: the trait bound `for<'a> [closure@/checkout/src/test/ui/issues/issue-57843.rs:25:18: 25:21]: ClonableFn<&'a bool>` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 25,
        kind: Some(
            Error,
        ),
        msg: "implementation of `FnOnce` is not general enough",
]

thread '[ui] src/test/ui/issues/issue-57843.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/iterators/integral.rs stdout ----

error: /checkout/src/test/ui/iterators/integral.rs:2: unexpected error: '2:14: 2:16: `{integer}` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/iterators/integral.rs:4: unexpected error: '4:14: 4:22: `u8` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/iterators/integral.rs:6: unexpected error: '6:14: 6:22: `i8` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/iterators/integral.rs:8: unexpected error: '8:14: 8:23: `u16` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/iterators/integral.rs:10: unexpected error: '10:14: 10:23: `i16` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/iterators/integral.rs:12: unexpected error: '12:14: 12:23: `u32` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/iterators/integral.rs:14: unexpected error: '14:14: 14:23: `i32` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/iterators/integral.rs:16: unexpected error: '16:14: 16:23: `u64` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/iterators/integral.rs:18: unexpected error: '18:14: 18:23: `i64` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/iterators/integral.rs:20: unexpected error: '20:14: 20:25: `usize` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/iterators/integral.rs:22: unexpected error: '22:14: 22:25: `isize` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/iterators/integral.rs:24: unexpected error: '24:14: 24:18: `{float}` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/iterators/integral.rs:2: expected error not found: `{integer}` is not an iterator
error: /checkout/src/test/ui/iterators/integral.rs:4: expected error not found: `u8` is not an iterator

error: /checkout/src/test/ui/iterators/integral.rs:6: expected error not found: `i8` is not an iterator

---
error: /checkout/src/test/ui/iterators/integral.rs:20: expected error not found: `usize` is not an iterator

error: /checkout/src/test/ui/iterators/integral.rs:22: expected error not found: `isize` is not an iterator

error: /checkout/src/test/ui/iterators/integral.rs:24: expected error not found: `{float}` is not an iterator
error: 12 unexpected errors found, 12 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/integral.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/integral" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/integral/auxiliary"
    Error {
        line_num: 2,
        kind: Some(
            Error,
            Error,
        ),
        msg: "2:14: 2:16: `{integer}` cannot be converted to an iterator [E0277]",
    Error {
        line_num: 4,
        kind: Some(
            Error,
            Error,
        ),
        msg: "4:14: 4:22: `u8` cannot be converted to an iterator [E0277]",
    Error {
        line_num: 6,
        kind: Some(
            Error,
            Error,
        ),
        msg: "6:14: 6:22: `i8` cannot be converted to an iterator [E0277]",
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "8:14: 8:23: `u16` cannot be converted to an iterator [E0277]",
    Error {
        line_num: 10,
        kind: Some(
            Error,
            Error,
        ),
        msg: "10:14: 10:23: `i16` cannot be converted to an iterator [E0277]",
    Error {
        line_num: 12,
        kind: Some(
            Error,
            Error,
        ),
        msg: "12:14: 12:23: `u32` cannot be converted to an iterator [E0277]",
    Error {
        line_num: 14,
        kind: Some(
            Error,
            Error,
        ),
        msg: "14:14: 14:23: `i32` cannot be converted to an iterator [E0277]",
    Error {
        line_num: 16,
        kind: Some(
            Error,
            Error,
        ),
        msg: "16:14: 16:23: `u64` cannot be converted to an iterator [E0277]",
    Error {
        line_num: 18,
        kind: Some(
            Error,
            Error,
        ),
        msg: "18:14: 18:23: `i64` cannot be converted to an iterator [E0277]",
    Error {
        line_num: 20,
        kind: Some(
            Error,
            Error,
        ),
        msg: "20:14: 20:25: `usize` cannot be converted to an iterator [E0277]",
    Error {
        line_num: 22,
        kind: Some(
            Error,
            Error,
        ),
        msg: "22:14: 22:25: `isize` cannot be converted to an iterator [E0277]",
    Error {
        line_num: 24,
        kind: Some(
            Error,
            Error,
        ),
        msg: "24:14: 24:18: `{float}` cannot be converted to an iterator [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 2,
        kind: Some(
            Error,
        ),
        msg: "`{integer}` is not an iterator",
    Error {
        line_num: 4,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`u8` is not an iterator",
    Error {
        line_num: 6,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`i8` is not an iterator",
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`u16` is not an iterator",
    Error {
        line_num: 10,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`i16` is not an iterator",
    Error {
        line_num: 12,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`u32` is not an iterator",
    Error {
        line_num: 14,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`i32` is not an iterator",
    Error {
        line_num: 16,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`u64` is not an iterator",
    Error {
        line_num: 18,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`i64` is not an iterator",
    Error {
        line_num: 20,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`usize` is not an iterator",
    Error {
        line_num: 22,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`isize` is not an iterator",
    Error {
        line_num: 24,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`{float}` is not an iterator",
]

thread '[ui] src/test/ui/iterators/integral.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/iterators/issue-28098.rs stdout ----

error: /checkout/src/test/ui/iterators/issue-28098.rs:6: unexpected error: '6:14: 6:19: `bool` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/iterators/issue-28098.rs:25: unexpected error: '25:14: 25:19: `bool` cannot be converted to an iterator [E0277]'
error: /checkout/src/test/ui/iterators/issue-28098.rs:6: expected error not found: `bool` is not an iterator

error: /checkout/src/test/ui/iterators/issue-28098.rs:25: expected error not found: `bool` is not an iterator


error: 2 unexpected errors found, 2 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/issue-28098.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/issue-28098" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/issue-28098/auxiliary"
    Error {
        line_num: 6,
        kind: Some(
            Error,
            Error,
        ),
        msg: "6:14: 6:19: `bool` cannot be converted to an iterator [E0277]",
    Error {
        line_num: 25,
        kind: Some(
            Error,
            Error,
        ),
        msg: "25:14: 25:19: `bool` cannot be converted to an iterator [E0277]",
]

not found errors (from test file): [
    Error {
---
thread '[ui] src/test/ui/iterators/issue-28098.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13

---- [ui] src/test/ui/iterators/string.rs stdout ----

error: /checkout/src/test/ui/iterators/string.rs:2: unexpected error: '2:14: 2:27: `String` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/iterators/string.rs:4: unexpected error: '4:14: 4:16: `&str` cannot be converted to an iterator [E0277]'
error: /checkout/src/test/ui/iterators/string.rs:2: expected error not found: `String` is not an iterator

error: /checkout/src/test/ui/iterators/string.rs:4: expected error not found: `&str` is not an iterator


error: 2 unexpected errors found, 2 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/string.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/string" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/string/auxiliary"
    Error {
        line_num: 2,
        kind: Some(
            Error,
            Error,
        ),
        msg: "2:14: 2:27: `String` cannot be converted to an iterator [E0277]",
    Error {
        line_num: 4,
        kind: Some(
            Error,
            Error,
        ),
        msg: "4:14: 4:16: `&str` cannot be converted to an iterator [E0277]",
]

not found errors (from test file): [
    Error {
---
thread '[ui] src/test/ui/iterators/string.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13

---- [ui] src/test/ui/lifetimes/issue-79187-2.rs stdout ----

error: /checkout/src/test/ui/lifetimes/issue-79187-2.rs:8: unexpected error: '8:5: 8:13: the trait bound `[closure@/checkout/src/test/ui/lifetimes/issue-79187-2.rs:8:14: 8:17]: Foo` is not satisfied [E0277]'

error: /checkout/src/test/ui/lifetimes/issue-79187-2.rs:11: unexpected error: '11:5: 11:13: the trait bound `[closure@/checkout/src/test/ui/lifetimes/issue-79187-2.rs:11:14: 11:23]: Foo` is not satisfied [E0277]'

error: /checkout/src/test/ui/lifetimes/issue-79187-2.rs:14: unexpected error: '14:5: 14:13: the trait bound `[closure@/checkout/src/test/ui/lifetimes/issue-79187-2.rs:14:14: 14:31]: Foo` is not satisfied [E0277]'

error: /checkout/src/test/ui/lifetimes/issue-79187-2.rs:8: expected error not found: implementation of `FnOnce` is not general enough
error: /checkout/src/test/ui/lifetimes/issue-79187-2.rs:8: expected error not found: mismatched types

error: /checkout/src/test/ui/lifetimes/issue-79187-2.rs:11: expected error not found: lifetime may not live long enough

---
error: /checkout/src/test/ui/lifetimes/issue-79187-2.rs:14: expected error not found: mismatched types

error: 3 unexpected errors found, 6 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/issue-79187-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-79187-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-79187-2/auxiliary"
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "8:5: 8:13: the trait bound `[closure@/checkout/src/test/ui/lifetimes/issue-79187-2.rs:8:14: 8:17]: Foo` is not satisfied [E0277]",
    Error {
        line_num: 11,
        kind: Some(
            Error,
            Error,
        ),
        msg: "11:5: 11:13: the trait bound `[closure@/checkout/src/test/ui/lifetimes/issue-79187-2.rs:11:14: 11:23]: Foo` is not satisfied [E0277]",
    Error {
        line_num: 14,
        kind: Some(
            Error,
            Error,
        ),
        msg: "14:5: 14:13: the trait bound `[closure@/checkout/src/test/ui/lifetimes/issue-79187-2.rs:14:14: 14:31]: Foo` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 8,
        kind: Some(
            Error,
        ),
        msg: "implementation of `FnOnce` is not general enough",
    Error {
        line_num: 8,
        kind: Some(
            Error,
---
        line_num: 11,
        kind: Some(
            Error,
        ),
        msg: "lifetime may not live long enough",
    Error {
        line_num: 11,
        kind: Some(
            Error,
---
        line_num: 14,
        kind: Some(
            Error,
        ),
        msg: "lifetime may not live long enough",
    Error {
        line_num: 14,
        kind: Some(
            Error,
---
thread '[ui] src/test/ui/lifetimes/issue-79187-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13

---- [ui] src/test/ui/mismatched_types/cast-rfc0401.rs stdout ----

error: /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:53: unexpected error: '53:13: 53:18: the trait bound `[u8]: Foo` is not satisfied [E0277]'
error: /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:62: unexpected error: '62:13: 62:14: the trait bound `str: Foo` is not satisfied [E0277]'

error: 2 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/cast-rfc0401.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/cast-rfc0401" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/cast-rfc0401/auxiliary"
    Error {
        line_num: 53,
        kind: Some(
            Error,
            Error,
        ),
        msg: "53:13: 53:18: the trait bound `[u8]: Foo` is not satisfied [E0277]",
    Error {
        line_num: 62,
        kind: Some(
            Error,
---
thread '[ui] src/test/ui/mismatched_types/cast-rfc0401.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13

---- [ui] src/test/ui/mismatched_types/closure-mismatch.rs stdout ----

error: /checkout/src/test/ui/mismatched_types/closure-mismatch.rs:8: unexpected error: '8:5: 8:8: the trait bound `[closure@/checkout/src/test/ui/mismatched_types/closure-mismatch.rs:8:9: 8:12]: Foo` is not satisfied [E0277]'

error: /checkout/src/test/ui/mismatched_types/closure-mismatch.rs:8: expected error not found: implementation of `FnOnce` is not general enough
error: /checkout/src/test/ui/mismatched_types/closure-mismatch.rs:8: expected error not found: mismatched types

error: 1 unexpected errors found, 2 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/closure-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-mismatch" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-mismatch/auxiliary"
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "8:5: 8:8: the trait bound `[closure@/checkout/src/test/ui/mismatched_types/closure-mismatch.rs:8:9: 8:12]: Foo` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 8,
        kind: Some(
            Error,
        ),
        msg: "implementation of `FnOnce` is not general enough",
    Error {
        line_num: 8,
        kind: Some(
            Error,
---
thread '[ui] src/test/ui/mismatched_types/closure-mismatch.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13

---- [ui] src/test/ui/parser/struct-literal-in-for.rs stdout ----

error: /checkout/src/test/ui/parser/struct-literal-in-for.rs:12: unexpected error: '12:14: 14:11: `bool` cannot be converted to an iterator [E0277]'
error: /checkout/src/test/ui/parser/struct-literal-in-for.rs:12: expected error not found: `bool` is not an iterator

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/struct-literal-in-for.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/struct-literal-in-for" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/struct-literal-in-for/auxiliary"
    Error {
        line_num: 12,
        kind: Some(
            Error,
            Error,
        ),
        msg: "12:14: 14:11: `bool` cannot be converted to an iterator [E0277]",
]

not found errors (from test file): [
    Error {
---
thread '[ui] src/test/ui/parser/struct-literal-in-for.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13

---- [ui] src/test/ui/range/range-1.rs stdout ----

error: /checkout/src/test/ui/range/range-1.rs:9: unexpected error: '9:14: 9:25: `std::ops::Range<bool>` cannot be converted to an iterator [E0277]'
error: /checkout/src/test/ui/range/range-1.rs:9: expected error not found: `bool: Step` is not satisfied

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/range/range-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/range-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/range-1/auxiliary"
    Error {
        line_num: 9,
        kind: Some(
            Error,
            Error,
        ),
        msg: "9:14: 9:25: `std::ops::Range<bool>` cannot be converted to an iterator [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 9,
        kind: Some(
            Error,
        ),
        msg: "`bool: Step` is not satisfied",
]

thread '[ui] src/test/ui/range/range-1.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/specialization/defaultimpl/specialization-trait-not-implemented.rs stdout ----

error: /checkout/src/test/ui/specialization/defaultimpl/specialization-trait-not-implemented.rs:22: unexpected error: '22:29: 22:36: no method named `foo_one` found for struct `MyStruct` in the current scope [E0599]'
error: /checkout/src/test/ui/specialization/defaultimpl/specialization-trait-not-implemented.rs:22: expected error not found: the method

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/defaultimpl/specialization-trait-not-implemented.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-trait-not-implemented" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-trait-not-implemented/auxiliary"
    Error {
        line_num: 22,
        kind: Some(
            Error,
            Error,
        ),
        msg: "22:29: 22:36: no method named `foo_one` found for struct `MyStruct` in the current scope [E0599]",
]

not found errors (from test file): [
    Error {
---
---- [ui] src/test/ui/specialization/issue-38091-2.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/issue-38091-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-38091-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-38091-2/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(specialization)]
   |            ^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete

error[E0275]: overflow evaluating the requirement `T: Iterate<'_>`
   |
   |
LL | impl<'a, T> Iterate<'a> for T
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_38091_2`)
note: required because of the requirements on the impl of `Check` for `T`
   |
   |
LL | impl<'a, T> Check for T where <T as Iterate<'a>>::Ty: Valid {}
   |             ^^^^^     ^
note: required because of the requirements on the impl of `Iterate<'_>` for `T`
   |
   |
LL | impl<'a, T> Iterate<'a> for T
   = note: 126 redundant requirements hidden
   = note: 126 redundant requirements hidden
   = note: required because of the requirements on the impl of `Iterate<'a>` for `T`

error[E0275]: overflow evaluating the requirement `T: Iterate<'_>`
   |
   |
LL | impl<'a, T> Check for T where <T as Iterate<'a>>::Ty: Valid {}
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_38091_2`)
note: required because of the requirements on the impl of `Check` for `T`
   |
   |
LL | impl<'a, T> Check for T where <T as Iterate<'a>>::Ty: Valid {}
   |             ^^^^^     ^
note: required because of the requirements on the impl of `Iterate<'_>` for `T`
   |
   |
LL | impl<'a, T> Iterate<'a> for T
   = note: 126 redundant requirements hidden
   = note: 126 redundant requirements hidden
   = note: required because of the requirements on the impl of `Iterate<'a>` for `T`
error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0275`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/specialization/issue-39448.rs stdout ----

error: /checkout/src/test/ui/specialization/issue-39448.rs:45: unexpected error: '45:13: 45:15: the trait bound `T: FromA<U>` is not satisfied [E0277]'

error: /checkout/src/test/ui/specialization/issue-39448.rs:45: unexpected error: '45:19: 45:21: the trait bound `U: FromA<T>` is not satisfied [E0277]'
error: /checkout/src/test/ui/specialization/issue-39448.rs:45: expected error not found: overflow evaluating the requirement

error: 2 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/issue-39448.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-39448" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-39448/auxiliary"
    Error {
        line_num: 45,
        kind: Some(
            Error,
            Error,
        ),
        msg: "45:13: 45:15: the trait bound `T: FromA<U>` is not satisfied [E0277]",
    Error {
        line_num: 45,
        kind: Some(
            Error,
            Error,
        ),
        msg: "45:19: 45:21: the trait bound `U: FromA<T>` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
---
thread '[ui] src/test/ui/specialization/issue-39448.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13

---- [ui] src/test/ui/specialization/issue-38091.rs stdout ----

error: /checkout/src/test/ui/specialization/issue-38091.rs:8: unexpected error: '8:13: 8:24: overflow evaluating the requirement `T: Iterate<'_>` [E0275]'

error: /checkout/src/test/ui/specialization/issue-38091.rs:18: unexpected error: '18:1: 18:24: overflow evaluating the requirement `T: Iterate<'_>` [E0275]'

error: /checkout/src/test/ui/specialization/issue-38091.rs:12: expected error not found: the trait bound `(): Valid` is not satisfied
error: 2 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/issue-38091.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-38091" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-38091/auxiliary"
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "8:13: 8:24: overflow evaluating the requirement `T: Iterate<'_>` [E0275]",
    Error {
        line_num: 18,
        kind: Some(
            Error,
            Error,
        ),
        msg: "18:1: 18:24: overflow evaluating the requirement `T: Iterate<'_>` [E0275]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 12,
        kind: Some(
            Error,
        ),
        msg: "the trait bound `(): Valid` is not satisfied",
]

thread '[ui] src/test/ui/specialization/issue-38091.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/specialization/min_specialization/issue-79224.rs stdout ----

error: /checkout/src/test/ui/specialization/min_specialization/issue-79224.rs:18: unexpected error: '18:17: 18:24: the trait bound `B: ToOwned` is not satisfied [E0277]'

error: /checkout/src/test/ui/specialization/min_specialization/issue-79224.rs:19: unexpected error: '19:12: 19:17: the trait bound `B: ToOwned` is not satisfied [E0277]'

error: /checkout/src/test/ui/specialization/min_specialization/issue-79224.rs:18: expected error not found: the trait bound `B: Clone` is not satisfied [E0277]

error: /checkout/src/test/ui/specialization/min_specialization/issue-79224.rs:19: expected error not found: the trait bound `B: Clone` is not satisfied [E0277]
error: 2 unexpected errors found, 2 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/min_specialization/issue-79224.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/min_specialization/issue-79224" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/min_specialization/issue-79224/auxiliary"
    Error {
        line_num: 18,
        kind: Some(
            Error,
            Error,
        ),
        msg: "18:17: 18:24: the trait bound `B: ToOwned` is not satisfied [E0277]",
    Error {
        line_num: 19,
        kind: Some(
            Error,
            Error,
        ),
        msg: "19:12: 19:17: the trait bound `B: ToOwned` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 18,
        kind: Some(
            Error,
        ),
        msg: "the trait bound `B: Clone` is not satisfied [E0277]",
    Error {
        line_num: 19,
        kind: Some(
            Error,
            Error,
        ),
        msg: "the trait bound `B: Clone` is not satisfied [E0277]",
]

thread '[ui] src/test/ui/specialization/min_specialization/issue-79224.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/suggestions/into-str.rs stdout ----

error: /checkout/src/test/ui/suggestions/into-str.rs:4: unexpected error: '4:9: 4:22: the trait bound `String: Into<&str>` is not satisfied [E0277]'

error: /checkout/src/test/ui/suggestions/into-str.rs:4: expected error not found: the trait bound `&str: From<String>` is not satisfied
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/into-str.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/into-str" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/into-str/auxiliary"
    Error {
        line_num: 4,
        kind: Some(
            Error,
---
        line_num: 4,
        kind: Some(
            Error,
        ),
        msg: "the trait bound `&str: From<String>` is not satisfied",
]

thread '[ui] src/test/ui/suggestions/into-str.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/suggestions/issue-96555.rs stdout ----

error: /checkout/src/test/ui/suggestions/issue-96555.rs:4: unexpected error: '4:12: 4:18: `()` cannot be converted to a future [E0277]'

error: /checkout/src/test/ui/suggestions/issue-96555.rs:5: unexpected error: '5:12: 5:18: `()` cannot be converted to a future [E0277]'

error: /checkout/src/test/ui/suggestions/issue-96555.rs:6: unexpected error: '6:12: 6:18: `()` cannot be converted to a future [E0277]'
error: /checkout/src/test/ui/suggestions/issue-96555.rs:4: expected error not found: `()` is not a future

error: /checkout/src/test/ui/suggestions/issue-96555.rs:5: expected error not found: `()` is not a future


error: /checkout/src/test/ui/suggestions/issue-96555.rs:6: expected error not found: `()` is not a future

error: 3 unexpected errors found, 3 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-96555.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-96555" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-96555/auxiliary"
    Error {
        line_num: 4,
        kind: Some(
            Error,
            Error,
        ),
        msg: "4:12: 4:18: `()` cannot be converted to a future [E0277]",
    Error {
        line_num: 5,
        kind: Some(
            Error,
            Error,
        ),
        msg: "5:12: 5:18: `()` cannot be converted to a future [E0277]",
    Error {
        line_num: 6,
        kind: Some(
            Error,
            Error,
        ),
        msg: "6:12: 6:18: `()` cannot be converted to a future [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 4,
        kind: Some(
            Error,
        ),
        msg: "`()` is not a future",
    Error {
        line_num: 5,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`()` is not a future",
    Error {
        line_num: 6,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`()` is not a future",
]

thread '[ui] src/test/ui/suggestions/issue-96555.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/suggestions/slice-issue-87994.rs stdout ----

error: /checkout/src/test/ui/suggestions/slice-issue-87994.rs:3: unexpected error: '3:12: 3:18: `[i32]` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/suggestions/slice-issue-87994.rs:11: unexpected error: '11:13: 11:20: `[K]` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/suggestions/slice-issue-87994.rs:3: expected error not found: [i32]` is not an iterator [E0277]
error: /checkout/src/test/ui/suggestions/slice-issue-87994.rs:3: expected error not found: known at compilation time


error: /checkout/src/test/ui/suggestions/slice-issue-87994.rs:11: expected error not found: [K]` is not an iterator [E0277]
error: /checkout/src/test/ui/suggestions/slice-issue-87994.rs:11: expected error not found: known at compilation time

error: 2 unexpected errors found, 4 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/slice-issue-87994.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/slice-issue-87994" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/slice-issue-87994/auxiliary"
    Error {
        line_num: 3,
        kind: Some(
            Error,
            Error,
        ),
        msg: "3:12: 3:18: `[i32]` cannot be converted to an iterator [E0277]",
    Error {
        line_num: 11,
        kind: Some(
            Error,
            Error,
        ),
        msg: "11:13: 11:20: `[K]` cannot be converted to an iterator [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 3,
        kind: Some(
            Error,
        ),
        msg: "[i32]` is not an iterator [E0277]",
    Error {
        line_num: 3,
        kind: Some(
            Error,
            Error,
        ),
        msg: "known at compilation time",
    Error {
        line_num: 11,
        kind: Some(
            Error,
            Error,
        ),
        msg: "[K]` is not an iterator [E0277]",
    Error {
        line_num: 11,
        kind: Some(
            Error,
            Error,
        ),
        msg: "known at compilation time",
]

thread '[ui] src/test/ui/suggestions/slice-issue-87994.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/suggestions/suggest-remove-refs-1.rs stdout ----

error: /checkout/src/test/ui/suggestions/suggest-remove-refs-1.rs:6: unexpected error: '6:19: 6:40: `&Enumerate<std::slice::Iter<'_, {integer}>>` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/suggestions/suggest-remove-refs-1.rs:6: expected error not found: `&Enumerate<std::slice::Iter<'_, {integer}>>` is not an iterator
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-remove-refs-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-remove-refs-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-remove-refs-1/auxiliary"
    Error {
        line_num: 6,
        kind: Some(
            Error,
            Error,
        ),
        msg: "6:19: 6:40: `&Enumerate<std::slice::Iter<'_, {integer}>>` cannot be converted to an iterator [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 6,
        kind: Some(
            Error,
        ),
        msg: "`&Enumerate<std::slice::Iter<'_, {integer}>>` is not an iterator",
]

thread '[ui] src/test/ui/suggestions/suggest-remove-refs-1.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/suggestions/suggest-remove-refs-3.rs stdout ----

error: /checkout/src/test/ui/suggestions/suggest-remove-refs-3.rs:6: unexpected error: '6:19: 9:21: `&&&&&Enumerate<std::slice::Iter<'_, {integer}>>` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/suggestions/suggest-remove-refs-3.rs:6: expected error not found: `&&&&&Enumerate<std::slice::Iter<'_, {integer}>>` is not an
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-remove-refs-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-remove-refs-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-remove-refs-3/auxiliary"
    Error {
        line_num: 6,
        kind: Some(
            Error,
            Error,
        ),
        msg: "6:19: 9:21: `&&&&&Enumerate<std::slice::Iter<'_, {integer}>>` cannot be converted to an iterator [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 6,
        kind: Some(
            Error,
        ),
        msg: "`&&&&&Enumerate<std::slice::Iter<'_, {integer}>>` is not an",
]

thread '[ui] src/test/ui/suggestions/suggest-remove-refs-3.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/suggestions/suggest-remove-refs-2.rs stdout ----

error: /checkout/src/test/ui/suggestions/suggest-remove-refs-2.rs:6: unexpected error: '6:19: 6:48: `&&&&&Enumerate<std::slice::Iter<'_, {integer}>>` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/suggestions/suggest-remove-refs-2.rs:6: expected error not found: `&&&&&Enumerate<std::slice::Iter<'_, {integer}>>` is not an iterator
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-remove-refs-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-remove-refs-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-remove-refs-2/auxiliary"
    Error {
        line_num: 6,
        kind: Some(
            Error,
            Error,
        ),
        msg: "6:19: 6:48: `&&&&&Enumerate<std::slice::Iter<'_, {integer}>>` cannot be converted to an iterator [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 6,
        kind: Some(
            Error,
        ),
        msg: "`&&&&&Enumerate<std::slice::Iter<'_, {integer}>>` is not an iterator",
]

thread '[ui] src/test/ui/suggestions/suggest-remove-refs-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/traits/bound/assoc-fn-bound-root-obligation.rs stdout ----

error: /checkout/src/test/ui/traits/bound/assoc-fn-bound-root-obligation.rs:2: unexpected error: '2:20: 2:25: the trait bound `u8: Pattern<'_>` is not satisfied [E0277]'

error: /checkout/src/test/ui/traits/bound/assoc-fn-bound-root-obligation.rs:2: unexpected note: 'the trait `Pattern<'_>` is not implemented for `u8`'
error: /checkout/src/test/ui/traits/bound/assoc-fn-bound-root-obligation.rs:2: unexpected note: '2:20: 2:25: required by a bound in `core::str::<impl str>::strip_suffix`'

error: /checkout/src/test/ui/traits/bound/assoc-fn-bound-root-obligation.rs:1: unexpected note: 'required by a bound in this'


error: /checkout/src/test/ui/traits/bound/assoc-fn-bound-root-obligation.rs:2: expected error not found: expected a `FnMut<(char,)>` closure, found `u8`

error: /checkout/src/test/ui/traits/bound/assoc-fn-bound-root-obligation.rs:2: expected note not found: expected an `FnMut<(char,)>` closure, found `u8`

error: /checkout/src/test/ui/traits/bound/assoc-fn-bound-root-obligation.rs:2: expected help message not found: the trait `FnMut<(char,)>` is not implemented for `u8`

error: /checkout/src/test/ui/traits/bound/assoc-fn-bound-root-obligation.rs:2: expected note not found: required because of the requirements on the impl of `Pattern<'_>` for `u8`
error: 4 unexpected errors found, 4 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/bound/assoc-fn-bound-root-obligation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/bound/assoc-fn-bound-root-obligation" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/bound/assoc-fn-bound-root-obligation/auxiliary"
    Error {
        line_num: 2,
        kind: Some(
            Error,
            Error,
        ),
        msg: "2:20: 2:25: the trait bound `u8: Pattern<'_>` is not satisfied [E0277]",
    Error {
        line_num: 2,
        kind: Some(
            Note,
            Note,
        ),
        msg: "the trait `Pattern<'_>` is not implemented for `u8`",
    Error {
        line_num: 2,
        kind: Some(
            Note,
---
        line_num: 2,
        kind: Some(
            Error,
        ),
        msg: "expected a `FnMut<(char,)>` closure, found `u8`",
    Error {
        line_num: 2,
        kind: Some(
            Note,
            Note,
        ),
        msg: "expected an `FnMut<(char,)>` closure, found `u8`",
    Error {
        line_num: 2,
        kind: Some(
            Help,
            Help,
        ),
        msg: "the trait `FnMut<(char,)>` is not implemented for `u8`",
    Error {
        line_num: 2,
        kind: Some(
            Note,
            Note,
        ),
        msg: "required because of the requirements on the impl of `Pattern<'_>` for `u8`",
]

thread '[ui] src/test/ui/traits/bound/assoc-fn-bound-root-obligation.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/traits/cycle-cache-err-60010.rs stdout ----

error: /checkout/src/test/ui/traits/cycle-cache-err-60010.rs:27: unexpected error: '27:13: 27:54: the trait bound `RootDatabase: SourceDatabase` is not satisfied [E0277]'
error: /checkout/src/test/ui/traits/cycle-cache-err-60010.rs:27: expected error not found: overflow

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/cycle-cache-err-60010.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010/auxiliary"
    Error {
        line_num: 27,
        kind: Some(
            Error,
            Error,
        ),
        msg: "27:13: 27:54: the trait bound `RootDatabase: SourceDatabase` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
---
thread '[ui] src/test/ui/traits/cycle-cache-err-60010.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13

---- [ui] src/test/ui/traits/inductive-overflow/simultaneous.rs stdout ----

error: /checkout/src/test/ui/traits/inductive-overflow/simultaneous.rs:18: unexpected error: '18:11: 18:12: the trait bound `{integer}: Combo` is not satisfied [E0277]'

error: /checkout/src/test/ui/traits/inductive-overflow/simultaneous.rs:18: expected error not found: overflow evaluating the requirement `{integer}: Tweedle
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/inductive-overflow/simultaneous.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/simultaneous" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/simultaneous/auxiliary"
    Error {
        line_num: 18,
        kind: Some(
            Error,
            Error,
        ),
        msg: "18:11: 18:12: the trait bound `{integer}: Combo` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 18,
        kind: Some(
            Error,
        ),
        msg: "overflow evaluating the requirement `{integer}: Tweedle",
]

thread '[ui] src/test/ui/traits/inductive-overflow/simultaneous.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/traits/inductive-overflow/two-traits.rs stdout ----

error: /checkout/src/test/ui/traits/inductive-overflow/two-traits.rs:20: unexpected error: '20:14: 20:21: the trait bound `*mut (): Magic` is not satisfied [E0277]'
error: /checkout/src/test/ui/traits/inductive-overflow/two-traits.rs:20: expected error not found: E0275

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/inductive-overflow/two-traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/two-traits" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/two-traits/auxiliary"
    Error {
        line_num: 20,
        kind: Some(
            Error,
            Error,
        ),
        msg: "20:14: 20:21: the trait bound `*mut (): Magic` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
---
thread '[ui] src/test/ui/traits/inductive-overflow/two-traits.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13

---- [ui] src/test/ui/traits/inductive-overflow/supertrait.rs stdout ----

error: /checkout/src/test/ui/traits/inductive-overflow/supertrait.rs:13: unexpected error: '13:23: 13:30: the trait bound `NoClone: Magic` is not satisfied [E0277]'
error: /checkout/src/test/ui/traits/inductive-overflow/supertrait.rs:13: expected error not found: E0275

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/inductive-overflow/supertrait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/supertrait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/supertrait/auxiliary"
    Error {
        line_num: 13,
        kind: Some(
            Error,
            Error,
        ),
        msg: "13:23: 13:30: the trait bound `NoClone: Magic` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
---
thread '[ui] src/test/ui/traits/inductive-overflow/supertrait.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13

---- [ui] src/test/ui/traits/suggest-deferences/root-obligation.rs stdout ----

error: /checkout/src/test/ui/traits/suggest-deferences/root-obligation.rs:6: unexpected error: '6:38: 6:39: the trait bound `&char: Pattern<'_>` is not satisfied [E0277]'

error: /checkout/src/test/ui/traits/suggest-deferences/root-obligation.rs:6: expected error not found: expected a `Fn<(char,)>` closure, found `char`
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/suggest-deferences/root-obligation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/root-obligation" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/root-obligation/auxiliary"
    Error {
        line_num: 6,
        kind: Some(
            Error,
            Error,
        ),
        msg: "6:38: 6:39: the trait bound `&char: Pattern<'_>` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 6,
        kind: Some(
            Error,
        ),
        msg: "expected a `Fn<(char,)>` closure, found `char`",
]

thread '[ui] src/test/ui/traits/suggest-deferences/root-obligation.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/type-alias-impl-trait/multiple-def-uses-in-one-fn.rs stdout ----

error: /checkout/src/test/ui/type-alias-impl-trait/multiple-def-uses-in-one-fn.rs:9: unexpected error: '9:45: 9:63: the trait bound `&A: Into<&'static B>` is not satisfied [E0277]'

error: /checkout/src/test/ui/type-alias-impl-trait/multiple-def-uses-in-one-fn.rs:9: expected error not found: the trait bound `&'static B: From<&A>` is not satisfied
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/multiple-def-uses-in-one-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/multiple-def-uses-in-one-fn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/multiple-def-uses-in-one-fn/auxiliary"
    Error {
        line_num: 9,
        kind: Some(
            Error,
            Error,
        ),
        msg: "9:45: 9:63: the trait bound `&A: Into<&'static B>` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 9,
        kind: Some(
            Error,
        ),
        msg: "the trait bound `&'static B: From<&A>` is not satisfied",
]

thread '[ui] src/test/ui/type-alias-impl-trait/multiple-def-uses-in-one-fn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/unsized/issue-71659.rs stdout ----

error: /checkout/src/test/ui/unsized/issue-71659.rs:30: unexpected error: '30:15: 30:19: the trait bound `dyn Foo: Unsize<[i32]>` is not satisfied [E0277]'

error: /checkout/src/test/ui/unsized/issue-71659.rs:30: expected error not found: the trait bound `dyn Foo: CastTo<[i32]>` is not satisfied
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized/issue-71659.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/issue-71659" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/issue-71659/auxiliary"
    Error {
        line_num: 30,
        kind: Some(
            Error,
            Error,
        ),
        msg: "30:15: 30:19: the trait bound `dyn Foo: Unsize<[i32]>` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 30,
        kind: Some(
            Error,
        ),
        msg: "the trait bound `dyn Foo: CastTo<[i32]>` is not satisfied",
]

thread '[ui] src/test/ui/unsized/issue-71659.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/wf/hir-wf-check-erase-regions.rs stdout ----

error: /checkout/src/test/ui/wf/hir-wf-check-erase-regions.rs:7: unexpected error: '7:21: 7:64: `&T` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/wf/hir-wf-check-erase-regions.rs:10: unexpected error: '10:27: 10:41: `&T` cannot be converted to an iterator [E0277]'

error: /checkout/src/test/ui/wf/hir-wf-check-erase-regions.rs:7: expected error not found: `&T` is not an iterator

error: /checkout/src/test/ui/wf/hir-wf-check-erase-regions.rs:10: expected error not found: `&T` is not an iterator
error: 2 unexpected errors found, 2 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/wf/hir-wf-check-erase-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/hir-wf-check-erase-regions/hir-wf-check-erase-regions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/hir-wf-check-erase-regions" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/hir-wf-check-erase-regions/auxiliary"
    Error {
        line_num: 7,
        kind: Some(
            Error,
            Error,
        ),
        msg: "7:21: 7:64: `&T` cannot be converted to an iterator [E0277]",
    Error {
        line_num: 10,
        kind: Some(
            Error,
            Error,
        ),
        msg: "10:27: 10:41: `&T` cannot be converted to an iterator [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 7,
        kind: Some(
            Error,
        ),
        msg: "`&T` is not an iterator",
    Error {
        line_num: 10,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`&T` is not an iterator",
]

thread '[ui] src/test/ui/wf/hir-wf-check-erase-regions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13

