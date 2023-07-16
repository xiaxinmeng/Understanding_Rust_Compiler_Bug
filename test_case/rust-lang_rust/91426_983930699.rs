plain
.................................................................................................... 100/12431
...........................................iiiiiiiiiii............i....................i............ 200/12431
.................................................................................................... 300/12431
.................................................................................................... 400/12431
...........................F..F.......................F.F........................................... 500/12431
.......................................................FF......FFF............FFFF.FF..F.F......F... 600/12431
...F............................................F....F............................................i. 700/12431
...F.F....................................................................i......................... 800/12431
.................................................................................................... 1000/12431
.................................................................................................... 1100/12431
.................................................................................................... 1200/12431
.i.................................................................................................. 1300/12431
---
.i.................................................................................................. 1900/12431
.................................................................................................... 2000/12431
.....................................................................i..........................F... 2100/12431
.................................................................................................... 2200/12431
F......F............................................................................................ 2300/12431
.................................................................................................... 2500/12431
.................................................................................................... 2600/12431
.................................................................................................... 2600/12431
..........................................F....F...F....F........................................... 2700/12431
...........................................................................................F........ 2800/12431
...............................................................................F.................... 3000/12431
...............................................................................F.................... 3000/12431
.......................iiiii.....................................................F.F................ 3100/12431
.................................................................................................... 3300/12431
.................................................................................................... 3400/12431
.................................................................................................... 3500/12431
.....................................................................i........i.........i........... 3600/12431
.....................................................................i........i.........i........... 3600/12431
.................................................................................................... 3700/12431
............................................ii...................................................... 3800/12431
...................................................................i................................ 3900/12431
..............................................................F..FF...F..F.....F...........F.FF..... 4000/12431
..F................F.F...F..F.F.........F.F..FF..................................................... 4100/12431
.................................................................................................... 4300/12431
.................................................................................................... 4400/12431
.................................................................................................... 4500/12431
.................................................................................................... 4600/12431
---
..............................................ii................................ii.................. 7100/12431
......................................i............................................................. 7200/12431
..................................................................................................i. 7300/12431
.................................................................................................... 7400/12431
.................................FFFF.....F.....F.F.FFFF..FF.F......................F............... 7500/12431
...................F.............................ii................i....i..ii....................... 7600/12431
.................................................................................................... 7800/12431
.................................................................................................... 7900/12431
................................................................................................i..i 8000/12431
i.............................................................ii.................................... 8100/12431
---
.................................................................................................... 9600/12431
.................................................................................................... 9700/12431
.................................................................................................... 9800/12431
.................................................................................................... 9900/12431
..........................F........F.F....F.F....F.........F.....F.................................. 10000/12431
.....................i.............................................................................. 10200/12431
.................iiiiii..i.iiiiii.i................................................................. 10300/12431
..................................................................................FF................ 10400/12431
.................................................................................................... 10500/12431
---
failures:

---- [ui] ui/associated-types/defaults-cyclic-fail-1.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/defaults-cyclic-fail-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-cyclic-fail-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-cyclic-fail-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0275]: overflow evaluating the requirement `<bool as Tr>::B == _`
   |
   |
LL |     type A = Box<Self::B>;

free(): invalid pointer

------------------------------------------
------------------------------------------


---- [ui] ui/associated-types/defaults-cyclic-fail-2.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 11 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/defaults-cyclic-fail-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-cyclic-fail-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-cyclic-fail-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0275]: overflow evaluating the requirement `<bool as Tr>::B == _`
   |
   |
LL |     type A = Box<Self::B>;


