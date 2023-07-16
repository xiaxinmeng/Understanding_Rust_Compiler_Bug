plain
To only update this specific test, also pass `--test-args suggestions/impl-on-dyn-trait-with-implicit-static-bound-needing-more-suggestions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/impl-on-dyn-trait-with-implicit-static-bound-needing-more-suggestions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/impl-on-dyn-trait-with-implicit-static-bound-needing-more-suggestions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/impl-on-dyn-trait-with-implicit-static-bound-needing-more-suggestions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `val` does not live long enough
   |
   |
LL |     fn use_it<'a>(val: Box<dyn ObjectTrait<Assoc = i32>>) -> impl OtherTrait<'a> {
   |               -- lifetime `'a` defined here                  ------------------- opaque type requires that `val` is borrowed for `'a`
LL |         val.use_self() //~ ERROR E0597
   |         ^^^ borrowed value does not live long enough
LL |     }
   |     - `val` dropped here while still borrowed
   |
help: you can add a bound to the opaque type to make it last less than `'static` and match `'a`
   |
LL |     fn use_it<'a>(val: Box<dyn ObjectTrait<Assoc = i32>>) -> impl OtherTrait<'a> + 'a {

error[E0515]: cannot return value referencing function parameter `val`
  --> /checkout/src/test/ui/suggestions/impl-on-dyn-trait-with-implicit-static-bound-needing-more-suggestions.rs:43:9
   |
   |
LL |         val.use_self() //~ ERROR E0515
   |         ---^^^^^^^^^^^
   |         returns a value referencing data owned by the current function
   |         returns a value referencing data owned by the current function
   |         `val` is borrowed here
error[E0515]: cannot return value referencing function parameter `val`
  --> /checkout/src/test/ui/suggestions/impl-on-dyn-trait-with-implicit-static-bound-needing-more-suggestions.rs:109:9
   |
   |
LL |         val.use_self() //~ ERROR E0515
   |         ---^^^^^^^^^^^
   |         returns a value referencing data owned by the current function
   |         returns a value referencing data owned by the current function
   |         `val` is borrowed here

error[E0772]: `val` has lifetime `'a` but calling `use_self` introduces an implicit `'static` lifetime requirement
   |
   |
LL |     fn use_it<'a>(val: Box<dyn ObjectTrait<Assoc = i32> + 'a>) -> &'a () {
   |                        -------------------------------------- this data with lifetime `'a`...
LL |         val.use_self() //~ ERROR E0772
   |             ^^^^^^^^ ...is captured and required to live as long as `'static` here
   |
note: the used `impl` has a `'static` requirement
   |
   |
