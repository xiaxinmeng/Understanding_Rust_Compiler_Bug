plain
---- [ui] ui/moves/move-fn-self-receiver.rs stdout ----
diff of stderr:

70    |
71 LL |     fn use_pin_box_self(self: Pin<Box<Self>>) {}
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
72    |                         ^^^^
+ help: consider calling `.as_mut()` to borrow the type's contents
+    |
+ LL |     pin_box_foo.as_mut().use_pin_box_self();
73 
73 
74 error[E0505]: cannot move out of `mut_foo` because it is borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-fn-self-receiver/move-fn-self-receiver.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-fn-self-receiver/move-fn-self-receiver.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args moves/move-fn-self-receiver.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/moves/move-fn-self-receiver.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-fn-self-receiver" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-fn-self-receiver/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0382]: use of moved value: `val.0`
   |
   |
LL |     val.0.into_iter().next();
   |           ----------- `val.0` moved due to this method call
LL |     val.0; //~ ERROR use of moved
   |     ^^^^^ value used here after move
   |
note: this function takes ownership of the receiver `self`, which moves `val.0`
   |
LL |     fn into_iter(self) -> Self::IntoIter;
   |                  ^^^^
   |                  ^^^^
   = note: move occurs because `val.0` has type `Vec<bool>`, which does not implement the `Copy` trait
error[E0382]: use of moved value: `foo`
  --> /checkout/src/test/ui/moves/move-fn-self-receiver.rs:34:5
   |
LL |     let foo = Foo;
LL |     let foo = Foo;
   |         --- move occurs because `foo` has type `Foo`, which does not implement the `Copy` trait
LL |     foo.use_self();
   |         ---------- `foo` moved due to this method call
LL |     foo; //~ ERROR use of moved
   |     ^^^ value used here after move
   |
note: this function takes ownership of the receiver `self`, which moves `foo`
   |
LL |     fn use_self(self) {}
   |                 ^^^^


error[E0382]: use of moved value: `second_foo`
   |
LL |     let second_foo = Foo;
LL |     let second_foo = Foo;
   |         ---------- move occurs because `second_foo` has type `Foo`, which does not implement the `Copy` trait
LL |     second_foo.use_self();
   |                ---------- `second_foo` moved due to this method call
LL |     second_foo; //~ ERROR use of moved
   |     ^^^^^^^^^^ value used here after move

error[E0382]: use of moved value: `boxed_foo`
   |
   |
LL |     let boxed_foo = Box::new(Foo);
   |         --------- move occurs because `boxed_foo` has type `Box<Foo>`, which does not implement the `Copy` trait
LL |     boxed_foo.use_box_self();
   |               -------------- `boxed_foo` moved due to this method call
LL |     boxed_foo; //~ ERROR use of moved
   |     ^^^^^^^^^ value used here after move
   |
note: this function takes ownership of the receiver `self`, which moves `boxed_foo`
   |
   |
LL |     fn use_box_self(self: Box<Self>) {}


error[E0382]: use of moved value: `pin_box_foo`
   |
   |
LL |     let pin_box_foo = Box::pin(Foo);
   |         ----------- move occurs because `pin_box_foo` has type `Pin<Box<Foo>>`, which does not implement the `Copy` trait
LL |     pin_box_foo.use_pin_box_self();
   |                 ------------------ `pin_box_foo` moved due to this method call
LL |     pin_box_foo; //~ ERROR use of moved
   |     ^^^^^^^^^^^ value used here after move
   |
note: this function takes ownership of the receiver `self`, which moves `pin_box_foo`
   |
   |
LL |     fn use_pin_box_self(self: Pin<Box<Self>>) {}
   |                         ^^^^
help: consider calling `.as_mut()` to borrow the type's contents
   |
LL |     pin_box_foo.as_mut().use_pin_box_self();


error[E0505]: cannot move out of `mut_foo` because it is borrowed
   |
   |
LL |     let ret = mut_foo.use_mut_self();
   |               ---------------------- borrow of `mut_foo` occurs here
