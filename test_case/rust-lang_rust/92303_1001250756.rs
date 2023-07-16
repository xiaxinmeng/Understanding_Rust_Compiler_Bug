plain
normalized stderr:
warning: trait objects without an explicit `dyn` are deprecated
  --> $DIR/issue-26186.rs:13:19
   |
LL |     type Target = FnMut() + 'a;
   |
   = note: `#[warn(bare_trait_objects)]` on by default
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
LL -     type Target = FnMut() + 'a;
LL +     type Target = dyn FnMut() + 'a;

warning: trait objects without an explicit `dyn` are deprecated
  --> $DIR/issue-26186.rs:37:19
   |
   |
LL |     type Target = FnMut() + 'static;
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     type Target = FnMut() + 'static;
LL +     type Target = dyn FnMut() + 'static;

warning: trait objects without an explicit `dyn` are deprecated
  --> $DIR/issue-26186.rs:56:23
   |
   |
LL |     let a: Rc<RefCell<FnMut()>> = Rc::new(RefCell::new(||{}));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     let a: Rc<RefCell<FnMut()>> = Rc::new(RefCell::new(||{}));
LL +     let a: Rc<RefCell<dyn FnMut()>> = Rc::new(RefCell::new(||{}));


warning: unused variable: `rgba_icon`
   |
   |
LL |         let rgba_icon = (*get_icon)();
   |             ^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_rgba_icon`
   = note: `#[warn(unused_variables)]` on by default


warning: struct is never constructed: `FunctionIcon`
   |
LL | struct FunctionIcon {
   |        ^^^^^^^^^^^^
   |
   |
   = note: `#[warn(dead_code)]` on by default

warning: associated function is never used: `get_icon`
   |
   |
LL |     fn get_icon(&self) -> impl '_ + std::ops::DerefMut<Target=Box<dyn FnMut() -> u32>> {


warning: associated function is never used: `load_icon`
   |
   |
LL |     fn load_icon(&self)  {

warning: trait objects without an explicit `dyn` are deprecated
  --> $DIR/issue-26186.rs:13:19
   |
   |
LL |     type Target = FnMut() + 'a;
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     type Target = FnMut() + 'a;
LL +     type Target = dyn FnMut() + 'a;

warning: trait objects without an explicit `dyn` are deprecated
  --> $DIR/issue-26186.rs:37:19
   |
   |
LL |     type Target = FnMut() + 'static;
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     type Target = FnMut() + 'static;
LL +     type Target = dyn FnMut() + 'static;

warning: 9 warnings emitted


---
To only update this specific test, also pass `--test-args issues/issue-26186.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-26186.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26186/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26186/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/issues/issue-26186.rs:13:19
   |
LL |     type Target = FnMut() + 'a;
   |
   = note: `#[warn(bare_trait_objects)]` on by default
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
   |
LL -     type Target = FnMut() + 'a;
LL +     type Target = dyn FnMut() + 'a;

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/issues/issue-26186.rs:37:19
   |
   |
LL |     type Target = FnMut() + 'static;
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     type Target = FnMut() + 'static;
LL +     type Target = dyn FnMut() + 'static;

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/issues/issue-26186.rs:56:23
   |
   |
LL |     let a: Rc<RefCell<FnMut()>> = Rc::new(RefCell::new(||{}));
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     let a: Rc<RefCell<FnMut()>> = Rc::new(RefCell::new(||{}));
LL +     let a: Rc<RefCell<dyn FnMut()>> = Rc::new(RefCell::new(||{}));


warning: unused variable: `rgba_icon`
   |
   |
LL |         let rgba_icon = (*get_icon)();
   |             ^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_rgba_icon`
   = note: `#[warn(unused_variables)]` on by default


warning: struct is never constructed: `FunctionIcon`
   |
LL | struct FunctionIcon {
   |        ^^^^^^^^^^^^
   |
   |
   = note: `#[warn(dead_code)]` on by default

warning: associated function is never used: `get_icon`
   |
   |
LL |     fn get_icon(&self) -> impl '_ + std::ops::DerefMut<Target=Box<dyn FnMut() -> u32>> {


warning: associated function is never used: `load_icon`
   |
   |
LL |     fn load_icon(&self)  {

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/issues/issue-26186.rs:13:19
   |
   |
LL |     type Target = FnMut() + 'a;
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     type Target = FnMut() + 'a;
LL +     type Target = dyn FnMut() + 'a;

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/issues/issue-26186.rs:37:19
   |
   |
LL |     type Target = FnMut() + 'static;
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     type Target = FnMut() + 'static;
LL +     type Target = dyn FnMut() + 'static;

warning: 9 warnings emitted


---
test result: FAILED. 12397 passed; 1 failed; 119 ignored; 0 measured; 0 filtered out; finished in 158.68s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:54
