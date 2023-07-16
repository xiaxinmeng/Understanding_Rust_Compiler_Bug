plain

---- [ui] ui/coherence/coherence-orphan.rs stdout ----
diff of stderr:

1 error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
-   --> $DIR/coherence-orphan.rs:17:1
-    |
- LL | impl !Send for Vec<isize> { }
-    | |              |
-    | |              |
-    | |              `Vec` is not defined in the current crate
-    | impl doesn't use only types from inside the current crate
-    = note: define and implement a trait or new type instead
- 
- 
- error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
13   --> $DIR/coherence-orphan.rs:10:1
14    |
15 LL | impl TheTrait<usize> for isize { }
17    | |    |                   |
17    | |    |                   |
18    | |    |                   `isize` is not defined in the current crate
19    | |    `usize` is not defined in the current crate
+    | impl doesn't use only types from inside the current crate
+    = note: define and implement a trait or new type instead
+ 
+ 
+ error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
+   --> $DIR/coherence-orphan.rs:17:1
+    |
+ LL | impl !Send for Vec<isize> { }
+    | |              |
+    | |              |
+    | |              `Vec` is not defined in the current crate
20    | impl doesn't use only types from inside the current crate
22    = note: define and implement a trait or new type instead


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-orphan/coherence-orphan.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args coherence/coherence-orphan.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-orphan.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-orphan" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-orphan/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
   |
   |
LL | impl TheTrait<usize> for isize { }
   | |    |                   |
   | |    |                   |
   | |    |                   `isize` is not defined in the current crate
   | |    `usize` is not defined in the current crate
   | impl doesn't use only types from inside the current crate
   = note: define and implement a trait or new type instead


error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
   |
   |
LL | impl !Send for Vec<isize> { }
   | |              |
   | |              |
   | |              `Vec` is not defined in the current crate
   | impl doesn't use only types from inside the current crate
   = note: define and implement a trait or new type instead

error: aborting due to 2 previous errors

---

---- [ui] ui/methods/method-ambig-two-traits-cross-crate.rs stdout ----
diff of stderr:

4 LL | fn main() { 1_usize.me(); }
5    |                     ^^ multiple `me` found
6    |
- note: candidate #1 is defined in an impl of the trait `Me2` for the type `usize`
+    = note: candidate #1 is defined in an impl of the trait `Me` for the type `usize`
+ note: candidate #2 is defined in an impl of the trait `Me2` for the type `usize`
9    |
9    |
10 LL | impl Me2 for usize { fn me(&self) -> usize { *self } }
11    |                      ^^^^^^^^^^^^^^^^^^^^^
11    |                      ^^^^^^^^^^^^^^^^^^^^^
-    = note: candidate #2 is defined in an impl of the trait `Me` for the type `usize`
13 help: disambiguate the associated function for candidate #1
14    |
- LL | fn main() { Me2::me(&1_usize); }
- help: disambiguate the associated function for candidate #2
-    |
-    |
19 LL | fn main() { Me::me(&1_usize); }
+ help: disambiguate the associated function for candidate #2
+    |
+    |
+ LL | fn main() { Me2::me(&1_usize); }
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
21 
22 error: aborting due to previous error
23 
---
To only update this specific test, also pass `--test-args methods/method-ambig-two-traits-cross-crate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-ambig-two-traits-cross-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-cross-crate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-cross-crate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0034]: multiple applicable items in scope
  --> /checkout/src/test/ui/methods/method-ambig-two-traits-cross-crate.rs:11:21
   |
LL | fn main() { 1_usize.me(); } //~ ERROR E0034
   |                     ^^ multiple `me` found
   |
   = note: candidate #1 is defined in an impl of the trait `Me` for the type `usize`
note: candidate #2 is defined in an impl of the trait `Me2` for the type `usize`
   |
   |
LL | impl Me2 for usize { fn me(&self) -> usize { *self } }
help: disambiguate the associated function for candidate #1
   |
   |
LL | fn main() { Me::me(&1_usize); } //~ ERROR E0034
help: disambiguate the associated function for candidate #2
   |
   |
LL | fn main() { Me2::me(&1_usize); } //~ ERROR E0034

error: aborting due to previous error

For more information about this error, try `rustc --explain E0034`.
---
test result: FAILED. 12344 passed; 2 failed; 118 ignored; 0 measured; 0 filtered out; finished in 138.21s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:03
