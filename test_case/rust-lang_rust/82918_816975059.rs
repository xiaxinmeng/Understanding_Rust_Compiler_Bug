plain
.................................................................................................... 9400/11747
.................................................................................................... 9500/11747
...................................................................................i.......i........ 9600/11747
.................................................................................................... 9700/11747
.............................iiiiiii..iiiiii.i...................................................... 9800/11747
.................................................................................................... 10000/11747
.................................................................................................... 10100/11747
.................................................................................................... 10200/11747
.................................................................................................... 10300/11747
---

---- [ui] ui/anon-params/anon-params-denied-2018.rs stdout ----
diff of stderr:

138 LL |     fn baz(a:usize, _: b, c: usize) -> usize {
140 
- error: aborting due to 9 previous errors
- error: aborting due to 9 previous errors
+ warning: anonymous parameters are deprecated and will be removed in the next edition.
+   --> $DIR/anon-params-denied-2018.rs:12:44
+    |
+ LL |     fn foo_with_qualified_path(<Bar as T>::Baz);
+    |                                            ^^^ help: try naming the parameter or explicitly ignoring it: `_: Baz`
+    = note: `#[warn(anonymous_parameters)]` on by default
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
+    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
+ 
+ warning: anonymous parameters are deprecated and will be removed in the next edition.
+   --> $DIR/anon-params-denied-2018.rs:15:53
+    |
+ LL |     fn foo_with_qualified_path_and_ref(&<Bar as T>::Baz);
+    |                                                     ^^^ help: try naming the parameter or explicitly ignoring it: `_: Baz`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
+ 
+ 
+ warning: anonymous parameters are deprecated and will be removed in the next edition.
+   --> $DIR/anon-params-denied-2018.rs:18:54
+    |
+ LL |     fn foo_with_multiple_qualified_paths(<Bar as T>::Baz, <Bar as T>::Baz);
+    |                                                      ^^^ help: try naming the parameter or explicitly ignoring it: `_: Baz`
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
+    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
+ 
+ error: aborting due to 9 previous errors; 3 warnings emitted
+ error: aborting due to 9 previous errors; 3 warnings emitted
142 
143 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anon-params/anon-params-denied-2018/anon-params-denied-2018.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args anon-params/anon-params-denied-2018.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/anon-params/anon-params-denied-2018.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anon-params/anon-params-denied-2018" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anon-params/anon-params-denied-2018/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected one of `:`, `@`, or `|`, found `)`
   |
   |
LL |     fn foo(i32); //~ expected one of `:`, `@`, or `|`, found `)`
   |               ^ expected one of `:`, `@`, or `|`
   |
   = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
help: if this is a `self` type, give it a parameter name
   |
LL |     fn foo(self: i32); //~ expected one of `:`, `@`, or `|`, found `)`
   |            ^^^^^^^^^
help: if this is a parameter name, give it a type
   |
LL |     fn foo(i32: TypeName); //~ expected one of `:`, `@`, or `|`, found `)`
help: if this is a type, explicitly ignore the parameter name
   |
   |
LL |     fn foo(_: i32); //~ expected one of `:`, `@`, or `|`, found `)`


error: expected one of `:`, `@`, or `|`, found `)`
   |
   |
LL |     fn foo_with_ref(&mut i32);
   |                             ^ expected one of `:`, `@`, or `|`
   |
   = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
help: if this is a `self` type, give it a parameter name
   |
LL |     fn foo_with_ref(self: &mut i32);
   |                     ^^^^^^^^^^^^^^
help: if this is a parameter name, give it a type
   |
LL |     fn foo_with_ref(i32: &mut TypeName);
help: if this is a type, explicitly ignore the parameter name
   |
   |
LL |     fn foo_with_ref(_: &mut i32);


error: expected one of `(`, `...`, `..=`, `..`, `::`, `:`, `{`, or `|`, found `)`
   |
   |
LL |     fn foo_with_qualified_path(<Bar as T>::Baz);
   |                                               ^ expected one of 8 possible tokens
   |
   = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
help: explicitly ignore the parameter name
   |
LL |     fn foo_with_qualified_path(_: <Bar as T>::Baz);


error: expected one of `(`, `...`, `..=`, `..`, `::`, `:`, `{`, or `|`, found `)`
   |
   |
LL |     fn foo_with_qualified_path_and_ref(&<Bar as T>::Baz);
   |                                                        ^ expected one of 8 possible tokens
   |
   = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
help: explicitly ignore the parameter name
   |
LL |     fn foo_with_qualified_path_and_ref(_: &<Bar as T>::Baz);


error: expected one of `(`, `...`, `..=`, `..`, `::`, `:`, `{`, or `|`, found `,`
   |
   |
LL |     fn foo_with_multiple_qualified_paths(<Bar as T>::Baz, <Bar as T>::Baz);
   |                                                         ^ expected one of 8 possible tokens
   |
   = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
help: explicitly ignore the parameter name
   |
LL |     fn foo_with_multiple_qualified_paths(_: <Bar as T>::Baz, <Bar as T>::Baz);


error: expected one of `(`, `...`, `..=`, `..`, `::`, `:`, `{`, or `|`, found `)`
   |
   |
LL |     fn foo_with_multiple_qualified_paths(<Bar as T>::Baz, <Bar as T>::Baz);
   |                                                                          ^ expected one of 8 possible tokens
   |
   = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
help: explicitly ignore the parameter name
   |
LL |     fn foo_with_multiple_qualified_paths(<Bar as T>::Baz, _: <Bar as T>::Baz);


error: expected one of `:`, `@`, or `|`, found `,`
   |
   |
LL |     fn bar_with_default_impl(String, String) {}
   |                                    ^ expected one of `:`, `@`, or `|`
   |
   = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
help: if this is a `self` type, give it a parameter name
   |
LL |     fn bar_with_default_impl(self: String, String) {}
   |                              ^^^^^^^^^^^^
help: if this is a parameter name, give it a type
   |
LL |     fn bar_with_default_impl(String: TypeName, String) {}
help: if this is a type, explicitly ignore the parameter name
   |
   |
LL |     fn bar_with_default_impl(_: String, String) {}


error: expected one of `:`, `@`, or `|`, found `)`
   |
   |
LL |     fn bar_with_default_impl(String, String) {}
   |                                            ^ expected one of `:`, `@`, or `|`
   |
   = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
help: if this is a parameter name, give it a type
LL |     fn bar_with_default_impl(String, String: TypeName) {}
   |                                      ^^^^^^^^^^^^^^^^
help: if this is a type, explicitly ignore the parameter name
   |
   |
LL |     fn bar_with_default_impl(String, _: String) {}


error: expected one of `:`, `@`, or `|`, found `,`
   |
   |
LL |     fn baz(a:usize, b, c: usize) -> usize { //~ ERROR expected one of `:`
   |                      ^ expected one of `:`, `@`, or `|`
   |
   = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
help: if this is a parameter name, give it a type
   |
LL |     fn baz(a:usize, b: TypeName, c: usize) -> usize { //~ ERROR expected one of `:`
help: if this is a type, explicitly ignore the parameter name
   |
   |
LL |     fn baz(a:usize, _: b, c: usize) -> usize { //~ ERROR expected one of `:`


warning: anonymous parameters are deprecated and will be removed in the next edition.
   |
   |
LL |     fn foo_with_qualified_path(<Bar as T>::Baz);
   |                                            ^^^ help: try naming the parameter or explicitly ignoring it: `_: Baz`
   = note: `#[warn(anonymous_parameters)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>


warning: anonymous parameters are deprecated and will be removed in the next edition.
   |
   |
LL |     fn foo_with_qualified_path_and_ref(&<Bar as T>::Baz);
   |                                                     ^^^ help: try naming the parameter or explicitly ignoring it: `_: Baz`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>


warning: anonymous parameters are deprecated and will be removed in the next edition.
   |
   |
LL |     fn foo_with_multiple_qualified_paths(<Bar as T>::Baz, <Bar as T>::Baz);
   |                                                      ^^^ help: try naming the parameter or explicitly ignoring it: `_: Baz`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>

error: aborting due to 9 previous errors; 3 warnings emitted
---
test result: FAILED. 11649 passed; 1 failed; 97 ignored; 0 measured; 0 filtered out; finished in 121.97s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:27
