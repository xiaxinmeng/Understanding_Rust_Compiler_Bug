plain
.................................................................................................... 500/11280
................................................................................i................... 600/11280
....................................................i............................................... 700/11280
.................................................................................................... 800/11280
.........................F..F....F......F..........F........FF.FF..F............F................... 900/11280
.......F..........................................................................F..F.............. 1000/11280
..............................................................FF..i.......................FFF....... 1100/11280
F................................................................................................... 1200/11280
...............................ii.i..............i.................................................. 1400/11280
.................................................................................................... 1500/11280
.................................................................................................... 1600/11280
.................................................................................................... 1700/11280
---
.................................................................................................... 2300/11280
.................................................................................................... 2400/11280
..................F................................................................................. 2500/11280
.....................i..i........................................................................... 2600/11280
...................................................................................................F 2700/11280
......F...FF............iiiii....................................................................... 2800/11280
.................................................................................................... 3000/11280
.......................................................F............................................ 3100/11280
.................................................................................................... 3200/11280
.................................................................................................... 3300/11280
---
.................................................................................................... 6800/11280
.................................................................................................... 6900/11280
....................i............................................................................... 7000/11280
...................ii................i.i..ii........................................................ 7100/11280
............F..F.....................................................................F.............. 7200/11280
.............................................F...................................................... 7300/11280
..................................................................i..ii............................. 7500/11280
..................................ii................................................................ 7600/11280
......................................................i............................................. 7700/11280
.......................................i............................................................ 7800/11280
---
............................................................................i......i................ 9300/11280
.................................................................................................... 9400/11280
..............iiiiii..iiiiii.i...................................................................... 9500/11280
.................................................................................................... 9600/11280
............................F.F..................................................................... 9700/11280
.................................................................................................... 9900/11280
.................................................................................................... 10000/11280
................................................................................................F... 10100/11280
.........................................................................................i.......... 10200/11280
.........................................................................................i.......... 10200/11280
.................................................................................................... 10300/11280
.................................................................................................... 10400/11280
.................................................................................................... 10500/11280
........................F........................................................................... 10600/11280
......................................i............................................................. 10700/11280
............................................................................F....................... 10800/11280
................F..............FF................................................................... 10900/11280
.................................................................................................... 11100/11280
......................................................................i.i........................... 11200/11280
................................................................................
failures:
failures:

---- [ui] ui/borrowck/borrow-immutable-upvar-mutation-impl-trait.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrow-immutable-upvar-mutation-impl-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrow-immutable-upvar-mutation-impl-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrow-immutable-upvar-mutation-impl-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `PlaceRef { local: _1, projection: [Deref] }`,
 right: `PlaceRef { local: _1, projection: [Deref, Field(field[0], usize)] }`', compiler/rustc_mir/src/borrow_check/diagnostics/mutability_errors.rs:40:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (01d122ab8 2021-01-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `bar::{closure#0}`
#1 [mir_borrowck] borrow-checking `bar`
end of query stack
------------------------------------------


---- [ui] ui/borrowck/borrow-immutable-upvar-mutation.rs stdout ----
---- [ui] ui/borrowck/borrow-immutable-upvar-mutation.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrow-immutable-upvar-mutation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrow-immutable-upvar-mutation" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrow-immutable-upvar-mutation/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `PlaceRef { local: _1, projection: [Deref] }`,
 right: `PlaceRef { local: _1, projection: [Deref, Field(field[0], &mut i32), Deref] }`', compiler/rustc_mir/src/borrow_check/diagnostics/mutability_errors.rs:40:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (01d122ab8 2021-01-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `main::{closure#0}`
#1 [mir_borrowck] borrow-checking `main`
end of query stack
------------------------------------------


---- [ui] ui/borrowck/borrow-raw-address-of-mutability.rs stdout ----
---- [ui] ui/borrowck/borrow-raw-address-of-mutability.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrow-raw-address-of-mutability.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrow-raw-address-of-mutability" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrow-raw-address-of-mutability/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
   |
LL |     let x = 0;
LL |     let x = 0;
   |         - help: consider changing this to be mutable: `mut x`
