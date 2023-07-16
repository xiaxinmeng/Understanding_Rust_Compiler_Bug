plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +ab9da90ed4ba41ceb256e79694e644038a2cce75:refs/remotes/pull/79790/merge
---
.................................................................................................... 9000/11170
.................................................................................................... 9100/11170
..............................................................i......i.............................. 9200/11170
.................................................................................................... 9300/11170
.iiiiii..iiiiii.i................................................................................... 9400/11170
.................................................................................................... 9600/11170
.................................................................................................... 9700/11170
.................................................................................................... 9800/11170
.................................................................................................... 9900/11170
---
..............................................................i..................................... 11100/11170
......................................................................
failures:

---- [ui] ui/traits/negative-impls/explicitly-unimplemented-error-message.rs stdout ----


16    |        the method is available for `Rc<Qux>` here
18    = help: items from traits can only be used if the trait is implemented and in scope
18    = help: items from traits can only be used if the trait is implemented and in scope
-    = note: `Clone` defines an item `clone`, but is explicitely unimplemented
20 
21 error[E0599]: no method named `bar` found for type `u32` in the current scope
22   --> $DIR/explicitly-unimplemented-error-message.rs:39:11
25    |           ^^^ method not found in `u32`
26    |
27    = help: items from traits can only be used if the trait is implemented and in scope
27    = help: items from traits can only be used if the trait is implemented and in scope
-    = note: `Bar` defines an item `bar`, but is explicitely unimplemented
29 
30 error[E0599]: no method named `foo` found for struct `Qux` in the current scope
31   --> $DIR/explicitly-unimplemented-error-message.rs:44:9
37    |         ^^^ method not found in `Qux`
38    |
39    = help: items from traits can only be used if the trait is implemented and in scope
39    = help: items from traits can only be used if the trait is implemented and in scope
-    = note: the following traits define an item `foo`, but are explicitely unimplemented:
-            Foo
-            FooBar
44 error[E0599]: no method named `foo` found for type `u32` in the current scope
45   --> $DIR/explicitly-unimplemented-error-message.rs:49:11

53    |
53    |
54 LL | trait Foo {
55    | ^^^^^^^^^
-    = note: `FooBar` defines an item `foo`, but is explicitely unimplemented
+    = note: the trait `FooBar` defines an item `foo`, but is explicitely unimplemented
58 error: aborting due to 4 previous errors
59 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/negative-impls/explicitly-unimplemented-error-message/explicitly-unimplemented-error-message.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/negative-impls/explicitly-unimplemented-error-message.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/negative-impls/explicitly-unimplemented-error-message.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/negative-impls/explicitly-unimplemented-error-message" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/negative-impls/explicitly-unimplemented-error-message/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: no method named `clone` found for struct `Qux` in the current scope
  --> /checkout/src/test/ui/traits/negative-impls/explicitly-unimplemented-error-message.rs:34:9
   |
LL | struct Qux;
   | ----------- method `clone` not found for this
...
LL |     Qux.clone();
   |         ^^^^^ method not found in `Qux`
  ::: /checkout/library/core/src/clone.rs:119:8
   |
   |
LL |     fn clone(&self) -> Self;
   |        |
   |        |
   |        the method is available for `Arc<Qux>` here
   |        the method is available for `Rc<Qux>` here
   = help: items from traits can only be used if the trait is implemented and in scope


error[E0599]: no method named `bar` found for type `u32` in the current scope
  --> /checkout/src/test/ui/traits/negative-impls/explicitly-unimplemented-error-message.rs:39:11
   |
LL |     0_u32.bar();
   |           ^^^ method not found in `u32`
   = help: items from traits can only be used if the trait is implemented and in scope


error[E0599]: no method named `foo` found for struct `Qux` in the current scope
  --> /checkout/src/test/ui/traits/negative-impls/explicitly-unimplemented-error-message.rs:44:9
   |
LL | struct Qux;
   | ----------- method `foo` not found for this
...
LL |     Qux.foo();
   |         ^^^ method not found in `Qux`
   = help: items from traits can only be used if the trait is implemented and in scope

error[E0599]: no method named `foo` found for type `u32` in the current scope
  --> /checkout/src/test/ui/traits/negative-impls/explicitly-unimplemented-error-message.rs:49:11
  --> /checkout/src/test/ui/traits/negative-impls/explicitly-unimplemented-error-message.rs:49:11
   |
LL |     0_u32.foo();
   |           ^^^ method not found in `u32`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `Foo` defines an item `foo`, perhaps you need to implement it
  --> /checkout/src/test/ui/traits/negative-impls/explicitly-unimplemented-error-message.rs:18:1
LL | trait Foo {
   | ^^^^^^^^^
   | ^^^^^^^^^
   = note: the trait `FooBar` defines an item `foo`, but is explicitely unimplemented
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0599`.

---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:55
