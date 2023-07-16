plain
.................................................................................................... 10300/11934
.................................................................................................... 10400/11934
.................................................................................................... 10500/11934
.................................................................................................... 10600/11934
....................F...........F..F.....F....iF.....................................i.............. 10700/11934
.................................................................................................... 10900/11934
.................................................................................................... 11000/11934
.................................................................................................... 11100/11934
.................................................................................................... 11200/11934
---

---- [ui] ui/lto-duplicate-symbols.rs stdout ----
diff of stderr:

1 warning: Linking globals named 'foo': symbol multiply defined!
2 
- error: failed to load bc of "lto-duplicate-symbols2.lto_duplicate_symbols2.288b404e693a75b4-cgu.0.rcgu.o":
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ error: failed to load bc of "lto-duplicate-symbols2.lto_duplicate_symbols2.288b404e693a75b4-cgu.0.rcgu.o": 
5 error: aborting due to previous error; 1 warning emitted
6 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-duplicate-symbols/lto-duplicate-symbols.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lto-duplicate-symbols.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lto-duplicate-symbols.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-duplicate-symbols" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-duplicate-symbols/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: Linking globals named 'foo': symbol multiply defined!

error: failed to load bc of "lto-duplicate-symbols2.lto_duplicate_symbols2.288b404e693a75b4-cgu.0.rcgu.o": 
error: aborting due to previous error; 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/symbol-names/basic.rs#v0 stdout ----

error in revision `v0`: /checkout/src/test/ui/symbol-names/basic.rs:8: unexpected error: '8:1: 8:21: symbol-name(_RNvCsj6j3mjPNGKx_5basic4main)'

error in revision `v0`: /checkout/src/test/ui/symbol-names/basic.rs:8: unexpected error: '8:1: 8:21: demangling(basic[de7d5b6b69c71f37]::main)'

error in revision `v0`: /checkout/src/test/ui/symbol-names/basic.rs:8: expected error not found: symbol-name(_RNvCs21hi0yVfW1J_5basic4main)

error in revision `v0`: /checkout/src/test/ui/symbol-names/basic.rs:8: expected error not found: demangling(basic[17891616a171812d]::main)

error in revision `v0`: 2 unexpected errors found, 2 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/basic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "v0" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.v0" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.v0/auxiliary"
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "8:1: 8:21: symbol-name(_RNvCsj6j3mjPNGKx_5basic4main)",
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "8:1: 8:21: demangling(basic[de7d5b6b69c71f37]::main)",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 8,
        kind: Some(
            Error,
        ),
        msg: "symbol-name(_RNvCs21hi0yVfW1J_5basic4main)",
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "demangling(basic[17891616a171812d]::main)",
]


thread '[ui] ui/symbol-names/basic.rs#v0' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1524:13
---- [ui] ui/symbol-names/const-generics-demangling.rs stdout ----
diff of stderr:

52 LL | #[rustc_symbol_name]
52 LL | #[rustc_symbol_name]
53    | ^^^^^^^^^^^^^^^^^^^^
54 
- error: symbol-name(_RMs_CsaP8qXevlYG3_25const_generics_demanglingINtB3_4CharKc2202_E)
+ error: symbol-name(_RMs1_CsaP8qXevlYG3_25const_generics_demanglingINtB3_4CharKc2202_E)
57    |
58 LL | #[rustc_symbol_name]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-demangling/const-generics-demangling.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args symbol-names/const-generics-demangling.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/const-generics-demangling.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-demangling" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-demangling/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_RMCsaP8qXevlYG3_25const_generics_demanglingINtB0_8UnsignedKhb_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<const_generics_demangling[7e153590edc26969]::Unsigned<11: u8>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<const_generics_demangling::Unsigned<11>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs_CsaP8qXevlYG3_25const_generics_demanglingINtB2_6SignedKsn98_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<const_generics_demangling[7e153590edc26969]::Signed<-152: i16>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<const_generics_demangling::Signed<-152>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs0_CsaP8qXevlYG3_25const_generics_demanglingINtB3_4BoolKb1_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<const_generics_demangling[7e153590edc26969]::Bool<true: bool>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<const_generics_demangling::Bool<true>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs1_CsaP8qXevlYG3_25const_generics_demanglingINtB3_4CharKc2202_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<const_generics_demangling[7e153590edc26969]::Char<'∂': char>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<const_generics_demangling::Char<'∂'>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: aborting due to 12 previous errors


