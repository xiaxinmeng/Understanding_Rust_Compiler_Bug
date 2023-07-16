plain
...i..ii.i.......................................................................................... 10200/12557
...........................i........................................................................ 10300/12557
.......................iiiiii.i..iiiiii.i........................................................... 10400/12557
.................................................................................................... 10500/12557
............................................................F...........F........................... 10600/12557
.................................................................................................... 10800/12557
.................................................................................................... 10900/12557
.................................................................................................... 11000/12557
.................................................................................................... 11100/12557
---

---- [ui] ui/error-codes/E0507.rs stdout ----
diff of stderr:

- error[E0507]: cannot move out of dereference of `Ref<'_, TheDarkKnight>`
+ error[E0507]: cannot move out of dereference of `std::cell::Ref<'_, TheDarkKnight>`
2   --> $DIR/E0507.rs:12:5
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
4 LL |     x.borrow().nothing_is_true();

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0507/E0507.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0507.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0507.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0507" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0507/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0507]: cannot move out of dereference of `std::cell::Ref<'_, TheDarkKnight>`
   |
   |
LL |     x.borrow().nothing_is_true(); //~ ERROR E0507
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ move occurs because value has type `TheDarkKnight`, which does not implement the `Copy` trait
error: aborting due to previous error

For more information about this error, try `rustc --explain E0507`.

---
diff of stderr:

18    |            expected due to this
19    |
20    = note: expected type `usize`
-               found enum `Result<Value, ()>`
+               found enum `Result<unclosed_delim_mod::Value, ()>`
23 error: aborting due to 2 previous errors
24 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unclosed-delimiter-in-dep/unclosed-delimiter-in-dep.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/unclosed-delimiter-in-dep.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/unclosed-delimiter-in-dep.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unclosed-delimiter-in-dep" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unclosed-delimiter-in-dep/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: mismatched closing delimiter: `}`
  --> /checkout/src/test/ui/parser/unclosed_delim_mod.rs:5:7
   |
LL | pub fn new() -> Result<Value, ()> {
LL |     Ok(Value {
   |       ^ unclosed delimiter
LL |     }
LL | }
LL | }
   | ^ mismatched closing delimiter

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/unclosed-delimiter-in-dep.rs:4:20
   |
LL |     let _: usize = unclosed_delim_mod::new();
   |            -----   ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `usize`, found enum `Result`
   |            expected due to this
   |
   = note: expected type `usize`
   = note: expected type `usize`
              found enum `Result<unclosed_delim_mod::Value, ()>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.


------------------------------------------


---- [ui] ui/span/destructor-restrictions.rs stdout ----
diff of stderr:

7    |          borrowed value does not live long enough
8    |          a temporary with access to the borrow is created here ...
9 LL |     };
-    |     -- ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `Ref<'_, i32>`
+    |     -- ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `std::cell::Ref<'_, i32>`
11    |     |
12    |     `*a` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/destructor-restrictions/destructor-restrictions.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/destructor-restrictions/destructor-restrictions.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args span/destructor-restrictions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/destructor-restrictions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/destructor-restrictions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/destructor-restrictions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `*a` does not live long enough
   |
   |
LL |         *a.borrow() + 1
   |          |
   |          |
   |          borrowed value does not live long enough
   |          a temporary with access to the borrow is created here ...
LL |     }; //~^ ERROR `*a` does not live long enough
   |     -- ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `std::cell::Ref<'_, i32>`
   |     |
   |     `*a` dropped here while still borrowed
   = note: the temporary is part of an expression at the end of a block;
   = note: the temporary is part of an expression at the end of a block;
           consider forcing this temporary to be dropped sooner, before the block's local variables are dropped
help: for example, you could save the expression's value in a new local variable `x` and then make `x` be the expression at the end of the block
   |
LL |         let x = *a.borrow() + 1; x
   |         +++++++                +++
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.

---
diff of stderr:

