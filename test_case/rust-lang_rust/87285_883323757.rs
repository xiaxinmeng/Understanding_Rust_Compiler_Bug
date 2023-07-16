plain
Successfully built 2ded9e871c6b
Successfully tagged rust-ci:latest
Built container sha256:2ded9e871c6b5191e508c9ce54e31e17d4dbf1f6a0ccb4fc0e426b221a43777e
Uploading finished image to https://ci-caches.rust-lang.org/docker/b8522bdbde0170e6b8638c2ee7936d2221f0e051b4634ed61ad299a1775319b857fabeb7dc5d6fa62739b362a15aa7c74c4f64f6f12f1718e9d3e82ccdcf01ba
upload failed: - to s3://rust-lang-ci-sccache2/docker/b8522bdbde0170e6b8638c2ee7936d2221f0e051b4634ed61ad299a1775319b857fabeb7dc5d6fa62739b362a15aa7c74c4f64f6f12f1718e9d3e82ccdcf01ba Unable to locate credentials
 * branch              master     -> FETCH_HEAD
[CI_JOB_NAME=x86_64-gnu-llvm-10]
[CI_JOB_NAME=x86_64-gnu-llvm-10]
---
Suite("src/test/rustdoc-ui") not skipped for "bootstrap::test::RustdocUi" -- not in ["src/tools/tidy"]
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 124 tests
.......................F..................................F.......F...............................F. 100/124
F.......................
Some tests failed in compiletest suite=rustdoc-ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [ui] rustdoc-ui/assoc-item-not-in-scope.rs stdout ----
diff of stderr:


1 error: unresolved link to `S::fmt`
-   --> $DIR/assoc-item-not-in-scope.rs:4:15
3    |
3    |
4 LL | /// Link to [`S::fmt`]
-    |               ^^^^^^ the struct `S` has no field or associated item named `fmt`
+    |              ^^^^^^^^ the struct `S` has no field or associated item named `fmt`
7 note: the lint level is defined here
8   --> $DIR/assoc-item-not-in-scope.rs:1:9



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/assoc-item-not-in-scope/assoc-item-not-in-scope.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args assoc-item-not-in-scope.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/assoc-item-not-in-scope.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/assoc-item-not-in-scope" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/assoc-item-not-in-scope/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unresolved link to `S::fmt`
   |
   |
LL | /// Link to [`S::fmt`]
   |              ^^^^^^^^ the struct `S` has no field or associated item named `fmt`
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/assoc-item-not-in-scope.rs:1:9
   |
   |
LL | #![deny(rustdoc::broken_intra_doc_links)]

error: aborting due to previous error


---
diff of stderr:

19    |      ^^^^^^^^^
20 
21 error: `ambiguous` is both a struct and a function
-   --> $DIR/ambiguity.rs:27:7
+   --> $DIR/ambiguity.rs:27:6
23    |
24 LL | /// [`ambiguous`] is ambiguous.
-    |       ^^^^^^^^^ ambiguous link
+    |      ^^^^^^^^^^^ ambiguous link
26    |
27 help: to link to the struct, prefix with `struct@`


- LL | /// [`struct@ambiguous`] is ambiguous.
-    |       ^^^^^^^^^^^^^^^^
+ LL | /// [struct@ambiguous] is ambiguous.
31 help: to link to the function, add parentheses
32    |
32    |
- LL | /// [`ambiguous()`] is ambiguous.
-    |       ^^^^^^^^^^^
+ LL | /// [ambiguous()] is ambiguous.
35 
35 
36 error: `ambiguous` is both a struct and a function
37   --> $DIR/ambiguity.rs:29:6
49    |      ^^^^^^^^^^^
50 
50 
51 error: `multi_conflict` is a struct, a function, and a macro
-   --> $DIR/ambiguity.rs:31:7
+   --> $DIR/ambiguity.rs:31:6
53    |
54 LL | /// [`multi_conflict`] is a three-way conflict.
-    |       ^^^^^^^^^^^^^^ ambiguous link
+    |      ^^^^^^^^^^^^^^^^ ambiguous link
56    |
57 help: to link to the struct, prefix with `struct@`


