plain
.................................................................................................... 2100/12171
.................................................................................................... 2200/12171
.................................................................................................... 2300/12171
.................................................................................................... 2400/12171
........F........................F........................................................F......... 2500/12171
............................................................F....................................... 2600/12171
.................................................................................................... 2800/12171
.................................................................................................... 2900/12171
........iiiii....................................................................................... 3000/12171
.................................................................................................... 3100/12171
---
.................................................................................................... 9300/12171
.................................................................................................... 9400/12171
.................................................................................................... 9500/12171
.................................................................................................... 9600/12171
...........................................................F.FF..F..F.FFF...FF.......F.............. 9700/12171
....................ii.i............................................................................ 9900/12171
.................................................................................................... 10000/12171
..............iiiiii.i...iiiiiii.................................................................... 10100/12171
.................................................................................................... 10200/12171
---
failures:

---- [ui] ui/closures/2229_closure_analysis/match/match-edge-cases_1.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/match/match-edge-cases_1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/match/match-edge-cases_1/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-Zunstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/match/match-edge-cases_1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'when specializing: Opaque is not covered by Opaque', compiler/rustc_mir_build/src/thir/pattern/deconstruct_pat.rs:1458:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
note: rustc 1.57.0-nightly (a9f6e3d5d 2021-09-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_match] match-checking `edge_case_raw_ptr::{closure#0}`
#1 [analysis] running analysis passes on this crate

------------------------------------------



---- [ui] ui/consts/const_in_pattern/issue-44333.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_in_pattern/issue-44333.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/issue-44333/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/issue-44333/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: function pointers and unsized pointers in patterns behave unpredictably and should not be relied upon. See https://github.com/rust-lang/rust/issues/70861 for details.
   |
   |
LL |         FOO => println!("foo"), //~ WARN pointers in patterns behave unpredictably
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const_in_pattern/issue-44333.rs:3:9
   |
   |
LL | #![warn(pointer_structural_match)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #62411 <https://github.com/rust-lang/rust/issues/70861>

warning: function pointers and unsized pointers in patterns behave unpredictably and should not be relied upon. See https://github.com/rust-lang/rust/issues/70861 for details.
   |
   |
LL |         BAR => println!("bar"), //~ WARN pointers in patterns behave unpredictably
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #62411 <https://github.com/rust-lang/rust/issues/70861>


thread 'rustc' panicked at 'when specializing: Opaque is not covered by Opaque', compiler/rustc_mir_build/src/thir/pattern/deconstruct_pat.rs:1458:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (a9f6e3d5d 2021-09-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_match] match-checking `main`
#1 [analysis] running analysis passes on this crate
warning: 2 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/consts/const_in_pattern/reject_non_structural.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_in_pattern/reject_non_structural.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/reject_non_structural" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/reject_non_structural/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: to use a constant of type `NoDerive` in a pattern, `NoDerive` must be annotated with `#[derive(PartialEq, Eq)]`
   |
   |
LL |     match Derive::Some(NoDerive) { ENUM => dbg!(ENUM), _ => panic!("whoops"), };


error: to use a constant of type `NoDerive` in a pattern, `NoDerive` must be annotated with `#[derive(PartialEq, Eq)]`
   |
   |
LL |     match Some(NoDerive) { FIELD => dbg!(FIELD), _ => panic!("whoops"), };


error: to use a constant of type `NoDerive` in a pattern, `NoDerive` must be annotated with `#[derive(PartialEq, Eq)]`
   |
   |
LL |     match Some(NoDerive) {INDIRECT => dbg!(INDIRECT), _ => panic!("whoops"), };


error: to use a constant of type `NoDerive` in a pattern, `NoDerive` must be annotated with `#[derive(PartialEq, Eq)]`
   |
   |
LL |     match (None, Some(NoDerive)) { TUPLE => dbg!(TUPLE), _ => panic!("whoops"), };


error: to use a constant of type `NoDerive` in a pattern, `NoDerive` must be annotated with `#[derive(PartialEq, Eq)]`
   |
   |
LL |     match Some(NoDerive) { TYPE_ASCRIPTION => dbg!(TYPE_ASCRIPTION), _ => panic!("whoops"), };


error: to use a constant of type `NoDerive` in a pattern, `NoDerive` must be annotated with `#[derive(PartialEq, Eq)]`
   |
   |
LL |     match [None, Some(NoDerive)] { ARRAY => dbg!(ARRAY), _ => panic!("whoops"), };


