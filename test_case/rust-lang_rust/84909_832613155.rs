plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out; finished in 0.12s

 finished in 0.181 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.32s

 finished in 2.377 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Suite("src/test/rustdoc-ui") not skipped for "bootstrap::test::RustdocUi" -- not in ["src/tools/tidy"]
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
...............................................................F...................F................ 100/116
Some tests failed in compiletest suite=rustdoc-ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [ui] rustdoc-ui/intra-doc/email-address-localhost.rs stdout ----
diff of stderr:
diff of stderr:

10 LL | #![deny(warnings)]
11    |         ^^^^^^^^
12    = note: `#[deny(rustdoc::broken_intra_doc_links)]` implied by `#[deny(warnings)]`
-    = note: see https://doc.rust-lang.org/nightly/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators for more info about disambiguators
+    = note: see https://doc.rust-lang.org/beta/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators for more info about disambiguators
15 error: aborting due to previous error
16 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/email-address-localhost/email-address-localhost.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args intra-doc/email-address-localhost.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/email-address-localhost.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/email-address-localhost" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/email-address-localhost/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown disambiguator `hello`
   |
   |
LL | //! Email me at <hello@localhost>.
   |
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/intra-doc/email-address-localhost.rs:1:9
   |
   |
LL | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(rustdoc::broken_intra_doc_links)]` implied by `#[deny(warnings)]`
   = note: see https://doc.rust-lang.org/beta/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators for more info about disambiguators
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] rustdoc-ui/intra-doc/unknown-disambiguator.rs stdout ----
diff of stderr:

10 LL | #![deny(warnings)]
11    |         ^^^^^^^^
12    = note: `#[deny(rustdoc::broken_intra_doc_links)]` implied by `#[deny(warnings)]`
-    = note: see https://doc.rust-lang.org/nightly/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators for more info about disambiguators
+    = note: see https://doc.rust-lang.org/beta/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators for more info about disambiguators
15 error: unknown disambiguator `bar`
16   --> $DIR/unknown-disambiguator.rs:3:35


18 LL | //! Linking to [foo@banana] and [`bar@banana!()`].
20    |
20    |
-    = note: see https://doc.rust-lang.org/nightly/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators for more info about disambiguators
+    = note: see https://doc.rust-lang.org/beta/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators for more info about disambiguators
23 error: unknown disambiguator `foo`
24   --> $DIR/unknown-disambiguator.rs:9:34


26 LL | //! And with weird backticks: [``foo@hello``] [foo`@`hello].
28    |
28    |
-    = note: see https://doc.rust-lang.org/nightly/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators for more info about disambiguators
+    = note: see https://doc.rust-lang.org/beta/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators for more info about disambiguators
31 error: unknown disambiguator `foo`
32   --> $DIR/unknown-disambiguator.rs:9:48


34 LL | //! And with weird backticks: [``foo@hello``] [foo`@`hello].
36    |
36    |
-    = note: see https://doc.rust-lang.org/nightly/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators for more info about disambiguators
+    = note: see https://doc.rust-lang.org/beta/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators for more info about disambiguators
39 error: unknown disambiguator ``
40   --> $DIR/unknown-disambiguator.rs:6:31


42 LL | //! And to [no disambiguator](@nectarine) and [another](@apricot!()).
44    |
44    |
-    = note: see https://doc.rust-lang.org/nightly/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators for more info about disambiguators
+    = note: see https://doc.rust-lang.org/beta/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators for more info about disambiguators
47 error: unknown disambiguator ``
48   --> $DIR/unknown-disambiguator.rs:6:57


50 LL | //! And to [no disambiguator](@nectarine) and [another](@apricot!()).
52    |
52    |
-    = note: see https://doc.rust-lang.org/nightly/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators for more info about disambiguators
+    = note: see https://doc.rust-lang.org/beta/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators for more info about disambiguators
55 error: aborting due to 6 previous errors
56 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/unknown-disambiguator/unknown-disambiguator.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args intra-doc/unknown-disambiguator.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/unknown-disambiguator.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/unknown-disambiguator" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/unknown-disambiguator/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown disambiguator `foo`
  --> /checkout/src/test/rustdoc-ui/intra-doc/unknown-disambiguator.rs:3:17
   |
LL | //! Linking to [foo@banana] and [`bar@banana!()`].
   |
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/intra-doc/unknown-disambiguator.rs:1:9
   |
   |
LL | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(rustdoc::broken_intra_doc_links)]` implied by `#[deny(warnings)]`
   = note: see https://doc.rust-lang.org/beta/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators for more info about disambiguators
error: unknown disambiguator `bar`
  --> /checkout/src/test/rustdoc-ui/intra-doc/unknown-disambiguator.rs:3:35
   |
   |
LL | //! Linking to [foo@banana] and [`bar@banana!()`].
   |
   |
   = note: see https://doc.rust-lang.org/beta/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators for more info about disambiguators
error: unknown disambiguator `foo`
  --> /checkout/src/test/rustdoc-ui/intra-doc/unknown-disambiguator.rs:9:34
   |
   |
LL | //! And with weird backticks: [``foo@hello``] [foo`@`hello].
   |
   |
   = note: see https://doc.rust-lang.org/beta/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators for more info about disambiguators
error: unknown disambiguator `foo`
  --> /checkout/src/test/rustdoc-ui/intra-doc/unknown-disambiguator.rs:9:48
   |
   |
LL | //! And with weird backticks: [``foo@hello``] [foo`@`hello].
   |
   |
   = note: see https://doc.rust-lang.org/beta/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators for more info about disambiguators
error: unknown disambiguator ``
  --> /checkout/src/test/rustdoc-ui/intra-doc/unknown-disambiguator.rs:6:31
   |
   |
LL | //! And to [no disambiguator](@nectarine) and [another](@apricot!()).
   |
   |
   = note: see https://doc.rust-lang.org/beta/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators for more info about disambiguators
error: unknown disambiguator ``
  --> /checkout/src/test/rustdoc-ui/intra-doc/unknown-disambiguator.rs:6:57
   |
   |
LL | //! And to [no disambiguator](@nectarine) and [another](@apricot!()).
   |
   |
   = note: see https://doc.rust-lang.org/beta/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators for more info about disambiguators
error: aborting due to 6 previous errors


------------------------------------------
---
test result: FAILED. 114 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.50s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc-ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:30:09