- LL | /// [`struct@multi_conflict`] is a three-way conflict.
-    |       ^^^^^^^^^^^^^^^^^^^^^
+ LL | /// [struct@multi_conflict] is a three-way conflict.
61 help: to link to the function, add parentheses
62    |
62    |
- LL | /// [`multi_conflict()`] is a three-way conflict.
-    |       ^^^^^^^^^^^^^^^^
+ LL | /// [multi_conflict()] is a three-way conflict.
+    |      ^^^^^^^^^^^^^^^^
65 help: to link to the macro, add an exclamation mark
66    |
- LL | /// [`multi_conflict!`] is a three-way conflict.
-    |       ^^^^^^^^^^^^^^^
+ LL | /// [multi_conflict!] is a three-way conflict.
69 
69 
70 error: `type_and_value` is both a module and a constant
71   --> $DIR/ambiguity.rs:33:16
83    |                ^^^^^^^^^^^^^^^^^^^^
84 
84 
85 error: `foo::bar` is both an enum and a function
-   --> $DIR/ambiguity.rs:35:43
+   --> $DIR/ambiguity.rs:35:42
87    |
88 LL | /// Ambiguous non-implied shortcut link [`foo::bar`].
-    |                                           ^^^^^^^^ ambiguous link
+    |                                          ^^^^^^^^^^ ambiguous link
90    |
91 help: to link to the enum, prefix with `enum@`


- LL | /// Ambiguous non-implied shortcut link [`enum@foo::bar`].
-    |                                           ^^^^^^^^^^^^^
+ LL | /// Ambiguous non-implied shortcut link [enum@foo::bar].
95 help: to link to the function, add parentheses
96    |
96    |
- LL | /// Ambiguous non-implied shortcut link [`foo::bar()`].
-    |                                           ^^^^^^^^^^
+ LL | /// Ambiguous non-implied shortcut link [foo::bar()].
99 
100 error: aborting due to 6 previous errors
101 

---
To only update this specific test, also pass `--test-args intra-doc/ambiguity.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/ambiguity.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/ambiguity" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/ambiguity/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `true` is both a module and a builtin type
   |
   |
LL | /// [true] //~ ERROR `true` is both a module and a builtin type
   |      ^^^^ ambiguous link
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/intra-doc/ambiguity.rs:1:9
   |
   |
LL | #![deny(rustdoc::broken_intra_doc_links)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: to link to the module, prefix with `mod@`
   |
LL | /// [mod@true] //~ ERROR `true` is both a module and a builtin type
   |      ^^^^^^^^
help: to link to the builtin type, prefix with `prim@`
   |
LL | /// [prim@true] //~ ERROR `true` is both a module and a builtin type


error: `ambiguous` is both a struct and a function
   |
   |
LL | /// [`ambiguous`] is ambiguous. //~ERROR `ambiguous`
   |      ^^^^^^^^^^^ ambiguous link
   |
help: to link to the struct, prefix with `struct@`
   |
LL | /// [struct@ambiguous] is ambiguous. //~ERROR `ambiguous`
help: to link to the function, add parentheses
   |
   |
LL | /// [ambiguous()] is ambiguous. //~ERROR `ambiguous`


error: `ambiguous` is both a struct and a function
   |
   |
LL | /// [ambiguous] is ambiguous. //~ERROR ambiguous
   |      ^^^^^^^^^ ambiguous link
   |
help: to link to the struct, prefix with `struct@`
   |
LL | /// [struct@ambiguous] is ambiguous. //~ERROR ambiguous
help: to link to the function, add parentheses
   |
   |
LL | /// [ambiguous()] is ambiguous. //~ERROR ambiguous


error: `multi_conflict` is a struct, a function, and a macro
   |
   |
LL | /// [`multi_conflict`] is a three-way conflict. //~ERROR `multi_conflict`
   |      ^^^^^^^^^^^^^^^^ ambiguous link
   |
help: to link to the struct, prefix with `struct@`
   |
LL | /// [struct@multi_conflict] is a three-way conflict. //~ERROR `multi_conflict`
help: to link to the function, add parentheses
   |
   |
LL | /// [multi_conflict()] is a three-way conflict. //~ERROR `multi_conflict`
   |      ^^^^^^^^^^^^^^^^
help: to link to the macro, add an exclamation mark
   |
LL | /// [multi_conflict!] is a three-way conflict. //~ERROR `multi_conflict`


error: `type_and_value` is both a module and a constant
   |
   |
LL | /// Ambiguous [type_and_value]. //~ERROR type_and_value
   |                ^^^^^^^^^^^^^^ ambiguous link
   |
help: to link to the module, prefix with `mod@`
   |
LL | /// Ambiguous [mod@type_and_value]. //~ERROR type_and_value
   |                ^^^^^^^^^^^^^^^^^^
help: to link to the constant, prefix with `const@`
   |
LL | /// Ambiguous [const@type_and_value]. //~ERROR type_and_value


error: `foo::bar` is both an enum and a function
   |
   |
LL | /// Ambiguous non-implied shortcut link [`foo::bar`]. //~ERROR `foo::bar`
   |                                          ^^^^^^^^^^ ambiguous link
   |
help: to link to the enum, prefix with `enum@`
   |
LL | /// Ambiguous non-implied shortcut link [enum@foo::bar]. //~ERROR `foo::bar`
help: to link to the function, add parentheses
   |
   |
LL | /// Ambiguous non-implied shortcut link [foo::bar()]. //~ERROR `foo::bar`

error: aborting due to 6 previous errors



------------------------------------------


---- [ui] rustdoc-ui/intra-doc/field-ice.rs stdout ----
diff of stderr:

1 error: incompatible link kind for `Foo::bar`
-   --> $DIR/field-ice.rs:5:7
+   --> $DIR/field-ice.rs:5:6
3    |
4 LL | /// [`Foo::bar()`]
-    |       ^^^^^^^^^^ help: to link to the field, remove the disambiguator: `Foo::bar`
+    |      ^^^^^^^^^^^^ help: to link to the field, remove the disambiguator: `Foo::bar`
7 note: the lint level is defined here
8   --> $DIR/field-ice.rs:1:9



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/field-ice/field-ice.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args intra-doc/field-ice.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/field-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/field-ice" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/field-ice/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: incompatible link kind for `Foo::bar`
   |
   |
LL | /// [`Foo::bar()`]
   |      ^^^^^^^^^^^^ help: to link to the field, remove the disambiguator: `Foo::bar`
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/intra-doc/field-ice.rs:1:9
   |
   |
LL | #![deny(rustdoc::broken_intra_doc_links)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: this link resolved to a field, which is not a function
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] rustdoc-ui/issue-74134.rs#private stdout ----
diff of stderr:

1 warning: public documentation for `public_item` links to private item `PrivateType`
-   --> $DIR/issue-74134.rs:19:11
3    |
3    |
4 LL |     /// [`PrivateType`]
+    |          ^^^^^^^^^^^^^ this item is private
6    |
7    = note: `#[warn(rustdoc::private_intra_doc_links)]` on by default
7    = note: `#[warn(rustdoc::private_intra_doc_links)]` on by default
8    = note: this link resolves only because you passed `--document-private-items`, but will break without

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-74134.private/issue-74134.private.stderr
To only update this specific test, also pass `--test-args issue-74134.rs`


