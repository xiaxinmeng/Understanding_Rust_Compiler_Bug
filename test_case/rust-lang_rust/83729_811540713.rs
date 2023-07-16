plain
......................ii...............i............................................................ 9000/11728
.................................................................................................... 9100/11728
.................................................................................................... 9200/11728
.................................................................................................... 9300/11728
.................................F............F...........................................F......... 9400/11728
.....................................................................i......i....................... 9600/11728
.................................................................................................... 9700/11728
.................................................................................................... 9700/11728
...............iiiiiii..iiiiii.i.................................................................... 9800/11728
.................................................................................................... 10000/11728
.................................................................................................... 10100/11728
.................................................................................................... 10200/11728
.................................................................................................... 10300/11728
---
diff of stderr:

5    |      ^^^ type aliases cannot be used as traits
6    |
7 help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
-   --> $DIR/two_files_data.rs:5:1
9    |
- LL | type Bar = dyn Foo;
-    | ^^^^^^^^^^^^^^^^^^^
+ LL | trait Bar = dyn Foo;
12 
13 error: aborting due to previous error
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
14 
14 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/two_files/two_files.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args codemap_tests/two_files.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/codemap_tests/two_files.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/two_files" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/two_files/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0404]: expected trait, found type alias `Bar`
  --> /checkout/src/test/ui/codemap_tests/two_files.rs:5:6
   |
LL | impl Bar for Baz { } //~ ERROR expected trait, found type alias
   |      ^^^ type aliases cannot be used as traits
   |
help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
   |
LL | trait Bar = dyn Foo;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0404`.
For more information about this error, try `rustc --explain E0404`.

------------------------------------------


---- [ui] ui/resolve/issue-5035.rs stdout ----
diff of stderr:

11    | ------- similarly named trait `I` defined here
12 LL | type K = dyn I;
13 LL | impl K for isize {}
-    |      |
-    |      type aliases cannot be used as traits
-    |      type aliases cannot be used as traits
-    |      help: a trait with a similar name exists: `I`
+    |      ^ type aliases cannot be used as traits
18    |
19 help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
-   --> $DIR/issue-5035.rs:2:1
21    |
- LL | type K = dyn I;
-    | ^^^^^^^^^^^^^^^
+ LL | trait K = dyn I;
+ help: a trait with a similar name exists
+    |
+    |
+ LL | impl I for isize {}
24 
25 error: aborting due to 2 previous errors
26 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-5035/issue-5035.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/issue-5035.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/issue-5035.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-5035" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-5035/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0432]: unresolved import `ImportError`
  --> /checkout/src/test/ui/resolve/issue-5035.rs:5:5
   |
LL | use ImportError; //~ ERROR unresolved import `ImportError` [E0432]
   |     ^^^^^^^^^^^ no `ImportError` in the root
error[E0404]: expected trait, found type alias `K`
  --> /checkout/src/test/ui/resolve/issue-5035.rs:3:6
   |
   |
LL | trait I {}
   | ------- similarly named trait `I` defined here
LL | type K = dyn I;
LL | impl K for isize {} //~ ERROR expected trait, found type alias `K`
   |      ^ type aliases cannot be used as traits
   |
help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
   |
LL | trait K = dyn I;
help: a trait with a similar name exists
   |
   |
LL | impl I for isize {} //~ ERROR expected trait, found type alias `K`

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0404, E0432.
---
diff of stderr:

5    |      ^^^ type aliases cannot be used as traits
6    |
7 help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
-   --> $DIR/issue-3907.rs:5:1
9    |
- LL | type Foo = dyn issue_3907::Foo;
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL | trait Foo = dyn issue_3907::Foo;
12 help: consider importing this trait instead
13    |
13    |
14 LL | use issue_3907::Foo;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-3907/issue-3907.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-3907/issue-3907.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/issue-3907.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/issue-3907.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-3907" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-3907/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0404]: expected trait, found type alias `Foo`
  --> /checkout/src/test/ui/resolve/issue-3907.rs:11:6
   |
LL | impl Foo for S { //~ ERROR expected trait, found type alias `Foo`
   |      ^^^ type aliases cannot be used as traits
   |
help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
   |
LL | trait Foo = dyn issue_3907::Foo;
help: consider importing this trait instead
   |
   |
LL | use issue_3907::Foo;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0404`.
---
diff of stderr:

11    |        ^^^^^^^^^^^^^^^^^^^^^^^ type aliases cannot be used as traits
12    |
13 help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
-   --> $DIR/unboxed-closure-sugar-nonexistent-trait.rs:4:1
15    |
- LL | type Typedef = isize;
-    | ^^^^^^^^^^^^^^^^^^^^^
+ LL | trait Typedef = isize;
18 
19 error: aborting due to 2 previous errors
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/unboxed-closure-sugar-nonexistent-trait/unboxed-closure-sugar-nonexistent-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/unboxed-closure-sugar-nonexistent-trait.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/unboxed-closure-sugar-nonexistent-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/unboxed-closure-sugar-nonexistent-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/unboxed-closure-sugar-nonexistent-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0405]: cannot find trait `Nonexist` in this scope
   |
   |
LL | fn f<F:Nonexist(isize) -> isize>(x: F) {}

error[E0404]: expected trait, found type alias `Typedef`
  --> /checkout/src/test/ui/resolve/unboxed-closure-sugar-nonexistent-trait.rs:6:8
   |
   |
LL | fn g<F:Typedef(isize) -> isize>(x: F) {}
   |        ^^^^^^^^^^^^^^^^^^^^^^^ type aliases cannot be used as traits
   |
help: you might have meant to use `#![feature(trait_alias)]` instead of a `type` alias
LL | trait Typedef = isize;
   |

error: aborting due to 2 previous errors
---
test result: FAILED. 11628 passed; 4 failed; 96 ignored; 0 measured; 0 filtered out; finished in 139.95s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:52
