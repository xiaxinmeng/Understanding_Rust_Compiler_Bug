plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 11714 tests
.................................................................................................... 100/11714
...............................................................F..........F.........F............... 200/11714
.................................................................................................... 400/11714
.................................................................................................... 500/11714
.................................................................................................... 600/11714
.........................i.......................................................................... 700/11714
---
.................................................................................................... 9400/11714
.................................................................................................... 9500/11714
.......................................................i......i..................................... 9600/11714
.................................................................................................... 9700/11714
.iiiiiii..iiiiii.i.................................................................................. 9800/11714
.................................................................................................... 10000/11714
.................................................................................................... 10100/11714
.................................................................................................... 10200/11714
.................................................................................................... 10300/11714
---
failures:

---- [ui] ui/asm/const.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/const/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/const/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: unknown token in expression
  --> /checkout/src/test/ui/asm/const.rs:41:15
   |
LL |         asm!("mov {}, {}", out(reg) a, const 5);
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:2:6
   |
LL |     mov %rax, 5

error: unknown token in expression
  --> /checkout/src/test/ui/asm/const.rs:45:15
   |
   |
LL |         asm!("mov {}, {}", out(reg) b, const constfn(5));
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:2:6
   |
LL |     mov %rax, 5

error: unknown token in expression
  --> /checkout/src/test/ui/asm/const.rs:49:15
   |
   |
LL |         asm!("mov {}, {}", out(reg) c, const constfn(5) + constfn(5));
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:2:6
   |
LL |     mov %rax, 10

error: unknown token in expression
  --> /checkout/src/test/ui/asm/const.rs:25:15
   |
   |
LL |         asm!("mov {}, {}", out(reg) a, const size_of::<T>());
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:2:6
   |
LL |     mov %rax, 1

error: unknown token in expression
  --> /checkout/src/test/ui/asm/const.rs:29:15
   |
   |
LL |         asm!("mov {}, {}", out(reg) b, const size_of::<T>() + constfn(5));
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:2:6
   |
LL |     mov %rax, 6

error: unknown token in expression
  --> /checkout/src/test/ui/asm/const.rs:33:15
   |
   |
LL |         asm!("mov {}, {}", out(reg) c, const T::C);
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:2:6
   |
LL |     mov %rax, 8

error: unknown token in expression
  --> /checkout/src/test/ui/asm/const.rs:25:15
   |
   |
LL |         asm!("mov {}, {}", out(reg) a, const size_of::<T>());
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:2:6
   |
LL |     mov %rax, 2

error: unknown token in expression
  --> /checkout/src/test/ui/asm/const.rs:29:15
   |
   |
LL |         asm!("mov {}, {}", out(reg) b, const size_of::<T>() + constfn(5));
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:2:6
   |
LL |     mov %rax, 7

error: unknown token in expression
  --> /checkout/src/test/ui/asm/const.rs:33:15
   |
   |
LL |         asm!("mov {}, {}", out(reg) c, const T::C);
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:2:6
   |
LL |     mov %rax, 16

error: aborting due to 9 previous errors



------------------------------------------


---- [ui] ui/asm/srcloc.rs stdout ----
diff of stderr:

1 error: invalid instruction mnemonic 'invalid_instruction'
-   --> $DIR/srcloc.rs:11:15
3    |
3    |
4 LL |         asm!("invalid_instruction");

11    |     ^^^^^^^^^^^^^^^^^^^
12 
12 
13 error: invalid instruction mnemonic 'invalid_instruction'
-   --> $DIR/srcloc.rs:15:13
15    |
16 LL |             invalid_instruction
17    |             ^


23    |             ^^^^^^^^^^^^^^^^^^^
24 
25 error: invalid instruction mnemonic 'invalid_instruction'
-   --> $DIR/srcloc.rs:20:13
27    |
28 LL |             invalid_instruction
29    |             ^


35    |             ^^^^^^^^^^^^^^^^^^^
36 
37 error: invalid instruction mnemonic 'invalid_instruction'
-   --> $DIR/srcloc.rs:26:13
39    |
40 LL |             invalid_instruction
41    |             ^


