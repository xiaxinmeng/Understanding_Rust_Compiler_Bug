plain

87    |
88 help: consider removing the borrow
89    |
- LL |         asm!("{}", const 0);
-    |                         --
+ LL -         asm!("{}", const &0);
+ LL +         asm!("{}", const 0);
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
92 
93 error[E0308]: mismatched types
94   --> $DIR/type-check-1.rs:62:25



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-1/type-check-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/type-check-1.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/type-check-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/type-check-1.rs:34:26
   |
LL |         let x = 0;
   |         ----- help: consider using `const` instead of `let`: `const x`
...
LL |         asm!("{}", const x);
   |                          ^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/type-check-1.rs:37:36
   |
LL |         let x = 0;
LL |         let x = 0;
   |         ----- help: consider using `const` instead of `let`: `const x`
...
LL |         asm!("{}", const const_foo(x));
   |                                    ^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/type-check-1.rs:40:36
   |
LL |         let x = 0;
LL |         let x = 0;
   |         ----- help: consider using `const` instead of `let`: `const x`
...
LL |         asm!("{}", const const_bar(x));
   |                                    ^ non-constant value
error: invalid asm output
  --> /checkout/src/test/ui/asm/type-check-1.rs:10:29
   |
   |
LL |         asm!("{}", out(reg) 1 + 2);
   |                             ^^^^^ cannot assign to this expression
error: invalid asm output
  --> /checkout/src/test/ui/asm/type-check-1.rs:12:31
   |
   |
LL |         asm!("{}", inout(reg) 1 + 2);
   |                               ^^^^^ cannot assign to this expression
error[E0277]: the size for values of type `[u64]` cannot be known at compilation time
  --> /checkout/src/test/ui/asm/type-check-1.rs:18:28
   |
   |
LL |         asm!("{}", in(reg) v[..]);
   |
   = help: the trait `Sized` is not implemented for `[u64]`
   = help: the trait `Sized` is not implemented for `[u64]`
   = note: all inline asm arguments must have a statically known size
error[E0277]: the size for values of type `[u64]` cannot be known at compilation time
  --> /checkout/src/test/ui/asm/type-check-1.rs:20:29
   |
   |
LL |         asm!("{}", out(reg) v[..]);
   |
   = help: the trait `Sized` is not implemented for `[u64]`
   = help: the trait `Sized` is not implemented for `[u64]`
   = note: all inline asm arguments must have a statically known size
error[E0277]: the size for values of type `[u64]` cannot be known at compilation time
  --> /checkout/src/test/ui/asm/type-check-1.rs:22:31
   |
   |
LL |         asm!("{}", inout(reg) v[..]);
   |
   = help: the trait `Sized` is not implemented for `[u64]`
   = help: the trait `Sized` is not implemented for `[u64]`
   = note: all inline asm arguments must have a statically known size
error[E0308]: mismatched types
  --> /checkout/src/test/ui/asm/type-check-1.rs:48:26
   |
   |
LL |         asm!("{}", const 0f32);
   |                          ^^^^ expected integer, found `f32`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/asm/type-check-1.rs:50:26
   |
   |
LL |         asm!("{}", const 0 as *mut u8);
   |                          ^^^^^^^^^^^^ expected integer, found *-ptr
   |
   = note:     expected type `{integer}`
           found raw pointer `*mut u8`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/asm/type-check-1.rs:52:26
   |
   |
LL |         asm!("{}", const &0);
   |                          ^^ expected integer, found `&{integer}`
help: consider removing the borrow
   |
   |
LL -         asm!("{}", const &0);
LL +         asm!("{}", const 0);

error[E0308]: mismatched types
  --> /checkout/src/test/ui/asm/type-check-1.rs:62:25
   |
   |
LL | global_asm!("{}", const 0f32);
   |                         ^^^^ expected integer, found `f32`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/asm/type-check-1.rs:64:25
   |
   |
LL | global_asm!("{}", const 0 as *mut u8);
   |                         ^^^^^^^^^^^^ expected integer, found *-ptr
   |
   = note:     expected type `{integer}`
           found raw pointer `*mut u8`
error: aborting due to 13 previous errors

Some errors have detailed explanations: E0277, E0308, E0435.
For more information about an error, try `rustc --explain E0277`.
---
test result: FAILED. 12036 passed; 1 failed; 111 ignored; 0 measured; 0 filtered out; finished in 113.59s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:11:11