LL |     let y = &raw mut x;                 //~ ERROR cannot borrow
   |             ^^^^^^^^^^ cannot borrow as mutable
thread 'rustc' panicked at 'assertion failed: `(left == right)`
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `PlaceRef { local: _1, projection: [Deref, Field(field[0], &mut i32)] }`,
 right: `PlaceRef { local: _1, projection: [Deref, Field(field[0], &mut i32), Deref] }`', compiler/rustc_mir/src/borrow_check/diagnostics/mutability_errors.rs:40:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (01d122ab8 2021-01-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `mutable_address_of_closure::{closure#0}`
#1 [mir_borrowck] borrow-checking `mutable_address_of_closure`
end of query stack

For more information about this error, try `rustc --explain E0596`.

------------------------------------------
------------------------------------------


---- [ui] ui/borrowck/borrowck-access-permissions.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-access-permissions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-access-permissions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-access-permissions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `PlaceRef { local: _12, projection: [] }`,
 right: `PlaceRef { local: _12, projection: [Deref] }`', compiler/rustc_mir/src/borrow_check/diagnostics/mutability_errors.rs:40:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (01d122ab8 2021-01-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `main`
#1 [analysis] running analysis passes on this crate
end of query stack
------------------------------------------


---- [ui] ui/borrowck/borrowck-assign-to-andmut-in-aliasable-loc.rs stdout ----
---- [ui] ui/borrowck/borrowck-assign-to-andmut-in-aliasable-loc.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-assign-to-andmut-in-aliasable-loc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-assign-to-andmut-in-aliasable-loc" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-assign-to-andmut-in-aliasable-loc/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `PlaceRef { local: _1, projection: [Deref] }`,
 right: `PlaceRef { local: _1, projection: [Deref, Field(field[0], &mut isize), Deref] }`', compiler/rustc_mir/src/borrow_check/diagnostics/mutability_errors.rs:40:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (01d122ab8 2021-01-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `a`
#1 [analysis] running analysis passes on this crate
end of query stack
------------------------------------------


---- [ui] ui/borrowck/borrowck-borrow-mut-base-ptr-in-aliasable-loc.rs stdout ----
---- [ui] ui/borrowck/borrowck-borrow-mut-base-ptr-in-aliasable-loc.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-borrow-mut-base-ptr-in-aliasable-loc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrow-mut-base-ptr-in-aliasable-loc" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrow-mut-base-ptr-in-aliasable-loc/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `PlaceRef { local: _2, projection: [Deref] }`,
 right: `PlaceRef { local: _2, projection: [Deref, Deref] }`', compiler/rustc_mir/src/borrow_check/diagnostics/mutability_errors.rs:40:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (01d122ab8 2021-01-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `foo`
#1 [analysis] running analysis passes on this crate
end of query stack
------------------------------------------


---- [ui] ui/borrowck/borrowck-borrow-immut-deref-of-box-as-mut.rs stdout ----
---- [ui] ui/borrowck/borrowck-borrow-immut-deref-of-box-as-mut.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-borrow-immut-deref-of-box-as-mut.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrow-immut-deref-of-box-as-mut" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrow-immut-deref-of-box-as-mut/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `PlaceRef { local: _1, projection: [] }`,
 right: `PlaceRef { local: _1, projection: [Deref] }`', compiler/rustc_mir/src/borrow_check/diagnostics/mutability_errors.rs:40:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (01d122ab8 2021-01-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `main`
#1 [analysis] running analysis passes on this crate
end of query stack
------------------------------------------


---- [ui] ui/borrowck/borrowck-borrow-from-stack-variable.rs stdout ----
---- [ui] ui/borrowck/borrowck-borrow-from-stack-variable.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-borrow-from-stack-variable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrow-from-stack-variable" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrow-from-stack-variable/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0499]: cannot borrow `foo.bar1` as mutable more than once at a time
   |
   |
LL |     let bar1 = &mut foo.bar1;
   |                ------------- first mutable borrow occurs here
LL |     let _bar2 = &mut foo.bar1;  //~ ERROR cannot borrow
   |                 ^^^^^^^^^^^^^ second mutable borrow occurs here
