plain

---- [ui] ui/borrowck/issue-64453.rs stdout ----
diff of stderr:

+ error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |
+    |
+ LL | static settings_dir: String = format!("");
+    |
+    = note: this error originates in the macro `$crate::__export::format_args` (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
+ 
1 error: `Arguments::<'a>::new_v1` is not yet stable as a const fn
3    |


21 LL |     let settings_data = from_string(settings_dir);
22    |                                     ^^^^^^^^^^^^ move occurs because `settings_dir` has type `String`, which does not implement the `Copy` trait
- error: aborting due to 3 previous errors
+ error: aborting due to 4 previous errors
25 
26 Some errors have detailed explanations: E0015, E0507.
---
To only update this specific test, also pass `--test-args borrowck/issue-64453.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-64453.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-64453" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-64453/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
  --> /checkout/src/test/ui/borrowck/issue-64453.rs:4:31
   |
LL | static settings_dir: String = format!("");
   |
   = note: this error originates in the macro `$crate::__export::format_args` (in Nightly builds, run with -Z macro-backtrace for more info)


error: `Arguments::<'a>::new_v1` is not yet stable as a const fn
   |
   |
LL | static settings_dir: String = format!("");
   |
   = help: add `#![feature(const_fmt_arguments_new)]` to the crate attributes to enable
   = note: this error originates in the macro `$crate::__export::format_args` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
  --> /checkout/src/test/ui/borrowck/issue-64453.rs:4:31
   |
LL | static settings_dir: String = format!("");
   |
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0507]: cannot move out of static item `settings_dir`
   |
   |
LL |     let settings_data = from_string(settings_dir);
   |                                     ^^^^^^^^^^^^ move occurs because `settings_dir` has type `String`, which does not implement the `Copy` trait
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0015, E0507.
For more information about an error, try `rustc --explain E0015`.
For more information about an error, try `rustc --explain E0015`.

------------------------------------------


---- [ui] ui/consts/const-eval/const_panic_2021.rs stdout ----
diff of stderr:

- error[E0080]: evaluation of constant value failed
+ error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
3    |
3    |
4 LL | const A: () = std::panic!("blåhaj");

-    |               ^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'blåhaj', $DIR/const_panic_2021.rs:7:15
6    |
-    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `$crate::const_format_args` (in Nightly builds, run with -Z macro-backtrace for more info)
8 
8 
- error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic_2021.rs:10:15
-    |
- LL | const B: () = std::panic!();
-    |
-    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
- 
- error[E0080]: evaluation of constant value failed
- error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic_2021.rs:13:15
-    |
- LL | const C: () = std::unreachable!();
-    |               ^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic_2021.rs:13:15
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
- 
- error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic_2021.rs:16:15
-   --> $DIR/const_panic_2021.rs:16:15
-    |
- LL | const D: () = std::unimplemented!();
-    |               ^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'not implemented', $DIR/const_panic_2021.rs:16:15
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
- 
- error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic_2021.rs:19:15
-   --> $DIR/const_panic_2021.rs:19:15
-    |
- LL | const E: () = std::panic!("{}", MSG);
-    |               ^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'hello', $DIR/const_panic_2021.rs:19:15
-    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
- 
- error[E0080]: evaluation of constant value failed
- error[E0080]: evaluation of constant value failed
+ error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
43    |
43    |
44 LL | const A_CORE: () = core::panic!("shark");

-    |                    ^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'shark', $DIR/const_panic_2021.rs:22:20
46    |
-    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `$crate::const_format_args` (in Nightly builds, run with -Z macro-backtrace for more info)
48 
48 
- error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic_2021.rs:25:20
-    |
- LL | const B_CORE: () = core::panic!();
-    |
-    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
+ error: aborting due to 2 previous errors
56 
56 
- error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic_2021.rs:28:20
-    |
- LL | const C_CORE: () = core::unreachable!();
-    |                    ^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic_2021.rs:28:20
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
- 
- error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic_2021.rs:31:20
-   --> $DIR/const_panic_2021.rs:31:20
-    |
- LL | const D_CORE: () = core::unimplemented!();
-    |                    ^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'not implemented', $DIR/const_panic_2021.rs:31:20
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
- 
- error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic_2021.rs:34:20
-   --> $DIR/const_panic_2021.rs:34:20
-    |
- LL | const E_CORE: () = core::panic!("{}", MSG);
-    |                    ^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'hello', $DIR/const_panic_2021.rs:34:20
-    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
- 
- error: aborting due to 10 previous errors
- 
- 
- For more information about this error, try `rustc --explain E0080`.
+ For more information about this error, try `rustc --explain E0015`.
84 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_2021/const_panic_2021.stderr
To only update this specific test, also pass `--test-args consts/const-eval/const_panic_2021.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic_2021.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_2021" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-Zunstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_2021/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:7:15
   |
LL | const A: () = std::panic!("blåhaj");
   |
   = note: this error originates in the macro `$crate::const_format_args` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:22:20
   |
LL | const A_CORE: () = core::panic!("shark");
   |
   = note: this error originates in the macro `$crate::const_format_args` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors
---
test result: FAILED. 12048 passed; 2 failed; 102 ignored; 0 measured; 0 filtered out; finished in 128.67s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:42
