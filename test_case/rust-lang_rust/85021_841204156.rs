plain
.................................................................................................... 9500/11868
.................................................................................................... 9600/11868
.......................................................................i......i..................... 9700/11868
.................................................................................................... 9800/11868
................iiiiiii..iiiiii.i................................................................... 9900/11868
.................................................................................................... 10100/11868
.................................................................................................... 10200/11868
.................................................................................................... 10300/11868
.................................................................................................... 10400/11868
---

---- [ui] ui/asm/type-check-2.rs stdout ----
diff of stderr:

1 error: asm `const` arguments must be integer or floating-point values
-   --> $DIR/type-check-2.rs:34:20
3    |
3    |
4 LL |         asm!("{}", const 0 as *mut u8);

6 
7 error: arguments for inline assembly must be copyable
-   --> $DIR/type-check-2.rs:54:32
-   --> $DIR/type-check-2.rs:54:32
+   --> $DIR/type-check-2.rs:55:32
9    |
10 LL |         asm!("{}", in(xmm_reg) SimdNonCopy(0.0, 0.0, 0.0, 0.0));

12    |
12    |
13    = note: `SimdNonCopy` does not implement the Copy trait
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
14 
- error: cannot use value of type `[closure@$DIR/type-check-2.rs:66:28: 66:38]` for inline assembly
-   --> $DIR/type-check-2.rs:66:28
+ error: cannot use value of type `[closure@$DIR/type-check-2.rs:67:28: 67:38]` for inline assembly
17    |
17    |
18 LL |         asm!("{}", in(reg) |x: i32| x);


21    = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
22 
23 error: cannot use value of type `Vec<i32>` for inline assembly
-   --> $DIR/type-check-2.rs:68:28
25    |
25    |
26 LL |         asm!("{}", in(reg) vec![0]);


30    = note: this error originates in the macro `vec` (in Nightly builds, run with -Z macro-backtrace for more info)
31 
32 error: cannot use value of type `(i32, i32, i32)` for inline assembly
-   --> $DIR/type-check-2.rs:70:28
34    |
34    |
35 LL |         asm!("{}", in(reg) (1, 2, 3));


38    = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
39 
40 error: cannot use value of type `[i32; 3]` for inline assembly
-   --> $DIR/type-check-2.rs:72:28
42    |
42    |
43 LL |         asm!("{}", in(reg) [1, 2, 3]);


46    = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
47 
48 error: cannot use value of type `fn() {main}` for inline assembly
-   --> $DIR/type-check-2.rs:80:31
50    |
50    |
51 LL |         asm!("{}", inout(reg) f);


54    = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
55 
56 error: cannot use value of type `&mut i32` for inline assembly
-   --> $DIR/type-check-2.rs:83:31
58    |
58    |
59 LL |         asm!("{}", inout(reg) r);


62    = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
63 
64 error: asm `const` arguments must be integer or floating-point values
-   --> $DIR/type-check-2.rs:99:19
66    |
66    |
67 LL | global_asm!("{}", const 0 as *mut u8);

69 
69 
70 error: asm `sym` operand must point to a fn or static
-   --> $DIR/type-check-2.rs:47:24
72    |
72    |
73 LL |         asm!("{}", sym C);

75 
75 
76 error: asm `sym` operand must point to a fn or static
-   --> $DIR/type-check-2.rs:49:24
78    |
78    |
79 LL |         asm!("{}", sym x);

81 
81 
82 error[E0381]: use of possibly-uninitialized variable: `x`
-   --> $DIR/type-check-2.rs:13:28
84    |
84    |
85 LL |         asm!("{}", in(reg) x);
86    |                            ^ use of possibly-uninitialized `x`
87 
87 
88 error[E0381]: use of possibly-uninitialized variable: `y`
-   --> $DIR/type-check-2.rs:16:9
90    |
90    |
91 LL |         asm!("{}", inout(reg) y);
92    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ use of possibly-uninitialized `y`
93 
93 
94 error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable
-   --> $DIR/type-check-2.rs:24:29
96    |
96    |
97 LL |         let v: Vec<u64> = vec![0, 1, 2];
98    |             - help: consider changing this to be mutable: `mut v`

101    |                             ^ cannot borrow as mutable
102 
103 error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable
-   --> $DIR/type-check-2.rs:26:31
105    |
105    |
106 LL |         let v: Vec<u64> = vec![0, 1, 2];
107    |             - help: consider changing this to be mutable: `mut v`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-2/type-check-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/type-check-2.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/type-check-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: asm `const` arguments must be integer or floating-point values
   |
   |
LL |         asm!("{}", const 0 as *mut u8);

error: arguments for inline assembly must be copyable
  --> /checkout/src/test/ui/asm/type-check-2.rs:55:32
   |
   |
LL |         asm!("{}", in(xmm_reg) SimdNonCopy(0.0, 0.0, 0.0, 0.0));
   |
   |
   = note: `SimdNonCopy` does not implement the Copy trait

error: cannot use value of type `[closure@/checkout/src/test/ui/asm/type-check-2.rs:67:28: 67:38]` for inline assembly
   |
   |
LL |         asm!("{}", in(reg) |x: i32| x);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly

error: cannot use value of type `Vec<i32>` for inline assembly
   |
   |
LL |         asm!("{}", in(reg) vec![0]);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
   = note: this error originates in the macro `vec` (in Nightly builds, run with -Z macro-backtrace for more info)

error: cannot use value of type `(i32, i32, i32)` for inline assembly
   |
   |
LL |         asm!("{}", in(reg) (1, 2, 3));
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly

error: cannot use value of type `[i32; 3]` for inline assembly
   |
   |
LL |         asm!("{}", in(reg) [1, 2, 3]);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly

error: cannot use value of type `fn() {main}` for inline assembly
   |
   |
LL |         asm!("{}", inout(reg) f);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly

error: cannot use value of type `&mut i32` for inline assembly
   |
   |
LL |         asm!("{}", inout(reg) r);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly

error: asm `const` arguments must be integer or floating-point values
   |
   |
LL | global_asm!("{}", const 0 as *mut u8);


error: asm `sym` operand must point to a fn or static
   |
   |
LL |         asm!("{}", sym C);


error: asm `sym` operand must point to a fn or static
   |
   |
LL |         asm!("{}", sym x);


error[E0381]: use of possibly-uninitialized variable: `x`
   |
   |
LL |         asm!("{}", in(reg) x);
   |                            ^ use of possibly-uninitialized `x`

error[E0381]: use of possibly-uninitialized variable: `y`
   |
   |
LL |         asm!("{}", inout(reg) y);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^ use of possibly-uninitialized `y`

error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable
   |
   |
LL |         let v: Vec<u64> = vec![0, 1, 2];
   |             - help: consider changing this to be mutable: `mut v`
LL |         asm!("{}", in(reg) v[0]);
LL |         asm!("{}", out(reg) v[0]);
   |                             ^ cannot borrow as mutable

error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable
   |
   |
LL |         let v: Vec<u64> = vec![0, 1, 2];
   |             - help: consider changing this to be mutable: `mut v`
...
LL |         asm!("{}", inout(reg) v[0]);
   |                               ^ cannot borrow as mutable
error: aborting due to 15 previous errors

Some errors have detailed explanations: E0381, E0596.
For more information about an error, try `rustc --explain E0381`.
---
test result: FAILED. 11771 passed; 1 failed; 96 ignored; 0 measured; 0 filtered out; finished in 122.78s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:06
