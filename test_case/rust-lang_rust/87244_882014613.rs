plain
...........................................i.i...................................................... 12100/12153
.....................................................
failures:

---- [ui] ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box.rs stdout ----

1 error: incompatible lifetime on type
-   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:17:5
+   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:16:5
+   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:16:5
3    |
4 LL |     type T<'a> = Box<dyn A + 'a>;

6    |
7 note: because of this predicate
-   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:13:17
-   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:13:17
+   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:12:17
9    |
10 LL |     type T<'a>: A;


12 note: a compatible `impl` exists, but...
-   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:10:1
+   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:9:1
14    |
15 LL | impl A for Box<dyn A> {}


- note: the lifetime `'a` as defined on the associated item at 17:12...
-   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:17:12
+ note: the lifetime `'a` as defined on the associated item at 16:12...
+   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:16:12
19    |
20 LL |     type T<'a> = Box<dyn A + 'a>;


22    = note: ...does not necessarily outlive the static lifetime
23 note: the used `impl` has a `'static` requirement
-   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:10:20
+   --> $DIR/issue-78113-lifetime-mismatch-dyn-trait-box.rs:9:20
25    |
26 LL | impl A for Box<dyn A> {}
27    |                    ^ this has an implicit `'static` lifetime requirement

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box/issue-78113-lifetime-mismatch-dyn-trait-box.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: incompatible lifetime on type
  --> /checkout/src/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box.rs:16:5
   |
LL |     type T<'a> = Box<dyn A + 'a>; //~ incompatible lifetime on type
   |
note: because of this predicate
  --> /checkout/src/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box.rs:12:17
   |
   |
LL |     type T<'a>: A;
   |                 ^
note: a compatible `impl` exists, but...
  --> /checkout/src/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box.rs:9:1
   |
LL | impl A for Box<dyn A> {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^
note: the lifetime `'a` as defined on the associated item at 16:12...
  --> /checkout/src/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box.rs:16:12
   |
LL |     type T<'a> = Box<dyn A + 'a>; //~ incompatible lifetime on type
   |            ^^
   = note: ...does not necessarily outlive the static lifetime
note: the used `impl` has a `'static` requirement
  --> /checkout/src/test/ui/generic-associated-types/issue-78113-lifetime-mismatch-dyn-trait-box.rs:9:20
   |
LL | impl A for Box<dyn A> {}
   |                    ^ this has an implicit `'static` lifetime requirement
help: consider relaxing the implicit `'static` requirement
   |
LL | impl A for Box<dyn A + '_> {}

error: aborting due to previous error


---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:34