error: to use a constant of type `NoDerive` in a pattern, `NoDerive` must be annotated with `#[derive(PartialEq, Eq)]`
   |
   |
LL |     match [Some(NoDerive); 2] { REPEAT => dbg!(REPEAT), _ => panic!("whoops"), };


error: to use a constant of type `NoDerive` in a pattern, `NoDerive` must be annotated with `#[derive(PartialEq, Eq)]`
   |
   |
LL |     match [Some(NoDerive); 2] { REPEAT => dbg!(REPEAT), _ => panic!("whoops"), };


error: to use a constant of type `NoDerive` in a pattern, `NoDerive` must be annotated with `#[derive(PartialEq, Eq)]`
   |
   |
LL |     match Some(NoDerive) { NoDerive::ASSOC => dbg!(NoDerive::ASSOC), _ => panic!("whoops"), };


error: to use a constant of type `NoDerive` in a pattern, `NoDerive` must be annotated with `#[derive(PartialEq, Eq)]`
   |
   |
LL |     match Some(NoDerive) { BLOCK => dbg!(BLOCK), _ => panic!("whoops"), };


warning: to use a constant of type `NoDerive` in a pattern, `NoDerive` must be annotated with `#[derive(PartialEq, Eq)]`
   |
   |
LL |     match &Some(NoDerive) { ADDR_OF => dbg!(ADDR_OF), _ => panic!("whoops"), };
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const_in_pattern/reject_non_structural.rs:12:9
   |
   |
LL | #![warn(indirect_structural_match)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #62411 <https://github.com/rust-lang/rust/issues/62411>


thread 'rustc' panicked at 'when specializing: Opaque is not covered by Opaque', compiler/rustc_mir_build/src/thir/pattern/deconstruct_pat.rs:1458:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (a9f6e3d5d 2021-09-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_match] match-checking `main`
#1 [analysis] running analysis passes on this crate
error: aborting due to 10 previous errors; 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/consts/issue-34784.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-34784.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-34784/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-34784/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'when specializing: Opaque is not covered by Opaque', compiler/rustc_mir_build/src/thir/pattern/deconstruct_pat.rs:1458:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (a9f6e3d5d 2021-09-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_match] match-checking `foo`
#1 [analysis] running analysis passes on this crate

------------------------------------------



---- [ui] ui/consts/issue-89088.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-89088.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-89088" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-89088/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'when specializing: Opaque is not covered by Opaque', compiler/rustc_mir_build/src/thir/pattern/deconstruct_pat.rs:1458:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (a9f6e3d5d 2021-09-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_match] match-checking `main`
#1 [analysis] running analysis passes on this crate

------------------------------------------



---- [ui] ui/pattern/usefulness/consts-opaque.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/usefulness/consts-opaque.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/consts-opaque" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/consts-opaque/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: to use a constant of type `Foo` in a pattern, `Foo` must be annotated with `#[derive(PartialEq, Eq)]`
   |
LL |         FOO => {}
   |         ^^^


error: unreachable pattern
  --> /checkout/src/test/ui/pattern/usefulness/consts-opaque.rs:32:9
   |
LL |         FOO => {}
   |         --- matches any value
LL |         //~^ ERROR must be annotated with `#[derive(PartialEq, Eq)]`
LL |         _ => {} // should not be emitting unreachable warning
   |         ^ unreachable pattern
note: the lint level is defined here
  --> /checkout/src/test/ui/pattern/usefulness/consts-opaque.rs:6:9
   |
LL | #![deny(unreachable_patterns)]
LL | #![deny(unreachable_patterns)]
   |         ^^^^^^^^^^^^^^^^^^^^

error: to use a constant of type `Foo` in a pattern, `Foo` must be annotated with `#[derive(PartialEq, Eq)]`
   |
   |
LL |         FOO_REF => {}

error: unreachable pattern
  --> /checkout/src/test/ui/pattern/usefulness/consts-opaque.rs:39:9
   |
   |
LL |         FOO_REF => {}
   |         ------- matches any value
LL |         //~^ ERROR must be annotated with `#[derive(PartialEq, Eq)]`
LL |         Foo(_) => {} // should not be emitting unreachable warning
   |         ^^^^^^ unreachable pattern

warning: to use a constant of type `Foo` in a pattern, `Foo` must be annotated with `#[derive(PartialEq, Eq)]`
   |
   |
LL |         FOO_REF_REF => {}
   |
   |
   = note: `#[warn(indirect_structural_match)]` on by default
   = note: for more information, see issue #62411 <https://github.com/rust-lang/rust/issues/62411>


