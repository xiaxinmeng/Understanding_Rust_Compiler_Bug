plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 478 tests
i...............................F..F.............i..............................F................... 100/478
................................................................................F................... 200/478
................................................................................................F... 300/478
F....F....F......i.................................................................i................ 400/478
.........................FFF..F.FF.FF.........................................
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [rustdoc] rustdoc/const-generics/const-impl.rs stdout ----


error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-impl" "/checkout/src/test/rustdoc/const-generics/const-impl.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
12: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.VSet.html '//div[@id="impl-Send"]/h3[@class="code-header in-band"]' 'impl<T, const ORDER: Order> Send for VSet<T, ORDER>'
13: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.VSet.html '//div[@id="impl-Sync"]/h3[@class="code-header in-band"]' 'impl<T, const ORDER: Order> Sync for VSet<T, ORDER>'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/const-generics/lazy_normalization_consts/const-equate-pred.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/lazy_normalization_consts/const-equate-pred" "/checkout/src/test/rustdoc/const-generics/lazy_normalization_consts/const-equate-pred.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
15: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//h3[@class="code-header in-band"]' 'impl Send for Foo'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/empty-section.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/empty-section" "/checkout/src/test/rustdoc/empty-section.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
8: @!has check failed
 `PATTERN` did not match
 // @!has - 'Auto Trait Implementations'
Encountered 1 errors

------------------------------------------

---

------------------------------------------
stderr:
------------------------------------------
warning: lint `broken_intra_doc_links` has been renamed to `rustdoc::broken_intra_doc_links`
  |
  |
1 | #![deny(broken_intra_doc_links)]
  |         ^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `rustdoc::broken_intra_doc_links`
  = note: `#[warn(renamed_and_removed_lints)]` on by default


thread 'rustc' panicked at 'Unexpected result when selecting std::str::EscapeDebug<'a> Obligation(predicate=Binder(ProjectionPredicate(ProjectionTy { substs: [_], item_def_id: DefId(2:7422 ~ core[54ad]::iter::traits::collect::IntoIterator::IntoIter) }, _), []), depth=2)', compiler/rustc_trait_selection/src/traits/auto_trait.rs:771:33

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---


---- [rustdoc] rustdoc/issue-50159.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-50159" "/checkout/src/test/rustdoc/issue-50159.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
14: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//h3[@class="code-header in-band"]' 'impl<B> Send for Switch<B> where <B as Signal>::Item: Send'
15: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//h3[@class="code-header in-band"]' 'impl<B> Sync for Switch<B> where <B as Signal>::Item: Sync'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-54705.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-54705" "/checkout/src/test/rustdoc/issue-54705.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
6: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="synthetic-implementations-list"]//*[@class="impl has-srclink"]//h3[@class="code-header in-band"]' "impl<'scope, S> Send for ScopeFutureContents<'scope, S> where S: Sync"
9: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="synthetic-implementations-list"]//*[@class="impl has-srclink"]//h3[@class="code-header in-band"]' "impl<'scope, S> Sync for ScopeFutureContents<'scope, S> where S: Sync"
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-56822.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-56822" "/checkout/src/test/rustdoc/issue-56822.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
20: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="synthetic-implementations-list"]//*[@class="impl has-srclink"]//h3[@class="code-header in-band"]' "impl<'a> Send for Parser<'a>"
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-80233-normalize-auto-trait.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-80233-normalize-auto-trait/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-80233-normalize-auto-trait" "/checkout/src/test/rustdoc/issue-80233-normalize-auto-trait.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'Unexpected result when selecting Question<T> Obligation(predicate=Binder(ProjectionPredicate(ProjectionTy { substs: [_], item_def_id: DefId(0:5 ~ issue_80233_normalize_auto_trait[695a]::Trait2::Type2) }, _), []), depth=2)', compiler/rustc_trait_selection/src/traits/auto_trait.rs:771:33

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---


---- [rustdoc] rustdoc/synthetic_auto/complex.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/complex" "/checkout/src/test/rustdoc/synthetic_auto/complex.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
23: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="synthetic-implementations-list"]//*[@class="impl has-srclink"]//h3[@class="code-header in-band"]' "impl<'a, T, K: ?Sized> Send for Outer<'a, T, K> where K: for<'b> Fn((&'b bool, &'a u8)) -> &'b i8, T: MyTrait<'a>, <T as MyTrait<'a>>::MyItem: Copy, 'a: 'static"
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/synthetic_auto/lifetimes.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/lifetimes" "/checkout/src/test/rustdoc/synthetic_auto/lifetimes.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
12: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="synthetic-implementations-list"]//*[@class="impl has-srclink"]//h3[@class="code-header in-band"]' "impl<'c, K> Send for Foo<'c, K> where K: for<'b> Fn(&'b bool) -> &'c u8, 'c: 'static"
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/synthetic_auto/manual.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/manual" "/checkout/src/test/rustdoc/synthetic_auto/manual.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
9: @count check failed
 Expected 4 occurrences but found 5
 // @count - '//*[@id="synthetic-implementations-list"]//*[@class="impl has-srclink"]' 4
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/synthetic_auto/nested.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/nested" "/checkout/src/test/rustdoc/synthetic_auto/nested.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
12: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="synthetic-implementations-list"]//*[@class="impl has-srclink"]//h3[@class="code-header in-band"]' 'impl<T> Send for Foo<T> where T: Copy'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/synthetic_auto/overflow.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/overflow" "/checkout/src/test/rustdoc/synthetic_auto/overflow.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
24: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//h3[@class="code-header in-band"]' "impl<'tcx> Send for BoundVarsCollector<'tcx>"
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/synthetic_auto/no-redundancy.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/no-redundancy" "/checkout/src/test/rustdoc/synthetic_auto/no-redundancy.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
12: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="synthetic-implementations-list"]//*[@class="impl has-srclink"]//h3[@class="code-header in-band"]' "impl<T> Send for Outer<T> where T: Send + Copy"
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/synthetic_auto/project.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/project" "/checkout/src/test/rustdoc/synthetic_auto/project.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
26: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="synthetic-implementations-list"]//*[@class="impl has-srclink"]//h3[@class="code-header in-band"]' "impl<'c, K> Send for Foo<'c, K> where K: MyTrait<MyItem = bool>, 'c: 'static"
29: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="synthetic-implementations-list"]//*[@class="impl has-srclink"]//h3[@class="code-header in-band"]' "impl<'c, K> Sync for Foo<'c, K> where K: MyTrait, <K as MyTrait>::MyItem: OtherTrait, 'c: 'static,"
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/synthetic_auto/self-referential.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/self-referential" "/checkout/src/test/rustdoc/synthetic_auto/self-referential.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
26: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="synthetic-implementations-list"]//*[@class="impl has-srclink"]//h3[@class="code-header in-band"]' "impl<P1> Send for WriteAndThen<P1>  where  <P1 as Pattern>::Value: Send"
Encountered 1 errors

------------------------------------------

---
test result: FAILED. 458 passed; 16 failed; 4 ignored; 0 measured; 0 filtered out; finished in 35.81s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:15:14
