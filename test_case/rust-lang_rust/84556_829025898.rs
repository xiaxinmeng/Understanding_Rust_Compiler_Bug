plain
.................................................................................................... 9400/11801
.................................................................................................... 9500/11801
.................................................................................................... 9600/11801
.........................i......i................................................................... 9700/11801
.......................................................................iiiiiii..iiiiii.i............ 9800/11801
.................................................................................................... 10000/11801
.................................................................................................... 10100/11801
.................................................................................................... 10200/11801
.................................................................................................... 10300/11801
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 31 ignored; 0 measured; 0 filtered out; finished in 0.11s

 finished in 0.181 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.35s

 finished in 2.426 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 25 tests
iiiiiiiiiiiii............

 finished in 2.629 seconds
Build completed successfully in 0:29:54
+ python2.7 ../x.py --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
---
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 11801 tests
......................i............................................................................. 100/11801
......................................iiiii.iii.iiiiiiiii........................................... 200/11801
.................................................................................................... 400/11801
.................................................................................................... 500/11801
..............................................................................................i..... 600/11801
.................................................................................................... 700/11801
---
.................................................................................................... 9300/11801
.................................................................................................... 9400/11801
.................................................................................................... 9500/11801
.................................................................................................... 9600/11801
.........................i......i.....................................................iiiiiii....... 9700/11801
.......................................................................iiiiiii..iiiiii.i............ 9800/11801
.................................................................................................... 10000/11801
.................................................................................................... 10100/11801
.................................................................................................... 10200/11801
.................................................................................................... 10300/11801
---

---- [ui] ui/consts/const-eval/validate_uninhabited_zsts.rs stdout ----
diff of 32bit.stderr:

1 warning: any use of this value will cause an error
-   --> $DIR/validate_uninhabited_zsts.rs:6:14
3    |
3    |
4 LL |     unsafe { std::mem::transmute(()) }

6    |              |
7    |              transmuting to uninhabited type
-    |              inside `foo` at $DIR/validate_uninhabited_zsts.rs:6:14
-    |              inside `foo` at $DIR/validate_uninhabited_zsts.rs:6:14
-    |              inside `FOO` at $DIR/validate_uninhabited_zsts.rs:16:26
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
+    |              inside `foo` at $DIR/validate_uninhabited_zsts.rs:5:14
+    |              inside `FOO` at $DIR/validate_uninhabited_zsts.rs:15:26
10 ...
11 LL | const FOO: [Empty; 3] = [foo(); 3];

13    |
14 note: the lint level is defined here
-   --> $DIR/validate_uninhabited_zsts.rs:15:8
-   --> $DIR/validate_uninhabited_zsts.rs:15:8
+   --> $DIR/validate_uninhabited_zsts.rs:14:8
16    |
17 LL | #[warn(const_err)]

20    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
21 
22 error[E0080]: it is undefined behavior to use this value
22 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/validate_uninhabited_zsts.rs:19:1
+   --> $DIR/validate_uninhabited_zsts.rs:18:1
24    |
25 LL | const BAR: [Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
26    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of uninhabited type Empty at [0]

29    = note: the raw bytes of the constant (size: 0, align: 1) {}
30 
31 warning: the type `!` does not permit zero-initialization
-   --> $DIR/validate_uninhabited_zsts.rs:6:14
33    |
33    |
34 LL |     unsafe { std::mem::transmute(()) }


41    = note: the `!` type has no valid value
42 
43 warning: the type `Empty` does not permit zero-initialization
-   --> $DIR/validate_uninhabited_zsts.rs:19:35
45    |
45    |
46 LL | const BAR: [Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];


The actual 32bit.stderr differed from the expected 32bit.stderr.
The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts/validate_uninhabited_zsts.32bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/validate_uninhabited_zsts.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: any use of this value will cause an error
   |
   |
LL |     unsafe { std::mem::transmute(()) }
   |              |
   |              transmuting to uninhabited type
   |              inside `foo` at /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:5:14
   |              inside `FOO` at /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:15:26
   |              inside `FOO` at /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:15:26
...
LL | const FOO: [Empty; 3] = [foo(); 3];
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:14:8
   |
   |
LL | #[warn(const_err)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:18:1
   |
LL | const BAR: [Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of uninhabited type Empty at [0]
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 0, align: 1) {}

warning: the type `!` does not permit zero-initialization
   |
   |
LL |     unsafe { std::mem::transmute(()) }
   |              |
   |              |
   |              this code causes undefined behavior when executed
   |              help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   = note: `#[warn(invalid_value)]` on by default
   = note: the `!` type has no valid value

warning: the type `Empty` does not permit zero-initialization
   |
   |
LL | const BAR: [Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
   |                                   |
   |                                   |
   |                                   this code causes undefined behavior when executed
   |                                   help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   = note: enums with no variants have no valid value
error: aborting due to previous error; 3 warnings emitted

For more information about this error, try `rustc --explain E0080`.

---
test result: FAILED. 11662 passed; 1 failed; 138 ignored; 0 measured; 0 filtered out; finished in 70.40s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--pass" "check" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test src/test/ui --pass=check --host= --target=i686-unknown-linux-gnu
Build completed unsuccessfully in 0:01:12
