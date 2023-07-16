plain
.................................................................................................... 3300/12161
.................................................................................................... 3400/12161
.................................................................................................... 3500/12161
..............................i........i.........i.................................................. 3600/12161
.............................................................................F....................ii 3700/12161
................i............................................i...................................... 3900/12161
.................................................................................................... 4000/12161
.................................................................................................... 4100/12161
.................................................................................................... 4200/12161
---
failures:

---- [ui] ui/consts/const-fn-not-in-trait.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-fn-not-in-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-fn-not-in-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-fn-not-in-trait/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------

------------------------------------------
stderr:
---

error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/consts/const-fn-not-in-trait.rs:7:5
   |
LL |     const fn g() -> u32 {

thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `NotConst`,
  left: `NotConst`,
 right: `Const`: trait fns cannot be marked const', compiler/rustc_hir/src/hir.rs:3085:17

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (d89339a99 2021-07-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [explicit_predicates_of] computing explicit predicates of `Foo::f`
#1 [predicates_defined_on] computing predicates of `Foo::f`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0379`.


------------------------------------------


---- [ui] ui/feature-gates/feature-gate-min_const_fn.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-min_const_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-min_const_fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-min_const_fn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/feature-gates/feature-gate-min_const_fn.rs:6:5
   |
LL |     const fn foo() -> u32; //~ ERROR functions in traits cannot be declared const

error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/feature-gates/feature-gate-min_const_fn.rs:7:5
   |
   |
LL |     const fn bar() -> u32 { 0 } //~ ERROR functions in traits cannot be declared const

error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/feature-gates/feature-gate-min_const_fn.rs:11:5
   |
   |
LL |     const fn foo() -> u32 { 0 } //~ ERROR functions in traits cannot be declared const

thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `NotConst`,
  left: `NotConst`,
 right: `Const`: trait fns cannot be marked const', compiler/rustc_hir/src/hir.rs:3085:17

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (d89339a99 2021-07-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [explicit_predicates_of] computing explicit predicates of `Foo::foo`
#1 [predicates_defined_on] computing predicates of `Foo::foo`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0379`.


------------------------------------------


---- [ui] ui/issues/issue-54954.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-54954.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54954" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54954/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/issues/issue-54954.rs:5:5
   |
LL |     const fn const_val<T: Sized>() -> usize {

thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `NotConst`,
  left: `NotConst`,
 right: `Const`: trait fns cannot be marked const', compiler/rustc_hir/src/hir.rs:3085:17

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (d89339a99 2021-07-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [explicit_predicates_of] computing explicit predicates of `Tt::const_val`
#1 [predicates_defined_on] computing predicates of `Tt::const_val`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0379`.


------------------------------------------


---- [ui] ui/mismatched_types/const-fn-in-trait.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/const-fn-in-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/const-fn-in-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/const-fn-in-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/mismatched_types/const-fn-in-trait.rs:3:5
   |
LL |     const fn g(); //~ ERROR cannot be declared const

error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/mismatched_types/const-fn-in-trait.rs:7:5
   |
   |
LL |     const fn f() -> u32 { 22 } //~ ERROR cannot be declared const

thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `NotConst`,
  left: `NotConst`,
 right: `Const`: trait fns cannot be marked const', compiler/rustc_hir/src/hir.rs:3085:17

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (d89339a99 2021-07-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [explicit_predicates_of] computing explicit predicates of `Foo::g`
#1 [predicates_defined_on] computing predicates of `Foo::g`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0379`.


------------------------------------------


---- [ui] ui/parser/fn-header-semantic-fail.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/fn-header-semantic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/fn-header-semantic-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/fn-header-semantic-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: functions cannot be both `const` and `async`
   |
   |
LL |     const async unsafe extern "C" fn ff5() {} // OK.
   |     ^^^^^-^^^^^------------------------------
   |     |     |
   |     |     `async` because of this
   |     `const` because of this

error[E0706]: functions in traits cannot be declared `async`
   |
   |
LL |         async fn ft1(); //~ ERROR functions in traits cannot be declared `async`
   |         |
   |         |
   |         `async` because of this
   |
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:18:9
   |
   |
LL |         const fn ft3(); //~ ERROR functions in traits cannot be declared const

error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:20:9
   |
   |
LL |         const async unsafe extern "C" fn ft5();


error[E0706]: functions in traits cannot be declared `async`
   |
   |
LL |         const async unsafe extern "C" fn ft5();
   |               |
   |               |
   |               `async` because of this
   |
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait

error: functions cannot be both `const` and `async`
   |
   |
LL |         const async unsafe extern "C" fn ft5();
   |         ^^^^^-^^^^^----------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this

error[E0706]: functions in traits cannot be declared `async`
   |
   |
LL |         async fn ft1() {} //~ ERROR functions in traits cannot be declared `async`
   |         |
   |         |
   |         `async` because of this
   |
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:31:9
   |
   |
LL |         const fn ft3() {} //~ ERROR functions in traits cannot be declared const

error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:33:9
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}


error[E0706]: functions in traits cannot be declared `async`
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}
   |               |
   |               |
   |               `async` because of this
   |
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait

error: functions cannot be both `const` and `async`
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}
   |         ^^^^^-^^^^^------------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this

error: functions cannot be both `const` and `async`
   |
   |
LL |         const async unsafe extern "C" fn fi5() {}
   |         ^^^^^-^^^^^------------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this

error: functions in `extern` blocks cannot have qualifiers
   |
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
LL |         async fn fe1(); //~ ERROR functions in `extern` blocks cannot have qualifiers
   |
help: remove the qualifiers
   |
   |
LL |         fn fe1(); //~ ERROR functions in `extern` blocks cannot have qualifiers


error: functions in `extern` blocks cannot have qualifiers
   |
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
LL |         async fn fe1(); //~ ERROR functions in `extern` blocks cannot have qualifiers
LL |         unsafe fn fe2(); //~ ERROR functions in `extern` blocks cannot have qualifiers
   |
help: remove the qualifiers
   |
   |
LL |         fn fe2(); //~ ERROR functions in `extern` blocks cannot have qualifiers


error: functions in `extern` blocks cannot have qualifiers
   |
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
...
LL |         const fn fe3(); //~ ERROR functions in `extern` blocks cannot have qualifiers
   |
help: remove the qualifiers
   |
   |
LL |         fn fe3(); //~ ERROR functions in `extern` blocks cannot have qualifiers


error: functions in `extern` blocks cannot have qualifiers
   |
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
...
LL |         extern "C" fn fe4(); //~ ERROR functions in `extern` blocks cannot have qualifiers
   |
help: remove the qualifiers
   |
   |
LL |         fn fe4(); //~ ERROR functions in `extern` blocks cannot have qualifiers


error: functions in `extern` blocks cannot have qualifiers
   |
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
...
LL |         const async unsafe extern "C" fn fe5(); //~ ERROR functions in `extern` blocks
   |
help: remove the qualifiers
   |
   |
LL |         fn fe5(); //~ ERROR functions in `extern` blocks


error: functions cannot be both `const` and `async`
   |
   |
LL |         const async unsafe extern "C" fn fe5(); //~ ERROR functions in `extern` blocks
   |         ^^^^^-^^^^^----------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `NotConst`,
  left: `NotConst`,
 right: `Const`: trait fns cannot be marked const', compiler/rustc_hir/src/hir.rs:3085:17

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (d89339a99 2021-07-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [explicit_predicates_of] computing explicit predicates of `main::X::ft3`
#1 [predicates_defined_on] computing predicates of `main::X::ft3`
error: aborting due to 18 previous errors

Some errors have detailed explanations: E0379, E0706.
For more information about an error, try `rustc --explain E0379`.
---
test result: FAILED. 12054 passed; 5 failed; 102 ignored; 0 measured; 0 filtered out; finished in 127.90s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:50
