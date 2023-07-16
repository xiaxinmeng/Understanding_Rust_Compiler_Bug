plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 446 tests
i......FF.............F...F......FF.....F.i.......F.FF.........F..........F.......F................. 100/446
.....F.FF.F.F...........................................................................F.........F. 200/446
.FF..FFFFFF.....F.......F.F.F.....................F....F............................................ 300/446
..F...F......................................F......i............................................... 400/446
......................F......................F
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [rustdoc] rustdoc/assoc-types.rs stdout ----


error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-types" "/checkout/src/test/rustdoc/assoc-types.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
7: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//*[@id="tymethod.index"]//code' "fn index<'a>(&'a self, index: I) -> &'a Self::Output"
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/async-fn.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/async-fn" "/checkout/src/test/rustdoc/async-fn.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
38: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//code' 'pub async fn f\(\)$'
39: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//code' 'pub async unsafe fn g\(\)$'
40: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//code' 'pub async fn mut_self\(self, first: usize\)$'
80: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//div[@class="method has-srclink"]' 'pub async fn complicated_lifetimes( &self, context: &impl Bar) -> impl Iterator<Item = &usize>'
83: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//div[@class="method has-srclink"]' "pub async fn readable<T>(&self) -> Result<AsyncFdReadyGuard<'_, T>, ()>"
85: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//div[@class="method has-srclink"]' "pub async fn mut_self(&mut self)"
Encountered 6 errors

------------------------------------------



---- [rustdoc] rustdoc/const-display.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-display" "/checkout/src/test/rustdoc/const-display.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
41: @has check failed
 `XPATH PATTERN` did not match
     // @has 'foo/struct.Foo.html' '//div[@id="method.gated"]/code' 'pub unsafe fn gated() -> u32'
46: @has check failed
 `XPATH PATTERN` did not match
     // @has 'foo/struct.Foo.html' '//div[@id="method.stable_impl"]/code' 'pub const fn stable_impl() -> u32'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/const-fn.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-fn" "/checkout/src/test/rustdoc/const-fn.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
11: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="method has-srclink"]' 'const fn new()'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/const-generics/const-generics-docs.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-generics-docs" "/checkout/src/test/rustdoc/const-generics/const-generics-docs.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
17: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="rust trait"]' 'fn hey<const P: usize>() -> usize'
44: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//*[@id="method.hey"]' 'pub fn hey<const N: usize>(&self) -> Bar<u8, N>'
52: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//*[@id="method.hey"]' 'pub fn hey<const N: usize>(&self) -> Foo<N> where u8: Trait<N>'
Encountered 3 errors

------------------------------------------



---- [rustdoc] rustdoc/const.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const" "/checkout/src/test/rustdoc/const.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
6: @has check failed
 `XPATH PATTERN` did not match
     // @has const/struct.Foo.html '//*[@id="method.new"]//code' 'const unsafe fn new'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/default-trait-method.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/default-trait-method" "/checkout/src/test/rustdoc/default-trait-method.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
4: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="tymethod.foo"]' 'fn foo()'
6: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="tymethod.bar"]' 'fn bar()'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.baz"]' 'fn baz()'
17: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.foo"]' 'default fn foo()'
18: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.bar"]' 'fn bar()'
20: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.baz"]' 'fn baz()'
Encountered 6 errors

------------------------------------------



---- [rustdoc] rustdoc/deref-recursive-pathbuf.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-recursive-pathbuf" "/checkout/src/test/rustdoc/deref-recursive-pathbuf.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
6: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="impl-items"]//*[@id="method.as_path"]' 'pub fn as_path(&self)'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="impl-items"]//*[@id="method.exists"]' 'pub fn exists(&self)'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/deref-recursive.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-recursive" "/checkout/src/test/rustdoc/deref-recursive.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
6: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="impl-items"]//*[@id="method.bar"]' 'pub fn bar(&self)'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="impl-items"]//*[@id="method.baz"]' 'pub fn baz(&self)'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/deref-typedef.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-typedef" "/checkout/src/test/rustdoc/deref-typedef.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
5: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="impl-items"]//*[@id="method.foo_a"]' 'pub fn foo_a(&self)'
6: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="impl-items"]//*[@id="method.foo_b"]' 'pub fn foo_b(&self)'
7: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="impl-items"]//*[@id="method.foo_c"]' 'pub fn foo_c(&self)'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="impl-items"]//*[@id="method.foo_j"]' 'pub fn foo_j(&self)'
Encountered 4 errors

