plain
.................................................................................................... 8900/11247
.................................................................................................... 9000/11247
.................................................................................................... 9100/11247
.................................................................................................... 9200/11247
.....................F.....................i......i................................................. 9300/11247
..................................................................................iiiiii..iiiiii.i.. 9400/11247
.................................................................................................... 9600/11247
.................................................................................................... 9700/11247
.................................................................................................... 9800/11247
.................................................................................................... 9900/11247
---
---- [ui] ui/rfc-2294-if-let-guard/warns.rs stdout ----
diff of stderr:

9    |
10 LL | #[deny(irrefutable_let_patterns)]
11    |        ^^^^^^^^^^^^^^^^^^^^^^^^
+    = help: for more information, see https://doc.rust-lang.org/nightly/rustc/lints/listing/warn-by-default.html#irrefutable-let-patterns
13 error: unreachable pattern
14   --> $DIR/warns.rs:16:25



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2294-if-let-guard/warns/warns.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2294-if-let-guard/warns.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2294-if-let-guard/warns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2294-if-let-guard/warns" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2294-if-let-guard/warns/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: irrefutable if-let guard
   |
   |
LL |         Some(x) if let () = x => {}
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/rfc-2294-if-let-guard/warns.rs:4:8
   |
   |
LL | #[deny(irrefutable_let_patterns)]
   |        ^^^^^^^^^^^^^^^^^^^^^^^^
   = help: for more information, see https://doc.rust-lang.org/nightly/rustc/lints/listing/warn-by-default.html#irrefutable-let-patterns
error: unreachable pattern
  --> /checkout/src/test/ui/rfc-2294-if-let-guard/warns.rs:16:25
   |
   |
LL |         x if let None | None = x => {}
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/rfc-2294-if-let-guard/warns.rs:13:8
   |
   |
LL | #[deny(unreachable_patterns)]

error: aborting due to 2 previous errors


---
diff of stderr:

10    | |_______- in this macro invocation
11    |
12    = note: `#[warn(irrefutable_let_patterns)]` on by default
+    = help: for more information, see https://doc.rust-lang.org/nightly/rustc/lints/listing/warn-by-default.html#irrefutable-let-patterns
14 
15 warning: irrefutable while-let pattern

23 LL | |     });
23 LL | |     });
24    | |_______- in this macro invocation
25    |
+    = help: for more information, see https://doc.rust-lang.org/nightly/rustc/lints/listing/warn-by-default.html#irrefutable-let-patterns
27 
28 warning: irrefutable while-let pattern

33 LL | |         break;
33 LL | |         break;
34 LL | |     }
35    | |_____^
+    |
+    = help: for more information, see https://doc.rust-lang.org/nightly/rustc/lints/listing/warn-by-default.html#irrefutable-let-patterns
37 warning: 3 warnings emitted
38 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/while-let/while-let.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args while-let.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/while-let.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/while-let/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/while-let/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: irrefutable while-let pattern
  --> /checkout/src/test/ui/while-let.rs:7:13
   |
LL |               while let $p = $e $b
...
...
LL | /     foo!(_a, 1, {
LL | |         println!("irrefutable pattern");
LL | |     });
   | |_______- in this macro invocation
   = note: `#[warn(irrefutable_let_patterns)]` on by default
   = note: `#[warn(irrefutable_let_patterns)]` on by default
   = help: for more information, see https://doc.rust-lang.org/nightly/rustc/lints/listing/warn-by-default.html#irrefutable-let-patterns

warning: irrefutable while-let pattern
  --> /checkout/src/test/ui/while-let.rs:7:13
   |
   |
LL |               while let $p = $e $b
...
...
LL | /     bar!(_a, 1, {
LL | |         println!("irrefutable pattern");
LL | |     });
   | |_______- in this macro invocation
   |
   = help: for more information, see https://doc.rust-lang.org/nightly/rustc/lints/listing/warn-by-default.html#irrefutable-let-patterns

warning: irrefutable while-let pattern
  --> /checkout/src/test/ui/while-let.rs:27:5
   |
   |
LL | /     while let _a = 1 { //~ WARN irrefutable while-let
LL | |         println!("irrefutable pattern");
LL | |         break;
LL | |     }
   |
   |
   = help: for more information, see https://doc.rust-lang.org/nightly/rustc/lints/listing/warn-by-default.html#irrefutable-let-patterns
warning: 3 warnings emitted


------------------------------------------
---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:17:34
