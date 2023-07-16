plain
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  IMAGE: x86_64-gnu-llvm-12
##[endgroup]
fatal: unknown commit 53fd98ca776cb875bc9e5514f56b52eb74f9e7a9
All commits in `HEAD` are present in `master`
src/ci/scripts/run-build-from-ci.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-12
---
To only update this specific test, also pass `--test-args simd/portable-intrinsics-arent-exposed.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd/portable-intrinsics-arent-exposed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/portable-intrinsics-arent-exposed" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/portable-intrinsics-arent-exposed/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0432]: unresolved import `std::simd`
  --> /checkout/src/test/ui/simd/portable-intrinsics-arent-exposed.rs:4:10
   |
LL | use std::simd::intrinsics; //~ERROR E0603
   |          ^^^^ could not find `simd` in `std`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.


------------------------------------------


---- [ui] ui/suggestions/issue-71394-no-from-impl.rs stdout ----
diff of stderr:

4 LL |     let _: &[i8] = data.into();
5    |                         ^^^^ the trait `From<&[u8]>` is not implemented for `&[i8]`
-    = help: the following implementations were found:
-    = help: the following implementations were found:
-              <[T; LANES] as From<Simd<T, LANES>>>
-              <[bool; LANES] as From<Mask<T, LANES>>>
10    = note: required because of the requirements on the impl of `Into<&[i8]>` for `&[u8]`
12 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-71394-no-from-impl/issue-71394-no-from-impl.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/issue-71394-no-from-impl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-71394-no-from-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-71394-no-from-impl" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-71394-no-from-impl/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `&[i8]: From<&[u8]>` is not satisfied
   |
   |
LL |     let _: &[i8] = data.into();
   |                         ^^^^ the trait `From<&[u8]>` is not implemented for `&[i8]`
   |
   = note: required because of the requirements on the impl of `Into<&[i8]>` for `&[u8]`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/traits/issue-85360-eval-obligation-ice.rs stdout ----
diff of stderr:

16 LL | fn test<T: Sized>(_: T) {}
18 
18 
- error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp2<Pos>> as std::marker::Sized>, polarity:Positive), [])) = Ok(EvaluatedToOkModuloRegions)
+ error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp2<Pos>> as std::marker::Sized>, polarity:Positive), [])) = Ok(EvaluatedToOk)
21    |
21    |
22 LL |     test::<MaskedStorage<GenericComp2<Pos>>>(make());

25 LL | fn test<T: Sized>(_: T) {}
26    |         - predicate
27 
- error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp2<Pos>> as std::marker::Sized>, polarity:Positive), [])) = Ok(EvaluatedToOkModuloRegions)
+ error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp2<Pos>> as std::marker::Sized>, polarity:Positive), [])) = Ok(EvaluatedToOk)
30    |
30    |
31 LL |     test::<MaskedStorage<GenericComp2<Pos>>>(make());

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-85360-eval-obligation-ice/issue-85360-eval-obligation-ice.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/issue-85360-eval-obligation-ice.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-85360-eval-obligation-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-85360-eval-obligation-ice" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-85360-eval-obligation-ice/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp<Pos>> as std::marker::Sized>, polarity:Positive), [])) = Ok(EvaluatedToOk)
   |
   |
LL |     test::<MaskedStorage<GenericComp<Pos>>>(make());
...
...
LL | fn test<T: Sized>(_: T) {}
   |         - predicate

error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp<Pos>> as std::marker::Sized>, polarity:Positive), [])) = Ok(EvaluatedToOk)
   |
   |
LL |     test::<MaskedStorage<GenericComp<Pos>>>(make());
...
...
LL | fn test<T: Sized>(_: T) {}


error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp2<Pos>> as std::marker::Sized>, polarity:Positive), [])) = Ok(EvaluatedToOk)
   |
   |
LL |     test::<MaskedStorage<GenericComp2<Pos>>>(make());
...
...
LL | fn test<T: Sized>(_: T) {}
   |         - predicate

error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp2<Pos>> as std::marker::Sized>, polarity:Positive), [])) = Ok(EvaluatedToOk)
   |
   |
LL |     test::<MaskedStorage<GenericComp2<Pos>>>(make());
...
...
LL | fn test<T: Sized>(_: T) {}

error: aborting due to 4 previous errors


---
test result: FAILED. 12302 passed; 3 failed; 111 ignored; 0 measured; 0 filtered out; finished in 118.23s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "beta" "--color" "always"


Build completed unsuccessfully in 0:11:28