------------------------------------------


---- [ui] ui/symbol-names/impl1.rs#v0 stdout ----

error in revision `v0`: /checkout/src/test/ui/symbol-names/impl1.rs:14: unexpected error: '14:9: 14:29: symbol-name(_RNvMNtCs2qSCrjELJET_5impl13fooNtB2_3Foo3bar)'

error in revision `v0`: /checkout/src/test/ui/symbol-names/impl1.rs:14: unexpected error: '14:9: 14:29: demangling(<impl1[1c5860ab79c9e305]::foo::Foo>::bar)'

error in revision `v0`: /checkout/src/test/ui/symbol-names/impl1.rs:32: unexpected error: '32:9: 32:29: symbol-name(_RNvMNtCs2qSCrjELJET_5impl13barNtNtB4_3foo3Foo3baz)'

error in revision `v0`: /checkout/src/test/ui/symbol-names/impl1.rs:32: unexpected error: '32:9: 32:29: demangling(<impl1[1c5860ab79c9e305]::foo::Foo>::baz)'

error in revision `v0`: /checkout/src/test/ui/symbol-names/impl1.rs:62: unexpected error: '62:13: 62:33: symbol-name(_RNvXNCNvCs2qSCrjELJET_5impl14mains_0ARDNtB6_3Foop5AssocFG_KCRL0_hvEuNtB6_9AutoTraitEL_j3_NtB2_3Bar6method)'

