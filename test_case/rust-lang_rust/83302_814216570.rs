plain
.................................................................................................... 5500/11736
.................................................................................................... 5600/11736
.................................................................................................... 5700/11736
.................................................................................................... 5800/11736
.......................F...F........................i............................................... 5900/11736
.........................................................i.......................................... 6100/11736
.................................................................................................... 6200/11736
...........................................................................................ii.ii.... 6300/11736
...i...i............................................................................................ 6400/11736
---

17 
18 error: unexpected token: `{
19     let res =
-         ::alloc::fmt::format(::core::fmt::Arguments::new_v1(&[""],
-                                                             &match (&"u8",) {
-                                                                  (arg0,) =>
-                                                                  [::core::fmt::ArgumentV1::new(arg0,
-                                                                                                ::core::fmt::Display::fmt)],
-                                                              }));
+         ::alloc::fmt::format(unsafe {
+                                  ::core::fmt::Arguments::new_v1(&[""],
+                                                                 &match (&"u8",)
+                                                                      (arg0,)
+                                                                      =>
+                                                                      =>
+                                                                      [::core::fmt::ArgumentV1::new(arg0,
+                                                                                                    ::core::fmt::Display::fmt)],
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+                              });
26     res
27 }.as_str()`
27 }.as_str()`
28   --> $DIR/key-value-expansion.rs:48:23


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/key-value-expansion/key-value-expansion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args attributes/key-value-expansion.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/attributes/key-value-expansion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/key-value-expansion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/key-value-expansion/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unexpected token: `(7u32)`
  --> /checkout/src/test/ui/attributes/key-value-expansion.rs:21:6
   |
LL | bug!((column!())); //~ ERROR unexpected token: `(7u32)`


error: unexpected token: `"bug" + "found"`
   |
   |
LL |         bug!("bug" + stringify!(found)); //~ ERROR unexpected token: `"bug" + "found"`
...
...
LL | bug!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: unexpected token: `{
error: unexpected token: `{
    let res =
        ::alloc::fmt::format(unsafe {
                                 ::core::fmt::Arguments::new_v1(&[""],
                                                                &match (&"u8",)
                                                                     (arg0,)
                                                                     =>
                                                                     =>
                                                                     [::core::fmt::ArgumentV1::new(arg0,
                                                                                                   ::core::fmt::Display::fmt)],
                             });
    res
}.as_str()`
  --> /checkout/src/test/ui/attributes/key-value-expansion.rs:48:23
  --> /checkout/src/test/ui/attributes/key-value-expansion.rs:48:23
   |
LL |         doc_comment! {format!("{coor}", coor = stringify!($t1)).as_str()}
...
...
LL | some_macro!(u8);
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 3 previous errors
error: aborting due to 3 previous errors


------------------------------------------


---- [ui] ui/issues/issue-46845.rs stdout ----
normalized stderr:
warning: unnecessary `unsafe` block
   |
   |
LL |     println!("{}", unsafe { f[0].a });
   |                    ^^^^^^ unnecessary `unsafe` block
   = note: `#[warn(unused_unsafe)]` on by default

warning: 1 warning emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46845/issue-46845.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-46845.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-46845.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46845/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "mir-opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46845/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unnecessary `unsafe` block
   |
   |
LL |     println!("{}", unsafe { f[0].a });
   |                    ^^^^^^ unnecessary `unsafe` block
   = note: `#[warn(unused_unsafe)]` on by default

warning: 1 warning emitted



------------------------------------------


---- [ui] ui/issues/issue-46855.rs stdout ----
normalized stderr:
warning: unnecessary `unsafe` block
   |
   |
LL |     println!("{:?}", unsafe { f[0].a });
   |                      ^^^^^^ unnecessary `unsafe` block
   = note: `#[warn(unused_unsafe)]` on by default

warning: 1 warning emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46855/issue-46855.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-46855.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-46855.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46855/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zmir-opt-level=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46855/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unnecessary `unsafe` block
   |
   |
LL |     println!("{:?}", unsafe { f[0].a });
   |                      ^^^^^^ unnecessary `unsafe` block
   = note: `#[warn(unused_unsafe)]` on by default

warning: 1 warning emitted

---
---- [ui] ui/unsafe/unsafe-around-compiler-generated-unsafe.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/unsafe-around-compiler-generated-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-around-compiler-generated-unsafe" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-around-compiler-generated-unsafe/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
test result: FAILED. 11636 passed; 4 failed; 96 ignored; 0 measured; 0 filtered out; finished in 139.87s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:41
