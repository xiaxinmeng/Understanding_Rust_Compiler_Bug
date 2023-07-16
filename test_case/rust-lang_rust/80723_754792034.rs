plain
.................................................................................................... 9000/11245
.................................................................................................... 9100/11245
.................................................................................................... 9200/11245
.........................................i......i................................................... 9300/11245
................................................................................iiiiii..iiiiii.i.... 9400/11245
.................................................................................................... 9600/11245
.................................................................................................... 9700/11245
.................................................................................................... 9800/11245
.................................................................................................... 9900/11245
---
.................................................................................................... 10500/11245
...F................................................................................................ 10600/11245
..i................................................................................................. 10700/11245
.................................................................................................... 10800/11245
......................F.F....F...................................................................... 10900/11245
.................................................................................................... 11100/11245
....................................i............................................................... 11200/11245
.............................................
failures:
failures:

---- [ui] ui/issues/issue-11820.rs stdout ----
normalized stderr:
warning: call to noop method
   |
   |
LL |   let _: &NoClone = rnc.clone();
   |
   |
   = note: `#[warn(noop_method_call)]` on by default

warning: call to noop method
   |
   |
LL |   let _: &Option<NoClone> = rsnc.clone();

warning: 2 warnings emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11820/issue-11820.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-11820.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-11820.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11820/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11820/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: call to noop method
   |
   |
LL |   let _: &NoClone = rnc.clone();
   |
   |
   = note: `#[warn(noop_method_call)]` on by default

warning: call to noop method
   |
   |
LL |   let _: &Option<NoClone> = rsnc.clone();

warning: 2 warnings emitted



------------------------------------------


---- [ui] ui/issues/issue-37725.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-37725.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37725" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37725/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: Encountered error `Unimplemented` selecting `Binder(<&mut () as Foo>)` during codegen
   = note: delayed at compiler/rustc_trait_selection/src/traits/codegen.rs:68:32


thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:974:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (6ff6e3bc0 2021-01-05) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack

------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-66768.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-66768.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66768" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66768/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Binder(<Matrix<f64, DimU2, DimU1, <DefaultAllocator as Allocator<f64, DimU2>>::Buffer> as std::convert::Into<Point<f64, DimU2>>>)`,
 right: `Binder(<Matrix<f64, DimU2, DimU1, ArrayStorage<f64, DimU2, DimU1>> as std::convert::Into<Point<f64, DimU2>>>)`', compiler/rustc_trait_selection/src/traits/codegen.rs:30:5

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (6ff6e3bc0 2021-01-05) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [codegen_fulfill_obligation] checking if `std::convert::Into` fulfills its obligations
#1 [resolve_instance] resolving instance `<Matrix<f64, DimU2, DimU1, <DefaultAllocator as Allocator<f64, DimU2>>::Buffer> as std::convert::Into<Point<f64, DimU2>>>::into`
end of query stack
------------------------------------------


---- [ui] ui/trivial-bounds/trivial-bounds-inconsistent.rs stdout ----
---- [ui] ui/trivial-bounds/trivial-bounds-inconsistent.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trivial-bounds/trivial-bounds-inconsistent.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/trivial-bounds-inconsistent/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trivial-bounds/trivial-bounds-inconsistent/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
   |
   |
LL | enum E where i32: Foo { V } //~ WARNING trivial_bounds
   |
   |
   = note: `#[warn(trivial_bounds)]` on by default

warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
   |
   |
LL | struct S where i32: Foo; //~ WARNING trivial_bounds


warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
   |
   |
LL | trait T where i32: Foo {} //~ WARNING trivial_bounds


warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
   |
   |
LL | union U where i32: Foo { f: i32 } //~ WARNING trivial_bounds


warning: where clauses are not enforced in type aliases
   |
   |
LL | type Y where i32: Foo = ();
   |
   |
   = note: `#[warn(type_alias_bounds)]` on by default
help: the clause will not be checked when the type alias is used, and should be removed
   |
LL | type Y  = ();


warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
   |
   |
LL | type Y where i32: Foo = ();


warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
   |
   |
LL | impl Foo for () where i32: Foo { //~ WARNING trivial_bounds


warning: Trait bound i32: Foo does not depend on any type or lifetime parameters
   |
   |
LL | fn f() where i32: Foo { //~ WARNING trivial_bounds


warning: Trait bound &'static str: Foo does not depend on any type or lifetime parameters
   |
   |
LL | fn g() where &'static str: Foo { //~ WARNING trivial_bounds


warning: Trait bound str: Sized does not depend on any type or lifetime parameters
   |
   |
LL | struct TwoStrs(str, str) where str: Sized; //~ WARNING trivial_bounds


warning: Trait bound for<'a> Dst<(dyn A + 'a)>: Sized does not depend on any type or lifetime parameters
   |
   |
LL | fn unsized_local() where for<'a> Dst<dyn A + 'a>: Sized { //~ WARNING trivial_bounds


warning: Trait bound str: Sized does not depend on any type or lifetime parameters
   |
   |
LL | fn return_str() -> str where str: Sized { //~ WARNING trivial_bounds


warning: Trait bound String: Neg does not depend on any type or lifetime parameters
   |
   |
LL | fn use_op(s: String) -> String where String: ::std::ops::Neg<Output=String> {


warning: Trait bound i32: Iterator does not depend on any type or lifetime parameters
   |
   |
LL | fn use_for() where i32: Iterator { //~ WARNING trivial_bounds

warning: 14 warnings emitted


error: internal compiler error: Encountered error `Unimplemented` selecting `Binder(<i32 as Foo>)` during codegen
   = note: delayed at compiler/rustc_trait_selection/src/traits/codegen.rs:68:32


error: internal compiler error: Encountered error `Unimplemented` selecting `Binder(<&str as Foo>)` during codegen
   = note: delayed at compiler/rustc_trait_selection/src/traits/codegen.rs:68:32


thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:974:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (6ff6e3bc0 2021-01-05) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack

------------------------------------------
------------------------------------------


---- [ui] ui/underscore-imports/hygiene-2.rs stdout ----
normalized stderr:
warning: call to noop method
  --> $DIR/hygiene-2.rs:32:5
   |
LL |     (&()).deref();
   |
   |
   = note: `#[warn(noop_method_call)]` on by default
warning: 1 warning emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/hygiene-2/hygiene-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args underscore-imports/hygiene-2.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-imports/hygiene-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/hygiene-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/hygiene-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: call to noop method
   |
   |
LL |     (&()).deref();
   |
   |
   = note: `#[warn(noop_method_call)]` on by default
warning: 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/underscore-imports/cycle.rs stdout ----
normalized stderr:
warning: call to noop method
  --> $DIR/cycle.rs:17:5
   |
LL |     (&0).deref();
   |
   |
   = note: `#[warn(noop_method_call)]` on by default
warning: 1 warning emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/cycle/cycle.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args underscore-imports/cycle.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-imports/cycle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/cycle" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/cycle/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: call to noop method
   |
   |
LL |     (&0).deref();
   |
   |
   = note: `#[warn(noop_method_call)]` on by default
warning: 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/underscore-imports/macro-expanded.rs stdout ----
normalized stderr:
warning: call to noop method
   |
   |
LL |             (&()).deref();
...
...
LL | m!();
   |
   |
   = note: `#[warn(noop_method_call)]` on by default


warning: call to noop method
   |
   |
LL |         (&()).deref();
...
...
LL | n!();
   |
   = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

warning: 2 warnings emitted
warning: 2 warnings emitted




The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/macro-expanded/macro-expanded.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args underscore-imports/macro-expanded.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-imports/macro-expanded.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/macro-expanded" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/macro-expanded/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: call to noop method
   |
   |
LL |             (&()).deref();
...
...
LL | m!();
   |
   |
   = note: `#[warn(noop_method_call)]` on by default


warning: call to noop method
   |
   |
LL |         (&()).deref();
...
...
LL | n!();
   |
   = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

warning: 2 warnings emitted
---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:53
