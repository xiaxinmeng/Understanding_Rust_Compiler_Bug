plain

---- [ui] ui/fmt/format-string-error-2.rs stdout ----
diff of stderr:

+ warning: multiple lines skipped by escaped newline
+    |
+ LL |       format!("{ \
+    |  ________________^
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ LL | |
+ LL | |     b");
+    | |____^ skipping everything up to and including this point
+ 
+ warning: multiple lines skipped by escaped newline
+    |
+ LL |       b \
+    |  _______^
+ LL | |
---
2   --> $DIR/format-string-error-2.rs:77:20
3    |

171    |
172    = note: if you intended to print `}`, you can escape it using `}}`
- error: aborting due to 18 previous errors
+ error: aborting due to 18 previous errors; 2 warnings emitted
175 
176 
176 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error-2/format-string-error-2.stderr
To only update this specific test, also pass `--test-args fmt/format-string-error-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/format-string-error-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/format-string-error-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: multiple lines skipped by escaped newline
   |
LL |       format!("{ \
   |  ________________^
LL | |
LL | |
LL | |     b");
   | |____^ skipping everything up to and including this point

warning: multiple lines skipped by escaped newline
   |
LL |       b \
   |  _______^
LL | |
LL | |
LL | |     ");
   | |____^ skipping everything up to and including this point

error: incorrect unicode escape sequence
  --> /checkout/src/test/ui/fmt/format-string-error-2.rs:77:20
   |
LL |     println!("\x7B}\u8 {", 1);
   |                    ^^^ help: format of unicode escape sequences uses braces: `\u{8}`

error: invalid format string: expected `'}'`, found `'a'`
   |
LL |     format!("{
   |              - because of this opening brace
LL |     a");
LL |     a");
   |     ^ expected `}` in format string
   |
   = note: if you intended to print `{`, you can escape it using `{{`

error: invalid format string: expected `'}'`, found `'b'`
   |
LL |     format!("{ \
   |              - because of this opening brace
LL | 
LL | 
LL |     b");
   |     ^ expected `}` in format string
   |
   = note: if you intended to print `{`, you can escape it using `{{`

error: invalid format string: expected `'}'`, found `'\\'`
   |
   |
LL |     format!(r#"{ \
   |                - ^ expected `}` in format string
   |                because of this opening brace
   |
   |
   = note: if you intended to print `{`, you can escape it using `{{`

error: invalid format string: expected `'}'`, found `'\\'`
   |
   |
LL |     format!(r#"{ \n
   |                - ^ expected `}` in format string
   |                because of this opening brace
   |
   |
   = note: if you intended to print `{`, you can escape it using `{{`

error: invalid format string: expected `'}'`, found `'e'`
   |
LL |     format!("{ \n
   |              - because of this opening brace
LL | \n
LL | \n
LL |     e");
   |     ^ expected `}` in format string
   |
   = note: if you intended to print `{`, you can escape it using `{{`

error: invalid format string: expected `'}'`, found `'a'`
   |
LL |     {
   |     - because of this opening brace
LL |     a");
LL |     a");
   |     ^ expected `}` in format string
   |
   = note: if you intended to print `{`, you can escape it using `{{`

error: invalid format string: expected `'}'`, found `'a'`
   |
LL |     {
   |     - because of this opening brace
LL |     a
LL |     a
   |     ^ expected `}` in format string
   |
   = note: if you intended to print `{`, you can escape it using `{{`

error: invalid format string: expected `'}'`, found `'b'`
   |
LL |     { \
   |     - because of this opening brace
LL |         \
LL |         \
LL |     b");
   |     ^ expected `}` in format string
   |
   = note: if you intended to print `{`, you can escape it using `{{`

error: invalid format string: expected `'}'`, found `'b'`
   |
LL |     { \
   |     - because of this opening brace
LL |         \
LL |         \
LL |     b \
   |     ^ expected `}` in format string
   |
   = note: if you intended to print `{`, you can escape it using `{{`

error: invalid format string: expected `'}'`, found `'\\'`
   |
   |
LL | raw  { \
   |      - ^ expected `}` in format string
   |      because of this opening brace
   |
   |
   = note: if you intended to print `{`, you can escape it using `{{`

error: invalid format string: expected `'}'`, found `'\\'`
   |
   |
LL | raw  { \n
   |      - ^ expected `}` in format string
   |      because of this opening brace
   |
   |
   = note: if you intended to print `{`, you can escape it using `{{`

error: invalid format string: expected `'}'`, found `'e'`
   |
LL |   { \n
   |   - because of this opening brace
LL | \n
LL | \n
LL |     e");
   |     ^ expected `}` in format string
   |
   = note: if you intended to print `{`, you can escape it using `{{`

error: invalid format string: expected `'}'`, found `'a'`
   |
LL |     {
   |     - because of this opening brace
LL |     asdf}
LL |     asdf}
   |     ^ expected `}` in format string
   |
   = note: if you intended to print `{`, you can escape it using `{{`

error: 1 positional argument in format string, but no arguments were given
   |
   |
LL |     println!("\t{}");


error: invalid format string: expected `'}'` but string was terminated
   |
   |
LL |     println!("\x7B}\u{8} {", 1);
   |                          -^ expected `'}'` in format string
   |                          because of this opening brace
   |
   |
   = note: if you intended to print `{`, you can escape it using `{{`

error: invalid format string: unmatched `}` found
   |
   |
LL |     println!(r#"\x7B}\u{8} {"#, 1);
   |                     ^ unmatched `}` in format string
   |
   = note: if you intended to print `}`, you can escape it using `}}`

error: invalid format string: unmatched `}` found
   |
   |
LL |     println!(r#"\x7B}\u8 {"#, 1);
   |                     ^ unmatched `}` in format string
   |
   = note: if you intended to print `}`, you can escape it using `}}`
error: aborting due to 18 previous errors; 2 warnings emitted


------------------------------------------
---
test result: FAILED. 11984 passed; 1 failed; 103 ignored; 0 measured; 0 filtered out; finished in 121.09s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:00