LL |     impl MyTrait for Box<dyn ObjectTrait<Assoc = i32>> {
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^ this has an implicit `'static` lifetime requirement
LL |         fn use_self(&self) -> &() { panic!() }
   |            -------- calling this method introduces the `impl`'s 'static` requirement
help: consider relaxing the implicit `'static` requirement
   |
LL |     impl MyTrait for Box<dyn ObjectTrait<Assoc = i32> + '_> {

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0515, E0597, E0772.
---
To only update this specific test, also pass `--test-args suggestions/impl-on-dyn-trait-with-implicit-static-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/impl-on-dyn-trait-with-implicit-static-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/impl-on-dyn-trait-with-implicit-static-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/impl-on-dyn-trait-with-implicit-static-bound/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0759]: `val` has lifetime `'a` but it needs to satisfy a `'static` lifetime requirement
   |
   |
LL |     fn use_it<'a, T>(val: &'a dyn ObjectTrait<T>) -> impl OtherTrait<'a> + 'a {
   |                           ---------------------- this data with lifetime `'a`...
LL |         val.use_self::<T>() //~ ERROR E0759
   |             ^^^^^^^^ ...is captured and required to live as long as `'static` here
   |
note: the used `impl` has a `'static` requirement
   |
   |
LL |     impl<T> MyTrait<T> for dyn ObjectTrait<T> {
   |                                ^^^^^^^^^^^^^^ this has an implicit `'static` lifetime requirement
LL |         fn use_self<K>(&self) -> &() { panic!() }
   |            -------- calling this method introduces the `impl`'s 'static` requirement
help: consider relaxing the implicit `'static` requirement
   |
LL |     impl<T> MyTrait<T> for dyn ObjectTrait<T> + '_ {


error[E0772]: `val` has lifetime `'a` but calling `use_self` introduces an implicit `'static` lifetime requirement
   |
   |
LL |     fn use_it<'a>(val: &'a dyn ObjectTrait) -> impl OtherTrait<'a> + 'a {
   |                        ------------------- this data with lifetime `'a`...
LL |         val.use_self() //~ ERROR E0772
   |             ^^^^^^^^ ...is captured and required to live as long as `'static` here because of an implicit lifetime bound on the inherent `impl`
   |
note: the used `impl` has a `'static` requirement
   |
   |
LL |     impl dyn ObjectTrait {
   |              ^^^^^^^^^^^ this has an implicit `'static` lifetime requirement
LL |         fn use_self(&self) -> &() { panic!() }
   |            -------- calling this method introduces the `impl`'s 'static` requirement
help: consider relaxing the implicit `'static` requirement
   |
LL |     impl dyn ObjectTrait + '_ {


error[E0759]: `val` has lifetime `'a` but it needs to satisfy a `'static` lifetime requirement
   |
   |
LL |     fn use_it<'a>(val: &'a dyn ObjectTrait) -> impl OtherTrait<'a> {
   |                        ------------------- this data with lifetime `'a`...
LL |         val.use_self() //~ ERROR E0759
   |             ^^^^^^^^ ...is captured and required to live as long as `'static` here
   |
note: the used `impl` has a `'static` requirement
   |
   |
LL |         fn use_self(&self) -> &() { panic!() }
   |            -------- calling this method introduces the `impl`'s 'static` requirement
...
LL |     impl MyTrait for dyn ObjectTrait {}
   |                          ^^^^^^^^^^^ this has an implicit `'static` lifetime requirement
help: consider relaxing the implicit `'static` requirement
   |
LL |     impl MyTrait for dyn ObjectTrait + '_ {}
   |                                      ++++
help: to declare that the `impl Trait` captures data from argument `val`, you can add an explicit `'a` lifetime bound
   |
LL |     fn use_it<'a>(val: &'a dyn ObjectTrait) -> impl OtherTrait<'a> + 'a {


error[E0759]: `val` has lifetime `'a` but it needs to satisfy a `'static` lifetime requirement
   |
   |
LL |     fn use_it<'a>(val: &'a dyn ObjectTrait) -> impl OtherTrait<'a> + 'a {
   |                        ------------------- this data with lifetime `'a`...
LL |         MyTrait::use_self(val) //~ ERROR E0759
   |                           ^^^ ...is captured here...
   |
note: ...and is required to live as long as `'static` here
   |
   |
LL |         MyTrait::use_self(val) //~ ERROR E0759
   |         ^^^^^^^^^^^^^^^^^
note: the used `impl` has a `'static` requirement
   |
   |
LL |         fn use_self(&self) -> &() { panic!() }
   |            -------- calling this method introduces the `impl`'s 'static` requirement
...
LL |     impl MyTrait for dyn ObjectTrait {}
   |                          ^^^^^^^^^^^ this has an implicit `'static` lifetime requirement
help: consider relaxing the implicit `'static` requirement
   |
LL |     impl MyTrait for dyn ObjectTrait + '_ {}


error[E0772]: `val` has lifetime `'a` but calling `use_self` introduces an implicit `'static` lifetime requirement
   |
   |
LL |     fn use_it<'a>(val: &'a dyn ObjectTrait) -> &'a () {
   |                        ------------------- this data with lifetime `'a`...
LL |         val.use_self() //~ ERROR E0772
   |             ^^^^^^^^ ...is captured and required to live as long as `'static` here
   |
note: the used `impl` has a `'static` requirement
   |
   |
LL |     impl MyTrait for dyn ObjectTrait {
   |                          ^^^^^^^^^^^ this has an implicit `'static` lifetime requirement
LL |         fn use_self(&self) -> &() { panic!() }
   |            -------- calling this method introduces the `impl`'s 'static` requirement
help: consider relaxing the implicit `'static` requirement
   |
LL |     impl MyTrait for dyn ObjectTrait + '_ {


error[E0772]: `val` has lifetime `'a` but calling `use_self` introduces an implicit `'static` lifetime requirement
   |
   |
LL |     fn use_it<'a>(val: &'a Box<dyn ObjectTrait + 'a>) -> &'a () {
   |                        ----------------------------- this data with lifetime `'a`...
LL |         val.use_self() //~ ERROR E0772
   |             ^^^^^^^^ ...is captured and required to live as long as `'static` here
   |
note: the used `impl` has a `'static` requirement
   |
   |
LL |     impl MyTrait for Box<dyn ObjectTrait> {
   |                              ^^^^^^^^^^^ this has an implicit `'static` lifetime requirement
LL |         fn use_self(&self) -> &() { panic!() }
   |            -------- calling this method introduces the `impl`'s 'static` requirement
help: consider relaxing the implicit `'static` requirement
   |
LL |     impl MyTrait for Box<dyn ObjectTrait + '_> {

error: aborting due to 6 previous errors

Some errors have detailed explanations: E0759, E0772.
---
test result: FAILED. 12053 passed; 2 failed; 101 ignored; 0 measured; 0 filtered out; finished in 124.51s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:11:47
