plain

---- [ui] ui/type-alias-impl-trait/bounds-are-checked.rs stdout ----
diff of stderr:

1 warning: unnecessary lifetime parameter `'a`
-   --> $DIR/bounds-are-checked.rs:10:6
3    |
3    |
4 LL | fn f<'a: 'static>(t: &'a str) -> X<'a> {


7    = help: you can use the `'static` lifetime directly, in place of `'a`
9 error[E0308]: mismatched types
-   --> $DIR/bounds-are-checked.rs:7:14
+   --> $DIR/bounds-are-checked.rs:6:14
11    |
11    |
12 LL | type X<'a> = impl Into<&'static str> + From<&'a str>;
13    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ lifetime mismatch
14    |
14    |
15    = note: expected trait `From<&'a str>`
16               found trait `From<&'static str>`
- note: the lifetime `'a` as defined on the item at 7:8...
-   --> $DIR/bounds-are-checked.rs:7:8
+ note: the lifetime `'a` as defined on the item at 6:8...
19    |
19    |
20 LL | type X<'a> = impl Into<&'static str> + From<&'a str>;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/bounds-are-checked/bounds-are-checked.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/bounds-are-checked/bounds-are-checked.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/bounds-are-checked.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/bounds-are-checked.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/bounds-are-checked" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/bounds-are-checked/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unnecessary lifetime parameter `'a`
   |
   |
LL | fn f<'a: 'static>(t: &'a str) -> X<'a> {
   |
   |
   = help: you can use the `'static` lifetime directly, in place of `'a`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/type-alias-impl-trait/bounds-are-checked.rs:6:14
   |
   |
LL | type X<'a> = impl Into<&'static str> + From<&'a str>;
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ lifetime mismatch
   |
   = note: expected trait `From<&'a str>`
              found trait `From<&'static str>`
note: the lifetime `'a` as defined on the item at 6:8...
   |
   |
LL | type X<'a> = impl Into<&'static str> + From<&'a str>;
   |        ^^
   = note: ...does not necessarily outlive the static lifetime
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0308`.


------------------------------------------


---- [ui] ui/type-alias-impl-trait/type-alias-impl-trait-unconstrained-lifetime.rs stdout ----
diff of stderr:

1 error[E0207]: the lifetime parameter `'a` is not constrained by the impl trait, self type, or predicates
-   --> $DIR/type-alias-impl-trait-unconstrained-lifetime.rs:11:6
3    |
3    |
4 LL | impl<'a, I: Iterator<Item = i32>> Trait for (i32, I) {
5    |      ^^ unconstrained lifetime parameter

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/type-alias-impl-trait-unconstrained-lifetime/type-alias-impl-trait-unconstrained-lifetime.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/type-alias-impl-trait-unconstrained-lifetime.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/type-alias-impl-trait-unconstrained-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/type-alias-impl-trait-unconstrained-lifetime" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/type-alias-impl-trait-unconstrained-lifetime/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0207]: the lifetime parameter `'a` is not constrained by the impl trait, self type, or predicates
   |
   |
LL | impl<'a, I: Iterator<Item = i32>> Trait for (i32, I) {
   |      ^^ unconstrained lifetime parameter
error: aborting due to previous error

For more information about this error, try `rustc --explain E0207`.

---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:56
