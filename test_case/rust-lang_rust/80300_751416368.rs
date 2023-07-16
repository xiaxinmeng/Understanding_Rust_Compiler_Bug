plain
.................................................................................................... 9000/11200
.................................................................................................... 9100/11200
..........................................................................................i......i.. 9200/11200
.................................................................................................... 9300/11200
.............................iiiiii..iiiiii.i....................................................... 9400/11200
.................................................................................................... 9600/11200
.................................................................................................... 9700/11200
.....................................................................................test [ui] ui/issues/issue-74564-if-expr-stack-overflow.rs has been running for over 60 seconds
............... 9800/11200
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.062 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i...i.i...ii...i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.36s

 finished in 2.428 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
 Documenting std v0.0.0 (/checkout/library/std)
warning: this attribute can only be applied at the crate level
  --> library/std/src/../../backtrace/src/lib.rs:44:8
   |
44 | #![doc(html_root_url = "https://docs.rs/backtrace")]
   |
   |
   = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level for more information
help: to apply to the crate, use an inner attribute
   |
44 | #![doc(h!tml_root_url = "https://docs.rs/backtrace")]

warning: 1 warning emitted

    Finished release [optimized] target(s) in 20.01s
---

---- [ui] rustdoc-ui/check-doc-attrs.rs stdout ----
diff of stderr:

4 LL | #![doc(inline)]
6    |
6    |
+ note: not a `use` item
+    |
+    |
+ LL | / #![doc(inline)]
+ LL | |
+ LL | |
+ LL | | #[doc(inline)]
+ ...  |
+ LL | |
+ LL | | fn main() {}
+    | |____________^
7    = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#docno_inlinedocinline for more information
9 error: conflicting doc inlining attributes


22 LL | #[doc(no_inline)]
24    |
24    |
+ note: not a `use` item
+    |
+    |
+ LL | struct Foo;
+    | ^^^^^^^^^^^
25    = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#docno_inlinedocinline for more information
27 warning: this attribute can only be applied at the crate level

31    |       ^^^^^^^^^^^^^^
32    |
32    |
33    = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level for more information
+ help: to apply to the crate, use an inner attribute
+    |
+ LL | #[doc(h!tml_no_source)]
34 
34 
35 warning: this attribute can only be applied to a `use` item


38 LL | #[doc(no_inline)]
40    |
40    |
+ note: not a `use` item
+    |
+ LL | fn main() {}
+    | ^^^^^^^^^^^^
+    | ^^^^^^^^^^^^
41    = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#docno_inlinedocinline for more information
43 warning: this attribute can only be applied at the crate level

47    |       ^^^^^^^^^^^^^^
48    |
48    |
49    = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level for more information
+ help: to apply to the crate, use an inner attribute
+    |
+ LL | #[doc(h!tml_no_source)]
50 
51 error: aborting due to previous error; 5 warnings emitted
52 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-doc-attrs/check-doc-attrs.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args check-doc-attrs.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/check-doc-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-doc-attrs" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-doc-attrs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: this attribute can only be applied to a `use` item
   |
   |
LL | #![doc(inline)]
   |
note: not a `use` item
  --> /checkout/src/test/rustdoc-ui/check-doc-attrs.rs:1:1
   |
   |
LL | / #![doc(inline)]
LL | | //~^ WARNING can only be applied to a `use` item
LL | |
LL | | #[doc(inline)]
...  |
LL | | //~^ WARNING can only be applied at the crate level
LL | | fn main() {}
   | |____________^
   = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#docno_inlinedocinline for more information
error: conflicting doc inlining attributes
  --> /checkout/src/test/rustdoc-ui/check-doc-attrs.rs:4:7
   |
   |
LL | #[doc(inline)]
   |       ^^^^^^ this attribute...
LL | #[doc(no_inline)]
   |       ^^^^^^^^^ ...conflicts with this attribute
   = help: remove one of the conflicting attributes


warning: this attribute can only be applied to a `use` item
   |
   |
LL | #[doc(no_inline)]
   |
note: not a `use` item
  --> /checkout/src/test/rustdoc-ui/check-doc-attrs.rs:17:1
   |
   |
LL | struct Foo;
   | ^^^^^^^^^^^
   = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#docno_inlinedocinline for more information
warning: this attribute can only be applied at the crate level
  --> /checkout/src/test/rustdoc-ui/check-doc-attrs.rs:15:7
   |
   |
LL | #[doc(html_no_source)]
   |
   |
   = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level for more information
help: to apply to the crate, use an inner attribute
   |
LL | #[doc(h!tml_no_source)]


warning: this attribute can only be applied to a `use` item
   |
   |
LL | #[doc(no_inline)]
   |
note: not a `use` item
  --> /checkout/src/test/rustdoc-ui/check-doc-attrs.rs:23:1
   |
   |
LL | fn main() {}
   | ^^^^^^^^^^^^
   = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#docno_inlinedocinline for more information
warning: this attribute can only be applied at the crate level
  --> /checkout/src/test/rustdoc-ui/check-doc-attrs.rs:21:7
   |
   |
LL | #[doc(html_no_source)]
   |
   |
   = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level for more information
help: to apply to the crate, use an inner attribute
   |
LL | #[doc(h!tml_no_source)]

error: aborting due to previous error; 5 warnings emitted


---
test result: FAILED. 84 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.01s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc-ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:34:45