/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x949073)[0x7f3af9655073]
/lib/x86_64-linux-gnu/libc.so.6(+0x46210)[0x7f3af89bb210]
/lib/x86_64-linux-gnu/libc.so.6(+0x9af0b)[0x7f3af8a0ff0b]
/lib/x86_64-linux-gnu/libc.so.6(__libc_malloc+0x1b9)[0x7f3af8a12419]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x2cb2ee4)[0x7f3afb9beee4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x2b961fb)[0x7f3afb8a21fb]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(_RNvNtNtCscKwZgGSxNnu_21rustc_trait_selection6traits2wf11obligations+0x130)[0x7f3afb89fdc0]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(_RNvMs2_NtNtCscKwZgGSxNnu_21rustc_trait_selection6traits7fulfillNtB5_16FulfillProcessor28progress_changed_obligations+0x5d1)[0x7f3afb933a21]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x2c60e2e)[0x7f3afb96ce2e]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x2c2612a)[0x7f3afb93212a]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(_RNvXs0_NtNtCscKwZgGSxNnu_21rustc_trait_selection6traits7fulfillNtB5_18FulfillmentContextNtNtNtCsdl6HVa6FMGo_11rustc_infer6traits6engine11TraitEngine36select_with_constness_where_possible+0x31)[0x7f3afb933031]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(_RNvXs0_NtNtCscKwZgGSxNnu_21rustc_trait_selection6traits7fulfillNtB5_18FulfillmentContextNtNtNtCsdl6HVa6FMGo_11rustc_infer6traits6engine11TraitEngine34select_all_with_constness_or_error+0x18)[0x7f3afb932ea8]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x10d6e37)[0x7f3af9de2e37]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x125c25a)[0x7f3af9f6825a]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x135ec65)[0x7f3afa06ac65]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x130db5d)[0x7f3afa019b5d]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x12fef9d)[0x7f3afa00af9d]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x1e8f958)[0x7f3afab9b958]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x1f98780)[0x7f3afaca4780]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(_RNvXs3_NtNtCs74uJiubMvTs_12rustc_typeck5check7wfcheckNtB5_26CheckTypeWellFormedVisitorNtNtCscCIE4J16uxr_9rustc_hir10intravisit7Visitor15visit_impl_item+0x159)[0x7f3afa01fcc9]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x12f5c00)[0x7f3afa001c00]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x11a67a6)[0x7f3af9eb27a6]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x12e20ef)[0x7f3af9fee0ef]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(_RNvNtNtCs74uJiubMvTs_12rustc_typeck5check5check12check_wf_new+0x1d)[0x7f3afa085edd]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x1216232)[0x7f3af9f22232]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(_RNvCs74uJiubMvTs_12rustc_typeck11check_crate+0xd2)[0x7f3af9ef18a2]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(_RNvNtCsEbFNim0J46_15rustc_interface6passes8analysis+0x61)[0x7f3af9826171]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x1ec3a1a)[0x7f3afabcfa1a]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x1fae470)[0x7f3afacba470]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x92ccc2)[0x7f3af9638cc2]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x99c3e3)[0x7f3af96a83e3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x98bebb)[0x7f3af9697ebb]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x9c1ad3)[0x7f3af96cdad3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x92eace)[0x7f3af963aace]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-0e8ed714a14d9524.so(+0x94658a)[0x7f3af965258a]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-d43b7595eae8ead4.so(rust_metadata_std_cf3cc7aa1f647c1+0xc2ce3)[0x7f3af8c2fce3]
/lib/x86_64-linux-gnu/libpthread.so.0(+0x9609)[0x7f3af2fa6609]
/lib/x86_64-linux-gnu/libc.so.6(clone+0x43)[0x7f3af8a97293]
------------------------------------------


---- [ui] ui/associated-types/impl-wf-cycle-1.rs stdout ----
---- [ui] ui/associated-types/impl-wf-cycle-1.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/impl-wf-cycle-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/impl-wf-cycle-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/impl-wf-cycle-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0275]: overflow evaluating the requirement `<(T,) as Grault>::A == _`
   |
   |
LL | / impl<T: Grault> Grault for (T,)
LL | | where
LL | |     Self::A: Baz,
LL | |     Self::B: Fiz,
...  |
LL | |     //~^ ERROR overflow evaluating the requirement `<(T,) as Grault>::A == _`
LL | | }
   |
   |
note: required because of the requirements on the impl of `Grault` for `(T,)`
   |
   |
LL | impl<T: Grault> Grault for (T,)
   |                 ^^^^^^     ^^^^
   = note: 1 redundant requirement hidden
   = note: required because of the requirements on the impl of `Grault` for `(T,)`
malloc_consolidate(): invalid chunk size

------------------------------------------



