plain
........................................................................................ 2816/13221
........................................................................................ 2904/13221
...........................................................................i............ 2992/13221
.........................................i.............................................. 3080/13221
....................................................................FFF...F............. 3168/13221
..................iiiii................................................................. 3344/13221
........................................................................................ 3432/13221
........................................................................................ 3520/13221
................................................................................F....... 3608/13221
---
........................................................................................ 5280/13221
........................................................................................ 5368/13221
........................................................................................ 5456/13221
........................................................................................ 5544/13221
........................F..............................F................................ 5632/13221
........................................................................................ 5808/13221
........................................................................................ 5896/13221
........................................................................................ 5984/13221
.....................................F.................................................. 6072/13221
.....................................F.................................................. 6072/13221
........................................................................................ 6160/13221
........................................................................................ 6248/13221
.............................................................................i.......... 6336/13221
........................................................................................ 6424/13221
........................................................................i............... 6512/13221
.......FF..............................................................F................ 6600/13221
................ii.ii........i....i..................................................... 6776/13221
..........i............................................................................. 6864/13221
................................................................i...i................... 6952/13221
......................i..................i.............i................................ 7040/13221
---
........................................................................................ 7656/13221
.......................................................ii............................... 7744/13221
........................................................................................ 7832/13221
........................................................................................ 7920/13221
..........................................F..................F.ii................i....i. 8008/13221
........................................................................................ 8184/13221
........................................................................................ 8272/13221
........................................................................................ 8360/13221
........................................................................................ 8448/13221
........................................................................................ 8448/13221
...............i..ii..............................................................ii.... 8536/13221
........................................................................................ 8624/13221
iiii...F................................................................................ 8712/13221
.........................................................i.............................. 8888/13221
........................................................................................ 8976/13221
..............................................i......................................... 9064/13221
........................................................................................ 9152/13221
---
..........................i............................................................. 10736/13221
....................................iiiiii.i...iiiiiii.................................. 10824/13221
........................................................................................ 10912/13221
........................................................................................ 11000/13221
...................................................F.................................... 11088/13221
...F........F........................................................................... 11176/13221
........................................................................................ 11352/13221
........................................................................................ 11440/13221
........................................................................................ 11440/13221
..........F.........F...........F....................F.................................. 11528/13221
......................................F................................................. 11616/13221
.i.....i.....................i.......................................................... 11792/13221
........................................................................................ 11880/13221
........................................................................................ 11968/13221
........................................................................................ 12056/13221
........................................................................................ 12056/13221
........................................................................................ 12144/13221
........................................................................................ 12232/13221
........................................................................................ 12320/13221
...........F.FF.FF.F...F......F..................i...................................... 12408/13221
...............F....F................................................................... 12496/13221
........................................................................................ 12672/13221
........................................................................................ 12760/13221
.......................FF............................................................... 12848/13221
........................................................................................ 12936/13221
........................................................................................ 12936/13221
........................................................................................ 13024/13221
........................................................................................ 13112/13221
..iii...................................................................F...F........... 13200/13221
failures:

---- [ui] src/test/ui/associated-type-bounds/bounds-on-assoc-in-trait.rs stdout ----


error: /checkout/src/test/ui/associated-type-bounds/bounds-on-assoc-in-trait.rs:18: unexpected error: '18:28: 18:33: the trait bound `<<Self as Case1>::A as Iterator>::Item: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/associated-type-bounds/bounds-on-assoc-in-trait.rs:18: expected error not found: `<<Self as Case1>::A as Iterator>::Item` doesn't implement `Debug`
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/bounds-on-assoc-in-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/bounds-on-assoc-in-trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/bounds-on-assoc-in-trait/auxiliary"
    Error {
        line_num: 18,
        kind: Some(
            Error,
            Error,
        ),
        msg: "18:28: 18:33: the trait bound `<<Self as Case1>::A as Iterator>::Item: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 18,
        kind: Some(
            Error,
        ),
        msg: "`<<Self as Case1>::A as Iterator>::Item` doesn't implement `Debug`",
]

thread '[ui] src/test/ui/associated-type-bounds/bounds-on-assoc-in-trait.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [ui] src/test/ui/binop/issue-77910-1.rs stdout ----

error: /checkout/src/test/ui/binop/issue-77910-1.rs:8: unexpected error: '8:5: 8:23: the trait bound `for<'r> fn(&'r i32) -> &'r i32 {foo}: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/binop/issue-77910-1.rs:8: expected error not found: `for<'r> fn(&'r i32) -> &'r i32 {foo}` doesn't implement `Debug`
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/issue-77910-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-77910-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-77910-1/auxiliary"
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "8:5: 8:23: the trait bound `for<'r> fn(&'r i32) -> &'r i32 {foo}: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 8,
        kind: Some(
            Error,
        ),
        msg: "`for<'r> fn(&'r i32) -> &'r i32 {foo}` doesn't implement `Debug`",
]

thread '[ui] src/test/ui/binop/issue-77910-1.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13


---- [ui] src/test/ui/compare-method/traits-misc-mismatch-1.rs stdout ----

error: /checkout/src/test/ui/compare-method/traits-misc-mismatch-1.rs:27: unexpected error: '27:26: 27:29: the trait bound `T: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/compare-method/traits-misc-mismatch-1.rs:51: unexpected error: '51:30: 51:32: the trait bound `T: FnPtr` is not satisfied [E0277]'
error: /checkout/src/test/ui/compare-method/traits-misc-mismatch-1.rs:27: expected error not found: E0276

error: /checkout/src/test/ui/compare-method/traits-misc-mismatch-1.rs:51: expected error not found: E0276