LL |     *bar1;
   |     ----- first borrow later used here

error[E0502]: cannot borrow `foo.bar1` as immutable because it is also borrowed as mutable
   |
   |
LL |     let bar1 = &mut foo.bar1;
   |                ------------- mutable borrow occurs here
LL |     let _bar2 = &foo.bar1;  //~ ERROR cannot borrow
   |                 ^^^^^^^^^ immutable borrow occurs here
LL |     *bar1;
   |     ----- mutable borrow later used here

error[E0502]: cannot borrow `foo.bar1` as mutable because it is also borrowed as immutable
   |
   |
LL |     let bar1 = &foo.bar1;
   |                --------- immutable borrow occurs here
LL |     let _bar2 = &mut foo.bar1;  //~ ERROR cannot borrow
   |                 ^^^^^^^^^^^^^ mutable borrow occurs here
LL |     *bar1;
   |     ----- immutable borrow later used here

error[E0499]: cannot borrow `foo.bar1` as mutable more than once at a time
   |
   |
LL |     let bar1 = &mut foo.bar1;
   |                ------------- first mutable borrow occurs here
LL |     match foo {
LL |         Foo { bar1: ref mut _bar1, bar2: _ } => {} //
   |                     ^^^^^^^^^^^^^ second mutable borrow occurs here
LL |     *bar1;
LL |     *bar1;
   |     ----- first borrow later used here

error[E0502]: cannot borrow `foo.bar1` as immutable because it is also borrowed as mutable
   |
   |
LL |     let bar1 = &mut foo.bar1.int1;
   |                ------------------ mutable borrow occurs here
LL |     let _foo1 = &foo.bar1; //~ ERROR cannot borrow
   |                 ^^^^^^^^^ immutable borrow occurs here
LL |     let _foo2 = &foo; //~ ERROR cannot borrow
LL |     *bar1;
   |     ----- mutable borrow later used here

error[E0502]: cannot borrow `foo` as immutable because it is also borrowed as mutable
   |
   |
LL |     let bar1 = &mut foo.bar1.int1;
   |                ------------------ mutable borrow occurs here
LL |     let _foo1 = &foo.bar1; //~ ERROR cannot borrow
LL |     let _foo2 = &foo; //~ ERROR cannot borrow
   |                 ^^^^ immutable borrow occurs here
LL |     *bar1;
   |     ----- mutable borrow later used here

error[E0499]: cannot borrow `foo.bar1` as mutable more than once at a time
   |
   |
LL |     let bar1 = &mut foo.bar1.int1;
   |                ------------------ first mutable borrow occurs here
LL |     let _foo1 = &mut foo.bar1; //~ ERROR cannot borrow
   |                 ^^^^^^^^^^^^^ second mutable borrow occurs here
LL |     *bar1;
   |     ----- first borrow later used here

error[E0499]: cannot borrow `foo` as mutable more than once at a time
   |
   |
LL |     let bar1 = &mut foo.bar1.int1;
   |                ------------------ first mutable borrow occurs here
LL |     let _foo2 = &mut foo; //~ ERROR cannot borrow
   |                 ^^^^^^^^ second mutable borrow occurs here
LL |     *bar1;
   |     ----- first borrow later used here

error[E0502]: cannot borrow `foo.bar1` as mutable because it is also borrowed as immutable
   |
   |
LL |     let bar1 = &foo.bar1.int1;
   |                -------------- immutable borrow occurs here
LL |     let _foo1 = &mut foo.bar1; //~ ERROR cannot borrow
   |                 ^^^^^^^^^^^^^ mutable borrow occurs here
LL |     *bar1;
   |     ----- immutable borrow later used here

error[E0502]: cannot borrow `foo` as mutable because it is also borrowed as immutable
   |
   |
LL |     let bar1 = &foo.bar1.int1;
   |                -------------- immutable borrow occurs here
LL |     let _foo2 = &mut foo; //~ ERROR cannot borrow
   |                 ^^^^^^^^ mutable borrow occurs here
LL |     *bar1;
   |     ----- immutable borrow later used here
thread 'rustc' panicked at 'assertion failed: `(left == right)`
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `PlaceRef { local: _1, projection: [] }`,
 right: `PlaceRef { local: _1, projection: [Field(field[0], Bar)] }`', compiler/rustc_mir/src/borrow_check/diagnostics/mutability_errors.rs:40:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (01d122ab8 2021-01-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `borrow_mut_from_imm`
#1 [analysis] running analysis passes on this crate
end of query stack

Some errors have detailed explanations: E0499, E0502.
For more information about an error, try `rustc --explain E0499`.


------------------------------------------


---- [ui] ui/borrowck/borrowck-borrow-from-owned-ptr.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-borrow-from-owned-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrow-from-owned-ptr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrow-from-owned-ptr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0499]: cannot borrow `foo.bar1` as mutable more than once at a time
   |
   |