---- [ui] ui/associated-types/impl-wf-cycle-2.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/impl-wf-cycle-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/impl-wf-cycle-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/impl-wf-cycle-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0275]: overflow evaluating the requirement `<(T,) as Grault>::A == _`
   |
   |
LL | / impl<T: Grault> Grault for (T,)
LL | | where
LL | |     Self::A: Copy,
LL | | {
LL | |     type A = ();
LL | |     //~^ ERROR overflow evaluating the requirement `<(T,) as Grault>::A == _`
LL | | }
   |
   |
note: required because of the requirements on the impl of `Grault` for `(T,)`
   |
   |
LL | impl<T: Grault> Grault for (T,)
   |                 ^^^^^^     ^^^^
malloc_consolidate(): invalid chunk size

------------------------------------------



---- [ui] ui/async-await/async-fn-size-uninit-locals.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-fn-size-uninit-locals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-size-uninit-locals/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-size-uninit-locals/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
free(): double free detected in tcache 2
------------------------------------------


---- [ui] ui/async-await/async-fn-size-moved-locals.rs stdout ----
---- [ui] ui/async-await/async-fn-size-moved-locals.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-fn-size-moved-locals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-size-moved-locals/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-size-moved-locals/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
free(): double free detected in tcache 2
------------------------------------------


---- [ui] ui/async-await/async-await.rs#nomiropt stdout ----
---- [ui] ui/async-await/async-await.rs#nomiropt stdout ----

error in revision `nomiropt`: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nomiropt" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.nomiropt/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "mir-opt-level=0" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.nomiropt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
free(): double free detected in tcache 2
------------------------------------------


---- [ui] ui/async-await/async-await.rs#thirunsafeck stdout ----
---- [ui] ui/async-await/async-await.rs#thirunsafeck stdout ----

error in revision `thirunsafeck`: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thirunsafeck" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.thirunsafeck/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zthir-unsafeck" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.thirunsafeck/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
free(): double free detected in tcache 2
------------------------------------------


---- [ui] ui/async-await/async-await.rs#default stdout ----
---- [ui] ui/async-await/async-await.rs#default stdout ----

error in revision `default`: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "default" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.default/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
free(): double free detected in tcache 2
------------------------------------------


---- [ui] ui/async-await/drop-order/drop-order-for-locals-when-cancelled.rs stdout ----
---- [ui] ui/async-await/drop-order/drop-order-for-locals-when-cancelled.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/drop-order/drop-order-for-locals-when-cancelled.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-locals-when-cancelled/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-locals-when-cancelled/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
free(): double free detected in tcache 2
------------------------------------------


---- [ui] ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.rs#default stdout ----
---- [ui] ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.rs#default stdout ----

error in revision `default`: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "default" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.default/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
free(): double free detected in tcache 2
------------------------------------------


---- [ui] ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.rs#nomiropt stdout ----
---- [ui] ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.rs#nomiropt stdout ----

error in revision `nomiropt`: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nomiropt" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.nomiropt/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-Z" "mir-opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.nomiropt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
free(): double free detected in tcache 2
------------------------------------------


---- [ui] ui/async-await/issue-60709.rs stdout ----
---- [ui] ui/async-await/issue-60709.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-60709.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-60709/a" "-Crpath" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Copt-level=z" "-Cdebuginfo=2" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-60709/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
free(): double free detected in tcache 2
------------------------------------------


---- [ui] ui/async-await/issue-61793.rs stdout ----
---- [ui] ui/async-await/issue-61793.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-61793.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61793" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61793/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
free(): double free detected in tcache 2
------------------------------------------


---- [ui] ui/async-await/drop-order/drop-order-for-async-fn-parameters.rs#nomiropt stdout ----
---- [ui] ui/async-await/drop-order/drop-order-for-async-fn-parameters.rs#nomiropt stdout ----

error in revision `nomiropt`: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nomiropt" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters.nomiropt/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-Z" "mir-opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters.nomiropt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
free(): double free detected in tcache 2
------------------------------------------


---- [ui] ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs stdout ----
---- [ui] ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
free(): double free detected in tcache 2
------------------------------------------


