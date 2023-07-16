plain
......................................ii............................................................ 7600/11245
...........................................................i........................................ 7700/11245
............................................i....................................................... 7800/11245
...................................i................................................................ 7900/11245
.......................F..F......................................................................... 8000/11245
.................................................................................................... 8200/11245
.................................................................................................i.. 8300/11245
.................................................................................................... 8400/11245
.................................................................................................... 8500/11245
---
.................................................................................................... 9000/11245
.................................................................................................... 9100/11245
.................................................................................................... 9200/11245
.........................................i......i................................................... 9300/11245
.................................................................................iiiiii....iiiiiii.. 9400/11245
.................................................................................................... 9600/11245
.................................................................................................... 9700/11245
.................................................................................................... 9800/11245
.................................................................................................... 9900/11245
---
failures:

---- [ui] ui/parser/issue-14303-path.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-14303-path.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-path" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-path/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'lifetimes ought to have been inferred', compiler/rustc_typeck/src/astconv/generics.rs:289:48

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.51.0-nightly (e365f626f 2021-01-08) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [fn_sig] computing function signature of `bar`
#1 [collect_mod_item_types] collecting item types in top-level module
end of query stack
------------------------------------------


---- [ui] ui/parser/issue-14303-fncall.rs stdout ----
---- [ui] ui/parser/issue-14303-fncall.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-14303-fncall.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-fncall" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-fncall/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'lifetimes ought to have been inferred', compiler/rustc_typeck/src/astconv/generics.rs:289:48

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.51.0-nightly (e365f626f 2021-01-08) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -Z borrowck=mir -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `foo`
#1 [typeck_item_bodies] type-checking all item bodies
end of query stack
------------------------------------------


---- [ui] ui/suggestions/suggest-move-types.rs stdout ----
---- [ui] ui/suggestions/suggest-move-types.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-move-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-move-types" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-move-types/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: generic arguments must come before the first constraint
   |
   |
LL | struct A<T, M: One<A=(), T>> {
   |                    ----  ^ generic argument
   |                    constraint
   |
   |
help: move the constraint after the generic argument
   |
LL | struct A<T, M: One<T, A = ()>> {


error: generic arguments must come before the first constraint
   |
   |
LL | struct Al<'a, T, M: OneWithLifetime<A=(), T, 'a>> {
   |                                     ----  ^  ^^ generic arguments
   |                                     constraint
   |
   |
help: move the constraint after the generic arguments
   |
LL | struct Al<'a, T, M: OneWithLifetime<'a, T, A = ()>> {


error: generic arguments must come before the first constraint
   |
   |
LL | struct B<T, U, V, M: Three<A=(), B=(), C=(), T, U, V>> {
   |                            ----  ----  ----  ^  ^  ^ generic arguments
   |                            constraints
   |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
help: move the constraints after the generic arguments
   |
LL | struct B<T, U, V, M: Three<T, U, V, A = (), B = (), C = ()>> {


error: generic arguments must come before the first constraint
   |
   |
LL | struct Bl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<A=(), B=(), C=(), T, U, V, 'a, 'b, 'c>> {
   |                                                     ----  ----  ----  ^  ^  ^  ^^  ^^  ^^ generic arguments
   |                                                     constraints
   |
   |
help: move the constraints after the generic arguments
   |
LL | struct Bl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<'a, 'b, 'c, T, U, V, A = (), B = (), C = ()>> {


error: generic arguments must come before the first constraint
   |
   |
LL | struct C<T, U, V, M: Three<T, A=(), B=(), C=(), U, V>> {
   |                            ^  ----  ----  ----  ^  ^ generic arguments
   |                               constraints
   |
   |
help: move the constraints after the generic arguments
   |
LL | struct C<T, U, V, M: Three<T, U, V, A = (), B = (), C = ()>> {


error: generic arguments must come before the first constraint
   |
   |
LL | struct Cl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<T, 'a, A=(), B=(), C=(), U, 'b, V, 'c>> {
   |                                                     ^  ^^  ----  ----  ----  ^  ^^  ^  ^^ generic arguments
   |                                                            constraints
   |
   |
help: move the constraints after the generic arguments
   |
LL | struct Cl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<'a, 'b, 'c, T, U, V, A = (), B = (), C = ()>> {


error: generic arguments must come before the first constraint
   |
   |
LL | struct D<T, U, V, M: Three<T, A=(), B=(), U, C=(), V>> {
   |                            ^  ----  ----  ^  ----  ^ generic arguments
   |                               constraints
   |
   |
help: move the constraints after the generic arguments
   |
LL | struct D<T, U, V, M: Three<T, U, V, A = (), B = (), C = ()>> {


error: generic arguments must come before the first constraint
   |
   |
LL | struct Dl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<T, 'a, A=(), B=(), U, 'b, C=(), V, 'c>> {
   |                                                     ^  ^^  ----  ----  ^  ^^  ----  ^  ^^ generic arguments
   |                                                            constraints
   |
   |
help: move the constraints after the generic arguments
   |
LL | struct Dl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<'a, 'b, 'c, T, U, V, A = (), B = (), C = ()>> {


thread 'rustc' panicked at 'lifetimes ought to have been inferred', compiler/rustc_typeck/src/astconv/generics.rs:289:48

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.51.0-nightly (e365f626f 2021-01-08) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [explicit_predicates_of] computing explicit predicates of `Al`
#1 [predicates_defined_on] computing predicates of `Al`
end of query stack


------------------------------------------



---- [ui] ui/traits/trait-object-vs-lifetime.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-object-vs-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-vs-lifetime" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-vs-lifetime/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0224]: at least one trait is required for an object type
   |
   |
LL |     let _: S<'static, dyn 'static +>;


error[E0107]: wrong number of lifetime arguments: expected 1, found 2
   |
   |
LL |     let _: S<'static, 'static>;
   |                       ^^^^^^^ unexpected lifetime argument

error[E0107]: wrong number of type arguments: expected 1, found 0
   |
   |
LL |     let _: S<'static, 'static>;
   |            ^^^^^^^^^^^^^^^^^^^ expected 1 type argument

error[E0224]: at least one trait is required for an object type
   |
   |
LL |     let _: S<dyn 'static +, 'static>;


thread 'rustc' panicked at 'lifetimes ought to have been inferred', compiler/rustc_typeck/src/astconv/generics.rs:289:48

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.51.0-nightly (e365f626f 2021-01-08) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
end of query stack

Some errors have detailed explanations: E0107, E0224.
For more information about an error, try `rustc --explain E0107`.

---
test result: FAILED. 11155 passed; 4 failed; 86 ignored; 0 measured; 0 filtered out; finished in 135.17s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:04
