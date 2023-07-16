plain
............................i.ii.................................................................... 12300/12341
.........................................
failures:

---- [ui] ui/rfcs/rfc-2528-type_changing-struct-update/type-generic-update.rs stdout ----

+ warning: the feature `type_changing_struct_update` is incomplete and may not be safe to use and/or cause compiler crashes
+   --> $DIR/type-generic-update.rs:1:12
+    |
---

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2528-type_changing-struct-update/type-generic-update/type-generic-update.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfcs/rfc-2528-type_changing-struct-update/type-generic-update.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfcs/rfc-2528-type_changing-struct-update/type-generic-update.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2528-type_changing-struct-update/type-generic-update" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2528-type_changing-struct-update/type-generic-update/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error[E0308]: mismatched types
  --> /checkout/src/test/ui/rfcs/rfc-2528-type_changing-struct-update/type-generic-update.rs:31:11
   |
LL |         ..m1
   |           ^^ field type mismatch: Machine.state
   = note: expected type `i32`
              found type `f64`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/rfcs/rfc-2528-type_changing-struct-update/type-generic-update.rs:35:11
   |
LL |         ..m1
   |           ^^ field type mismatch: Machine.state
   = note: expected type `i32`
              found type `f64`

error: aborting due to 2 previous errors; 1 warning emitted
error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0308`.

------------------------------------------


---- [ui] ui/rfcs/rfc-2528-type_changing-struct-update/lifetime_update.rs stdout ----

+ warning: the feature `type_changing_struct_update` is incomplete and may not be safe to use and/or cause compiler crashes
+   --> $DIR/lifetime_update.rs:1:12
+    |
+    |
+ LL | #![feature(type_changing_struct_update)]
+    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    |
+    = note: `#[warn(incomplete_features)]` on by default
+    = note: see issue #86555 <https://github.com/rust-lang/rust/issues/86555> for more information
+ 
1 error[E0597]: `s` does not live long enough
2   --> $DIR/lifetime_update.rs:16:17

10 LL | }
10 LL | }
11    | - `s` dropped here while still borrowed
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
14 
15 For more information about this error, try `rustc --explain E0597`.
15 For more information about this error, try `rustc --explain E0597`.
16 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2528-type_changing-struct-update/lifetime_update/lifetime_update.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfcs/rfc-2528-type_changing-struct-update/lifetime_update.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfcs/rfc-2528-type_changing-struct-update/lifetime_update.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2528-type_changing-struct-update/lifetime_update" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2528-type_changing-struct-update/lifetime_update/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #86555 <https://github.com/rust-lang/rust/issues/86555> for more information

error[E0597]: `s` does not live long enough
  --> /checkout/src/test/ui/rfcs/rfc-2528-type_changing-struct-update/lifetime_update.rs:16:17
LL |         lt_str: &s,
LL |         lt_str: &s,
   |                 ^^ borrowed value does not live long enough
...
LL |     let m2: Machine<'static, State1> = Machine {
   |             ------------------------ type annotation requires that `s` is borrowed for `'static`
LL | }
LL | }
   | - `s` dropped here while still borrowed
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0597`.

---
test result: FAILED. 12229 passed; 2 failed; 110 ignored; 0 measured; 0 filtered out; finished in 138.82s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:00
