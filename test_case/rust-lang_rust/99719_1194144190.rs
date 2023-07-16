plain
......................................i..........i..........i........................... 3872/13234
........................................................................................ 3960/13234
....................................i................................................... 4048/13234
................................................................................i....... 4136/13234
..................F...F................................................................. 4224/13234
.........F.............................................................................. 4400/13234
........................................................................................ 4488/13234
........................................................................................ 4576/13234
........................................................................................ 4664/13234
---
........................................................................................ 6424/13234
.............................................................................i.......... 6512/13234
........................................................................................ 6600/13234
......................................................i................................. 6688/13234
.....................ii.iiF..F....i....i..F............................................. 6776/13234
....................................................................i....i.............. 6952/13234
...........................i..................i.............i........................... 7040/13234
.............................i.......................................................... 7128/13234
..............................................i......................................... 7216/13234
---
........................................................................................ 11352/13234
........................................................................................ 11440/13234
........................................................................................ 11528/13234
........................................................................................ 11616/13234
F......................FF.F.........................................................i... 11704/13234
........................................................................................ 11880/13234
........................................................................................ 11968/13234
........................................................................................ 12056/13234
........................................................................................ 12144/13234
---
failures:

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
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
]

thread '[ui] src/test/ui/for/for-c-in-str.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

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