thread 'rustc' panicked at 'when specializing: Opaque is not covered by Opaque', compiler/rustc_mir_build/src/thir/pattern/deconstruct_pat.rs:1458:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (a9f6e3d5d 2021-09-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_match] match-checking `main`
#1 [analysis] running analysis passes on this crate
error: aborting due to 4 previous errors; 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/rfc-1445-restrict-constants-in-patterns/allow-hide-behind-direct-unsafe-ptr-embedded.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1445-restrict-constants-in-patterns/allow-hide-behind-direct-unsafe-ptr-embedded.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1445-restrict-constants-in-patterns/allow-hide-behind-direct-unsafe-ptr-embedded/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1445-restrict-constants-in-patterns/allow-hide-behind-direct-unsafe-ptr-embedded/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'when specializing: Opaque is not covered by Opaque', compiler/rustc_mir_build/src/thir/pattern/deconstruct_pat.rs:1458:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (a9f6e3d5d 2021-09-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_match] match-checking `main`
#1 [analysis] running analysis passes on this crate

------------------------------------------



---- [ui] ui/rfc-1445-restrict-constants-in-patterns/allow-hide-behind-direct-unsafe-ptr-param.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1445-restrict-constants-in-patterns/allow-hide-behind-direct-unsafe-ptr-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1445-restrict-constants-in-patterns/allow-hide-behind-direct-unsafe-ptr-param/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1445-restrict-constants-in-patterns/allow-hide-behind-direct-unsafe-ptr-param/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'when specializing: Opaque is not covered by Opaque', compiler/rustc_mir_build/src/thir/pattern/deconstruct_pat.rs:1458:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (a9f6e3d5d 2021-09-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_match] match-checking `main`
#1 [analysis] running analysis passes on this crate

------------------------------------------



---- [ui] ui/rfc-1445-restrict-constants-in-patterns/allow-hide-behind-indirect-unsafe-ptr-embedded.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1445-restrict-constants-in-patterns/allow-hide-behind-indirect-unsafe-ptr-embedded.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1445-restrict-constants-in-patterns/allow-hide-behind-indirect-unsafe-ptr-embedded/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1445-restrict-constants-in-patterns/allow-hide-behind-indirect-unsafe-ptr-embedded/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'when specializing: Opaque is not covered by Opaque', compiler/rustc_mir_build/src/thir/pattern/deconstruct_pat.rs:1458:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (a9f6e3d5d 2021-09-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_match] match-checking `main`
#1 [analysis] running analysis passes on this crate

------------------------------------------



---- [ui] ui/rfc-1445-restrict-constants-in-patterns/allow-hide-behind-indirect-unsafe-ptr-param.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1445-restrict-constants-in-patterns/allow-hide-behind-indirect-unsafe-ptr-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1445-restrict-constants-in-patterns/allow-hide-behind-indirect-unsafe-ptr-param/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1445-restrict-constants-in-patterns/allow-hide-behind-indirect-unsafe-ptr-param/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'when specializing: Opaque is not covered by Opaque', compiler/rustc_mir_build/src/thir/pattern/deconstruct_pat.rs:1458:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (a9f6e3d5d 2021-09-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_match] match-checking `main`
#1 [analysis] running analysis passes on this crate

------------------------------------------



---- [ui] ui/rfc-1445-restrict-constants-in-patterns/cant-hide-behind-doubly-indirect-param.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1445-restrict-constants-in-patterns/cant-hide-behind-doubly-indirect-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1445-restrict-constants-in-patterns/cant-hide-behind-doubly-indirect-param/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1445-restrict-constants-in-patterns/cant-hide-behind-doubly-indirect-param/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: to use a constant of type `NoDerive` in a pattern, `NoDerive` must be annotated with `#[derive(PartialEq, Eq)]`
   |
   |
LL |         WRAP_DOUBLY_INDIRECT_PARAM => { panic!("WRAP_DOUBLY_INDIRECT_PARAM matched itself"); }
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/rfc-1445-restrict-constants-in-patterns/cant-hide-behind-doubly-indirect-param.rs:7:9
   |
   |
LL | #![warn(indirect_structural_match)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #62411 <https://github.com/rust-lang/rust/issues/62411>


thread 'rustc' panicked at 'when specializing: Opaque is not covered by Opaque', compiler/rustc_mir_build/src/thir/pattern/deconstruct_pat.rs:1458:9

error: internal compiler error: unexpected panic
---
test result: FAILED. 12052 passed; 17 failed; 102 ignored; 0 measured; 0 filtered out; finished in 140.63s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:46