LL |     let bar1 = &mut foo.bar1;
   |                ------------- first mutable borrow occurs here
LL |     let _bar2 = &mut foo.bar1;  //~ ERROR cannot borrow
   |                 ^^^^^^^^^^^^^ second mutable borrow occurs here
LL |     *bar1;
   |     ----- first borrow later used here

error[E0502]: cannot borrow `foo.bar1` as immutable because it is also borrowed as mutable
   |
   |
LL |     let bar1 = &mut foo.bar1;
   |                ------------- mutable borrow occurs here
LL |     let _bar2 = &foo.bar1;  //~ ERROR cannot borrow
   |                 ^^^^^^^^^ immutable borrow occurs here
LL |     *bar1;
   |     ----- mutable borrow later used here

error[E0502]: cannot borrow `foo.bar1` as mutable because it is also borrowed as immutable
   |
   |
LL |     let bar1 = &foo.bar1;
   |                --------- immutable borrow occurs here
LL |     let _bar2 = &mut foo.bar1;  //~ ERROR cannot borrow
   |                 ^^^^^^^^^^^^^ mutable borrow occurs here
LL |     *bar1;
   |     ----- immutable borrow later used here

error[E0499]: cannot borrow `foo.bar1` as mutable more than once at a time
   |
   |
LL |     let bar1 = &mut foo.bar1;
   |                ------------- first mutable borrow occurs here
LL |     match *foo {
LL |         Foo { bar1: ref mut _bar1, bar2: _ } => {}
   |                     ^^^^^^^^^^^^^ second mutable borrow occurs here
LL |     *bar1;
LL |     *bar1;
   |     ----- first borrow later used here

error[E0502]: cannot borrow `foo.bar1` as immutable because it is also borrowed as mutable
   |
   |
LL |     let bar1 = &mut foo.bar1.int1;
   |                ------------------ mutable borrow occurs here
LL |     let _foo1 = &foo.bar1; //~ ERROR cannot borrow
   |                 ^^^^^^^^^ immutable borrow occurs here
LL |     let _foo2 = &*foo; //~ ERROR cannot borrow
LL |     *bar1;
   |     ----- mutable borrow later used here

error[E0502]: cannot borrow `*foo` as immutable because it is also borrowed as mutable
   |
   |
LL |     let bar1 = &mut foo.bar1.int1;
   |                ------------------ mutable borrow occurs here
LL |     let _foo1 = &foo.bar1; //~ ERROR cannot borrow
LL |     let _foo2 = &*foo; //~ ERROR cannot borrow
   |                 ^^^^^ immutable borrow occurs here
LL |     *bar1;
   |     ----- mutable borrow later used here

error[E0499]: cannot borrow `foo.bar1` as mutable more than once at a time
   |
   |
LL |     let bar1 = &mut foo.bar1.int1;
   |                ------------------ first mutable borrow occurs here
LL |     let _foo1 = &mut foo.bar1; //~ ERROR cannot borrow
   |                 ^^^^^^^^^^^^^ second mutable borrow occurs here
LL |     *bar1;
   |     ----- first borrow later used here

error[E0499]: cannot borrow `*foo` as mutable more than once at a time
---
test result: FAILED. 11146 passed; 47 failed; 87 ignored; 0 measured; 0 filtered out; finished in 132.03s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:49