error: 2 unexpected errors found, 2 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/compare-method/traits-misc-mismatch-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/compare-method/traits-misc-mismatch-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/compare-method/traits-misc-mismatch-1/auxiliary"
    Error {
        line_num: 27,
        kind: Some(
            Error,
            Error,
        ),
        msg: "27:26: 27:29: the trait bound `T: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 51,
        kind: Some(
            Error,
            Error,
        ),
        msg: "51:30: 51:32: the trait bound `T: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
---
thread '[ui] src/test/ui/compare-method/traits-misc-mismatch-1.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13

---- [ui] src/test/ui/derives/derives-span-PartialOrd-enum-struct-variant.rs stdout ----

error: /checkout/src/test/ui/derives/derives-span-PartialOrd-enum-struct-variant.rs:9: unexpected error: '9:6: 9:14: the trait bound `Error: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/derives/derives-span-PartialOrd-enum-struct-variant.rs:9: expected error not found: can't compare `Error` with `Error`
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-PartialOrd-enum-struct-variant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialOrd-enum-struct-variant" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialOrd-enum-struct-variant/auxiliary"
    Error {
        line_num: 9,
        kind: Some(
            Error,
            Error,
        ),
        msg: "9:6: 9:14: the trait bound `Error: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 9,
        kind: Some(
            Error,
        ),
        msg: "can't compare `Error` with `Error`",
]

thread '[ui] src/test/ui/derives/derives-span-PartialOrd-enum-struct-variant.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13


---- [ui] src/test/ui/derives/derives-span-PartialOrd-enum.rs stdout ----

error: /checkout/src/test/ui/derives/derives-span-PartialOrd-enum.rs:9: unexpected error: '9:6: 9:11: the trait bound `Error: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/derives/derives-span-PartialOrd-enum.rs:9: expected error not found: can't compare `Error` with `Error`
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-PartialOrd-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialOrd-enum" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialOrd-enum/auxiliary"
    Error {
        line_num: 9,
        kind: Some(
            Error,
            Error,
        ),
        msg: "9:6: 9:11: the trait bound `Error: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 9,
        kind: Some(
            Error,
        ),
        msg: "can't compare `Error` with `Error`",
]

thread '[ui] src/test/ui/derives/derives-span-PartialOrd-enum.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13


---- [ui] src/test/ui/derives/derives-span-PartialOrd-struct.rs stdout ----

error: /checkout/src/test/ui/derives/derives-span-PartialOrd-struct.rs:8: unexpected error: '8:5: 8:13: the trait bound `Error: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/derives/derives-span-PartialOrd-struct.rs:8: expected error not found: can't compare `Error` with `Error`
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-PartialOrd-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialOrd-struct" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialOrd-struct/auxiliary"
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "8:5: 8:13: the trait bound `Error: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 8,
        kind: Some(
            Error,
        ),
        msg: "can't compare `Error` with `Error`",
]

thread '[ui] src/test/ui/derives/derives-span-PartialOrd-struct.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13


---- [ui] src/test/ui/derives/derives-span-PartialOrd-tuple-struct.rs stdout ----

error: /checkout/src/test/ui/derives/derives-span-PartialOrd-tuple-struct.rs:8: unexpected error: '8:5: 8:10: the trait bound `Error: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/derives/derives-span-PartialOrd-tuple-struct.rs:8: expected error not found: can't compare `Error` with `Error`
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-PartialOrd-tuple-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialOrd-tuple-struct" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialOrd-tuple-struct/auxiliary"
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "8:5: 8:10: the trait bound `Error: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 8,
        kind: Some(
            Error,
        ),
        msg: "can't compare `Error` with `Error`",
]

thread '[ui] src/test/ui/derives/derives-span-PartialOrd-tuple-struct.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13


---- [ui] src/test/ui/error-codes/E0277-3.rs stdout ----

error: /checkout/src/test/ui/error-codes/E0277-3.rs:6: unexpected error: '6:9: 6:10: the trait bound `S: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/error-codes/E0277-3.rs:6: expected error not found: can't compare `S` with `S`
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0277-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0277-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0277-3/auxiliary"
    Error {
        line_num: 6,
        kind: Some(
            Error,
            Error,
        ),
        msg: "6:9: 6:10: the trait bound `S: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 6,
        kind: Some(
            Error,
        ),
        msg: "can't compare `S` with `S`",
]

thread '[ui] src/test/ui/error-codes/E0277-3.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13


---- [ui] src/test/ui/generic-associated-types/issue-87429-associated-type-default.rs stdout ----