10    | -
11    | |
12    | `y` dropped here while still borrowed
-    | ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `Ref<'_, String>`
+    | ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `std::cell::Ref<'_, String>`
15    = note: the temporary is part of an expression at the end of a block;
15    = note: the temporary is part of an expression at the end of a block;
16            consider forcing this temporary to be dropped sooner, before the block's local variables are dropped

28    |         borrowed value does not live long enough
29    |         a temporary with access to the borrow is created here ...
30 LL |     };
-    |     -- ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `Ref<'_, String>`
+    |     -- ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `std::cell::Ref<'_, String>`
32    |     |
33    |     `y` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-23338-locals-die-before-temps-of-body/issue-23338-locals-die-before-temps-of-body.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-23338-locals-die-before-temps-of-body/issue-23338-locals-die-before-temps-of-body.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args span/issue-23338-locals-die-before-temps-of-body.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-23338-locals-die-before-temps-of-body.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-23338-locals-die-before-temps-of-body" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-23338-locals-die-before-temps-of-body/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `y` does not live long enough
   |
   |
LL |     y.borrow().clone()
   |     |
   |     |
   |     borrowed value does not live long enough
   |     a temporary with access to the borrow is created here ...
LL | }
   | |
   | |
   | `y` dropped here while still borrowed
   | ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `std::cell::Ref<'_, String>`
   = note: the temporary is part of an expression at the end of a block;
   = note: the temporary is part of an expression at the end of a block;
           consider forcing this temporary to be dropped sooner, before the block's local variables are dropped
help: for example, you could save the expression's value in a new local variable `x` and then make `x` be the expression at the end of the block
   |
LL |     let x = y.borrow().clone(); x
   |     +++++++                   +++

error[E0597]: `y` does not live long enough
   |
   |
LL |         y.borrow().clone()
   |         |
   |         |
   |         borrowed value does not live long enough
   |         a temporary with access to the borrow is created here ...
LL |     };
   |     -- ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `std::cell::Ref<'_, String>`
   |     |
   |     `y` dropped here while still borrowed
   = note: the temporary is part of an expression at the end of a block;
   = note: the temporary is part of an expression at the end of a block;
           consider forcing this temporary to be dropped sooner, before the block's local variables are dropped
help: for example, you could save the expression's value in a new local variable `x` and then make `x` be the expression at the end of the block
   |
LL |         let x = y.borrow().clone(); x
   |         +++++++                   +++
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0597`.


------------------------------------------


---- [ui] ui/suggestions/suggest-assoc-fn-call-with-turbofish-through-deref.rs stdout ----
diff of stderr:

- error[E0599]: no method named `hello` found for struct `RefMut<'_, HasAssocMethod>` in the current scope
+ error[E0599]: no method named `hello` found for struct `std::cell::RefMut<'_, HasAssocMethod>` in the current scope
2   --> $DIR/suggest-assoc-fn-call-with-turbofish-through-deref.rs:11:11
3    |
4 LL |     state.hello();

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-assoc-fn-call-with-turbofish-through-deref/suggest-assoc-fn-call-with-turbofish-through-deref.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/suggest-assoc-fn-call-with-turbofish-through-deref.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-assoc-fn-call-with-turbofish-through-deref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-assoc-fn-call-with-turbofish-through-deref" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-assoc-fn-call-with-turbofish-through-deref/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: no method named `hello` found for struct `std::cell::RefMut<'_, HasAssocMethod>` in the current scope
   |
   |
LL |     state.hello();
   |     |     |
   |     |     this is an associated function, not a method
   |     |     this is an associated function, not a method
   |     help: use associated function syntax instead: `HasAssocMethod::hello`
   |
   = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
note: the candidate is defined in an impl for the type `HasAssocMethod`
   |
LL |     fn hello() {}
   |     ^^^^^^^^^^

---
test result: FAILED. 12433 passed; 5 failed; 119 ignored; 0 measured; 0 filtered out; finished in 144.08s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:56