error in revision `private`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/issue-74134.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "private" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-74134.private" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--document-private-items" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-74134.private/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: public documentation for `public_item` links to private item `PrivateType`
   |
   |
LL |     /// [`PrivateType`]
   |
   = note: `#[warn(rustdoc::private_intra_doc_links)]` on by default
   = note: `#[warn(rustdoc::private_intra_doc_links)]` on by default
   = note: this link resolves only because you passed `--document-private-items`, but will break without
warning: 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] rustdoc-ui/issue-74134.rs#public stdout ----
diff of stderr:

1 warning: public documentation for `public_item` links to private item `PrivateType`
-   --> $DIR/issue-74134.rs:19:11
3    |
3    |
4 LL |     /// [`PrivateType`]
+    |          ^^^^^^^^^^^^^ this item is private
6    |
7    = note: `#[warn(rustdoc::private_intra_doc_links)]` on by default
7    = note: `#[warn(rustdoc::private_intra_doc_links)]` on by default
8    = note: this link will resolve properly if you pass `--document-private-items`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-74134.public/issue-74134.public.stderr
To only update this specific test, also pass `--test-args issue-74134.rs`


error in revision `public`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/issue-74134.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "public" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-74134.public" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-74134.public/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: public documentation for `public_item` links to private item `PrivateType`
   |
   |
LL |     /// [`PrivateType`]
   |
   = note: `#[warn(rustdoc::private_intra_doc_links)]` on by default
   = note: `#[warn(rustdoc::private_intra_doc_links)]` on by default
   = note: this link will resolve properly if you pass `--document-private-items`
warning: 1 warning emitted


------------------------------------------
---
test result: FAILED. 119 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.84s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc-ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:24:07