---- [ui] ui/async-await/drop-order/drop-order-for-async-fn-parameters.rs#default stdout ----
---- [ui] ui/async-await/drop-order/drop-order-for-async-fn-parameters.rs#default stdout ----

error in revision `default`: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "default" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters.default/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters.default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
free(): double free detected in tcache 2
------------------------------------------


---- [ui] ui/async-await/drop-order/drop-order-when-cancelled.rs#default stdout ----
---- [ui] ui/async-await/drop-order/drop-order-when-cancelled.rs#default stdout ----

error in revision `default`: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/drop-order/drop-order-when-cancelled.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "default" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-when-cancelled.default/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-when-cancelled.default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
free(): double free detected in tcache 2
------------------------------------------


---- [ui] ui/async-await/drop-order/drop-order-when-cancelled.rs#nomiropt stdout ----
---- [ui] ui/async-await/drop-order/drop-order-when-cancelled.rs#nomiropt stdout ----

error in revision `nomiropt`: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/drop-order/drop-order-when-cancelled.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nomiropt" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-when-cancelled.nomiropt/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-Z" "mir-opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-when-cancelled.nomiropt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
free(): double free detected in tcache 2
------------------------------------------


---- [ui] ui/async-await/issues/issue-55809.rs stdout ----
---- [ui] ui/async-await/issues/issue-55809.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-55809.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-55809/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-55809/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
free(): double free detected in tcache 2
------------------------------------------


---- [ui] ui/async-await/issue-72470-llvm-dominate.rs stdout ----
---- [ui] ui/async-await/issue-72470-llvm-dominate.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-72470-llvm-dominate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-72470-llvm-dominate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "opt-level=3" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-72470-llvm-dominate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
free(): double free detected in tcache 2
------------------------------------------


---- [ui] ui/async-await/large_moves.rs#attribute stdout ----
---- [ui] ui/async-await/large_moves.rs#attribute stdout ----

error in revision `attribute`: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/large_moves.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "attribute" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/large_moves.attribute" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/large_moves.attribute/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: moving 10024 bytes
  --> /checkout/src/test/ui/async-await/large_moves.rs:12:13
   |
LL |       let x = async { //~ ERROR large_assignments
   |  _____________^
LL | |         let y = [0; 9999];
LL | |         dbg!(y);
LL | |         thing(&y).await;
LL | |         dbg!(y);
LL | |     };
   | |_____^ value moved from here
note: the lint level is defined here
  --> /checkout/src/test/ui/async-await/large_moves.rs:1:9
   |
LL | #![deny(large_assignments)]
LL | #![deny(large_assignments)]
   |         ^^^^^^^^^^^^^^^^^

error: moving 10024 bytes
  --> /checkout/src/test/ui/async-await/large_moves.rs:18:14
   |
LL |     let z = (x, 42); //~ ERROR large_assignments
   |              ^ value moved from here
error: moving 10024 bytes
  --> /checkout/src/test/ui/async-await/large_moves.rs:18:13
   |
   |
LL |     let z = (x, 42); //~ ERROR large_assignments
   |             ^^^^^^^ value moved from here
error: moving 10024 bytes
  --> /checkout/src/test/ui/async-await/large_moves.rs:20:13
   |
   |
LL |     let a = z.0; //~ ERROR large_assignments
   |             ^^^ value moved from here

free(): double free detected in tcache 2
------------------------------------------


---- [ui] ui/async-await/large_moves.rs#option stdout ----
---- [ui] ui/async-await/large_moves.rs#option stdout ----

error in revision `option`: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (core dumped)
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/large_moves.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "option" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/large_moves.option" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zmove-size-limit=1000" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/large_moves.option/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: moving 10024 bytes
  --> /checkout/src/test/ui/async-await/large_moves.rs:12:13
   |
LL |       let x = async { //~ ERROR large_assignments
   |  _____________^
LL | |         let y = [0; 9999];
LL | |         dbg!(y);
LL | |         thing(&y).await;
LL | |         dbg!(y);
LL | |     };
   | |_____^ value moved from here
---
test result: FAILED. 12231 passed; 85 failed; 115 ignored; 0 measured; 0 filtered out; finished in 141.74s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:18
