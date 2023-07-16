plain
.................................................................................................... 400/12554
.................................................................................................... 500/12554
....................................i............................................................... 600/12554
.................................................................................................... 700/12554
......ii..F..........................................................................i.............. 800/12554
.................................................................................................... 1000/12554
.................................................................................................... 1100/12554
.................................................................................................... 1200/12554
.............i...................................................................................... 1300/12554
---
24 LL |     x.await;
25    |       ^^^^^ unknown field
26    |
+    = note: available fields are: `pointer`
27    = note: to `.await` a `Future`, switch to Rust 2018 or later
28    = help: set `edition = "2021"` in `Cargo.toml`
29    = note: for more on editions, read https://doc.rust-lang.org/edition-guide

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-switching-edition-on-await/suggest-switching-edition-on-await.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/suggest-switching-edition-on-await.rs`

error: 1 errors occurred comparing output.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/suggest-switching-edition-on-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-switching-edition-on-await" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-switching-edition-on-await/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0609]: no field `await` on type `await_on_struct_missing::S`
   |
LL |     x.await;
   |       ^^^^^ unknown field
   |
   |
   = note: to `.await` a `Future`, switch to Rust 2018 or later
   = help: set `edition = "2021"` in `Cargo.toml`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0609]: no field `await` on type `await_on_struct_similar::S`
   |
LL |     x.await;
   |       ^^^^^ help: a field with a similar name exists: `awai`
   |
   |
   = note: to `.await` a `Future`, switch to Rust 2018 or later
   = help: set `edition = "2021"` in `Cargo.toml`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0609]: no field `await` on type `Pin<&mut dyn Future<Output = ()>>`
   |
LL |     x.await;
   |       ^^^^^ unknown field
   |
   |
   = note: available fields are: `pointer`
   = note: to `.await` a `Future`, switch to Rust 2018 or later
   = help: set `edition = "2021"` in `Cargo.toml`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0609]: no field `await` on type `impl Future<Output = ()>`
   |
LL |     x.await;
   |       ^^^^^
   |
   |
   = note: to `.await` a `Future`, switch to Rust 2018 or later
   = help: set `edition = "2021"` in `Cargo.toml`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0609`.

---
test result: FAILED. 12432 passed; 1 failed; 121 ignored; 0 measured; 0 filtered out; finished in 117.21s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:26
