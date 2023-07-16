plain
..........................................................................................F......... 1700/11377
..F................................................................................................. 1800/11377
....i............................................................................................... 1900/11377
.................................................................................................... 2000/11377
.........................F...F...................................................................... 2100/11377
.................................................................................................... 2300/11377
.................................................................................................... 2400/11377
.................................................................................................... 2500/11377
..............................................................................................i..i.. 2600/11377
---

---- [ui] ui/const-generics/const_evaluatable_checked/different-fn.rs stdout ----
diff of stderr:

4 LL |     [0; size_of::<Foo<T>>()]
6    |
6    |
- help: try adding a `where` bound using this expression: where [u8; size_of::<Foo<T>>()]: Sized
+ help: try adding a `where` bound using this expression: `where [u8; size_of::<Foo<T>>()]: Sized`
9    |
9    |
10 LL |     [0; size_of::<Foo<T>>()]

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const_evaluatable_checked/different-fn/different-fn.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const_evaluatable_checked/different-fn/different-fn.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/const_evaluatable_checked/different-fn.rs`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const_evaluatable_checked/different-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const_evaluatable_checked/different-fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const_evaluatable_checked/different-fn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/const_evaluatable_checked/different-fn.rs:10:9
   |
LL |     [0; size_of::<Foo<T>>()]
   |
   |
help: try adding a `where` bound using this expression: `where [u8; size_of::<Foo<T>>()]: Sized`
   |
   |
LL |     [0; size_of::<Foo<T>>()]

error: aborting due to previous error



------------------------------------------


---- [ui] ui/const-generics/const_evaluatable_checked/cross_crate_predicate.rs stdout ----
diff of stderr:

4 LL |     let _ = const_evaluatable_lib::test1::<T>();
6    |
6    |
- help: try adding a `where` bound using this expression: where [u8; std::mem::size_of::<T>() - 1]: Sized
+ help: try adding a `where` bound using this expression: `where [u8; std::mem::size_of::<T>() - 1]: Sized`
8   --> $DIR/auxiliary/const_evaluatable_lib.rs:6:10
9    |
10 LL |     [u8; std::mem::size_of::<T>() - 1]: Sized,

16 LL |     let _ = const_evaluatable_lib::test1::<T>();
18    |
18    |
- help: try adding a `where` bound using this expression: where [u8; std::mem::size_of::<T>() - 1]: Sized
+ help: try adding a `where` bound using this expression: `where [u8; std::mem::size_of::<T>() - 1]: Sized`
20   --> $DIR/auxiliary/const_evaluatable_lib.rs:4:27
21    |
22 LL | pub fn test1<T>() -> [u8; std::mem::size_of::<T>() - 1]

28 LL |     let _ = const_evaluatable_lib::test1::<T>();
30    |
30    |
- help: try adding a `where` bound using this expression: where [u8; std::mem::size_of::<T>() - 1]: Sized
+ help: try adding a `where` bound using this expression: `where [u8; std::mem::size_of::<T>() - 1]: Sized`
32   --> $DIR/auxiliary/const_evaluatable_lib.rs:6:10
33    |
34 LL |     [u8; std::mem::size_of::<T>() - 1]: Sized,

40 LL |     let _ = const_evaluatable_lib::test1::<T>();
42    |
42    |
- help: try adding a `where` bound using this expression: where [u8; std::mem::size_of::<T>() - 1]: Sized
+ help: try adding a `where` bound using this expression: `where [u8; std::mem::size_of::<T>() - 1]: Sized`
44   --> $DIR/auxiliary/const_evaluatable_lib.rs:4:27
45    |
46 LL | pub fn test1<T>() -> [u8; std::mem::size_of::<T>() - 1]

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const_evaluatable_checked/cross_crate_predicate/cross_crate_predicate.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const_evaluatable_checked/cross_crate_predicate/cross_crate_predicate.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/const_evaluatable_checked/cross_crate_predicate.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const_evaluatable_checked/cross_crate_predicate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const_evaluatable_checked/cross_crate_predicate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const_evaluatable_checked/cross_crate_predicate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/const_evaluatable_checked/cross_crate_predicate.rs:7:13
   |
LL |     let _ = const_evaluatable_lib::test1::<T>();
   |
   |
help: try adding a `where` bound using this expression: `where [u8; std::mem::size_of::<T>() - 1]: Sized`
  --> /checkout/src/test/ui/const-generics/const_evaluatable_checked/auxiliary/const_evaluatable_lib.rs:6:10
   |
LL |     [u8; std::mem::size_of::<T>() - 1]: Sized,

error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/const_evaluatable_checked/cross_crate_predicate.rs:7:13
   |
   |
LL |     let _ = const_evaluatable_lib::test1::<T>();
   |
   |
help: try adding a `where` bound using this expression: `where [u8; std::mem::size_of::<T>() - 1]: Sized`
  --> /checkout/src/test/ui/const-generics/const_evaluatable_checked/auxiliary/const_evaluatable_lib.rs:4:27
   |
LL | pub fn test1<T>() -> [u8; std::mem::size_of::<T>() - 1]

error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/const_evaluatable_checked/cross_crate_predicate.rs:7:13
   |
   |
LL |     let _ = const_evaluatable_lib::test1::<T>();
   |
   |
help: try adding a `where` bound using this expression: `where [u8; std::mem::size_of::<T>() - 1]: Sized`
  --> /checkout/src/test/ui/const-generics/const_evaluatable_checked/auxiliary/const_evaluatable_lib.rs:6:10
   |
LL |     [u8; std::mem::size_of::<T>() - 1]: Sized,

error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/const_evaluatable_checked/cross_crate_predicate.rs:7:13
   |
   |
LL |     let _ = const_evaluatable_lib::test1::<T>();
   |
   |
help: try adding a `where` bound using this expression: `where [u8; std::mem::size_of::<T>() - 1]: Sized`
  --> /checkout/src/test/ui/const-generics/const_evaluatable_checked/auxiliary/const_evaluatable_lib.rs:4:27
   |
LL | pub fn test1<T>() -> [u8; std::mem::size_of::<T>() - 1]

error: aborting due to 4 previous errors



------------------------------------------


---- [ui] ui/const_evaluatable/needs_where_clause.rs stdout ----
diff of stderr:

4 LL |   b: [f32; complex_maths::<T>(N)],
6    |
6    |
- help: try adding a `where` bound using this expression: where [u8; complex_maths::<T>(N)]: Sized
+ help: try adding a `where` bound using this expression: `where [u8; complex_maths::<T>(N)]: Sized`
8   --> $DIR/needs_where_clause.rs:11:12
9    |
10 LL |   b: [f32; complex_maths::<T>(N)],

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const_evaluatable/needs_where_clause/needs_where_clause.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const_evaluatable/needs_where_clause/needs_where_clause.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const_evaluatable/needs_where_clause.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const_evaluatable/needs_where_clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const_evaluatable/needs_where_clause" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const_evaluatable/needs_where_clause/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unconstrained generic constant
  --> /checkout/src/test/ui/const_evaluatable/needs_where_clause.rs:11:6
   |
LL |   b: [f32; complex_maths::<T>(N)],
   |
   |
help: try adding a `where` bound using this expression: `where [u8; complex_maths::<T>(N)]: Sized`
   |
   |
LL |   b: [f32; complex_maths::<T>(N)],

error: aborting due to previous error



------------------------------------------


---- [ui] ui/const_evaluatable/no_where_clause.rs stdout ----
diff of stderr:

4 LL |   b: [f32; complex_maths(N)],
6    |
6    |
- help: try adding a `where` bound using this expression: where [u8; complex_maths(N)]: Sized
+ help: try adding a `where` bound using this expression: `where [u8; complex_maths(N)]: Sized`
9    |
9    |
10 LL |   b: [f32; complex_maths(N)],

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const_evaluatable/no_where_clause/no_where_clause.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const_evaluatable/no_where_clause/no_where_clause.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const_evaluatable/no_where_clause.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const_evaluatable/no_where_clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const_evaluatable/no_where_clause" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const_evaluatable/no_where_clause/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unconstrained generic constant
  --> /checkout/src/test/ui/const_evaluatable/no_where_clause.rs:10:6
   |
LL |   b: [f32; complex_maths(N)],
   |
   |
help: try adding a `where` bound using this expression: `where [u8; complex_maths(N)]: Sized`
   |
   |
LL |   b: [f32; complex_maths(N)],

error: aborting due to previous error


---
test result: FAILED. 11281 passed; 4 failed; 92 ignored; 0 measured; 0 filtered out; finished in 135.62s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:49
