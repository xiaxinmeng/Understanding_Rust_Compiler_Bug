plain

error: 1 errors occurred comparing output.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-fn-path-elision.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-path-elision" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-path-elision/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0726]: implicit elided lifetime not allowed here
   |
   |
LL | async fn error(lt: HasLifetime) { //~ ERROR implicit elided lifetime not allowed here
   |                    ^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
   |
   = note: assuming a `'static` lifetime...
error: aborting due to previous error

For more information about this error, try `rustc --explain E0726`.

---
To only update this specific test, also pass `--test-args impl-header-lifetime-elision/trait-elided.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-header-lifetime-elision/trait-elided.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-header-lifetime-elision/trait-elided" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-header-lifetime-elision/trait-elided/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0726]: implicit elided lifetime not allowed here
   |
   |
LL | impl MyTrait for u32 {
   |      ^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
   |
   = note: assuming a `'static` lifetime...
error: aborting due to previous error

For more information about this error, try `rustc --explain E0726`.

---
To only update this specific test, also pass `--test-args impl-header-lifetime-elision/path-elided.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-header-lifetime-elision/path-elided.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-header-lifetime-elision/path-elided" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-header-lifetime-elision/path-elided/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0726]: implicit elided lifetime not allowed here
   |
   |
LL | impl MyTrait for Foo {
   |                  ^^^- help: indicate the anonymous lifetime: `<'_>`
   |
   = note: assuming a `'static` lifetime...
error: aborting due to previous error

For more information about this error, try `rustc --explain E0726`.

---
To only update this specific test, also pass `--test-args issues/issue-10412.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-10412.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10412" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10412/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetimes cannot use keyword names
  --> /checkout/src/test/ui/issues/issue-10412.rs:1:20
   |
LL | trait Serializable<'self, T> { //~ ERROR lifetimes cannot use keyword names

error: lifetimes cannot use keyword names
  --> /checkout/src/test/ui/issues/issue-10412.rs:2:25
   |
   |
LL |     fn serialize(val : &'self T) -> Vec<u8>; //~ ERROR lifetimes cannot use keyword names

error: lifetimes cannot use keyword names
  --> /checkout/src/test/ui/issues/issue-10412.rs:3:38
   |
   |
LL |     fn deserialize(repr : &[u8]) -> &'self T; //~ ERROR lifetimes cannot use keyword names

error: lifetimes cannot use keyword names
  --> /checkout/src/test/ui/issues/issue-10412.rs:6:6
   |
   |
LL | impl<'self> Serializable<str> for &'self str { //~ ERROR lifetimes cannot use keyword names

error: lifetimes cannot use keyword names
  --> /checkout/src/test/ui/issues/issue-10412.rs:6:36
   |
   |
LL | impl<'self> Serializable<str> for &'self str { //~ ERROR lifetimes cannot use keyword names

error: lifetimes cannot use keyword names
  --> /checkout/src/test/ui/issues/issue-10412.rs:10:25
   |
   |
LL |     fn serialize(val : &'self str) -> Vec<u8> { //~ ERROR lifetimes cannot use keyword names

error: lifetimes cannot use keyword names
  --> /checkout/src/test/ui/issues/issue-10412.rs:13:37
   |
   |
LL |     fn deserialize(repr: &[u8]) -> &'self str { //~ ERROR lifetimes cannot use keyword names


error[E0726]: implicit elided lifetime not allowed here
   |
   |
LL | impl<'self> Serializable<str> for &'self str { //~ ERROR lifetimes cannot use keyword names
   |             ^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `Serializable<'_, str>`
   |
   = note: assuming a `'static` lifetime...
error[E0277]: the size for values of type `str` cannot be known at compilation time
  --> /checkout/src/test/ui/issues/issue-10412.rs:6:13
   |
   |
LL | trait Serializable<'self, T> { //~ ERROR lifetimes cannot use keyword names
   |                           - required by this bound in `Serializable`
...
LL | impl<'self> Serializable<str> for &'self str { //~ ERROR lifetimes cannot use keyword names
   |
   = help: the trait `Sized` is not implemented for `str`
   = help: the trait `Sized` is not implemented for `str`
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Serializable<'self, T: ?Sized> { //~ ERROR lifetimes cannot use keyword names

error: aborting due to 9 previous errors

Some errors have detailed explanations: E0277, E0726.
---

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-in-foreign-fn-decls-issue-80468/wf-in-foreign-fn-decls-issue-80468.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args wf/wf-in-foreign-fn-decls-issue-80468.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/wf/wf-in-foreign-fn-decls-issue-80468.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-in-foreign-fn-decls-issue-80468" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-in-foreign-fn-decls-issue-80468/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0726]: implicit elided lifetime not allowed here
  --> /checkout/src/test/ui/wf/wf-in-foreign-fn-decls-issue-80468.rs:13:16
   |
LL | impl Trait for Ref {} //~ ERROR:  implicit elided lifetime not allowed here
   |                ^^^- help: indicate the anonymous lifetime: `<'_>`
   |
   = note: assuming a `'static` lifetime...
error: incompatible lifetime on type
  --> /checkout/src/test/ui/wf/wf-in-foreign-fn-decls-issue-80468.rs:16:21
   |
   |
LL |     pub fn repro(_: Wrapper<Ref>); //~ ERROR: incompatible lifetime on type
   |
   |
note: because this has an unmet lifetime requirement
  --> /checkout/src/test/ui/wf/wf-in-foreign-fn-decls-issue-80468.rs:8:23
   |
LL | pub struct Wrapper<T: Trait>(T);
   |                       ^^^^^ introduces a `'static` lifetime requirement
note: the anonymous lifetime #1 defined on the method body at 16:5...
  --> /checkout/src/test/ui/wf/wf-in-foreign-fn-decls-issue-80468.rs:16:5
   |
LL |     pub fn repro(_: Wrapper<Ref>); //~ ERROR: incompatible lifetime on type
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...does not necessarily outlive the static lifetime introduced by the compatible `impl`
  --> /checkout/src/test/ui/wf/wf-in-foreign-fn-decls-issue-80468.rs:13:1
   |
LL | impl Trait for Ref {} //~ ERROR:  implicit elided lifetime not allowed here

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0726`.
---
test result: FAILED. 11980 passed; 5 failed; 103 ignored; 0 measured; 0 filtered out; finished in 107.21s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:10:59
