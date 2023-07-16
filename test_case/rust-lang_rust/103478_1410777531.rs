plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:055e3b93d15803815fe6f9cbc1b02b11be094e54)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
...........................i............................................................ 9768/14393
........................................................................................ 9856/14393
........................................i............................................... 9944/14393
........................................................................................ 10032/14393
.......................F....F.FFF.......F............................................... 10120/14393
........................................................................................ 10296/14393
........................................................................................ 10384/14393
........................................................................................ 10472/14393
........................................................................................ 10560/14393
---
..........................iii........................................................... 14344/14393
.................................................
failures:

---- [ui] tests/ui/parser/suggest_misplaced_generics/enum.rs stdout ----
error: /checkout/tests/ui/parser/suggest_misplaced_generics/enum.rs:5: expected suggestion not found: Foo<T>

error: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/parser/suggest_misplaced_generics/enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/enum" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/enum/auxiliary"
    Error {
        line_num: 5,
        kind: Some(
            Suggestion,
            Suggestion,
        ),
        msg: "Foo<T>",
]


thread '[ui] tests/ui/parser/suggest_misplaced_generics/enum.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1422:13

---- [ui] tests/ui/parser/suggest_misplaced_generics/fn-complex-generics.rs stdout ----

error: /checkout/tests/ui/parser/suggest_misplaced_generics/fn-complex-generics.rs:5: expected suggestion not found: f<'a, B: 'a + std::ops::Add<Output = u32>>
error: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/parser/suggest_misplaced_generics/fn-complex-generics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/fn-complex-generics" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/fn-complex-generics/auxiliary"
    Error {
        line_num: 5,
        kind: Some(
            Suggestion,
            Suggestion,
        ),
        msg: "f<'a, B: 'a + std::ops::Add<Output = u32>>",
]


thread '[ui] tests/ui/parser/suggest_misplaced_generics/fn-complex-generics.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1422:13

---- [ui] tests/ui/parser/suggest_misplaced_generics/fn-simple.rs stdout ----

error: /checkout/tests/ui/parser/suggest_misplaced_generics/fn-simple.rs:5: expected suggestion not found: id<T>
error: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/parser/suggest_misplaced_generics/fn-simple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/fn-simple" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/fn-simple/auxiliary"
    Error {
        line_num: 5,
        kind: Some(
            Suggestion,
            Suggestion,
        ),
        msg: "id<T>",
]


thread '[ui] tests/ui/parser/suggest_misplaced_generics/fn-simple.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1422:13

---- [ui] tests/ui/parser/suggest_misplaced_generics/struct.rs stdout ----
error: /checkout/tests/ui/parser/suggest_misplaced_generics/struct.rs:5: expected suggestion not found: Foo<T>

error: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/parser/suggest_misplaced_generics/struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/struct" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/struct/auxiliary"
    Error {
        line_num: 5,
        kind: Some(
            Suggestion,
            Suggestion,
        ),
        msg: "Foo<T>",
]


thread '[ui] tests/ui/parser/suggest_misplaced_generics/struct.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1422:13

---- [ui] tests/ui/parser/suggest_misplaced_generics/type.rs stdout ----
error: /checkout/tests/ui/parser/suggest_misplaced_generics/type.rs:5: expected suggestion not found: Foo<T>

error: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/parser/suggest_misplaced_generics/type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/type/auxiliary"
    Error {
        line_num: 5,
        kind: Some(
            Suggestion,
            Suggestion,
        ),
        msg: "Foo<T>",
]


thread '[ui] tests/ui/parser/suggest_misplaced_generics/type.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1422:13

---- [ui] tests/ui/parser/suggest_misplaced_generics/trait.rs stdout ----
error: /checkout/tests/ui/parser/suggest_misplaced_generics/trait.rs:5: expected suggestion not found: Foo<T>

error: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/parser/suggest_misplaced_generics/trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/trait/auxiliary"
    Error {
        line_num: 5,
        kind: Some(
            Suggestion,
            Suggestion,
        ),
        msg: "Foo<T>",
]


thread '[ui] tests/ui/parser/suggest_misplaced_generics/trait.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1422:13

failures:
    [ui] tests/ui/parser/suggest_misplaced_generics/enum.rs
    [ui] tests/ui/parser/suggest_misplaced_generics/fn-complex-generics.rs
