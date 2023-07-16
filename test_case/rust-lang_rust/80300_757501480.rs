plain
.................................................................................................... 9000/11246
.................................................................................................... 9100/11246
.................................................................................................... 9200/11246
..........................................i.......i................................................. 9300/11246
.................................................................................iiiiii..iiiiii.i... 9400/11246
.................................................................................................... 9600/11246
.................................................................................................... 9700/11246
.................................................................................................... 9800/11246
.................................................................................................... 9900/11246
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.068 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 10.844 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i...ii...i..i...ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.32s

 finished in 2.405 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
diff of stderr:

2   --> $DIR/check-doc-attrs.rs:3:8
3    |
4 LL | #![doc(inline)]
-    |        ^^^^^^
+    |        ^^^^^^ only applicable on `use` items
7 note: the lint level is defined here
8   --> $DIR/check-doc-attrs.rs:1:9

25   --> $DIR/check-doc-attrs.rs:16:7
25   --> $DIR/check-doc-attrs.rs:16:7
26    |
27 LL | #[doc(no_inline)]
-    |       ^^^^^^^^^
+    |       ^^^^^^^^^ only applicable on `use` items
30 LL | struct Foo;
31    | ----------- not a `use` item

48   --> $DIR/check-doc-attrs.rs:24:7
48   --> $DIR/check-doc-attrs.rs:24:7
49    |
50 LL | #[doc(no_inline)]
-    |       ^^^^^^^^^
+    |       ^^^^^^^^^ only applicable on `use` items
53 LL | fn main() {}
54    | ------------ not a `use` item



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
error: this attribute can only be applied to a `use` item
   |
   |
LL | #![doc(inline)]
   |        ^^^^^^ only applicable on `use` items
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/check-doc-attrs.rs:1:9
   |
   |
LL | #![deny(invalid_doc_attributes)]
   |         ^^^^^^^^^^^^^^^^^^^^^^
   = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#docno_inlinedocinline for more information
error: conflicting doc inlining attributes
  --> /checkout/src/test/rustdoc-ui/check-doc-attrs.rs:6:7
   |
   |
LL | #[doc(inline)]
   |       ^^^^^^ this attribute...
LL | #[doc(no_inline)]
   |       ^^^^^^^^^ ...conflicts with this attribute
   = help: remove one of the conflicting attributes


error: this attribute can only be applied to a `use` item
   |
   |
LL | #[doc(no_inline)]
   |       ^^^^^^^^^ only applicable on `use` items
LL | struct Foo;
   | ----------- not a `use` item
   |
   |
   = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#docno_inlinedocinline for more information
error: this attribute can only be applied at the crate level
  --> /checkout/src/test/rustdoc-ui/check-doc-attrs.rs:18:7
   |
   |
LL | #[doc(html_no_source)]
   |
   |
   = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level for more information
help: to apply to the crate, use an inner attribute
   |
LL | #![doc(html_no_source)]


error: this attribute can only be applied to a `use` item
   |
   |
LL | #[doc(no_inline)]
   |       ^^^^^^^^^ only applicable on `use` items
LL | fn main() {}
   | ------------ not a `use` item
   |
   |
   = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#docno_inlinedocinline for more information
error: this attribute can only be applied at the crate level
  --> /checkout/src/test/rustdoc-ui/check-doc-attrs.rs:26:7
   |
   |
LL | #[doc(html_no_source)]
   |
   |
   = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level for more information
help: to apply to the crate, use an inner attribute
   |
LL | #![doc(html_no_source)]

error: aborting due to 6 previous errors


---
test result: FAILED. 95 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.86s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc-ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:37:29
