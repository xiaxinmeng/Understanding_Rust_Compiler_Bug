plain
.............................................................................i..ii.................. 7900/12088
............................................ii...................................................... 8000/12088
.................................................................................................... 8100/12088
.................................................i................................i................. 8200/12088
...........................................i..FF.................................................... 8300/12088
.................................................................................................... 8500/12088
.....................................................i.............................................. 8600/12088
.................................................................................................... 8700/12088
.................................................................................................... 8800/12088
---
.............................................................................i.i.................... 12000/12088
........................................................................................
failures:

---- [ui] ui/attributes/key-value-non-ascii.rs stdout ----
\xEF\xAC\x83.rs

2   --> $DIR/key-value-non-ascii.rs:3:19
3    |
3    |
4 LL | #[rustc_dummy = b"ﬃ.rs"]
-    |                   |
-    |                   byte constant must be ASCII
-    |                   byte constant must be ASCII
-    |                   help: use a \xHH escape for a non-ASCII byte: `\xFB03`
+    |                   ^ byte constant must be ASCII
+    |
+ help: if you meant to use the UTF-8 encoding of 'ﬃ', use \xHH escapes
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |
+ LL | #[rustc_dummy = b"/xEF/xAC/x83.rs"]
9 
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/key-value-non-ascii/key-value-non-ascii.stderr
To only update this specific test, also pass `--test-args attributes/key-value-non-ascii.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/attributes/key-value-non-ascii.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/key-value-non-ascii" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/key-value-non-ascii/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: non-ASCII character in byte constant
  --> /checkout/src/test/ui/attributes/key-value-non-ascii.rs:3:19
   |
LL | #[rustc_dummy = b"ﬃ.rs"] //~ ERROR non-ASCII character in byte constant
   |                   ^ byte constant must be ASCII
   |
help: if you meant to use the UTF-8 encoding of 'ﬃ', use \xHH escapes
   |
LL | #[rustc_dummy = b"\xEF\xAC\x83.rs"] //~ ERROR non-ASCII character in byte constant

error: aborting due to previous error


---
25    |
26 LL |     b"é";
-    |       ^
-    |       |
-    |       byte constant must be ASCII
-    |       help: use a \xHH escape for a non-ASCII byte: `\xE9`
+    |       ^ byte constant must be ASCII
+    |
+ help: if you meant to use the unicode code point for 'é', use a \xHH escape
+    |
+ LL |     b"\xE9";
31 
31 
32 error: raw byte string must be ASCII


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/byte-string-literals/byte-string-literals.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/byte-string-literals/byte-string-literals.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/byte-string-literals.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/byte-string-literals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/byte-string-literals" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/byte-string-literals/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown byte escape: `f`
   |
   |
LL | static FOO: &'static [u8] = b"\f";  //~ ERROR unknown byte escape
   |                                ^ unknown byte escape
   |
   = help: for more information, visit <https://static.rust-lang.org/doc/master/reference.html#literals>

error: unknown byte escape: `f`
   |
   |
LL |     b"\f";  //~ ERROR unknown byte escape
   |        ^ unknown byte escape
   |
   = help: for more information, visit <https://static.rust-lang.org/doc/master/reference.html#literals>

error: invalid character in numeric character escape: `Z`
   |
   |
LL |     b"\x0Z";  //~ ERROR invalid character in numeric character escape: `Z`
   |          ^ invalid character in numeric character escape
error: non-ASCII character in byte constant
  --> /checkout/src/test/ui/parser/byte-string-literals.rs:6:7
   |
   |
LL |     b"é";  //~ ERROR non-ASCII character in byte constant
   |       ^ byte constant must be ASCII
   |
help: if you meant to use the unicode code point for 'é', use a \xHH escape
   |
LL |     b"\xE9";  //~ ERROR non-ASCII character in byte constant


error: raw byte string must be ASCII
   |
   |
LL |     br##"é"##;  //~ ERROR raw byte string must be ASCII
   |          ^ must be ASCII
error[E0766]: unterminated double quote byte string
  --> /checkout/src/test/ui/parser/byte-string-literals.rs:8:6
   |
   |
LL |       b"a  //~ ERROR unterminated double quote byte string
LL | | }
   | |__^

error: aborting due to 6 previous errors
---
37    |
38 LL |     b'é';
-    |       ^
-    |       |
-    |       byte constant must be ASCII
-    |       help: use a \xHH escape for a non-ASCII byte: `\xE9`
+    |       ^ byte constant must be ASCII
+    |
+ help: if you meant to use the unicode code point for 'é', use a \xHH escape
+    |
+ LL |     b'\xE9';
43 
44 error[E0763]: unterminated byte constant
45   --> $DIR/byte-literals.rs:11:6

---
To only update this specific test, also pass `--test-args parser/byte-literals.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/byte-literals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/byte-literals" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/byte-literals/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown byte escape: `f`
   |
   |
LL | static FOO: u8 = b'\f';  //~ ERROR unknown byte escape
   |                     ^ unknown byte escape
   |
   = help: for more information, visit <https://static.rust-lang.org/doc/master/reference.html#literals>

error: unknown byte escape: `f`
   |
   |
LL |     b'\f';  //~ ERROR unknown byte escape
   |        ^ unknown byte escape
   |
   = help: for more information, visit <https://static.rust-lang.org/doc/master/reference.html#literals>

error: invalid character in numeric character escape: `Z`
   |
   |
LL |     b'\x0Z';  //~ ERROR invalid character in numeric character escape: `Z`
   |          ^ invalid character in numeric character escape

error: byte constant must be escaped: `\t`
   |
   |
LL |     b'    ';  //~ ERROR byte constant must be escaped
   |       ^^^^ help: escape the character: `\t`

error: byte constant must be escaped: `'`
   |
   |
LL |     b''';  //~ ERROR byte constant must be escaped
   |       ^ help: escape the character: `\'`
error: non-ASCII character in byte constant
  --> /checkout/src/test/ui/parser/byte-literals.rs:10:7
   |
   |
LL |     b'é';  //~ ERROR non-ASCII character in byte constant
   |       ^ byte constant must be ASCII
   |
help: if you meant to use the unicode code point for 'é', use a \xHH escape
   |
LL |     b'\xE9';  //~ ERROR non-ASCII character in byte constant

error[E0763]: unterminated byte constant
  --> /checkout/src/test/ui/parser/byte-literals.rs:11:6
   |
   |
LL |     b'a  //~ ERROR unterminated byte constant [E0763]

error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0763`.
---
test result: FAILED. 11982 passed; 3 failed; 103 ignored; 0 measured; 0 filtered out; finished in 106.77s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:10:41
