plain
Successfully built e51e4b0ceb5c
Successfully tagged rust-ci:latest
Built container sha256:e51e4b0ceb5c97d0074dcc649dc0f46cc432aaf624746e97cc86677c84f2d3a8
Uploading finished image to https://ci-caches.rust-lang.org/docker/e6cb3de7fc810e25b29dc66ae05e79bdc99be06351486343892683bbcd3c450fed780a26b76bd2a9fd46be6bcc0ade79026c801040ae5c1991f9a502dcbc3381
upload failed: - to s3://rust-lang-ci-sccache2/docker/e6cb3de7fc810e25b29dc66ae05e79bdc99be06351486343892683bbcd3c450fed780a26b76bd2a9fd46be6bcc0ade79026c801040ae5c1991f9a502dcbc3381 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-9]
---
.................................................................................................... 9300/11543
.................................................................................................... 9400/11543
........................................................................i.......i................... 9500/11543
.................................................................................................... 9600/11543
..................iiiiiii..iiiiii.i................................................................. 9700/11543
.................................................................................................... 9900/11543
.................................................................................................... 10000/11543
.................................................................................................... 10100/11543
.................................................................................................... 10200/11543
---

---- [ui] ui/impl-trait/impl-trait-in-macro.rs stdout ----
diff of stderr:

+ error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+    |
+    |
+ LL |     ($($tr:tt)*) => { impl $($tr)* };
+ ...
+ ...
+ LL | fn more_fns<T: FnOnce(i!(Debug)) -> i!(Debug)>() {}
+    |
+    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
+    |
+    |
+ LL |     ($($tr:tt)*) => { impl $($tr)* };
+ ...
+ ...
+ LL | fn more_fns<T: FnOnce(i!(Debug)) -> i!(Debug)>() {}
+    |
+    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
1 error[E0308]: mismatched types
1 error[E0308]: mismatched types
2   --> $DIR/impl-trait-in-macro.rs:9:9
3    |

15    = note: a type parameter was expected, but a different one was found; you might be missing a type parameter or trait bound
16    = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
- error: aborting due to previous error
+ error: aborting due to 3 previous errors
19 
- For more information about this error, try `rustc --explain E0308`.
- For more information about this error, try `rustc --explain E0308`.
+ Some errors have detailed explanations: E0308, E0562.
+ For more information about an error, try `rustc --explain E0308`.
21 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-trait-in-macro/impl-trait-in-macro.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/impl-trait-in-macro.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/impl-trait-in-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-trait-in-macro" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-trait-in-macro/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
   |
   |
LL |     ($($tr:tt)*) => { impl $($tr)* };
...
...
LL | fn more_fns<T: FnOnce(i!(Debug)) -> i!(Debug)>() {}
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
   |
   |
LL |     ($($tr:tt)*) => { impl $($tr)* };
...
...
LL | fn more_fns<T: FnOnce(i!(Debug)) -> i!(Debug)>() {}
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/impl-trait/impl-trait-in-macro.rs:9:9
   |
LL |     ($($tr:tt)*) => { impl $($tr)* };
   |                       |
   |                       expected type parameter
   |                       found type parameter
...
...
LL |     a = y; //~ ERROR mismatched
   |         ^ expected type parameter `impl Debug`, found a different type parameter `impl Debug`
   |
   = note: expected type parameter `impl Debug` (type parameter `impl Debug`)
              found type parameter `impl Debug` (type parameter `impl Debug`)
   = note: a type parameter was expected, but a different one was found; you might be missing a type parameter or trait bound
   = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0308, E0562.
For more information about an error, try `rustc --explain E0308`.
---
test result: FAILED. 11449 passed; 1 failed; 93 ignored; 0 measured; 0 filtered out; finished in 134.08s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:41
