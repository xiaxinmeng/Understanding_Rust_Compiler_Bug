plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 12142 tests
.................................................................................................... 100/12142
.............................................ii.............iiF..................................... 200/12142
.................................................................................................... 400/12142
.................................................................................................... 500/12142
.................................................................................................... 600/12142
...............................................i.................................................... 700/12142
---
.............ii.ii.......i...i...................................................................... 6500/12142
............................................................i....i.................................. 6600/12142
..i...........................i..................................................................... 6700/12142
..........................................................................i......................... 6800/12142
............F..F.................................................................................... 6900/12142
................................i................................................................... 7100/12142
.................................................................................................... 7200/12142
.................................................................................................... 7300/12142
.................................................................................................... 7400/12142
---
.................................i.i................................................................ 12100/12142
..........................................
failures:

---- [ui] ui/asm/inline-syntax.rs#x86_64 stdout ----


1 warning: avoid using `.intel_syntax`, Intel syntax is the default
+    |
+    |
+ LL | global_asm!(".intel_syntax noprefix", "nop");
+    |
+    = note: `#[warn(bad_asm_style)]` on by default
+ 
+ 
+ warning: avoid using `.intel_syntax`, Intel syntax is the default
3    |
3    |
4 LL |         asm!(".intel_syntax noprefix", "nop");
5    |               ^^^^^^^^^^^^^^^^^^^^^^
-    |
-    = note: `#[warn(bad_asm_style)]` on by default
8 
8 
9 warning: avoid using `.intel_syntax`, Intel syntax is the default

35    |
35    |
36 LL |             .intel_syntax noprefix
- 
- 
- warning: avoid using `.intel_syntax`, Intel syntax is the default
-   --> $DIR/inline-syntax.rs:57:14
-    |
- LL | global_asm!(".intel_syntax noprefix", "nop");
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
44 
45 warning: 7 warnings emitted
46 
46 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/inline-syntax.x86_64/inline-syntax.x86_64.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/inline-syntax.rs`

error in revision `x86_64`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/inline-syntax.rs" "-Zthreads=1" "--cfg" "x86_64" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/inline-syntax.x86_64" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/inline-syntax.x86_64/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: avoid using `.intel_syntax`, Intel syntax is the default
   |
   |
LL | global_asm!(".intel_syntax noprefix", "nop");
   |
   = note: `#[warn(bad_asm_style)]` on by default


warning: avoid using `.intel_syntax`, Intel syntax is the default
   |
   |
LL |         asm!(".intel_syntax noprefix", "nop");


warning: avoid using `.intel_syntax`, Intel syntax is the default
   |
   |
LL |         asm!(".intel_syntax aaa noprefix", "nop");


warning: avoid using `.att_syntax`, prefer using `options(att_syntax)` instead
   |
   |
LL |         asm!(".att_syntax noprefix", "nop");


warning: avoid using `.att_syntax`, prefer using `options(att_syntax)` instead
   |
   |
LL |         asm!(".att_syntax bbb noprefix", "nop");


warning: avoid using `.intel_syntax`, Intel syntax is the default
   |
   |
LL |         asm!(".intel_syntax noprefix; nop");


warning: avoid using `.intel_syntax`, Intel syntax is the default
   |
   |
LL |             .intel_syntax noprefix

warning: 7 warnings emitted



------------------------------------------


---- [ui] ui/lint/semicolon-in-expressions-from-macros/allow-semicolon-in-expressions-from-macros.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/semicolon-in-expressions-from-macros/allow-semicolon-in-expressions-from-macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/semicolon-in-expressions-from-macros/allow-semicolon-in-expressions-from-macros" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/semicolon-in-expressions-from-macros/allow-semicolon-in-expressions-from-macros/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: failed to process buffered lint here (dummy = false)
   |
LL |         true;
   |             ^
...
...
LL |         _ => foo!()
   |              ------ in this macro invocation
   |
   = note: delayed at /checkout/compiler/rustc_lint/src/early.rs:400:18
   = note: this error: internal compiler error originates in the macro `foo` (in Nightly builds, run with -Z macro-backtrace for more info)

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1050:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (c1a35a796 2021-07-15) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack

------------------------------------------
---

4 LL |         true;
5    |             ^
6 ...
- LL |         foo!(warn_in_block)
-    |         ------------------- in this macro invocation
+ LL |     let _ = foo!(second);
9    |
10 note: the lint level is defined here
11   --> $DIR/semicolon-in-expressions-from-macros.rs:4:9

---
+    |
+ LL |         true;
+    |             ^
+ ...
+ LL |         foo!(warn_in_block)
+    |
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>
+    = note: this warning originates in the macro `foo` (in Nightly builds, run with -Z macro-backtrace for more info)
---
To only update this specific test, also pass `--test-args lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: trailing semicolon in macro used in expression position
  --> /checkout/src/test/ui/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros.rs:9:13
   |
LL |         true; //~  WARN trailing semicolon in macro
...
...
LL |     let _ = foo!(second);
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros.rs:4:9
   |
---

warning: trailing semicolon in macro used in expression position
  --> /checkout/src/test/ui/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros.rs:9:13
   |
LL |         true; //~  WARN trailing semicolon in macro
...
...
LL |     let _ = foo!(warn_in_expr);
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>
   = note: this warning originates in the macro `foo` (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this warning originates in the macro `foo` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: trailing semicolon in macro used in expression position
  --> /checkout/src/test/ui/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros.rs:9:13
   |
LL |         true; //~  WARN trailing semicolon in macro
...
...
LL |     let _ = #[allow(semicolon_in_expressions_from_macros)] foo!(allow_does_not_work);
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>
   = note: this warning originates in the macro `foo` (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this warning originates in the macro `foo` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: trailing semicolon in macro used in expression position
  --> /checkout/src/test/ui/lint/semicolon-in-expressions-from-macros/semicolon-in-expressions-from-macros.rs:9:13
   |
LL |         true; //~  WARN trailing semicolon in macro
...
...
LL |         foo!(warn_in_block)
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>
   = note: this warning originates in the macro `foo` (in Nightly builds, run with -Z macro-backtrace for more info)
---
test result: FAILED. 12038 passed; 3 failed; 101 ignored; 0 measured; 0 filtered out; finished in 120.63s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:29
