plain
.................................................................................................... 9100/11289
.................................................................................................... 9200/11289
.....................................................................................i......i....... 9300/11289
.................................................................................................... 9400/11289
.......................iiiiii..iiiiii..i............................................................ 9500/11289
.................................................................................................... 9700/11289
.................................................................................................... 9800/11289
.................................................................................................... 9900/11289
.................................................................................................... 10000/11289
---
...............................................................................i.i.................. 11200/11289
.........................................................................................
failures:

---- [ui] ui/const-generics/const_evaluatable_checked/eval-privacy.rs stdout ----

14    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
15    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
16 
16 
+ warning: private function `fn(u8) -> u8 {my_const_fn}` in public interface (error E0446)
+    |
+    |
+ LL | / impl<const U: u8> Trait for Const<U>
+ LL | |
+ LL | |
+ LL | | where
+ ...  |
+ LL | |     }
+ LL | | }
+    |
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
+ 
+ 
17 error[E0446]: private function `fn(u8) -> u8 {my_const_fn}` in public interface
19    |


23 LL | const fn my_const_fn(val: u8) -> u8 {
24    | ----------------------------------- `fn(u8) -> u8 {my_const_fn}` declared as private
- error: aborting due to previous error; 1 warning emitted
+ error: aborting due to previous error; 2 warnings emitted
27 
28 For more information about this error, try `rustc --explain E0446`.
28 For more information about this error, try `rustc --explain E0446`.
29 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const_evaluatable_checked/eval-privacy/eval-privacy.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/const_evaluatable_checked/eval-privacy.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const_evaluatable_checked/eval-privacy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const_evaluatable_checked/eval-privacy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const_evaluatable_checked/eval-privacy/auxiliary"
------------------------------------------

------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stderr:
------------------------------------------
warning: private function `fn(u8) -> u8 {my_const_fn}` in public interface (error E0446)
  --> /checkout/src/test/ui/const-generics/const_evaluatable_checked/eval-privacy.rs:12:1
   |
LL | / impl<const U: u8> Trait for Const<U>
LL | | //~^ WARN private function
LL | | //~| WARN this was previously
LL | | where
LL | |     }
LL | | }
   | |_^
   |
   |
   = note: `#[warn(private_in_public)]` on by default
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


warning: private function `fn(u8) -> u8 {my_const_fn}` in public interface (error E0446)
  --> /checkout/src/test/ui/const-generics/const_evaluatable_checked/eval-privacy.rs:12:1
   |
LL | / impl<const U: u8> Trait for Const<U>
LL | | //~^ WARN private function
LL | | //~| WARN this was previously
LL | | where
LL | |     }
LL | | }
   | |_^
   |
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>

error[E0446]: private function `fn(u8) -> u8 {my_const_fn}` in public interface
  --> /checkout/src/test/ui/const-generics/const_evaluatable_checked/eval-privacy.rs:18:5
   |
LL |     type AssocTy = Const<{ my_const_fn(U) }>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't leak private function
...
LL | const fn my_const_fn(val: u8) -> u8 {
   | ----------------------------------- `fn(u8) -> u8 {my_const_fn}` declared as private
error: aborting due to previous error; 2 warnings emitted

For more information about this error, try `rustc --explain E0446`.

---
test result: FAILED. 11201 passed; 1 failed; 87 ignored; 0 measured; 0 filtered out; finished in 141.38s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:33
