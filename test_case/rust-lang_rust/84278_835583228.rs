plain
.................................................................................................... 9500/11838
.................................................................................................... 9600/11838
......................................................i......i...................................... 9700/11838
.................................................................................................... 9800/11838
iiiiiii..iiiiii.i................................................................................... 9900/11838
.................................................................................................... 10100/11838
.................................................................................................... 10200/11838
.................................................................................................... 10300/11838
.................................................................................................... 10400/11838
---
diff of stdout:

20 /*
21 Expansions:
22 0: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: Root
- 1: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro(Bang, "foo")
+ 1: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro { kind: Bang, name: "foo", proc_macro: false }
25 SyntaxContexts:
25 SyntaxContexts:
26 #0: parent: #0, outer_mark: (ExpnId(0), Opaque)

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/unpretty-debug/unpretty-debug.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/unpretty-debug/unpretty-debug.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hygiene/unpretty-debug.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/unpretty-debug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/unpretty-debug" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunpretty=expanded,hygiene" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/unpretty-debug/auxiliary"
------------------------------------------
// check-pass
// check-pass
// compile-flags: -Zunpretty=expanded,hygiene

// Don't break whenever Symbol numbering changes
// normalize-stdout-test "\d+#" -> "0#"

// minimal junk
#![feature /* 486#0 */(no_core)]
#![no_core /* 729#0 */]

macro_rules! foo /* 1289#0 */ { ($ x : ident) => { y + $ x } }

fn bar /* 1292#0 */() {
    let x /* 1290#0 */ = 1;
    y /* 1291#1 */ + x /* 1290#0 */


fn y /* 1291#0 */() { }
/*
Expansions:
Expansions:
0: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: Root
1: parent: ExpnId(0), call_site_ctxt: #0, def_site_ctxt: #0, kind: Macro { kind: Bang, name: "foo", proc_macro: false }
SyntaxContexts:
SyntaxContexts:
#0: parent: #0, outer_mark: (ExpnId(0), Opaque)
#1: parent: #0, outer_mark: (ExpnId(1), SemiTransparent)

------------------------------------------
stderr:
------------------------------------------
---
---- [ui] ui/macros/same-sequence-span.rs stdout ----
diff of stderr:

14    |
15    = note: allowed there are: `=>`, `,` or `;`
16 
- error: `$x:expr` may be followed by `$y:tt`, which is not allowed for `expr` fragments
+ error: proc macro panicked
19    |
19    |
20 LL | proc_macro_sequence::make_foo!();
-    | ---------------------------------^^^^^^^^^^^^^
-    | |
-    | |
-    | not allowed after `expr` fragments
-    | in this macro invocation
25    |
25    |
-    = note: allowed there are: `=>`, `,` or `;`
+    = help: message: assertion failed: `(left == right)`
+    = help: message: assertion failed: `(left == right)`
+              left: `LineColumn { line: 21, column: 21 }`,
+             right: `LineColumn { line: 21, column: 8 }`
28 
- error: `$x:expr` may be followed by `=`, which is not allowed for `expr` fragments
-   --> $DIR/same-sequence-span.rs:19:1
-    |
- LL | proc_macro_sequence::make_foo!();
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not allowed after `expr` fragments
-    |
-    = note: allowed there are: `=>`, `,` or `;`
- 
- error: aborting due to 4 previous errors
+ error: aborting due to 3 previous errors
39 
---
To only update this specific test, also pass `--test-args macros/same-sequence-span.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/same-sequence-span.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/same-sequence-span" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/same-sequence-span/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `$x:expr` may be followed by `$y:tt`, which is not allowed for `expr` fragments
   |
   |
LL |     (1 $x:expr $($y:tt,)*   //~ERROR `$x:expr` may be followed by `$y:tt`
   |                  ^^^^^ not allowed after `expr` fragments
   |
   = note: allowed there are: `=>`, `,` or `;`

error: `$x:expr` may be followed by `=`, which is not allowed for `expr` fragments
   |
   |
LL |                $(= $z:tt)*  //~ERROR `$x:expr` may be followed by `=`
   |                  ^ not allowed after `expr` fragments
   |
   = note: allowed there are: `=>`, `,` or `;`
error: proc macro panicked
  --> /checkout/src/test/ui/macros/same-sequence-span.rs:19:1
   |
   |
LL | proc_macro_sequence::make_foo!(); //~ERROR `$x:expr` may be followed by `$y:tt`
   |
   = help: message: assertion failed: `(left == right)`
   = help: message: assertion failed: `(left == right)`
             left: `LineColumn { line: 21, column: 21 }`,
            right: `LineColumn { line: 21, column: 8 }`
error: aborting due to 3 previous errors


------------------------------------------
---
test result: FAILED. 11740 passed; 2 failed; 96 ignored; 0 measured; 0 filtered out; finished in 123.93s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:55
