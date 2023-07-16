plain
Suite("src/test/rustdoc-ui") not skipped for "bootstrap::test::RustdocUi" -- not in ["src/tools/tidy"]
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 120 tests
..........................................F...F......................................F.............. 100/120
Some tests failed in compiletest suite=rustdoc-ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [ui] rustdoc-ui/doc-spotlight.rs stdout ----
diff of stderr:
diff of stderr:

- warning: unknown `doc` attribute `spotlight`
-   --> $DIR/doc-spotlight.rs:6:7
-    |
- LL | #[doc(spotlight)]
-    |       ^^^^^^^^^ help: use `notable_trait` instead
-    |
-    = note: `#[warn(invalid_doc_attributes)]` on by default
-    = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
-    = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
-    = note: `doc(spotlight)` was renamed to `doc(notable_trait)`
-    = note: `doc(spotlight)` is now a no-op
- warning: 1 warning emitted
- 
- 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-spotlight/doc-spotlight.stderr
diff of fixed:

3 
4 #![feature(doc_notable_trait)]
5 
- #[doc(notable_trait)]
+ #[doc(spotlight)]
7 //~^ WARN unknown `doc` attribute `spotlight`
8 //~| WARN this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
9 trait MyTrait {}

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-spotlight/doc-spotlight.fixed
To only update this specific test, also pass `--test-args doc-spotlight.rs`

error: 2 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/doc-spotlight.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-spotlight" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-spotlight/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] rustdoc-ui/doc-attr.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/doc-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-attr" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-attr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

- error: this attribute can only be applied at the crate level
-   --> $DIR/invalid-doc-attr.rs:4:7
-    |
- LL | #[doc(test(no_crate_inject))]
-    |
- note: the lint level is defined here
-   --> $DIR/invalid-doc-attr.rs:2:9
-    |
-    |
- LL | #![deny(warnings)]
-    |         ^^^^^^^^
-    = note: `#[deny(invalid_doc_attributes)]` implied by `#[deny(warnings)]`
-    = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
-    = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
-    = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level for more information
- help: to apply to the crate, use an inner attribute
-    |
- LL | #![doc(test(no_crate_inject))]
- 
- error: this attribute can only be applied to a `use` item
-   --> $DIR/invalid-doc-attr.rs:9:7
-    |
-    |
- LL | #[doc(inline)]
-    |       ^^^^^^ only applicable on `use` items
- LL | pub fn foo() {}
-    | ------------ not a `use` item
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
-    = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#docno_inlinedocinline for more information
- error: this attribute can only be applied at the crate level
-   --> $DIR/invalid-doc-attr.rs:15:12
-    |
-    |
- LL |     #![doc(test(no_crate_inject))]
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
-    = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
-    = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level for more information
44 error: conflicting doc inlining attributes
45   --> $DIR/invalid-doc-attr.rs:28:7
46    |


51    |
52    = help: remove one of the conflicting attributes
53 
- error: this attribute can only be applied at the crate level
-   --> $DIR/invalid-doc-attr.rs:19:11
-    |
- LL |     #[doc(test(no_crate_inject))]
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
-    = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
-    = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level for more information
- error: this attribute can only be applied to a `use` item
-   --> $DIR/invalid-doc-attr.rs:22:11
-    |
-    |
- LL |     #[doc(inline)]
-    |           ^^^^^^ only applicable on `use` items
- ...
- LL |     pub fn baz() {}
-    |     ------------ not a `use` item
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
-    = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
-    = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#docno_inlinedocinline for more information
- error: aborting due to 6 previous errors
+ error: aborting due to previous error
78 
79 
---
To only update this specific test, also pass `--test-args invalid-doc-attr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/invalid-doc-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-doc-attr" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-doc-attr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: conflicting doc inlining attributes
  --> /checkout/src/test/rustdoc-ui/invalid-doc-attr.rs:28:7
   |
LL | #[doc(inline)]
   |       ^^^^^^ this attribute...
LL | #[doc(no_inline)]
   |       ^^^^^^^^^ ...conflicts with this attribute
   = help: remove one of the conflicting attributes

error: aborting due to previous error

---
test result: FAILED. 117 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.55s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc-ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:29:26