47    |             ^^^^^^^^^^^^^^^^^^^
48 
49 error: invalid instruction mnemonic 'invalid_instruction'
-   --> $DIR/srcloc.rs:33:13
51    |
52 LL |             invalid_instruction
53    |             ^


59    |             ^^^^^^^^^^^^^^^^^^^
60 
61 error: invalid instruction mnemonic 'invalid_instruction'
-   --> $DIR/srcloc.rs:38:14
63    |
63    |
64 LL |         asm!(concat!("invalid", "_", "instruction"));

71    |     ^^^^^^^^^^^^^^^^^^^
72 
72 
73 warning: scale factor without index register is ignored
-   --> $DIR/srcloc.rs:41:15
75    |
75    |
76 LL |         asm!("movaps %xmm3, (%esi, 2)", options(att_syntax));

83    |                          ^
84 
84 
85 error: invalid instruction mnemonic 'invalid_instruction'
-   --> $DIR/srcloc.rs:45:14
87    |
88 LL |             "invalid_instruction",
89    |              ^


95    |     ^^^^^^^^^^^^^^^^^^^
96 
97 error: invalid instruction mnemonic 'invalid_instruction'
-   --> $DIR/srcloc.rs:51:14
99    |
100 LL |             "invalid_instruction",
101    |              ^


107    | ^^^^^^^^^^^^^^^^^^^
108 
109 error: invalid instruction mnemonic 'invalid_instruction'
-   --> $DIR/srcloc.rs:58:14
111    |
112 LL |             "invalid_instruction",
113    |              ^


119    | ^^^^^^^^^^^^^^^^^^^
120 
121 error: invalid instruction mnemonic 'invalid_instruction'
-   --> $DIR/srcloc.rs:65:13
123    |
123    |
124 LL |             concat!("invalid", "_", "instruction"),

131    | ^^^^^^^^^^^^^^^^^^^
132 
132 
133 error: invalid instruction mnemonic 'invalid_instruction'
-   --> $DIR/srcloc.rs:72:13
135    |
135    |
136 LL |             concat!("invalid", "_", "instruction"),

143    | ^^^^^^^^^^^^^^^^^^^
144 
144 
145 error: invalid instruction mnemonic 'invalid_instruction1'
-   --> $DIR/srcloc.rs:79:14
147    |
148 LL |             "invalid_instruction1",
149    |              ^


155    |     ^^^^^^^^^^^^^^^^^^^^
156 
157 error: invalid instruction mnemonic 'invalid_instruction2'
-   --> $DIR/srcloc.rs:80:14
159    |
160 LL |             "invalid_instruction2",
161    |              ^


