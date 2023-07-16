plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.065 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii..........i....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.27s

 finished in 2.340 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; generated diffs will be harder to read

running 412 tests
...........F.............F.......F..i........F.F.................................................... 100/412
..........................................................................................F......... 300/412
..............................F..i.................................................................. 400/412
............
failures:
failures:
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [rustdoc] rustdoc/async-fn.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/async-fn" "/checkout/src/test/rustdoc/async-fn.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
39: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//code' 'pub async fn f\(\)$'
40: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//code' 'pub async unsafe fn g\(\)$'
41: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//code' 'pub async fn mut_self\(self, first: usize\)$'
81: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//h4[@class="method"]' 'pub async fn complicated_lifetimes( &self, context: &impl Bar) -> impl Iterator<Item = &usize>'
84: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//h4[@class="method"]' "pub async fn readable<T>(&self) -> Result<AsyncFdReadyGuard<'_, T>, ()>"
86: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//h4[@class="method"]' "pub async fn mut_self(&mut self)"
Encountered 6 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/async-fn.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/async-fn/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/async-fn.nightly" "/checkout/src/test/rustdoc/async-fn.rs" "--edition=2018"`', src/tools/compiletest/src/runtest.rs:1871:33

---- [rustdoc] rustdoc/const-display.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-display" "/checkout/src/test/rustdoc/const-display.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
43: @has check failed
 `XPATH PATTERN` did not match
     // @has 'foo/struct.Foo.html' '//h4[@id="method.gated"]/code' 'pub unsafe fn gated() -> u32'
48: @has check failed
 `XPATH PATTERN` did not match
     // @has 'foo/struct.Foo.html' '//h4[@id="method.stable_impl"]/code' 'pub const fn stable_impl() -> u32'
Encountered 2 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/const-display.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-display/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-display.nightly" "/checkout/src/test/rustdoc/const-display.rs"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/const-generics/const-generics-docs.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-generics-docs" "/checkout/src/test/rustdoc/const-generics/const-generics-docs.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
41: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//*[@id="associatedconstant.FOO_ASSOC"]' 'pub const FOO_ASSOC: usize'
44: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//*[@id="method.hey"]' 'pub fn hey<const N: usize>(&self) -> Bar<u8, N>'
52: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//*[@id="method.hey"]' 'pub fn hey<const N: usize>(&self) -> Foo<N> where u8: Trait<N>'
Encountered 3 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/const-generics/const-generics-docs.rs' panicked at 'failed to exec `"rustc" "/checkout/src/test/rustdoc/const-generics/auxiliary/extern_crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-generics-docs/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-generics-docs/auxiliary"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/deref-recursive.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-recursive" "/checkout/src/test/rustdoc/deref-recursive.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
8: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="impl-items"]//*[@id="method.bar"]' 'pub fn bar(&self)'
10: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="impl-items"]//*[@id="method.baz"]' 'pub fn baz(&self)'
Encountered 2 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/deref-recursive.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-recursive/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-recursive.nightly" "/checkout/src/test/rustdoc/deref-recursive.rs"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/deref-typedef.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-typedef" "/checkout/src/test/rustdoc/deref-typedef.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
7: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="impl-items"]//*[@id="method.foo_a"]' 'pub fn foo_a(&self)'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="impl-items"]//*[@id="method.foo_b"]' 'pub fn foo_b(&self)'
9: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="impl-items"]//*[@id="method.foo_c"]' 'pub fn foo_c(&self)'
10: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="impl-items"]//*[@id="method.foo_j"]' 'pub fn foo_j(&self)'
Encountered 4 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/deref-typedef.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-typedef/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-typedef.nightly" "/checkout/src/test/rustdoc/deref-typedef.rs"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/issue-76501.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-76501" "/checkout/src/test/rustdoc/issue-76501.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
11: @has check failed
 `XPATH PATTERN` did not match
     // @has 'issue_76501/struct.Struct.html' '//*[@class="method"]' 'pub const fn blurp() -> i32'
Encountered 1 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/issue-76501.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-76501/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-76501.nightly" "/checkout/src/test/rustdoc/issue-76501.rs"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/pub-method.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/pub-method" "/checkout/src/test/rustdoc/pub-method.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
13: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="method"]' 'pub fn new()'
Encountered 1 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/pub-method.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/pub-method/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/pub-method.nightly" "/checkout/src/test/rustdoc/pub-method.rs" "--document-private-items"`', src/tools/compiletest/src/runtest.rs:1871:33

failures:
    [rustdoc] rustdoc/async-fn.rs
    [rustdoc] rustdoc/const-display.rs
---
test result: FAILED. 403 passed; 7 failed; 2 ignored; 0 measured; 0 filtered out; finished in 36.70s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:18:22
