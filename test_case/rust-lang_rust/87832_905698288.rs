plain
1 error: any use of this value will cause an error
-   --> $DIR/const_eval_limit_reached.rs:6:11
+   --> $DIR/const_eval_limit_reached.rs:6:5
3    |
- LL | / const X: usize = {
- LL | |     let mut x = 0;
- LL | |     while x != 1000 {
-    | |           ^^^^^^^^^ exceeded interpreter step limit (see `#[const_eval_limit]`)
- LL | |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- LL | |     x
- LL | | };
-    | |__-
+ LL |  / const X: usize = {
+ LL |  / const X: usize = {
+ LL |  |     let mut x = 0;
+ LL |  |     while x != 1000 {
+ LL | ||
+ LL | ||
+ LL | ||         x += 1;
+ LL | ||     }
+ LL | ||     }
+    | ||_____^ exceeded interpreter step limit (see `#[const_eval_limit]`)
+ LL |  |
+ LL |  |     x
+ LL |  | };
13    |
13    |
14    = note: `#[deny(const_err)]` on by default


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_limit/const_eval_limit_reached/const_eval_limit_reached.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_limit/const_eval_limit_reached/const_eval_limit_reached.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const_limit/const_eval_limit_reached.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_limit/const_eval_limit_reached.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_limit/const_eval_limit_reached" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_limit/const_eval_limit_reached/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
LL |  / const X: usize = {
LL |  |     let mut x = 0;
LL |  |     while x != 1000 {
   |  |_____^
LL | ||         //~^ ERROR any use of this value will cause an error
LL | ||         //~| WARN this was previously accepted by the compiler but is being phased out
LL | ||         x += 1;
LL | ||     }
   | ||_____^ exceeded interpreter step limit (see `#[const_eval_limit]`)
LL |  |     x
LL |  | };
   |  |__-
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: aborting due to previous error



------------------------------------------


---- [ui] ui/union/union-unsafe.rs#mir stdout ----
diff of stderr:

55    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
56 
57 error[E0133]: access to union field is unsafe and requires unsafe function or block
-   --> $DIR/union-unsafe.rs:65:5
59    |
59    |
60 LL |     if let U1 { a: 12 } = u1 {}
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^ access to union field
+    |            ^^^^^^^^^^^^ access to union field
62    |
63    = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-unsafe.mir/union-unsafe.mir.stderr
To only update this specific test, also pass `--test-args union/union-unsafe.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-unsafe.mir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-unsafe.mir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     *(u.p) = 13; //~ ERROR access to union field is unsafe
   |     ^^^^^^^^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: assignment to union field that might need dropping is unsafe and requires unsafe function or block
   |
   |
LL |     u.a = (RefCell::new(0), 1); //~ ERROR assignment to union field that might need dropping
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ assignment to union field that might need dropping
   |
   = note: the previous content of the field will be dropped, which causes undefined behavior if the field was not properly initialized

error[E0133]: assignment to union field that might need dropping is unsafe and requires unsafe function or block
   |
   |
LL |     u.a.0 = RefCell::new(0); //~ ERROR assignment to union field that might need dropping
   |     ^^^^^^^^^^^^^^^^^^^^^^^ assignment to union field that might need dropping
   |
   = note: the previous content of the field will be dropped, which causes undefined behavior if the field was not properly initialized

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     *u3.a = T::default(); //~ ERROR access to union field is unsafe
   |      ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     *u3.a = T::default(); //~ ERROR access to union field is unsafe
   |      ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     let a = u1.a; //~ ERROR access to union field is unsafe
   |             ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     let U1 { a } = u1; //~ ERROR access to union field is unsafe
   |              ^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     if let U1 { a: 12 } = u1 {} //~ ERROR access to union field is unsafe
   |            ^^^^^^^^^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     *u2.a = String::from("new"); //~ ERROR access to union field is unsafe
   |      ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     *u3.a = 1; //~ ERROR access to union field is unsafe
   |      ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior

error[E0133]: access to union field is unsafe and requires unsafe function or block
   |
   |
LL |     *u3.a = String::from("new"); //~ ERROR access to union field is unsafe
   |      ^^^^ access to union field
   |
   = note: the field may not be properly initialized: using uninitialized data will cause undefined behavior
error: aborting due to 11 previous errors

For more information about this error, try `rustc --explain E0133`.

---
test result: FAILED. 12078 passed; 2 failed; 102 ignored; 0 measured; 0 filtered out; finished in 127.47s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:37
