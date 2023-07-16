plain

---- [ui] ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs#thir stdout ----
diff of stderr:

17 LL |     *PTR;
18    |     ^^^^ dereference of raw pointer
19    |
-    = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
+    = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
21 
22 error: use of mutable static is unsafe and requires unsafe block (error E0133)
23   --> $DIR/rfc-2585-unsafe_op_in_unsafe_fn.rs:16:5
59 LL |     *PTR;
59 LL |     *PTR;
60    |     ^^^^ dereference of raw pointer
61    |
-    = note: raw pointers may be NULL, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
63 
64 error: use of mutable static is unsafe and requires unsafe block (error E0133)
65   --> $DIR/rfc-2585-unsafe_op_in_unsafe_fn.rs:31:5

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.thir/rfc-2585-unsafe_op_in_unsafe_fn.thir.stderr
To only update this specific test, also pass `--test-args unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs`


error in revision `thir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.thir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zthir-unsafeck" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.thir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: call to unsafe function is unsafe and requires unsafe block (error E0133)
   |
LL |     unsf();
   |     ^^^^^^ call to unsafe function
   |
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs:4:9
   |
LL | #![deny(unsafe_op_in_unsafe_fn)]
   |         ^^^^^^^^^^^^^^^^^^^^^^
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: dereference of raw pointer is unsafe and requires unsafe block (error E0133)
   |
LL |     *PTR;
LL |     *PTR;
   |     ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior

error: use of mutable static is unsafe and requires unsafe block (error E0133)
   |
LL |     VOID = ();
   |     ^^^^ use of mutable static
   |
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

error: unnecessary `unsafe` block
   |
LL |     unsafe {}
LL |     unsafe {}
   |     ^^^^^^ unnecessary `unsafe` block
note: the lint level is defined here
  --> /checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs:5:9
   |
   |
LL | #![deny(unused_unsafe)]


error: call to unsafe function is unsafe and requires unsafe block (error E0133)
   |
LL |     unsf();
   |     ^^^^^^ call to unsafe function
   |
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/unsafe/rfc-2585-unsafe_op_in_unsafe_fn.rs:25:8
   |
LL | #[deny(warnings)]
   |        ^^^^^^^^
   = note: `#[deny(unsafe_op_in_unsafe_fn)]` implied by `#[deny(warnings)]`
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: dereference of raw pointer is unsafe and requires unsafe block (error E0133)
   |
LL |     *PTR;
LL |     *PTR;
   |     ^^^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior

error: use of mutable static is unsafe and requires unsafe block (error E0133)
   |
LL |     VOID = ();
   |     ^^^^ use of mutable static
   |
   |
   = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

error: unnecessary `unsafe` block
   |
LL |     unsafe {}
LL |     unsafe {}
   |     ^^^^^^ unnecessary `unsafe` block

error: unnecessary `unsafe` block
   |
   |
LL |     unsafe { unsafe { unsf() } }
   |     ------   ^^^^^^ unnecessary `unsafe` block
   |     |
   |     because it's nested under this `unsafe` block

error: unnecessary `unsafe` block
   |
   |
LL | unsafe fn allow_level() {
   | ----------------------- because it's nested under this `unsafe` fn
...
LL |     unsafe { unsf() }
   |     ^^^^^^ unnecessary `unsafe` block

error: unnecessary `unsafe` block
   |
   |
LL | unsafe fn nested_allow_level() {
   | ------------------------------ because it's nested under this `unsafe` fn
...
LL |         unsafe { unsf() }
   |         ^^^^^^ unnecessary `unsafe` block

error[E0133]: call to unsafe function is unsafe and requires unsafe block
   |
LL |     unsf();
   |     ^^^^^^ call to unsafe function
   |
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
LL |         unsf();
   |         ^^^^^^ call to unsafe function
   |
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
error: aborting due to 13 previous errors

For more information about this error, try `rustc --explain E0133`.

---
test result: FAILED. 11817 passed; 1 failed; 97 ignored; 0 measured; 0 filtered out; finished in 119.99s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:29
