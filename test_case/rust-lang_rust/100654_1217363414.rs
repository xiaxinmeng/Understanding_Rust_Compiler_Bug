plain
.................................i...................................................... 6424/13392
........................................................................................ 6512/13392
.............................i.........F................................................ 6600/13392
........................................................................................ 6688/13392
......i.................................................F.....iiFii.....F..i....i....... 6776/13392
........................................................................................ 6952/13392
............................i....i..........................................i........... 7040/13392
......i.............i.........................................................i......... 7128/13392
........................................................................................ 7216/13392
---
........................................................................................ 11088/13392
........................................................................................ 11176/13392
........................................................................................ 11264/13392
........................................................................................ 11352/13392
.....................F.F................................................................ 11440/13392
........................................................................................ 11616/13392
.....................................................................F.................. 11704/13392
........................................................................................ 11792/13392
............i.......i........i.....i.....................i.............................. 11880/13392
---
error: /checkout/src/test/ui/generator/issue-68112.rs:63: unexpected note: 'required by a bound introduced by this call'

error: 1 unexpected errors found, 0 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/issue-68112.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-68112" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-68112/auxiliary"
    Error {
        line_num: 63,
        kind: Some(
            Note,
---
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [ui] src/test/ui/issues/issue-31173.rs stdout ----

error: /checkout/src/test/ui/issues/issue-31173.rs:6: unexpected error: '6:25: 10:11: expected `TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:7:21: 7:25]>` to be an iterator that yields `&_`, but it yields `u8` [E0271]'

error: /checkout/src/test/ui/issues/issue-31173.rs:11: expected error not found: to be an iterator that yields `&_`, but it yields `u8`
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-31173.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31173" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31173/auxiliary"
    Error {
        line_num: 6,
        kind: Some(
            Error,
            Error,
        ),
        msg: "6:25: 10:11: expected `TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:7:21: 7:25]>` to be an iterator that yields `&_`, but it yields `u8` [E0271]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 11,
        kind: Some(
            Error,
        ),
        msg: "to be an iterator that yields `&_`, but it yields `u8`",
]

thread '[ui] src/test/ui/issues/issue-31173.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1433:13


---- [ui] src/test/ui/issues/issue-59488.rs stdout ----

error: /checkout/src/test/ui/issues/issue-59488.rs:30: unexpected error: '30:5: 30:28: `fn(usize) -> Foo {Foo::Bar}` doesn't implement `Debug` [E0277]'
error: 1 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-59488.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488/auxiliary"
    Error {
        line_num: 30,
        kind: Some(
            Error,
            Error,
        ),
        msg: "30:5: 30:28: `fn(usize) -> Foo {Foo::Bar}` doesn't implement `Debug` [E0277]",
]

thread '[ui] src/test/ui/issues/issue-59488.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1433:13


---- [ui] src/test/ui/iterators/collect-into-array.rs stdout ----

error: /checkout/src/test/ui/iterators/collect-into-array.rs:3: unexpected note: 'required by a bound introduced by this call'

error: 1 unexpected errors found, 0 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/collect-into-array.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/collect-into-array" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/collect-into-array/auxiliary"
    Error {
        line_num: 3,
        kind: Some(
            Note,
---
thread '[ui] src/test/ui/iterators/collect-into-array.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1433:13

---- [ui] src/test/ui/iterators/collect-into-slice.rs stdout ----

error: /checkout/src/test/ui/iterators/collect-into-slice.rs:7: unexpected error: '7:38: 7:45: the size for values of type `[i32]` cannot be known at compilation time [E0277]'
error: /checkout/src/test/ui/iterators/collect-into-slice.rs:7: unexpected note: 'doesn't have a size known at compile-time'

error: /checkout/src/test/ui/iterators/collect-into-slice.rs:7: unexpected note: 'required by a bound introduced by this call'


error: /checkout/src/test/ui/iterators/collect-into-slice.rs:7: unexpected note: '7:30: 7:37: required by a bound in `collect`'

error: /checkout/src/test/ui/iterators/collect-into-slice.rs:1: unexpected note: 'required by a bound in this'

error: 5 unexpected errors found, 0 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/collect-into-slice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/collect-into-slice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/collect-into-slice/auxiliary"
    Error {
        line_num: 7,
        kind: Some(
            Error,
            Error,
        ),
        msg: "7:38: 7:45: the size for values of type `[i32]` cannot be known at compilation time [E0277]",
    Error {
        line_num: 7,
        kind: Some(
            Note,
---
thread '[ui] src/test/ui/iterators/collect-into-slice.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1433:13

---- [ui] src/test/ui/iterators/issue-28098.rs stdout ----

error: /checkout/src/test/ui/iterators/issue-28098.rs:2: unexpected error: '2:13: 2:27: `()` is not an iterator [E0277]'

error: /checkout/src/test/ui/iterators/issue-28098.rs:8: unexpected error: '8:13: 8:27: `()` is not an iterator [E0277]'

error: /checkout/src/test/ui/iterators/issue-28098.rs:17: unexpected error: '17:13: 17:27: `()` is not an iterator [E0277]'

error: /checkout/src/test/ui/iterators/issue-28098.rs:20: unexpected error: '20:13: 20:27: `()` is not an iterator [E0277]'
error: 4 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/issue-28098.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/issue-28098" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/issue-28098/auxiliary"
    Error {
        line_num: 2,
        kind: Some(
            Error,
            Error,
        ),
        msg: "2:13: 2:27: `()` is not an iterator [E0277]",
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "8:13: 8:27: `()` is not an iterator [E0277]",
    Error {
        line_num: 17,
        kind: Some(
            Error,
            Error,
        ),
        msg: "17:13: 17:27: `()` is not an iterator [E0277]",
    Error {
        line_num: 20,
        kind: Some(
            Error,
            Error,
        ),
        msg: "20:13: 20:27: `()` is not an iterator [E0277]",
]

thread '[ui] src/test/ui/iterators/issue-28098.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1433:13


---- [ui] src/test/ui/mismatched_types/unboxed-closures-vtable-mismatch.rs stdout ----

error: /checkout/src/test/ui/mismatched_types/unboxed-closures-vtable-mismatch.rs:16: expected note not found: required by a bound introduced by this call

error: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/unboxed-closures-vtable-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/unboxed-closures-vtable-mismatch" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/unboxed-closures-vtable-mismatch/auxiliary"
    Error {
        line_num: 16,
        kind: Some(
            Note,
---
thread '[ui] src/test/ui/mismatched_types/unboxed-closures-vtable-mismatch.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1433:13

---- [ui] src/test/ui/never_type/defaulted-never-note.rs#fallback stdout ----

error in revision `fallback`: /checkout/src/test/ui/never_type/defaulted-never-note.rs:30: unexpected note: 'required by a bound introduced by this call'

error in revision `fallback`: 1 unexpected errors found, 0 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/never_type/defaulted-never-note.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "fallback" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/defaulted-never-note.fallback" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/defaulted-never-note.fallback/auxiliary"
    Error {
        line_num: 30,
        kind: Some(
            Note,
            Note,
        ),
        msg: "required by a bound introduced by this call",
    },
]

thread '[ui] src/test/ui/never_type/defaulted-never-note.rs#fallback' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1433:13
---- [ui] src/test/ui/on-unimplemented/multiple-impls.rs stdout ----


error: /checkout/src/test/ui/on-unimplemented/multiple-impls.rs:33: unexpected error: '33:5: 33:17: the trait bound `[i32]: Index<u32>` is not satisfied [E0277]'

error: /checkout/src/test/ui/on-unimplemented/multiple-impls.rs:35: unexpected error: '35:5: 35:17: the trait bound `[i32]: Index<Foo<u32>>` is not satisfied [E0277]'

error: /checkout/src/test/ui/on-unimplemented/multiple-impls.rs:37: unexpected error: '37:5: 37:17: the trait bound `[i32]: Index<Bar<u32>>` is not satisfied [E0277]'
error: 3 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/on-unimplemented/multiple-impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/on-unimplemented/multiple-impls" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/on-unimplemented/multiple-impls/auxiliary"
    Error {
        line_num: 33,
        kind: Some(
            Error,
            Error,
        ),
        msg: "33:5: 33:17: the trait bound `[i32]: Index<u32>` is not satisfied [E0277]",
    Error {
        line_num: 35,
        kind: Some(
            Error,
            Error,
        ),
        msg: "35:5: 35:17: the trait bound `[i32]: Index<Foo<u32>>` is not satisfied [E0277]",
    Error {
        line_num: 37,
        kind: Some(
            Error,
            Error,
        ),
        msg: "37:5: 37:17: the trait bound `[i32]: Index<Bar<u32>>` is not satisfied [E0277]",
]

thread '[ui] src/test/ui/on-unimplemented/multiple-impls.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1433:13


---- [ui] src/test/ui/on-unimplemented/on-impl.rs stdout ----

error: /checkout/src/test/ui/on-unimplemented/on-impl.rs:22: unexpected error: '22:5: 22:24: the trait bound `[i32]: Index<u32>` is not satisfied [E0277]'
error: 1 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/on-unimplemented/on-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/on-unimplemented/on-impl" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/on-unimplemented/on-impl/auxiliary"
    Error {
        line_num: 22,
        kind: Some(
            Error,
            Error,
        ),
        msg: "22:5: 22:24: the trait bound `[i32]: Index<u32>` is not satisfied [E0277]",
]

thread '[ui] src/test/ui/on-unimplemented/on-impl.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1433:13

---
error: /checkout/src/test/ui/str/str-idx.rs:5: unexpected error: '5:15: 5:28: the type `str` cannot be indexed by `{integer}` [E0277]'

error: 2 unexpected errors found, 0 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/str/str-idx.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-idx" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-idx/auxiliary"
    Error {
        line_num: 4,
        kind: Some(
            Error,
---
error: /checkout/src/test/ui/str/str-mut-idx.rs:11: unexpected error: '11:7: 11:24: the type `str` cannot be indexed by `{integer}` [E0277]'

error: 2 unexpected errors found, 0 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/str/str-mut-idx.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-mut-idx" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/str/str-mut-idx/auxiliary"
    Error {
        line_num: 9,
        kind: Some(
            Error,
---
error: /checkout/src/test/ui/suggestions/mut-borrow-needed-by-trait.rs:17: unexpected error: '17:14: 17:32: the trait bound `&dyn std::io::Write: std::io::Write` is not satisfied [E0277]'

error: 1 unexpected errors found, 0 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/mut-borrow-needed-by-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/mut-borrow-needed-by-trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/mut-borrow-needed-by-trait/auxiliary"
    Error {
        line_num: 17,
        kind: Some(
            Error,
---
error: /checkout/src/test/ui/traits/bound/assoc-fn-bound-root-obligation.rs:2: expected note not found: required by a bound introduced by this call

error: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/bound/assoc-fn-bound-root-obligation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/bound/assoc-fn-bound-root-obligation" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/bound/assoc-fn-bound-root-obligation/auxiliary"
    Error {
        line_num: 2,
        kind: Some(
            Note,
