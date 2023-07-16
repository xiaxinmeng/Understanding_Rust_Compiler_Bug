plain

---- [ui] ui/async-await/suggest-missing-await.rs stdout ----
diff of stderr:

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
53 LL | |     };
54    | |_____- `if` and `else` have incompatible types
55    |
-    = note:   expected type `impl Future`
+    = note:   expected type `impl Future<Output = ()>`
57            found unit type `()`
58 help: consider `await`ing on the `Future`

66 LL |       let _x = match 0usize {
67    |  ______________-
67    |  ______________-
68 LL | |         0 => dummy(),
-    | |              ------- this is found to be of type `impl Future`
+    | |              ------- this is found to be of type `impl Future<Output = ()>`
70 LL | |         1 => dummy(),
-    | |              ------- this is found to be of type `impl Future`
+    | |              ------- this is found to be of type `impl Future<Output = ()>`
72 LL | |         2 => dummy().await,
74 LL | |

80    |
80    |
81 LL | async fn dummy() {}
82    |                  ^ checked the `Output` of this `async fn`, expected opaque type
-    = note: expected opaque type `impl Future`
+    = note: expected opaque type `impl Future<Output = ()>`
84                 found unit type `()`
85 help: consider `await`ing on the `Future`

99    |
99    |
100 LL | async fn dummy() {}
101    |                  ^ checked the `Output` of this `async fn`, expected opaque type
-    = note: expected opaque type `impl Future`
+    = note: expected opaque type `impl Future<Output = ()>`
103                 found unit type `()`
104 help: consider `await`ing on the `Future`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await/suggest-missing-await.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await/suggest-missing-await.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/suggest-missing-await.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/suggest-missing-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/suggest-missing-await.rs:12:14
   |
LL |     take_u32(x)
   |              ^ expected `u32`, found opaque type
   |
note: while checking the return type of the `async fn`
   |
   |
LL | async fn make_u32() -> u32 {
   |                        ^^^ checked the `Output` of this `async fn`, found opaque type
   = note:     expected type `u32`
           found opaque type `impl Future<Output = u32>`
help: consider `await`ing on the `Future`
   |
LL |     take_u32(x.await)

error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/suggest-missing-await.rs:22:5
   |
   |
LL |     dummy()
   |     ^^^^^^^ expected `()`, found opaque type
   |
note: while checking the return type of the `async fn`
   |
   |
LL | async fn dummy() {}
   |                  ^ checked the `Output` of this `async fn`, found opaque type
   = note: expected unit type `()`
            found opaque type `impl Future<Output = ()>`
help: consider `await`ing on the `Future`
   |
LL |     dummy().await
help: consider using a semicolon here
   |
LL |     dummy();
   |            +
   |            +

error[E0308]: `if` and `else` have incompatible types
   |
LL |       let _x = if true {
   |  ______________-
LL | |         dummy()
LL | |         dummy()
   | |         ------- expected because of this
LL | |         //~^ HELP consider `await`ing on the `Future`
LL | |     } else {
LL | |         dummy().await
   | |         ^^^^^^^^^^^^^ expected opaque type, found `()`
LL | |         //~^ ERROR `if` and `else` have incompatible types [E0308]
LL | |     };
   | |_____- `if` and `else` have incompatible types
   |
   = note:   expected type `impl Future<Output = ()>`
           found unit type `()`
help: consider `await`ing on the `Future`
   |
LL |         dummy().await


error[E0308]: `match` arms have incompatible types
   |
LL |       let _x = match 0usize {
   |  ______________-
   |  ______________-
LL | |         0 => dummy(), //~ HELP consider `await`ing on the `Future`
   | |              ------- this is found to be of type `impl Future<Output = ()>`
LL | |         1 => dummy(),
   | |              ------- this is found to be of type `impl Future<Output = ()>`
LL | |         2 => dummy().await,
   | |              ^^^^^^^^^^^^^ expected opaque type, found `()`
LL | |         //~^ `match` arms have incompatible types [E0308]
LL | |     };
   | |_____- `match` arms have incompatible types
   |
note: while checking the return type of the `async fn`
   |
   |
LL | async fn dummy() {}
   |                  ^ checked the `Output` of this `async fn`, expected opaque type
   = note: expected opaque type `impl Future<Output = ()>`
                found unit type `()`
help: consider `await`ing on the `Future`
   |
LL ~         0 => dummy().await, //~ HELP consider `await`ing on the `Future`
LL ~         1 => dummy().await,

error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/suggest-missing-await.rs:53:9
   |
   |
LL |         () => {} //~ ERROR mismatched types [E0308]
   |
   |
note: while checking the return type of the `async fn`
   |
   |
LL | async fn dummy() {}
   |                  ^ checked the `Output` of this `async fn`, expected opaque type
   = note: expected opaque type `impl Future<Output = ()>`
                found unit type `()`
help: consider `await`ing on the `Future`
   |
LL |     let _x = match dummy().await { //~ HELP consider `await`ing on the `Future`

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0308`.
---
test result: FAILED. 12277 passed; 1 failed; 111 ignored; 0 measured; 0 filtered out; finished in 138.98s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:26