LL |     mut_foo; //~ ERROR cannot move out
   |     ^^^^^^^ move out of `mut_foo` occurs here
LL |     ret;


error[E0382]: use of moved value: `rc_foo`
   |
   |
LL |     let rc_foo = Rc::new(Foo);
   |         ------ move occurs because `rc_foo` has type `Rc<Foo>`, which does not implement the `Copy` trait
LL |     rc_foo.use_rc_self();
   |            ------------- `rc_foo` moved due to this method call
LL |     rc_foo; //~ ERROR use of moved
   |     ^^^^^^ value used here after move
   |
note: this function takes ownership of the receiver `self`, which moves `rc_foo`
   |
   |
LL |     fn use_rc_self(self: Rc<Self>) {}


error[E0382]: use of moved value: `foo_add`
   |
LL |     let foo_add = Foo;
LL |     let foo_add = Foo;
   |         ------- move occurs because `foo_add` has type `Foo`, which does not implement the `Copy` trait
LL |     foo_add + Foo;
   |     ------------- `foo_add` moved due to usage in operator
LL |     foo_add; //~ ERROR use of moved
   |     ^^^^^^^ value used here after move
note: calling this operator moves the left-hand side
  --> /checkout/library/core/src/ops/arith.rs:114:12
   |
   |
LL |     fn add(self, rhs: Rhs) -> Self::Output;

error[E0382]: use of moved value: `implicit_into_iter`
  --> /checkout/src/test/ui/moves/move-fn-self-receiver.rs:63:5
   |
   |
LL |     let implicit_into_iter = vec![true];
   |         ------------------ move occurs because `implicit_into_iter` has type `Vec<bool>`, which does not implement the `Copy` trait
LL |     for _val in implicit_into_iter {}
   |                 |
   |                 |
   |                 `implicit_into_iter` moved due to this implicit call to `.into_iter()`
   |                 help: consider borrowing to avoid moving into the for loop: `&implicit_into_iter`
LL |     implicit_into_iter; //~ ERROR use of moved
   |     ^^^^^^^^^^^^^^^^^^ value used here after move
error[E0382]: use of moved value: `explicit_into_iter`
  --> /checkout/src/test/ui/moves/move-fn-self-receiver.rs:67:5
   |
   |
LL |     let explicit_into_iter = vec![true];
   |         ------------------ move occurs because `explicit_into_iter` has type `Vec<bool>`, which does not implement the `Copy` trait
LL |     for _val in explicit_into_iter.into_iter() {}
   |                                    ----------- `explicit_into_iter` moved due to this method call
LL |     explicit_into_iter; //~ ERROR use of moved
   |     ^^^^^^^^^^^^^^^^^^ value used here after move
error[E0382]: use of moved value: `container`
  --> /checkout/src/test/ui/moves/move-fn-self-receiver.rs:71:5
   |
   |
LL |     let container = Container(vec![]);
   |         --------- move occurs because `container` has type `Container`, which does not implement the `Copy` trait
LL |     for _val in container.custom_into_iter() {}
   |                           ------------------ `container` moved due to this method call
LL |     container; //~ ERROR use of moved
   |     ^^^^^^^^^ value used here after move
   |
note: this function takes ownership of the receiver `self`, which moves `container`
   |
   |
LL |     fn custom_into_iter(self) -> impl Iterator<Item = bool> {

error[E0382]: use of moved value: `foo2`
  --> /checkout/src/test/ui/moves/move-fn-self-receiver.rs:75:9
   |
   |
LL |     let foo2 = Foo;
   |         ---- move occurs because `foo2` has type `Foo`, which does not implement the `Copy` trait
LL |     loop {
LL |         foo2.use_self(); //~ ERROR use of moved
   |         ^^^^ ---------- `foo2` moved due to this method call, in previous iteration of loop
error: aborting due to 12 previous errors

Some errors have detailed explanations: E0382, E0505.
For more information about an error, try `rustc --explain E0382`.
---
test result: FAILED. 12446 passed; 1 failed; 121 ignored; 0 measured; 0 filtered out; finished in 117.16s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:10