------------------------------------------



---- [rustdoc] rustdoc/doc-cfg.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-cfg" "/checkout/src/test/rustdoc/doc-cfg.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
6: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.unix_and_arm_only_function"]' 'fn unix_and_arm_only_function()'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.wasi_and_wasm32_only_function"]' 'fn wasi_and_wasm32_only_function()'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/extern-impl.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-impl" "/checkout/src/test/rustdoc/extern-impl.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
7: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//code' 'fn rust0()'
9: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//code' 'fn rust1()'
11: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//code' 'extern "C" fn c0()'
13: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//code' 'extern "C" fn c1()'
15: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//code' 'extern "system" fn system0()'
Encountered 5 errors

------------------------------------------



---- [rustdoc] rustdoc/extern-method.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-method" "/checkout/src/test/rustdoc/extern-method.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
9: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="tymethod.foo"]//code' 'extern "rust-call" fn foo'
10: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.foo_"]//code' 'extern "rust-call" fn foo_'
15: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//*[@id="tymethod.bar"]//code' 'extern "rust-call" fn bar'
17: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//*[@id="method.bar_"]//code' 'extern "rust-call" fn bar_'
Encountered 4 errors

------------------------------------------



---- [rustdoc] rustdoc/inline-default-methods.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline-default-methods" "/checkout/src/test/rustdoc/inline-default-methods.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
7: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="rust trait"]' 'fn bar(&self);'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="rust trait"]' 'fn foo(&mut self) { ... }'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/inline_cross/default-trait-method.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/default-trait-method" "/checkout/src/test/rustdoc/inline_cross/default-trait-method.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
6: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="tymethod.foo"]' 'fn foo()'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="tymethod.bar"]' 'fn bar()'
10: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.baz"]' 'fn baz()'
15: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.foo"]' 'default fn foo()'
16: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.bar"]' 'fn bar()'
18: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.baz"]' 'fn baz()'
Encountered 6 errors

------------------------------------------



---- [rustdoc] rustdoc/inline_cross/impl_trait.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/impl_trait" "/checkout/src/test/rustdoc/inline_cross/impl_trait.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
34: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.method"]//code' "pub fn method<'a>(_x: impl Clone + Into<Vec<u8, Global>> + 'a)"
39: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.async_foo"]' "pub async fn async_foo("
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/inline_cross/assoc-items.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/assoc-items" "/checkout/src/test/rustdoc/inline_cross/assoc-items.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
14: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.public_method"]' 'pub fn public_method()'
24: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.method_no_default"]' 'fn method_no_default()'
26: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.method_with_default"]' 'fn method_with_default()'
38: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="tymethod.method_no_default"]' 'fn method_no_default()'
40: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.method_with_default"]' 'fn method_with_default()'
Encountered 5 errors

------------------------------------------



---- [rustdoc] rustdoc/inline_cross/impl-inline-without-trait.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/impl-inline-without-trait" "/checkout/src/test/rustdoc/inline_cross/impl-inline-without-trait.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
10: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.my_trait_method"]' 'fn my_trait_method()'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-15169.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-15169" "/checkout/src/test/rustdoc/issue-15169.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
1: @has check failed
 `XPATH PATTERN` did not match
 // @has issue_15169/struct.Foo.html '//*[@id="method.eq"]' 'fn eq'
Encountered 1 errors

------------------------------------------
---
test result: FAILED. 404 passed; 39 failed; 3 ignored; 0 measured; 0 filtered out; finished in 24.76s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:11:55
