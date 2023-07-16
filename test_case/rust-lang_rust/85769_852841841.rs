plain
...i................................................................................................ 2000/11934
.................................................................................................... 2100/11934
.................................................................................................... 2200/11934
.........................i.......................................................................... 2300/11934
....F......ii......................F................................................................ 2400/11934
...............................................i.i.........................i........................ 2600/11934
...........i........................................................................................ 2700/11934
.................i..i............................................................................... 2800/11934
.................................................................................................... 2900/11934
---
1 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ref_to_int_match.rs:26:1
+   --> $DIR/ref_to_int_match.rs:25:1
3    |
4 LL | const BAR: Int = unsafe { Foo { r: &42 }.f };
5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc2, but expected initialized plain (non-pointer) bytes
10            }
11 
12 error: could not evaluate constant pattern
-   --> $DIR/ref_to_int_match.rs:8:14
-   --> $DIR/ref_to_int_match.rs:8:14
+   --> $DIR/ref_to_int_match.rs:7:14
14    |
15 LL |         10..=BAR => {},

17 
18 error: could not evaluate constant pattern
-   --> $DIR/ref_to_int_match.rs:8:14
-   --> $DIR/ref_to_int_match.rs:8:14
+   --> $DIR/ref_to_int_match.rs:7:14
20    |
21 LL |         10..=BAR => {},


The actual 32bit.stderr differed from the expected 32bit.stderr.
The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match/ref_to_int_match.32bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ref_to_int_match.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs:25:1
   |
LL | const BAR: Int = unsafe { Foo { r: &42 }.f }; //~ ERROR it is undefined behavior to use this value
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc2, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs:7:14
  --> /checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs:7:14
   |
LL |         10..=BAR => {}, //~ ERROR could not evaluate constant pattern

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs:7:14
   |
   |
LL |         10..=BAR => {}, //~ ERROR could not evaluate constant pattern

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.
---
1 warning: any use of this value will cause an error
-   --> $DIR/validate_uninhabited_zsts.rs:5:14
+   --> $DIR/validate_uninhabited_zsts.rs:4:14
3    |
4 LL |     unsafe { std::mem::transmute(()) }

6    |              |
7    |              transmuting to uninhabited type
-    |              inside `foo` at $DIR/validate_uninhabited_zsts.rs:5:14
-    |              inside `foo` at $DIR/validate_uninhabited_zsts.rs:5:14
-    |              inside `FOO` at $DIR/validate_uninhabited_zsts.rs:15:26
+    |              inside `foo` at $DIR/validate_uninhabited_zsts.rs:4:14
+    |              inside `FOO` at $DIR/validate_uninhabited_zsts.rs:14:26
10 ...
11 LL | const FOO: [Empty; 3] = [foo(); 3];

13    |
14 note: the lint level is defined here
-   --> $DIR/validate_uninhabited_zsts.rs:14:8
-   --> $DIR/validate_uninhabited_zsts.rs:14:8
+   --> $DIR/validate_uninhabited_zsts.rs:13:8
16    |
17 LL | #[warn(const_err)]

20    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
21 
22 error[E0080]: it is undefined behavior to use this value
22 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/validate_uninhabited_zsts.rs:18:1
+   --> $DIR/validate_uninhabited_zsts.rs:17:1
24    |
25 LL | const BAR: [Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
26    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of uninhabited type Empty at [0]

29    = note: the raw bytes of the constant (size: 0, align: 1) {}
30 
31 warning: the type `!` does not permit zero-initialization
-   --> $DIR/validate_uninhabited_zsts.rs:5:14
33    |
33    |
34 LL |     unsafe { std::mem::transmute(()) }


41    = note: the `!` type has no valid value
42 
43 warning: the type `Empty` does not permit zero-initialization
-   --> $DIR/validate_uninhabited_zsts.rs:18:35
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
  --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:4:14
   |
LL |     unsafe { std::mem::transmute(()) }
   |              |
   |              transmuting to uninhabited type
   |              inside `foo` at /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:4:14
   |              inside `FOO` at /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:14:26
   |              inside `FOO` at /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:14:26
...
LL | const FOO: [Empty; 3] = [foo(); 3];
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:13:8
   |
   |
LL | #[warn(const_err)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:17:1
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
test result: FAILED. 11793 passed; 2 failed; 139 ignored; 0 measured; 0 filtered out; finished in 70.16s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--pass" "check" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test src/test/ui --pass=check --host= --target=i686-unknown-linux-gnu
Build completed unsuccessfully in 0:01:12
