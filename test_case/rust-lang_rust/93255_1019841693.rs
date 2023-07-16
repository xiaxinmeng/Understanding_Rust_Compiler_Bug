plain

---- [ui] ui/derives/deriving-copyclone.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `E: Clone` is not satisfied
3    |
3    |
- LL | const _: () = is_clone(E(A { a: 1, b: 2 })); //~ ERROR Clone
+ LL | const _: () = is_clone(E(A { a: 1, b: 2 }));
5    |               -------- ^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `E`
7    |               required by a bound introduced by this call


15 error[E0277]: the trait bound `B<C>: Copy` is not satisfied
17    |
17    |
- LL |     is_copy(B { a: 1, b: C }); //~ ERROR Copy
+ LL |     is_copy(B { a: 1, b: C });
19    |     ------- ^^^^^^^^^^^^^^^^ expected an implementor of trait `Copy`
21    |     required by a bound introduced by this call

33    = note: this error originates in the derive macro `Copy` (in Nightly builds, run with -Z macro-backtrace for more info)
34 help: consider borrowing here
34 help: consider borrowing here
35    |
- LL |     is_copy(&B { a: 1, b: C }); //~ ERROR Copy
+ LL |     is_copy(&B { a: 1, b: C });
38 
38 
39 error[E0277]: the trait bound `B<C>: Clone` is not satisfied
40   --> $DIR/deriving-copyclone.rs:44:14
41    |
41    |
- LL |     is_clone(B { a: 1, b: C }); //~ ERROR Clone
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ LL |     is_clone(B { a: 1, b: C });
43    |     -------- ^^^^^^^^^^^^^^^^ expected an implementor of trait `Clone`
45    |     required by a bound introduced by this call

57    = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
58 help: consider borrowing here
58 help: consider borrowing here
59    |
- LL |     is_clone(&B { a: 1, b: C }); //~ ERROR Clone
+ LL |     is_clone(&B { a: 1, b: C });
62 
62 
63 error[E0277]: the trait bound `B<D>: Copy` is not satisfied
64   --> $DIR/deriving-copyclone.rs:47:13
65    |
65    |
- LL |     is_copy(B { a: 1, b: D }); //~ ERROR Copy
+ LL |     is_copy(B { a: 1, b: D });
67    |     ------- ^^^^^^^^^^^^^^^^ expected an implementor of trait `Copy`
69    |     required by a bound introduced by this call

81    = note: this error originates in the derive macro `Copy` (in Nightly builds, run with -Z macro-backtrace for more info)
82 help: consider borrowing here
82 help: consider borrowing here
83    |
- LL |     is_copy(&B { a: 1, b: D }); //~ ERROR Copy
+ LL |     is_copy(&B { a: 1, b: D });
86 
87 error: aborting due to 4 previous errors



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-copyclone/deriving-copyclone.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args derives/deriving-copyclone.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/deriving-copyclone.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-copyclone" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-copyclone/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `E: Clone` is not satisfied
  --> /checkout/src/test/ui/derives/deriving-copyclone.rs:55:24
   |
LL | const _: () = is_clone(E(A { a: 1, b: 2 })); //~ ERROR Clone
   |               -------- ^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `E`
   |               required by a bound introduced by this call
   |
note: required by a bound in `is_clone`
  --> /checkout/src/test/ui/derives/deriving-copyclone.rs:31:22
  --> /checkout/src/test/ui/derives/deriving-copyclone.rs:31:22
   |
LL | const fn is_clone<T: ~const Clone>(_: T) {}
   |                      ^^^^^^^^^^^^ required by this bound in `is_clone`

error[E0277]: the trait bound `B<C>: Copy` is not satisfied
  --> /checkout/src/test/ui/derives/deriving-copyclone.rs:43:13
   |
LL |     is_copy(B { a: 1, b: C }); //~ ERROR Copy
   |     ------- ^^^^^^^^^^^^^^^^ expected an implementor of trait `Copy`
   |     required by a bound introduced by this call
   |
   |
note: required because of the requirements on the impl of `Copy` for `B<C>`
  --> /checkout/src/test/ui/derives/deriving-copyclone.rs:11:10
LL | #[derive(Copy, Clone)]
   |          ^^^^
note: required by a bound in `is_copy`
  --> /checkout/src/test/ui/derives/deriving-copyclone.rs:30:15
  --> /checkout/src/test/ui/derives/deriving-copyclone.rs:30:15
   |
LL | fn is_copy<T: Copy>(_: T) {}
   |               ^^^^ required by this bound in `is_copy`
   = note: this error originates in the derive macro `Copy` (in Nightly builds, run with -Z macro-backtrace for more info)
   |
   |
LL |     is_copy(&B { a: 1, b: C }); //~ ERROR Copy


error[E0277]: the trait bound `B<C>: Clone` is not satisfied
  --> /checkout/src/test/ui/derives/deriving-copyclone.rs:44:14
   |
LL |     is_clone(B { a: 1, b: C }); //~ ERROR Clone
   |     -------- ^^^^^^^^^^^^^^^^ expected an implementor of trait `Clone`
   |     required by a bound introduced by this call
   |
note: required because of the requirements on the impl of `Clone` for `B<C>`
  --> /checkout/src/test/ui/derives/deriving-copyclone.rs:11:16
  --> /checkout/src/test/ui/derives/deriving-copyclone.rs:11:16
   |
LL | #[derive(Copy, Clone)]
   |                ^^^^^
note: required by a bound in `is_clone`
  --> /checkout/src/test/ui/derives/deriving-copyclone.rs:31:22
   |
LL | const fn is_clone<T: ~const Clone>(_: T) {}
   |                      ^^^^^^^^^^^^ required by this bound in `is_clone`
help: consider borrowing here
   |
   |
LL |     is_clone(&B { a: 1, b: C }); //~ ERROR Clone


error[E0277]: the trait bound `B<D>: Copy` is not satisfied
  --> /checkout/src/test/ui/derives/deriving-copyclone.rs:47:13
   |
LL |     is_copy(B { a: 1, b: D }); //~ ERROR Copy
   |     ------- ^^^^^^^^^^^^^^^^ expected an implementor of trait `Copy`
   |     required by a bound introduced by this call
   |
   |
note: required because of the requirements on the impl of `Copy` for `B<D>`
  --> /checkout/src/test/ui/derives/deriving-copyclone.rs:11:10
LL | #[derive(Copy, Clone)]
   |          ^^^^
note: required by a bound in `is_copy`
  --> /checkout/src/test/ui/derives/deriving-copyclone.rs:30:15
  --> /checkout/src/test/ui/derives/deriving-copyclone.rs:30:15
   |
LL | fn is_copy<T: Copy>(_: T) {}
   |               ^^^^ required by this bound in `is_copy`
   = note: this error originates in the derive macro `Copy` (in Nightly builds, run with -Z macro-backtrace for more info)
   |
   |
LL |     is_copy(&B { a: 1, b: D }); //~ ERROR Copy

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0277`.
---
test result: FAILED. 12431 passed; 1 failed; 121 ignored; 0 measured; 0 filtered out; finished in 124.91s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:22
