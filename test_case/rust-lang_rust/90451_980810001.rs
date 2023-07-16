plain
diff of stdout:

9   --> $DIR/display-output.rs:13:12
10    |
11 LL | fn foo(x: &std::fmt::Display) {}
+    |            ^^^^^^^^^^^^^^^^^
13    |
14    = note: `#[warn(bare_trait_objects)]` on by default
15    = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
15    = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!

16    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
+ help: use `dyn`
+    |
+ LL - fn foo(x: &std::fmt::Display) {}
+ LL + fn foo(x: &dyn std::fmt::Display) {}
17 
18 warning: unused variable: `x`
19   --> $DIR/display-output.rs:11:5


47    |         ^^^^^^
48    = note: `#[warn(dead_code)]` implied by `#[warn(unused)]`
49 
- warning: 4 warnings emitted
+ warning: trait objects without an explicit `dyn` are deprecated
+   --> $DIR/display-output.rs:13:12
+    |
+ LL | fn foo(x: &std::fmt::Display) {}
+    |
+    = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
+    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
+ help: use `dyn`
+ help: use `dyn`
+    |
+ LL - fn foo(x: &std::fmt::Display) {}
+ LL + fn foo(x: &dyn std::fmt::Display) {}
+ 
+ warning: trait objects without an explicit `dyn` are deprecated
+   --> $DIR/display-output.rs:13:12
+    |
+    |
+ LL | fn foo(x: &std::fmt::Display) {}
+    |
+    = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
+    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
+ help: use `dyn`
+ help: use `dyn`
+    |
+ LL - fn foo(x: &std::fmt::Display) {}
+ LL + fn foo(x: &dyn std::fmt::Display) {}
+ 
+ warning: 6 warnings emitted
51 
52 
---
To only update this specific test, also pass `--test-args display-output.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/display-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/display-output" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--test" "--test-args=--show-output" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/display-output/auxiliary"
------------------------------------------

running 1 test
test /checkout/src/test/rustdoc-ui/display-output.rs - foo (line 9) ... ok
test /checkout/src/test/rustdoc-ui/display-output.rs - foo (line 9) ... ok

successes:

---- /checkout/src/test/rustdoc-ui/display-output.rs - foo (line 9) stdout ----
warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/rustdoc-ui/display-output.rs:13:12
   |
LL | fn foo(x: &std::fmt::Display) {}
   |
   = note: `#[warn(bare_trait_objects)]` on by default
   = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
   |
LL - fn foo(x: &std::fmt::Display) {}
LL + fn foo(x: &dyn std::fmt::Display) {}

warning: unused variable: `x`
  --> /checkout/src/test/rustdoc-ui/display-output.rs:11:5
   |
---

warning: unused variable: `x`
  --> /checkout/src/test/rustdoc-ui/display-output.rs:13:8
   |
LL | fn foo(x: &std::fmt::Display) {}
   |        ^ help: if this is intentional, prefix it with an underscore: `_x`
warning: function is never used: `foo`
  --> /checkout/src/test/rustdoc-ui/display-output.rs:13:4
   |
   |
LL | fn foo(x: &std::fmt::Display) {}
   |
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/display-output.rs:9:9
   |
   |
LL | #![warn(unused)]
   |         ^^^^^^
   = note: `#[warn(dead_code)]` implied by `#[warn(unused)]`

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/rustdoc-ui/display-output.rs:13:12
   |
LL | fn foo(x: &std::fmt::Display) {}
   |
   = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL - fn foo(x: &std::fmt::Display) {}
LL + fn foo(x: &dyn std::fmt::Display) {}

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/rustdoc-ui/display-output.rs:13:12
   |
   |
LL | fn foo(x: &std::fmt::Display) {}
   |
   = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL - fn foo(x: &std::fmt::Display) {}
LL + fn foo(x: &dyn std::fmt::Display) {}

warning: 6 warnings emitted


---
test result: FAILED. 144 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 9.07s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc-ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:35:29