error in revision `v0`: /checkout/src/test/ui/symbol-names/impl1.rs:62: unexpected error: '62:13: 62:33: demangling(<[&dyn impl1[1c5860ab79c9e305]::Foo<Assoc = for<'a> extern "C" fn(&'a u8, ...)> + impl1[1c5860ab79c9e305]::AutoTrait; 3: usize] as impl1[1c5860ab79c9e305]::main::{closure#1}::Bar>::method)'

error in revision `v0`: /checkout/src/test/ui/symbol-names/impl1.rs:14: expected error not found: symbol-name(_RNvMNtCs21hi0yVfW1J_5impl13fooNtB2_3Foo3bar)

error in revision `v0`: /checkout/src/test/ui/symbol-names/impl1.rs:14: expected error not found: demangling(<impl1[17891616a171812d]::foo::Foo>::bar)

error in revision `v0`: /checkout/src/test/ui/symbol-names/impl1.rs:32: expected error not found: symbol-name(_RNvMNtCs21hi0yVfW1J_5impl13barNtNtB4_3foo3Foo3baz)

error in revision `v0`: /checkout/src/test/ui/symbol-names/impl1.rs:32: expected error not found: demangling(<impl1[17891616a171812d]::foo::Foo>::baz)

error in revision `v0`: /checkout/src/test/ui/symbol-names/impl1.rs:62: expected error not found: symbol-name(_RNvXNCNvCs21hi0yVfW1J_5impl14mains_0ARDNtB6_3Foop5AssocFG_KCRL0_hvEuNtB6_9AutoTraitEL_j3_NtB2_3Bar6method)

error in revision `v0`: /checkout/src/test/ui/symbol-names/impl1.rs:62: expected error not found: demangling(<[&dyn impl1[17891616a171812d]::Foo<Assoc = for<'a> extern "C" fn(&'a u8, ...)> + impl1[17891616a171812d]::AutoTrait; 3: usize] as impl1[17891616a171812d]::main::{closure#1}::Bar>::method)

error in revision `v0`: 6 unexpected errors found, 6 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/impl1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "v0" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.v0" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.v0/auxiliary"
    Error {
        line_num: 14,
        kind: Some(
            Error,
            Error,
        ),
        msg: "14:9: 14:29: symbol-name(_RNvMNtCs2qSCrjELJET_5impl13fooNtB2_3Foo3bar)",
    Error {
        line_num: 14,
        kind: Some(
            Error,
            Error,
        ),
        msg: "14:9: 14:29: demangling(<impl1[1c5860ab79c9e305]::foo::Foo>::bar)",
    Error {
        line_num: 32,
        kind: Some(
            Error,
            Error,
        ),
        msg: "32:9: 32:29: symbol-name(_RNvMNtCs2qSCrjELJET_5impl13barNtNtB4_3foo3Foo3baz)",
    Error {
        line_num: 32,
        kind: Some(
            Error,
            Error,
        ),
        msg: "32:9: 32:29: demangling(<impl1[1c5860ab79c9e305]::foo::Foo>::baz)",
    Error {
        line_num: 62,
        kind: Some(
            Error,
            Error,
        ),
        msg: "62:13: 62:33: symbol-name(_RNvXNCNvCs2qSCrjELJET_5impl14mains_0ARDNtB6_3Foop5AssocFG_KCRL0_hvEuNtB6_9AutoTraitEL_j3_NtB2_3Bar6method)",
    Error {
        line_num: 62,
        kind: Some(
            Error,
            Error,
        ),
        msg: "62:13: 62:33: demangling(<[&dyn impl1[1c5860ab79c9e305]::Foo<Assoc = for<'a> extern \"C\" fn(&'a u8, ...)> + impl1[1c5860ab79c9e305]::AutoTrait; 3: usize] as impl1[1c5860ab79c9e305]::main::{closure#1}::Bar>::method)",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 14,
        kind: Some(
            Error,
        ),
        msg: "symbol-name(_RNvMNtCs21hi0yVfW1J_5impl13fooNtB2_3Foo3bar)",
    Error {
        line_num: 14,
        kind: Some(
            Error,
            Error,
        ),
        msg: "demangling(<impl1[17891616a171812d]::foo::Foo>::bar)",
    Error {
        line_num: 32,
        kind: Some(
            Error,
            Error,
        ),
        msg: "symbol-name(_RNvMNtCs21hi0yVfW1J_5impl13barNtNtB4_3foo3Foo3baz)",
    Error {
        line_num: 32,
        kind: Some(
            Error,
            Error,
        ),
        msg: "demangling(<impl1[17891616a171812d]::foo::Foo>::baz)",
    Error {
        line_num: 62,
        kind: Some(
            Error,
            Error,
        ),
        msg: "symbol-name(_RNvXNCNvCs21hi0yVfW1J_5impl14mains_0ARDNtB6_3Foop5AssocFG_KCRL0_hvEuNtB6_9AutoTraitEL_j3_NtB2_3Bar6method)",
    Error {
        line_num: 62,
        kind: Some(
            Error,
            Error,
        ),
        msg: "demangling(<[&dyn impl1[17891616a171812d]::Foo<Assoc = for<'a> extern \"C\" fn(&'a u8, ...)> + impl1[17891616a171812d]::AutoTrait; 3: usize] as impl1[17891616a171812d]::main::{closure#1}::Bar>::method)",
]


thread '[ui] ui/symbol-names/impl1.rs#v0' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1524:13

---- [ui] ui/symbol-names/issue-60925.rs#v0 stdout ----

error in revision `v0`: /checkout/src/test/ui/symbol-names/issue-60925.rs:21: unexpected error: '21:9: 21:29: symbol-name(_RNvMNtCs8dUWfuENynB_11issue_609253fooINtB2_3FooNtNtB4_4llvm3FooE3foo)'

error in revision `v0`: /checkout/src/test/ui/symbol-names/issue-60925.rs:21: unexpected error: '21:9: 21:29: demangling(<issue_60925[5fcbb46c6fac4139]::foo::Foo<issue_60925[5fcbb46c6fac4139]::llvm::Foo>>::foo)'

error in revision `v0`: /checkout/src/test/ui/symbol-names/issue-60925.rs:21: expected error not found: symbol-name(_RNvMNtCs21hi0yVfW1J_11issue_609253fooINtB2_3FooNtNtB4_4llvm3FooE3foo)

error in revision `v0`: /checkout/src/test/ui/symbol-names/issue-60925.rs:21: expected error not found: demangling(<issue_60925[17891616a171812d]::foo::Foo<issue_60925[17891616a171812d]::llvm::Foo>>::foo)

error in revision `v0`: 2 unexpected errors found, 2 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/issue-60925.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "v0" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.v0" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.v0/auxiliary"
    Error {
        line_num: 21,
        kind: Some(
            Error,
            Error,
        ),
        msg: "21:9: 21:29: symbol-name(_RNvMNtCs8dUWfuENynB_11issue_609253fooINtB2_3FooNtNtB4_4llvm3FooE3foo)",
    Error {
        line_num: 21,
        kind: Some(
            Error,
            Error,
        ),
        msg: "21:9: 21:29: demangling(<issue_60925[5fcbb46c6fac4139]::foo::Foo<issue_60925[5fcbb46c6fac4139]::llvm::Foo>>::foo)",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 21,
        kind: Some(
            Error,
        ),
        msg: "symbol-name(_RNvMNtCs21hi0yVfW1J_11issue_609253fooINtB2_3FooNtNtB4_4llvm3FooE3foo)",
    Error {
        line_num: 21,
        kind: Some(
            Error,
            Error,
        ),
        msg: "demangling(<issue_60925[17891616a171812d]::foo::Foo<issue_60925[17891616a171812d]::llvm::Foo>>::foo)",
]


thread '[ui] ui/symbol-names/issue-60925.rs#v0' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1524:13

---- [ui] ui/symbol-names/issue-75326.rs#v0 stdout ----

error in revision `v0`: /checkout/src/test/ui/symbol-names/issue-75326.rs:41: unexpected error: '41:5: 41:25: symbol-name(_RNvXINICsiMBouZZ1iuD_11issue_75326s_0pppEINtB5_3FooppENtB5_9Iterator24nextB5_)'

error in revision `v0`: /checkout/src/test/ui/symbol-names/issue-75326.rs:41: unexpected error: '41:5: 41:25: demangling(<issue_75326[dac9b7624645f95d]::Foo<_, _> as issue_75326[dac9b7624645f95d]::Iterator2>::next)'

error in revision `v0`: /checkout/src/test/ui/symbol-names/issue-75326.rs:41: expected error not found: symbol-name(_RNvXINICs21hi0yVfW1J_11issue_75326s_0pppEINtB5_3FooppENtB5_9Iterator24nextB5_)

error in revision `v0`: /checkout/src/test/ui/symbol-names/issue-75326.rs:41: expected error not found: demangling(<issue_75326[17891616a171812d]::Foo<_, _> as issue_75326[17891616a171812d]::Iterator2>::next)

error in revision `v0`: 2 unexpected errors found, 2 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/issue-75326.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "v0" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-75326.v0" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-75326.v0/auxiliary"
    Error {
        line_num: 41,
        kind: Some(
            Error,
            Error,
        ),
        msg: "41:5: 41:25: symbol-name(_RNvXINICsiMBouZZ1iuD_11issue_75326s_0pppEINtB5_3FooppENtB5_9Iterator24nextB5_)",
    Error {
        line_num: 41,
        kind: Some(
            Error,
            Error,
        ),
        msg: "41:5: 41:25: demangling(<issue_75326[dac9b7624645f95d]::Foo<_, _> as issue_75326[dac9b7624645f95d]::Iterator2>::next)",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 41,
        kind: Some(
            Error,
        ),
        msg: "symbol-name(_RNvXINICs21hi0yVfW1J_11issue_75326s_0pppEINtB5_3FooppENtB5_9Iterator24nextB5_)",
    Error {
        line_num: 41,
        kind: Some(
            Error,
            Error,
        ),
        msg: "demangling(<issue_75326[17891616a171812d]::Foo<_, _> as issue_75326[17891616a171812d]::Iterator2>::next)",
]


thread '[ui] ui/symbol-names/issue-75326.rs#v0' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1524:13

failures:
    [ui] ui/lto-duplicate-symbols.rs
    [ui] ui/symbol-names/basic.rs#v0
---
test result: FAILED. 11831 passed; 6 failed; 97 ignored; 0 measured; 0 filtered out; finished in 119.12s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:19
