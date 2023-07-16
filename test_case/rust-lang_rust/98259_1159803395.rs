plain
failures:

---- [ui] src/test/ui/impl-trait/auto-trait-leak2.rs stdout ----

error: /checkout/src/test/ui/impl-trait/auto-trait-leak2.rs:13: unexpected note: '`Rc<Cell<i32>>` cannot be sent between threads safely'

error: /checkout/src/test/ui/impl-trait/auto-trait-leak2.rs:5: unexpected note: 'within this `impl Fn(i32)`'
error: /checkout/src/test/ui/impl-trait/auto-trait-leak2.rs:13: unexpected note: 'required by a bound introduced by this call'


error: /checkout/src/test/ui/impl-trait/auto-trait-leak2.rs:5: unexpected note: '5:16: 5:28: required because it appears within the type `impl Fn(i32)`'
error: /checkout/src/test/ui/impl-trait/auto-trait-leak2.rs:5: unexpected note: 'in this expansion of desugaring of `impl Trait`'


error: /checkout/src/test/ui/impl-trait/auto-trait-leak2.rs:10: unexpected note: '10:12: 10:16: required by a bound in `send`'

error: /checkout/src/test/ui/impl-trait/auto-trait-leak2.rs:10: unexpected note: 'required by this bound in `send`'

error: /checkout/src/test/ui/impl-trait/auto-trait-leak2.rs:16: unexpected note: '`Rc<Cell<i32>>` cannot be sent between threads safely'

error: /checkout/src/test/ui/impl-trait/auto-trait-leak2.rs:22: unexpected note: 'within this `impl Fn(i32)`'
error: /checkout/src/test/ui/impl-trait/auto-trait-leak2.rs:16: unexpected note: 'required by a bound introduced by this call'


error: /checkout/src/test/ui/impl-trait/auto-trait-leak2.rs:22: unexpected note: '22:15: 22:27: required because it appears within the type `impl Fn(i32)`'
error: /checkout/src/test/ui/impl-trait/auto-trait-leak2.rs:22: unexpected note: 'in this expansion of desugaring of `impl Trait`'

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: /checkout/src/test/ui/impl-trait/auto-trait-leak2.rs:10: unexpected note: '10:12: 10:16: required by a bound in `send`'

error: /checkout/src/test/ui/impl-trait/auto-trait-leak2.rs:10: unexpected note: 'required by this bound in `send`'
error: /checkout/src/test/ui/impl-trait/auto-trait-leak2.rs:5: expected note not found: used within the type

error: /checkout/src/test/ui/impl-trait/auto-trait-leak2.rs:22: expected note not found: used within the type


error: 14 unexpected errors found, 2 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/auto-trait-leak2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak2/auxiliary"
    Error {
        line_num: 13,
        kind: Some(
            Note,
            Note,
        ),
        msg: "`Rc<Cell<i32>>` cannot be sent between threads safely",
    Error {
        line_num: 5,
        kind: Some(
            Note,
            Note,
        ),
        msg: "within this `impl Fn(i32)`",
    Error {
        line_num: 13,
        kind: Some(
            Note,
---
        line_num: 5,
        kind: Some(
            Note,
        ),
        msg: "5:16: 5:28: required because it appears within the type `impl Fn(i32)`",
    Error {
        line_num: 5,
        kind: Some(
            Note,
---
        line_num: 10,
        kind: Some(
            Note,
        ),
        msg: "10:12: 10:16: required by a bound in `send`",
    Error {
        line_num: 10,
        kind: Some(
            Note,
            Note,
        ),
        msg: "required by this bound in `send`",
    Error {
        line_num: 16,
        kind: Some(
            Note,
            Note,
        ),
        msg: "`Rc<Cell<i32>>` cannot be sent between threads safely",
    Error {
        line_num: 22,
        kind: Some(
            Note,
            Note,
        ),
        msg: "within this `impl Fn(i32)`",
    Error {
        line_num: 16,
        kind: Some(
            Note,
---
        line_num: 22,
        kind: Some(
            Note,
        ),
        msg: "22:15: 22:27: required because it appears within the type `impl Fn(i32)`",
    Error {
        line_num: 22,
        kind: Some(
            Note,
---
        line_num: 10,
        kind: Some(
            Note,
        ),
        msg: "10:12: 10:16: required by a bound in `send`",
    Error {
        line_num: 10,
        kind: Some(
            Note,
            Note,
        ),
        msg: "required by this bound in `send`",
]

not found errors (from test file): [
    Error {
---
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [ui] src/test/ui/type-alias-impl-trait/auto-trait-leakage2.rs stdout ----

error: /checkout/src/test/ui/type-alias-impl-trait/auto-trait-leakage2.rs:17: unexpected note: '`Rc<u32>` cannot be sent between threads safely'
error: /checkout/src/test/ui/type-alias-impl-trait/auto-trait-leakage2.rs:7: unexpected note: 'within this `Foo`'

error: /checkout/src/test/ui/type-alias-impl-trait/auto-trait-leakage2.rs:17: unexpected note: 'required by a bound introduced by this call'


error: /checkout/src/test/ui/type-alias-impl-trait/auto-trait-leakage2.rs:7: unexpected note: 'in this expansion of desugaring of `impl Trait`'

error: /checkout/src/test/ui/type-alias-impl-trait/auto-trait-leakage2.rs:14: unexpected note: '14:15: 14:19: required by a bound in `is_send`'

error: /checkout/src/test/ui/type-alias-impl-trait/auto-trait-leakage2.rs:14: unexpected note: 'required by this bound in `is_send`'
error: 6 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/auto-trait-leakage2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/auto-trait-leakage2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/auto-trait-leakage2/auxiliary"
    Error {
        line_num: 17,
        kind: Some(
            Note,
            Note,
        ),
        msg: "`Rc<u32>` cannot be sent between threads safely",
    Error {
        line_num: 7,
        kind: Some(
            Note,
            Note,
        ),
        msg: "within this `Foo`",
    Error {
        line_num: 17,
        kind: Some(
            Note,
