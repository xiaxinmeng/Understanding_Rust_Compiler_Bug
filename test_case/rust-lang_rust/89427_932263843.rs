plain
---- [ui] ui/traits/issue-77982.rs stdout ----
diff of stderr:

24    |              ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
25 LL |     opts.get(<String as AsRef<OsStr>>::as_ref(opt));
26    |              ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
- LL |     opts.get(<String as AsRef<str>>::as_ref(opt));
-    |              ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
29 LL |     opts.get(<String as AsRef<[u8]>>::as_ref(opt));
30    |              ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
+ LL |     opts.get(<String as AsRef<str>>::as_ref(opt));
31 
32 error[E0283]: type annotations needed
33   --> $DIR/issue-77982.rs:13:44

---

error: 1 errors occurred comparing output.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-77982.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:8:10
   |
LL |     opts.get(opt.as_ref()); //~ ERROR type annotations needed
   |          ^^^ ------------ this method call resolves to `&T`
   |          |
   |          cannot infer type for type parameter `Q` declared on the associated function `get`
   |
   = note: multiple `impl`s for `String: Borrow<_>` found in the following crates: `alloc`, `core`
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:8:18
   |
   |
LL |     opts.get(opt.as_ref()); //~ ERROR type annotations needed
   |              |   |
   |              |   |
   |              |   cannot infer type for type parameter `T` declared on the trait `AsRef`
   |              this method call resolves to `&T`
   |
   = note: multiple `impl`s for `String: AsRef<_>` found in the following crates: `alloc`, `std`
help: use the fully qualified path for the potential candidates
   |
LL |     opts.get(<String as AsRef<Path>>::as_ref(opt)); //~ ERROR type annotations needed
   |              ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
LL |     opts.get(<String as AsRef<OsStr>>::as_ref(opt)); //~ ERROR type annotations needed
   |              ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
LL |     opts.get(<String as AsRef<[u8]>>::as_ref(opt)); //~ ERROR type annotations needed
   |              ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
LL |     opts.get(<String as AsRef<str>>::as_ref(opt)); //~ ERROR type annotations needed

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:13:44
   |
   |
LL |     let ips: Vec<_> = (0..100_000).map(|_| u32::from(0u32.into())).collect();
   |                                            ^^^^^^^^^ ----------- this method call resolves to `T`
   |                                            |
   |                                            cannot infer type for type parameter `T` declared on the trait `From`
   |
   = note: multiple `impl`s for `u32: From<_>` found in the following crates: `core`, `std`
note: required by `from`
   |
   |
LL |     fn from(_: T) -> Self;

error[E0283]: type annotations needed for `Box<T>`
  --> /checkout/src/test/ui/traits/issue-77982.rs:36:16
   |
   |
LL |     let _ = ().foo(); //~ ERROR type annotations needed
   |         -      ^^^ cannot infer type for type parameter `T` declared on the trait `Foo`
   |         |
   |         consider giving this pattern the explicit type `Box<T>`, where the type parameter `T` is specified
   |
note: multiple `impl`s for `(): Foo<'_, _>` found
   |
   |
LL | impl Foo<'static, u32> for () {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | impl<'a> Foo<'a, i16> for () {}

error[E0283]: type annotations needed for `Box<T>`
  --> /checkout/src/test/ui/traits/issue-77982.rs:40:19
   |
   |
LL |     let _ = (&()).bar(); //~ ERROR type annotations needed
   |         -         ^^^ cannot infer type for type parameter `T` declared on the trait `Bar`
   |         |
   |         consider giving this pattern the explicit type `Box<T>`, where the type parameter `T` is specified
   |
note: multiple `impl`s for `&(): Bar<'_, _>` found
   |
   |
LL | impl<'a> Bar<'static, u32> for &'a () {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | impl<'a> Bar<'a, i16> for &'a () {}

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0283`.
---
test result: FAILED. 12067 passed; 1 failed; 163 ignored; 0 measured; 0 filtered out; finished in 88.63s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--pass" "check" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:01:31
