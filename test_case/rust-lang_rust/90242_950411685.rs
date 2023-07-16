plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 479 tests
i........F...........F............F..............i.....F............................................ 100/479
.........F..........F.F............................................................................. 200/479
.................................................................................................... 300/479
..................iF.....................................................F..........i............... 400/479
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...........................................................................F...

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
 // @matches - '//h4[@class="code-header"]' 'pub async fn f\(\)$'
39: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//h4[@class="code-header"]' 'pub async unsafe fn g\(\)$'
40: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//h4[@class="code-header"]' 'pub async fn mut_self\(self, first: usize\)$'
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
52: @has check failed
 `XPATH PATTERN` did not match
     // @has 'foo/struct.Foo.html' '//div[@id="method.gated"]/h4[@class="code-header"]' 'pub fn gated() -> u32'
58: @has check failed
 `XPATH PATTERN` did not match
     // @has 'foo/struct.Foo.html' '//div[@id="method.gated_unsafe"]/h4[@class="code-header"]' 'pub unsafe fn gated_unsafe() -> u32'
64: @has check failed
 `XPATH PATTERN` did not match
     // @has 'foo/struct.Foo.html' '//div[@id="method.stable_impl"]/h4[@class="code-header"]' 'pub const fn stable_impl() -> u32'
Encountered 3 errors

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
44: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//*[@id="method.hey"]' 'pub fn hey<const N: usize>(&self) -> Bar<u8, N>'
52: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//*[@id="method.hey"]' 'pub fn hey<const N: usize>(&self) -> Foo<N> where u8: Trait<N>'
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



---- [rustdoc] rustdoc/higher-ranked-trait-bounds.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/higher-ranked-trait-bounds" "/checkout/src/test/rustdoc/higher-ranked-trait-bounds.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
41: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//h4[@class="code-header"]' "pub fn bar<T>() where T: Trait<'a>,"
Encountered 1 errors

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
 // @has - '//*[@id="method.method"]//h4[@class="code-header"]' "pub fn method<'a>(_x: impl Clone + Into<Vec<u8, Global>> + 'a)"
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
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-76501.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-76501" "/checkout/src/test/rustdoc/issue-76501.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
11: @has check failed
 `XPATH PATTERN` did not match
     // @has 'issue_76501/struct.Struct.html' '//*[@class="method has-srclink"]' 'pub const fn blurp() -> i32'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/pub-method.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/pub-method" "/checkout/src/test/rustdoc/pub-method.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
13: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="method has-srclink"]' 'pub fn new()'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/visibility.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/visibility" "/checkout/src/test/rustdoc/visibility.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
26: @!has check failed
 `XPATH PATTERN` did not match
     // @!has 'foo/a/struct.FooAInA.html' '//pre' 'pub'
29: @!has check failed
 `XPATH PATTERN` did not match
     // @!has 'foo/a/struct.FooAPriv.html' '//pre' 'pub'
38: @!has check failed
 `XPATH PATTERN` did not match
         // @!has 'foo/a/b/struct.FooBInAB.html' '//pre' 'pub'
41: @!has check failed
 `XPATH PATTERN` did not match
         // @!has 'foo/a/b/struct.FooBPriv.html' '//pre' 'pub'
Encountered 4 errors

------------------------------------------

---
test result: FAILED. 465 passed; 10 failed; 4 ignored; 0 measured; 0 filtered out; finished in 44.85s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:16:46