error: /checkout/src/test/ui/generic-associated-types/issue-87429-associated-type-default.rs:14: unexpected error: '14:60: 14:63: the trait bound `Foo: FnPtr` is not satisfied [E0277]'
error: /checkout/src/test/ui/generic-associated-types/issue-87429-associated-type-default.rs:14: expected error not found: can't compare

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-87429-associated-type-default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87429-associated-type-default" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87429-associated-type-default/auxiliary"
    Error {
        line_num: 14,
        kind: Some(
            Error,
            Error,
        ),
        msg: "14:60: 14:63: the trait bound `Foo: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
---
thread '[ui] src/test/ui/generic-associated-types/issue-87429-associated-type-default.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13

---- [ui] src/test/ui/generic-associated-types/issue-87429-specialization.rs stdout ----

error: /checkout/src/test/ui/generic-associated-types/issue-87429-specialization.rs:21: unexpected error: '21:31: 21:34: the trait bound `Foo: FnPtr` is not satisfied [E0277]'
error: /checkout/src/test/ui/generic-associated-types/issue-87429-specialization.rs:21: expected error not found: can't compare

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-87429-specialization.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87429-specialization" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87429-specialization/auxiliary"
    Error {
        line_num: 21,
        kind: Some(
            Error,
            Error,
        ),
        msg: "21:31: 21:34: the trait bound `Foo: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
---
thread '[ui] src/test/ui/generic-associated-types/issue-87429-specialization.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13

---- [ui] src/test/ui/issues/issue-20162.rs stdout ----

error: /checkout/src/test/ui/issues/issue-20162.rs:5: unexpected error: '5:7: 5:11: the trait bound `X: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/issues/issue-20162.rs:5: expected error not found: `X: Ord` is not satisfied
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-20162.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20162" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20162/auxiliary"
    Error {
        line_num: 5,
        kind: Some(
            Error,
            Error,
        ),
        msg: "5:7: 5:11: the trait bound `X: FnPtr` is not satisfied [E0277]",
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
]

not found errors (from test file): [
not found errors (from test file): [
    Error {
        line_num: 5,
        kind: Some(
            Error,
        ),
        msg: "`X: Ord` is not satisfied",
]

thread '[ui] src/test/ui/issues/issue-20162.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13


---- [ui] src/test/ui/issues/issue-21160.rs stdout ----

error: /checkout/src/test/ui/issues/issue-21160.rs:8: unexpected error: '8:12: 8:15: the trait bound `Bar: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/issues/issue-21160.rs:8: expected error not found: `Bar: Hash` is not satisfied
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-21160.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21160" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21160/auxiliary"
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "8:12: 8:15: the trait bound `Bar: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 8,
        kind: Some(
            Error,
        ),
        msg: "`Bar: Hash` is not satisfied",
]

thread '[ui] src/test/ui/issues/issue-21160.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13


---- [ui] src/test/ui/issues/issue-34229.rs stdout ----

error: /checkout/src/test/ui/issues/issue-34229.rs:2: unexpected error: '2:46: 2:56: the trait bound `Comparable: FnPtr` is not satisfied [E0277]'
error: /checkout/src/test/ui/issues/issue-34229.rs:2: expected error not found: can't compare `Comparable`

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-34229.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34229" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34229/auxiliary"
    Error {
        line_num: 2,
        kind: Some(
            Error,
            Error,
        ),
        msg: "2:46: 2:56: the trait bound `Comparable: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 2,
        kind: Some(
            Error,
        ),
        msg: "can't compare `Comparable`",
]

thread '[ui] src/test/ui/issues/issue-34229.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13


---- [ui] src/test/ui/issues/issue-59488.rs stdout ----

error: /checkout/src/test/ui/issues/issue-59488.rs:30: unexpected error: '30:5: 30:28: the trait bound `fn(usize) -> Foo {Foo::Bar}: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/issues/issue-59488.rs:30: expected error not found: `fn(usize) -> Foo {Foo::Bar}` doesn't implement `Debug` [E0277]
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-59488.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488/auxiliary"
    Error {
        line_num: 30,
        kind: Some(
            Error,
            Error,
        ),
        msg: "30:5: 30:28: the trait bound `fn(usize) -> Foo {Foo::Bar}: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 30,
        kind: Some(
            Error,
        ),
        msg: "`fn(usize) -> Foo {Foo::Bar}` doesn't implement `Debug` [E0277]",
]

thread '[ui] src/test/ui/issues/issue-59488.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13


---- [ui] src/test/ui/issues/issue-62375.rs stdout ----

error: /checkout/src/test/ui/issues/issue-62375.rs:7: unexpected error: '7:10: 7:18: mismatched types [E0308]'

error: 1 unexpected errors found, 0 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-62375.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-62375" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-62375/auxiliary"
    Error {
        line_num: 7,
        kind: Some(
            Error,
---
thread '[ui] src/test/ui/issues/issue-62375.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13

---- [ui] src/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap.rs stdout ----

error: /checkout/src/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap.rs:6: unexpected error: '6:5: 6:21: the trait bound `fn() -> i32 {a}: FnPtr` is not satisfied [E0277]'
error: /checkout/src/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap.rs:6: expected error not found: doesn't implement

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap/auxiliary"
    Error {
        line_num: 6,
        kind: Some(
            Error,
            Error,
        ),
        msg: "6:5: 6:21: the trait bound `fn() -> i32 {a}: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
---
thread '[ui] src/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13

---- [ui] src/test/ui/mismatched_types/method-help-unsatisfied-bound.rs stdout ----

error: /checkout/src/test/ui/mismatched_types/method-help-unsatisfied-bound.rs:5: unexpected error: '5:7: 5:13: the trait bound `Foo: FnPtr` is not satisfied [E0277]'
error: /checkout/src/test/ui/mismatched_types/method-help-unsatisfied-bound.rs:5: expected error not found: `Foo` doesn't implement `Debug`

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/method-help-unsatisfied-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/method-help-unsatisfied-bound" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/method-help-unsatisfied-bound/auxiliary"
    Error {
        line_num: 5,
        kind: Some(
            Error,
            Error,
        ),
        msg: "5:7: 5:13: the trait bound `Foo: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 5,
        kind: Some(
            Error,
        ),
        msg: "`Foo` doesn't implement `Debug`",
]

thread '[ui] src/test/ui/mismatched_types/method-help-unsatisfied-bound.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13


---- [ui] src/test/ui/missing-trait-bounds/missing-trait-bound-for-op.rs stdout ----

error: failed to compile fixed code
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/missing-trait-bounds/missing-trait-bound-for-op.fixed" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing-trait-bounds/missing-trait-bound-for-op/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing-trait-bounds/missing-trait-bound-for-op/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0637]: `&` without an explicit lifetime name cannot be used here
  --> /checkout/src/test/ui/missing-trait-bounds/missing-trait-bound-for-op.fixed:3:39
   |
LL | pub fn foo<T>(s: &[T], t: &[T]) where &[T]: PartialEq<&[T]> {
   |                                       ^ explicit lifetime name needed here

error[E0637]: `&` without an explicit lifetime name cannot be used here
  --> /checkout/src/test/ui/missing-trait-bounds/missing-trait-bound-for-op.fixed:3:55
   |
LL | pub fn foo<T>(s: &[T], t: &[T]) where &[T]: PartialEq<&[T]> {
   |                                                       ^ explicit lifetime name needed here
error[E0310]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/missing-trait-bounds/missing-trait-bound-for-op.fixed:3:45
   |
   |
LL | pub fn foo<T>(s: &[T], t: &[T]) where &[T]: PartialEq<&[T]> {
   |                                             ^^^^^^^^^^^^^^^ ...so that the reference type `&'static [T]` does not outlive the data it points at
help: consider adding an explicit lifetime bound...
   |
   |
LL | pub fn foo<T: 'static>(s: &[T], t: &[T]) where &[T]: PartialEq<&[T]> {

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0310, E0637.
Some errors have detailed explanations: E0310, E0637.
For more information about an error, try `rustc --explain E0310`.
------------------------------------------


---- [ui] src/test/ui/on-unimplemented/no-debug.rs stdout ----

error: /checkout/src/test/ui/on-unimplemented/no-debug.rs:10: unexpected error: '10:27: 10:30: the trait bound `Foo: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/on-unimplemented/no-debug.rs:10: unexpected error: '10:32: 10:35: the trait bound `Bar: FnPtr` is not satisfied [E0277]'
error: /checkout/src/test/ui/on-unimplemented/no-debug.rs:10: expected error not found: `Foo` doesn't implement `Debug`


error: /checkout/src/test/ui/on-unimplemented/no-debug.rs:10: expected error not found: `Bar` doesn't implement `Debug`
error: 2 unexpected errors found, 2 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/on-unimplemented/no-debug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/on-unimplemented/no-debug" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/on-unimplemented/no-debug/auxiliary"
    Error {
        line_num: 10,
        kind: Some(
            Error,
            Error,
        ),
        msg: "10:27: 10:30: the trait bound `Foo: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 10,
        kind: Some(
            Error,
            Error,
        ),
        msg: "10:32: 10:35: the trait bound `Bar: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 10,
        kind: Some(
            Error,
        ),
        msg: "`Foo` doesn't implement `Debug`",
    Error {
        line_num: 10,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`Bar` doesn't implement `Debug`",
]

thread '[ui] src/test/ui/on-unimplemented/no-debug.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13


---- [ui] src/test/ui/range/range_traits-1.rs stdout ----

error: /checkout/src/test/ui/range/range_traits-1.rs:5: unexpected error: '5:5: 5:20: the trait bound `std::ops::Range<usize>: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/range/range_traits-1.rs:8: unexpected error: '8:5: 8:22: the trait bound `std::ops::RangeTo<usize>: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/range/range_traits-1.rs:11: unexpected error: '11:5: 11:24: the trait bound `std::ops::RangeFrom<usize>: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/range/range_traits-1.rs:14: unexpected error: '14:5: 14:17: the trait bound `std::ops::RangeFull: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/range/range_traits-1.rs:17: unexpected error: '17:5: 17:29: the trait bound `std::ops::RangeInclusive<usize>: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/range/range_traits-1.rs:20: unexpected error: '20:5: 20:31: the trait bound `std::ops::RangeToInclusive<usize>: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/range/range_traits-1.rs:5: unexpected error: '5:5: 5:20: the trait bound `std::ops::Range<usize>: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/range/range_traits-1.rs:8: unexpected error: '8:5: 8:22: the trait bound `std::ops::RangeTo<usize>: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/range/range_traits-1.rs:11: unexpected error: '11:5: 11:24: the trait bound `std::ops::RangeFrom<usize>: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/range/range_traits-1.rs:14: unexpected error: '14:5: 14:17: the trait bound `std::ops::RangeFull: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/range/range_traits-1.rs:17: unexpected error: '17:5: 17:29: the trait bound `std::ops::RangeInclusive<usize>: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/range/range_traits-1.rs:20: unexpected error: '20:5: 20:31: the trait bound `std::ops::RangeToInclusive<usize>: FnPtr` is not satisfied [E0277]'
error: /checkout/src/test/ui/range/range_traits-1.rs:5: expected error not found: can't compare

error: /checkout/src/test/ui/range/range_traits-1.rs:5: expected error not found: Ord

---
error: /checkout/src/test/ui/range/range_traits-1.rs:20: expected error not found: Ord

error: 12 unexpected errors found, 12 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/range/range_traits-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/range_traits-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/range_traits-1/auxiliary"
    Error {
        line_num: 5,
        kind: Some(
            Error,
            Error,
        ),
        msg: "5:5: 5:20: the trait bound `std::ops::Range<usize>: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "8:5: 8:22: the trait bound `std::ops::RangeTo<usize>: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 11,
        kind: Some(
            Error,
            Error,
        ),
        msg: "11:5: 11:24: the trait bound `std::ops::RangeFrom<usize>: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 14,
        kind: Some(
            Error,
            Error,
        ),
        msg: "14:5: 14:17: the trait bound `std::ops::RangeFull: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 17,
        kind: Some(
            Error,
            Error,
        ),
        msg: "17:5: 17:29: the trait bound `std::ops::RangeInclusive<usize>: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 20,
        kind: Some(
            Error,
            Error,
        ),
        msg: "20:5: 20:31: the trait bound `std::ops::RangeToInclusive<usize>: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 5,
        kind: Some(
            Error,
            Error,
        ),
        msg: "5:5: 5:20: the trait bound `std::ops::Range<usize>: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "8:5: 8:22: the trait bound `std::ops::RangeTo<usize>: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 11,
        kind: Some(
            Error,
            Error,
        ),
        msg: "11:5: 11:24: the trait bound `std::ops::RangeFrom<usize>: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 14,
        kind: Some(
            Error,
            Error,
        ),
        msg: "14:5: 14:17: the trait bound `std::ops::RangeFull: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 17,
        kind: Some(
            Error,
            Error,
        ),
        msg: "17:5: 17:29: the trait bound `std::ops::RangeInclusive<usize>: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 20,
        kind: Some(
            Error,
            Error,
        ),
        msg: "20:5: 20:31: the trait bound `std::ops::RangeToInclusive<usize>: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
---
thread '[ui] src/test/ui/range/range_traits-1.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13

---- [ui] src/test/ui/rfc-2361-dbg-macro/dbg-macro-requires-debug.rs stdout ----

error: /checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-requires-debug.rs:6: unexpected error: '6:23: 6:37: `&NotDebug` doesn't implement `Debug` [E0277]'

error: /checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-requires-debug.rs:6: expected error not found: `NotDebug` doesn't implement `Debug`
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-requires-debug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-requires-debug" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2361-dbg-macro/dbg-macro-requires-debug/auxiliary"
    Error {
        line_num: 6,
        kind: Some(
            Error,
            Error,
        ),
        msg: "6:23: 6:37: `&NotDebug` doesn't implement `Debug` [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 6,
        kind: Some(
            Error,
        ),
        msg: "`NotDebug` doesn't implement `Debug`",
]

thread '[ui] src/test/ui/rfc-2361-dbg-macro/dbg-macro-requires-debug.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13


---- [ui] src/test/ui/span/issue-71363.rs stdout ----

error: /checkout/src/test/ui/span/issue-71363.rs:4: unexpected error: '4:6: 4:23: the trait bound `MyError: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/span/issue-71363.rs:4: expected error not found: `MyError` doesn't implement `Debug`
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-71363.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-71363" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "simulate-remapped-rust-src-base=/rustc/xyz" "-Z" "translate-remapped-path-to-local-path=no" "-Z" "ui-testing=no" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-71363/auxiliary"
    Error {
        line_num: 4,
        kind: Some(
            Error,
            Error,
        ),
        msg: "4:6: 4:23: the trait bound `MyError: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 4,
        kind: Some(
            Error,
        ),
        msg: "`MyError` doesn't implement `Debug`",
]

thread '[ui] src/test/ui/span/issue-71363.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13


---- [ui] src/test/ui/specialization/default-generic-associated-type-bound.rs stdout ----

error: /checkout/src/test/ui/specialization/default-generic-associated-type-bound.rs:18: unexpected error: '18:26: 18:31: can't compare `&'a T` with `&'a T` [E0277]'

error: /checkout/src/test/ui/specialization/default-generic-associated-type-bound.rs:18: expected error not found: can't compare `T` with `T`
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/default-generic-associated-type-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/default-generic-associated-type-bound" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/default-generic-associated-type-bound/auxiliary"
    Error {
        line_num: 18,
        kind: Some(
            Error,
            Error,
        ),
        msg: "18:26: 18:31: can't compare `&'a T` with `&'a T` [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 18,
        kind: Some(
            Error,
        ),
        msg: "can't compare `T` with `T`",
]

thread '[ui] src/test/ui/specialization/default-generic-associated-type-bound.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13


---- [ui] src/test/ui/specialization/defaultimpl/specialization-wfcheck.rs stdout ----

error: /checkout/src/test/ui/specialization/defaultimpl/specialization-wfcheck.rs:7: unexpected error: '7:17: 7:32: the trait bound `U: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/specialization/defaultimpl/specialization-wfcheck.rs:7: expected error not found: the trait bound `U: Eq` is not satisfied
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/defaultimpl/specialization-wfcheck.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-wfcheck" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-wfcheck/auxiliary"
    Error {
        line_num: 7,
        kind: Some(
            Error,
            Error,
        ),
        msg: "7:17: 7:32: the trait bound `U: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 7,
        kind: Some(
            Error,
        ),
        msg: "the trait bound `U: Eq` is not satisfied",
]

thread '[ui] src/test/ui/specialization/defaultimpl/specialization-wfcheck.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13


---- [ui] src/test/ui/suggestions/bound-suggestions.rs stdout ----

error: /checkout/src/test/ui/suggestions/bound-suggestions.rs:9: unexpected error: '9:22: 9:23: the trait bound `impl Sized: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/suggestions/bound-suggestions.rs:15: unexpected error: '15:22: 15:23: the trait bound `T: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/suggestions/bound-suggestions.rs:21: unexpected error: '21:22: 21:23: the trait bound `T: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/suggestions/bound-suggestions.rs:27: unexpected error: '27:30: 27:31: the trait bound `Y: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/suggestions/bound-suggestions.rs:33: unexpected error: '33:22: 33:23: the trait bound `X: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/suggestions/bound-suggestions.rs:39: unexpected error: '39:22: 39:23: the trait bound `X: FnPtr` is not satisfied [E0277]'
error: /checkout/src/test/ui/suggestions/bound-suggestions.rs:9: expected error not found: doesn't implement

error: /checkout/src/test/ui/suggestions/bound-suggestions.rs:15: expected error not found: doesn't implement

---
error: /checkout/src/test/ui/suggestions/bound-suggestions.rs:39: expected error not found: doesn't implement

error: 6 unexpected errors found, 6 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/bound-suggestions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/bound-suggestions" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/bound-suggestions/auxiliary"
    Error {
        line_num: 9,
        kind: Some(
            Error,
            Error,
        ),
        msg: "9:22: 9:23: the trait bound `impl Sized: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 15,
        kind: Some(
            Error,
            Error,
        ),
        msg: "15:22: 15:23: the trait bound `T: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 21,
        kind: Some(
            Error,
            Error,
        ),
        msg: "21:22: 21:23: the trait bound `T: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 27,
        kind: Some(
            Error,
            Error,
        ),
        msg: "27:30: 27:31: the trait bound `Y: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 33,
        kind: Some(
            Error,
            Error,
        ),
        msg: "33:22: 33:23: the trait bound `X: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 39,
        kind: Some(
            Error,
            Error,
        ),
        msg: "39:22: 39:23: the trait bound `X: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
---
thread '[ui] src/test/ui/suggestions/bound-suggestions.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13

---- [ui] src/test/ui/suggestions/derive-macro-missing-bounds.rs stdout ----

error: /checkout/src/test/ui/suggestions/derive-macro-missing-bounds.rs:12: unexpected error: '12:21: 12:29: `&a::Inner<T>` doesn't implement `Debug` [E0277]'

error: /checkout/src/test/ui/suggestions/derive-macro-missing-bounds.rs:41: unexpected error: '41:21: 41:29: `&c::Inner<T>` doesn't implement `Debug` [E0277]'

error: /checkout/src/test/ui/suggestions/derive-macro-missing-bounds.rs:56: unexpected error: '56:21: 56:29: `&d::Inner<T>` doesn't implement `Debug` [E0277]'

error: /checkout/src/test/ui/suggestions/derive-macro-missing-bounds.rs:71: unexpected error: '71:21: 71:29: `&e::Inner<T>` doesn't implement `Debug` [E0277]'

error: /checkout/src/test/ui/suggestions/derive-macro-missing-bounds.rs:86: unexpected error: '86:21: 86:29: `&f::Inner<T>` doesn't implement `Debug` [E0277]'

error: /checkout/src/test/ui/suggestions/derive-macro-missing-bounds.rs:12: expected error not found: `a::Inner<T>` doesn't implement `Debug`

error: /checkout/src/test/ui/suggestions/derive-macro-missing-bounds.rs:41: expected error not found: the trait bound `T: c::Trait` is not satisfied

error: /checkout/src/test/ui/suggestions/derive-macro-missing-bounds.rs:56: expected error not found: the trait bound `T: d::Trait` is not satisfied

error: /checkout/src/test/ui/suggestions/derive-macro-missing-bounds.rs:71: expected error not found: the trait bound `T: e::Trait` is not satisfied

error: /checkout/src/test/ui/suggestions/derive-macro-missing-bounds.rs:86: expected error not found: the trait bound `T: f::Trait` is not satisfied
error: 5 unexpected errors found, 5 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/derive-macro-missing-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/derive-macro-missing-bounds" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/derive-macro-missing-bounds/auxiliary"
    Error {
        line_num: 12,
        kind: Some(
            Error,
            Error,
        ),
        msg: "12:21: 12:29: `&a::Inner<T>` doesn't implement `Debug` [E0277]",
    Error {
        line_num: 41,
        kind: Some(
            Error,
            Error,
        ),
        msg: "41:21: 41:29: `&c::Inner<T>` doesn't implement `Debug` [E0277]",
    Error {
        line_num: 56,
        kind: Some(
            Error,
            Error,
        ),
        msg: "56:21: 56:29: `&d::Inner<T>` doesn't implement `Debug` [E0277]",
    Error {
        line_num: 71,
        kind: Some(
            Error,
            Error,
        ),
        msg: "71:21: 71:29: `&e::Inner<T>` doesn't implement `Debug` [E0277]",
    Error {
        line_num: 86,
        kind: Some(
            Error,
            Error,
        ),
        msg: "86:21: 86:29: `&f::Inner<T>` doesn't implement `Debug` [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 12,
        kind: Some(
            Error,
        ),
        msg: "`a::Inner<T>` doesn't implement `Debug`",
    Error {
        line_num: 41,
        kind: Some(
            Error,
            Error,
        ),
        msg: "the trait bound `T: c::Trait` is not satisfied",
    Error {
        line_num: 56,
        kind: Some(
            Error,
            Error,
        ),
        msg: "the trait bound `T: d::Trait` is not satisfied",
    Error {
        line_num: 71,
        kind: Some(
            Error,
            Error,
        ),
        msg: "the trait bound `T: e::Trait` is not satisfied",
    Error {
        line_num: 86,
        kind: Some(
            Error,
            Error,
        ),
        msg: "the trait bound `T: f::Trait` is not satisfied",
]

thread '[ui] src/test/ui/suggestions/derive-macro-missing-bounds.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13


---- [ui] src/test/ui/suggestions/derive-clone-for-eq.rs stdout ----

error: failed to compile fixed code
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/derive-clone-for-eq.fixed" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/derive-clone-for-eq/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/derive-clone-for-eq/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0275]: overflow evaluating the requirement `Struct<_>: PartialEq`
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`derive_clone_for_eq`)
note: required because of the requirements on the impl of `PartialEq` for `Struct<_>`
  --> /checkout/src/test/ui/suggestions/derive-clone-for-eq.fixed:7:10
   |
LL | #[derive(PartialEq)]
   = note: 126 redundant requirements hidden
   = note: 126 redundant requirements hidden
   = note: required because of the requirements on the impl of `PartialEq` for `Struct<_>`
note: required because of the requirements on the impl of `Clone` for `Struct<_>`
  --> /checkout/src/test/ui/suggestions/derive-clone-for-eq.fixed:6:10
   |
LL | #[derive(Clone, Eq)] //~ ERROR [E0277]
   |          ^^^^^
   = note: this error originates in the derive macro `PartialEq` which comes from the expansion of the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0275`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/suggestions/impl-trait-with-missing-bounds.rs stdout ----

error: /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:6: unexpected error: '6:13: 6:23: the trait bound `<impl Iterator as Iterator>::Item: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:14: unexpected error: '14:13: 14:23: the trait bound `<impl Iterator as Iterator>::Item: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:22: unexpected error: '22:13: 22:23: the trait bound `<impl Iterator as Iterator>::Item: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:30: unexpected error: '30:13: 30:23: the trait bound `<impl Iterator as Iterator>::Item: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:37: unexpected error: '37:13: 37:23: the trait bound `<impl Iterator + std::fmt::Debug as Iterator>::Item: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:45: unexpected error: '45:13: 45:23: the trait bound `<impl Iterator as Iterator>::Item: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:6: expected error not found: `<impl Iterator as Iterator>::Item` doesn't implement `Debug`

error: /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:14: expected error not found: `<impl Iterator as Iterator>::Item` doesn't implement `Debug`

error: /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:22: expected error not found: `<impl Iterator as Iterator>::Item` doesn't implement `Debug`

error: /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:30: expected error not found: `<impl Iterator as Iterator>::Item` doesn't implement `Debug`

error: /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:37: expected error not found: `<impl Iterator + std::fmt::Debug as Iterator>::Item` doesn't implement

error: /checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs:45: expected error not found: `<impl Iterator as Iterator>::Item` doesn't implement `Debug`
error: 6 unexpected errors found, 6 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/impl-trait-with-missing-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/impl-trait-with-missing-bounds" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/impl-trait-with-missing-bounds/auxiliary"
    Error {
        line_num: 6,
        kind: Some(
            Error,
            Error,
        ),
        msg: "6:13: 6:23: the trait bound `<impl Iterator as Iterator>::Item: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 14,
        kind: Some(
            Error,
            Error,
        ),
        msg: "14:13: 14:23: the trait bound `<impl Iterator as Iterator>::Item: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 22,
        kind: Some(
            Error,
            Error,
        ),
        msg: "22:13: 22:23: the trait bound `<impl Iterator as Iterator>::Item: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 30,
        kind: Some(
            Error,
            Error,
        ),
        msg: "30:13: 30:23: the trait bound `<impl Iterator as Iterator>::Item: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 37,
        kind: Some(
            Error,
            Error,
        ),
        msg: "37:13: 37:23: the trait bound `<impl Iterator + std::fmt::Debug as Iterator>::Item: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 45,
        kind: Some(
            Error,
            Error,
        ),
        msg: "45:13: 45:23: the trait bound `<impl Iterator as Iterator>::Item: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 6,
        kind: Some(
            Error,
        ),
        msg: "`<impl Iterator as Iterator>::Item` doesn't implement `Debug`",
    Error {
        line_num: 14,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`<impl Iterator as Iterator>::Item` doesn't implement `Debug`",
    Error {
        line_num: 22,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`<impl Iterator as Iterator>::Item` doesn't implement `Debug`",
    Error {
        line_num: 30,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`<impl Iterator as Iterator>::Item` doesn't implement `Debug`",
    Error {
        line_num: 37,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`<impl Iterator + std::fmt::Debug as Iterator>::Item` doesn't implement",
    Error {
        line_num: 45,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`<impl Iterator as Iterator>::Item` doesn't implement `Debug`",
]

thread '[ui] src/test/ui/suggestions/impl-trait-with-missing-bounds.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13


---- [ui] src/test/ui/suggestions/missing-bound-in-derive-copy-impl-3.rs stdout ----

error: failed to compile fixed code
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/missing-bound-in-derive-copy-impl-3.fixed" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-bound-in-derive-copy-impl-3/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-bound-in-derive-copy-impl-3/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0405]: cannot find trait `FnPtr` in this scope
  --> /checkout/src/test/ui/suggestions/missing-bound-in-derive-copy-impl-3.fixed:11:27
   |
LL | pub struct AABB<K: Copy + FnPtr>{
   |
help: consider importing this trait
   |
   |
LL | use std::marker::FnPtr;

error[E0405]: cannot find trait `FnPtr` in this scope
  --> /checkout/src/test/ui/suggestions/missing-bound-in-derive-copy-impl-3.fixed:11:27
   |
   |
LL | pub struct AABB<K: Copy + FnPtr>{
   |
help: consider importing this trait
   |
   |
LL | use std::marker::FnPtr;

error[E0405]: cannot find trait `FnPtr` in this scope
  --> /checkout/src/test/ui/suggestions/missing-bound-in-derive-copy-impl-3.fixed:11:27
   |
   |
LL | pub struct AABB<K: Copy + FnPtr>{
   |
help: consider importing this trait
   |
   |
LL | use std::marker::FnPtr;

error[E0405]: cannot find trait `FnPtr` in this scope
  --> /checkout/src/test/ui/suggestions/missing-bound-in-derive-copy-impl-3.fixed:11:27
   |
   |
LL | pub struct AABB<K: Copy + FnPtr>{
   |
help: consider importing this trait
   |
   |
LL | use std::marker::FnPtr;

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0405`.
For more information about this error, try `rustc --explain E0405`.
------------------------------------------


---- [ui] src/test/ui/type-alias-impl-trait/generic_duplicate_param_use2.rs stdout ----

error: /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use2.rs:11: unexpected error: '11:5: 11:6: the trait bound `T: FnPtr` is not satisfied [E0277]'
error: /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use2.rs:11: expected error not found: `T` doesn't implement `Debug`

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_duplicate_param_use2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_duplicate_param_use2/auxiliary"
    Error {
        line_num: 11,
        kind: Some(
            Error,
            Error,
        ),
        msg: "11:5: 11:6: the trait bound `T: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 11,
        kind: Some(
            Error,
        ),
        msg: "`T` doesn't implement `Debug`",
]

thread '[ui] src/test/ui/type-alias-impl-trait/generic_duplicate_param_use2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13


---- [ui] src/test/ui/type-alias-impl-trait/generic_duplicate_param_use4.rs stdout ----

error: /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use4.rs:11: unexpected error: '11:5: 11:6: the trait bound `U: FnPtr` is not satisfied [E0277]'
error: /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use4.rs:11: expected error not found: `U` doesn't implement `Debug`

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_duplicate_param_use4" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_duplicate_param_use4/auxiliary"
    Error {
        line_num: 11,
        kind: Some(
            Error,
            Error,
        ),
        msg: "11:5: 11:6: the trait bound `U: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 11,
        kind: Some(
            Error,
        ),
        msg: "`U` doesn't implement `Debug`",
]

thread '[ui] src/test/ui/type-alias-impl-trait/generic_duplicate_param_use4.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13


---- [ui] src/test/ui/type-alias-impl-trait/generic_duplicate_param_use3.rs stdout ----

error: /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use3.rs:11: unexpected error: '11:5: 11:6: the trait bound `T: FnPtr` is not satisfied [E0277]'

error: /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use3.rs:16: unexpected error: '16:5: 16:6: the trait bound `U: FnPtr` is not satisfied [E0277]'
error: /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use3.rs:11: expected error not found: `T` doesn't implement `Debug`

error: /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use3.rs:16: expected error not found: `U` doesn't implement `Debug`


error: 2 unexpected errors found, 2 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_duplicate_param_use3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_duplicate_param_use3/auxiliary"
    Error {
        line_num: 11,
        kind: Some(
            Error,
            Error,
        ),
        msg: "11:5: 11:6: the trait bound `T: FnPtr` is not satisfied [E0277]",
    Error {
        line_num: 16,
        kind: Some(
            Error,
            Error,
        ),
        msg: "16:5: 16:6: the trait bound `U: FnPtr` is not satisfied [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 11,
        kind: Some(
            Error,
        ),
        msg: "`T` doesn't implement `Debug`",
    Error {
        line_num: 16,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`U` doesn't implement `Debug`",
]

thread '[ui] src/test/ui/type-alias-impl-trait/generic_duplicate_param_use3.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13


---- [ui] src/test/ui/type-alias-impl-trait/generic_duplicate_param_use5.rs stdout ----

error: /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use5.rs:11: unexpected error: '11:5: 11:11: `(T, U)` doesn't implement `Debug` [E0277]'

error: /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use5.rs:17: unexpected error: '17:5: 17:11: `(U, T)` doesn't implement `Debug` [E0277]'
error: /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use5.rs:11: expected error not found: `T` doesn't implement `Debug`

error: /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use5.rs:11: expected error not found: `U` doesn't implement `Debug`


error: /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use5.rs:17: expected error not found: `T` doesn't implement `Debug`

error: /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use5.rs:17: expected error not found: `U` doesn't implement `Debug`

error: 2 unexpected errors found, 4 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_duplicate_param_use5" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_duplicate_param_use5/auxiliary"
    Error {
        line_num: 11,
        kind: Some(
            Error,
            Error,
        ),
        msg: "11:5: 11:11: `(T, U)` doesn't implement `Debug` [E0277]",
    Error {
        line_num: 17,
        kind: Some(
            Error,
            Error,
        ),
        msg: "17:5: 17:11: `(U, T)` doesn't implement `Debug` [E0277]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 11,
        kind: Some(
            Error,
        ),
        msg: "`T` doesn't implement `Debug`",
    Error {
        line_num: 11,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`U` doesn't implement `Debug`",
    Error {
        line_num: 17,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`T` doesn't implement `Debug`",
    Error {
        line_num: 17,
        kind: Some(
            Error,
            Error,
        ),
        msg: "`U` doesn't implement `Debug`",
]

thread '[ui] src/test/ui/type-alias-impl-trait/generic_duplicate_param_use5.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1425:13


---- [ui] src/test/ui/type-alias-impl-trait/generic_duplicate_param_use8.rs stdout ----

error: /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use8.rs:10: unexpected error: '10:5: 10:14: `(T, u32)` doesn't implement `Debug` [E0277]'

error: /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use8.rs:15: unexpected error: '15:5: 15:14: `(U, u32)` doesn't implement `Debug` [E0277]'
error: /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use8.rs:10: expected error not found: `T` doesn't implement `Debug`

error: /checkout/src/test/ui/type-alias-impl-trait/generic_duplicate_param_use8.rs:15: expected error not found: `U` doesn't implement `Debug`