167    | ^^^^^^^^^^^^^^^^^^^^
168 
169 error: invalid instruction mnemonic 'invalid_instruction1'
-   --> $DIR/srcloc.rs:86:13
171    |
172 LL |             concat!(
173    |             ^


179    |     ^^^^^^^^^^^^^^^^^^^^
180 
181 error: invalid instruction mnemonic 'invalid_instruction2'
-   --> $DIR/srcloc.rs:86:13
183    |
184 LL |             concat!(
185    |             ^


191    | ^^^^^^^^^^^^^^^^^^^^
192 
193 error: invalid instruction mnemonic 'invalid_instruction1'
-   --> $DIR/srcloc.rs:95:13
195    |
196 LL |             concat!(
197    |             ^


203    |     ^^^^^^^^^^^^^^^^^^^^
204 
205 error: invalid instruction mnemonic 'invalid_instruction2'
-   --> $DIR/srcloc.rs:95:13
207    |
208 LL |             concat!(
209    |             ^


215    | ^^^^^^^^^^^^^^^^^^^^
216 
217 error: invalid instruction mnemonic 'invalid_instruction3'
-   --> $DIR/srcloc.rs:99:13
219    |
220 LL |             concat!(
221    |             ^


227    | ^^^^^^^^^^^^^^^^^^^^
228 
229 error: invalid instruction mnemonic 'invalid_instruction4'
-   --> $DIR/srcloc.rs:99:13
231    |
232 LL |             concat!(
233    |             ^


239    | ^^^^^^^^^^^^^^^^^^^^
240 
241 error: invalid instruction mnemonic 'invalid_instruction1'
-   --> $DIR/srcloc.rs:110:13
243    |
244 LL |             concat!(
245    |             ^


251    |     ^^^^^^^^^^^^^^^^^^^^
252 
253 error: invalid instruction mnemonic 'invalid_instruction2'
-   --> $DIR/srcloc.rs:110:13
255    |
256 LL |             concat!(
257    |             ^


263    | ^^^^^^^^^^^^^^^^^^^^
264 
265 error: invalid instruction mnemonic 'invalid_instruction3'
-   --> $DIR/srcloc.rs:114:13
267    |
268 LL |             concat!(
269    |             ^


275    | ^^^^^^^^^^^^^^^^^^^^
276 
277 error: invalid instruction mnemonic 'invalid_instruction4'
-   --> $DIR/srcloc.rs:114:13
279    |
280 LL |             concat!(
281    |             ^



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/srcloc/srcloc.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/srcloc.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/srcloc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/srcloc" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/srcloc/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: invalid instruction mnemonic 'invalid_instruction'
   |
   |
LL |         asm!("invalid_instruction");
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:2:2
LL |     invalid_instruction
   |     ^^^^^^^^^^^^^^^^^^^


error: invalid instruction mnemonic 'invalid_instruction'
   |
LL |             invalid_instruction
   |             ^
   |
   |
note: instantiated into assembly here
  --> <inline asm>:3:13
LL |             invalid_instruction
   |             ^^^^^^^^^^^^^^^^^^^


error: invalid instruction mnemonic 'invalid_instruction'
   |
LL |             invalid_instruction
   |             ^
   |
   |
note: instantiated into assembly here
  --> <inline asm>:3:13
LL |             invalid_instruction
   |             ^^^^^^^^^^^^^^^^^^^


error: invalid instruction mnemonic 'invalid_instruction'
   |
LL |             invalid_instruction
   |             ^
   |
   |
note: instantiated into assembly here
  --> <inline asm>:4:13
LL |             invalid_instruction
   |             ^^^^^^^^^^^^^^^^^^^


error: invalid instruction mnemonic 'invalid_instruction'
   |
LL |             invalid_instruction
   |             ^
   |
   |
note: instantiated into assembly here
  --> <inline asm>:4:13
LL |             invalid_instruction
   |             ^^^^^^^^^^^^^^^^^^^


error: invalid instruction mnemonic 'invalid_instruction'
   |
   |
LL |         asm!(concat!("invalid", "_", "instruction"));
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:2:2
LL |     invalid_instruction
   |     ^^^^^^^^^^^^^^^^^^^


warning: scale factor without index register is ignored
   |
   |
LL |         asm!("movaps %xmm3, (%esi, 2)", options(att_syntax));
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:1:23
   |
LL |     movaps %xmm3, (%esi, 2)


error: invalid instruction mnemonic 'invalid_instruction'
   |
LL |             "invalid_instruction",
   |              ^
   |
   |
note: instantiated into assembly here
  --> <inline asm>:2:2
LL |     invalid_instruction
   |     ^^^^^^^^^^^^^^^^^^^


error: invalid instruction mnemonic 'invalid_instruction'
   |
LL |             "invalid_instruction",
   |              ^
   |
   |
note: instantiated into assembly here
  --> <inline asm>:3:1
LL | invalid_instruction
   | ^^^^^^^^^^^^^^^^^^^


error: invalid instruction mnemonic 'invalid_instruction'
   |
LL |             "invalid_instruction",
   |              ^
   |
   |
note: instantiated into assembly here
  --> <inline asm>:4:1
LL | invalid_instruction
   | ^^^^^^^^^^^^^^^^^^^


error: invalid instruction mnemonic 'invalid_instruction'
   |
   |
LL |             concat!("invalid", "_", "instruction"),
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:3:1
LL | invalid_instruction
   | ^^^^^^^^^^^^^^^^^^^


error: invalid instruction mnemonic 'invalid_instruction'
   |
   |
LL |             concat!("invalid", "_", "instruction"),
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:3:1
---
test result: FAILED. 11618 passed; 3 failed; 93 ignored; 0 measured; 0 filtered out; finished in 137.47s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:24
