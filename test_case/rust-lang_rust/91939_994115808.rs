plain
---- [ui] ui/consts/const-eval/const-eval-overflow-4b.rs stdout ----
diff of stderr:

17    |
18 LL |     : [u32; 5i8 as char as usize]
19    |             ^^^^^^^^^^^ invalid cast
+    |
+ help: try `char::from_u32` instead
+    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ LL |     : [u32; 5i8 as char as usize]
20 
21 error: aborting due to 3 previous errors
22 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-4b/const-eval-overflow-4b.stderr
To only update this specific test, also pass `--test-args consts/const-eval/const-eval-overflow-4b.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-eval-overflow-4b.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-4b" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-overflow-4b/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/consts/const-eval/const-eval-overflow-4b.rs:9:30
   |
LL |     : [u32; (i8::MAX as i8 + 1u8) as usize]
   |                              ^^^ expected `i8`, found `u8`

error[E0277]: cannot add `u8` to `i8`
   |
   |
LL |     : [u32; (i8::MAX as i8 + 1u8) as usize]
   |                            ^ no implementation for `i8 + u8`
   = help: the trait `Add<u8>` is not implemented for `i8`


error[E0604]: only `u8` can be cast as `char`, not `i8`
   |
   |
LL |     : [u32; 5i8 as char as usize]
   |             ^^^^^^^^^^^ invalid cast
   |
help: try `char::from_u32` instead
   |
   |
LL |     : [u32; 5i8 as char as usize]

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0308, E0604.
---
3    |
4 LL |     1u32 as char;
5    |     ^^^^^^^^^^^^ invalid cast
+    |
+ help: try `char::from_u32` instead
+   --> $DIR/E0604.rs:2:5
+    |
+ LL |     1u32 as char;
6 
7 error: aborting due to previous error
8 

---
To only update this specific test, also pass `--test-args error-codes/E0604.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0604.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0604" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0604/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0604]: only `u8` can be cast as `char`, not `u32`
   |
   |
LL |     1u32 as char; //~ ERROR E0604
   |     ^^^^^^^^^^^^ invalid cast
   |
help: try `char::from_u32` instead
   |
   |
LL |     1u32 as char; //~ ERROR E0604

error: aborting due to previous error

For more information about this error, try `rustc --explain E0604`.
---
58    |
59 LL |     0u32 as char;
60    |     ^^^^^^^^^^^^ invalid cast
+    |
+ help: try `char::from_u32` instead
+    |
+    |
+ LL |     0u32 as char;
61 
61 
62 error[E0605]: non-primitive cast: `u8` as `Vec<u8>`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-festival/error-festival.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-festival/error-festival.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-festival.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-festival.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-festival" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-festival/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   |
LL |     const FOO: u32 = 0;
   |     ^^^^^^^^^^^^^^^^^^^

error[E0368]: binary assignment operation `+=` cannot be applied to type `&str`
   |
LL |     x += 2;
   |     -^^^^^
   |     |
   |     |
   |     cannot use `+=` on type `&str`

error[E0599]: no method named `z` found for reference `&str` in the current scope
   |
LL |     x.z();
   |       ^ method not found in `&str`


error[E0600]: cannot apply unary operator `!` to type `Question`
   |
   |
LL |     !Question::Yes;
   |     ^^^^^^^^^^^^^^ cannot apply unary operator `!`
   |
note: an implementation of `Not` might be missing for `Question`
   |
   |
LL | enum Question {
   | ^^^^^^^^^^^^^ must implement `Not`
  --> /checkout/library/core/src/ops/bit.rs:34:1
   |
   |
LL | / pub trait Not {
LL | |     /// The resulting type after applying the `!` operator.
LL | |     #[stable(feature = "rust1", since = "1.0.0")]
LL | |     type Output;
...  |
LL | |     fn not(self) -> Self::Output;
LL | | }


error[E0604]: only `u8` can be cast as `char`, not `u32`
   |
LL |     0u32 as char;
   |     ^^^^^^^^^^^^ invalid cast
   |
   |
help: try `char::from_u32` instead
   |
LL |     0u32 as char;
   |     ^^^^^^^^^^^^


error[E0605]: non-primitive cast: `u8` as `Vec<u8>`
   |
LL |     x as Vec<u8>;
LL |     x as Vec<u8>;
   |     ^^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object
error[E0054]: cannot cast as `bool`
  --> /checkout/src/test/ui/error-festival.rs:33:24
   |
LL |     let x_is_nonzero = x as bool;
LL |     let x_is_nonzero = x as bool;
   |                        ^^^^^^^^^ help: compare with zero instead: `x != 0`

error[E0606]: casting `&u8` as `u32` is invalid
   |
   |
LL |     let y: u32 = x as u32;
   |                  -^^^^^^^
   |                  |
   |                  cannot cast `&u8` as `u32`
   |                  help: dereference the expression: `*x`

error[E0607]: cannot cast thin pointer `*const u8` to fat pointer `*const [u8]`
   |
LL |     v as *const [u8];
   |     ^^^^^^^^^^^^^^^^

---
---- [ui] ui/mismatched_types/cast-rfc0401.rs stdout ----
diff of stderr:

99    |
100 LL |     let _ = 0x61u32 as char;
101    |             ^^^^^^^^^^^^^^^ invalid cast
+    |
+ help: try `char::from_u32` instead
+    |
+    |
+ LL |     let _ = 0x61u32 as char;
102 
102 
103 error[E0606]: casting `bool` as `f32` is invalid


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/cast-rfc0401/cast-rfc0401.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/cast-rfc0401/cast-rfc0401.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mismatched_types/cast-rfc0401.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/cast-rfc0401.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/cast-rfc0401" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/cast-rfc0401/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0606]: casting `*const U` as `*const V` is invalid
   |
   |
LL |     u as *const V //~ ERROR is invalid
   |
   |
   = note: vtable kinds may not match

error[E0606]: casting `*const U` as `*const str` is invalid
   |
   |
LL |     u as *const str //~ ERROR is invalid
   |
   |
   = note: vtable kinds may not match

error[E0609]: no field `f` on type `fn() {main}`
   |
   |
LL |     let _ = main.f as *const u32; //~ ERROR no field


error[E0605]: non-primitive cast: `*const u8` as `&u8`
   |
   |
LL |     let _ = v as &u8; //~ ERROR non-primitive cast
   |             ^^^^^^^^ invalid cast
help: consider borrowing the value
   |
   |
LL -     let _ = v as &u8; //~ ERROR non-primitive cast
LL +     let _ = &*v; //~ ERROR non-primitive cast


error[E0605]: non-primitive cast: `*const u8` as `E`
   |
   |
LL |     let _ = v as E; //~ ERROR non-primitive cast
   |             ^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object

error[E0605]: non-primitive cast: `*const u8` as `fn()`
   |
   |
LL |     let _ = v as fn(); //~ ERROR non-primitive cast
   |             ^^^^^^^^^ invalid cast

error[E0605]: non-primitive cast: `*const u8` as `(u32,)`
   |
   |
LL |     let _ = v as (u32,); //~ ERROR non-primitive cast
   |             ^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object

error[E0605]: non-primitive cast: `Option<&*const u8>` as `*const u8`
   |
   |
LL |     let _ = Some(&v) as *const u8; //~ ERROR non-primitive cast
   |             ^^^^^^^^^^^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object

error[E0606]: casting `*const u8` as `f32` is invalid
   |
   |
LL |     let _ = v as f32; //~ ERROR is invalid


error[E0606]: casting `fn() {main}` as `f64` is invalid
   |
   |
LL |     let _ = main as f64; //~ ERROR is invalid


error[E0606]: casting `&*const u8` as `usize` is invalid
   |
   |
LL |     let _ = &v as usize; //~ ERROR is invalid
   |
   |
   = help: cast through a raw pointer first

error[E0606]: casting `f32` as `*const u8` is invalid
   |
   |
LL |     let _ = f as *const u8; //~ ERROR is invalid

error[E0054]: cannot cast as `bool`
  --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:39:13
   |
   |
LL |     let _ = 3_i32 as bool; //~ ERROR cannot cast
   |             ^^^^^^^^^^^^^ help: compare with zero instead: `3_i32 != 0`
error[E0054]: cannot cast as `bool`
  --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:40:13
   |
   |
LL |     let _ = E::A as bool; //~ ERROR cannot cast
   |             ^^^^^^^^^^^^ unsupported cast

error[E0604]: only `u8` can be cast as `char`, not `u32`
   |
   |
LL |     let _ = 0x61u32 as char; //~ ERROR can be cast as
   |             ^^^^^^^^^^^^^^^ invalid cast
   |
help: try `char::from_u32` instead
   |
   |
LL |     let _ = 0x61u32 as char; //~ ERROR can be cast as


error[E0606]: casting `bool` as `f32` is invalid
   |
   |
LL |     let _ = false as f32; //~ ERROR is invalid
   |
   = help: cast through an integer first


error[E0606]: casting `E` as `f32` is invalid
   |
   |
LL |     let _ = E::A as f32; //~ ERROR is invalid
   |
   = help: cast through an integer first


error[E0606]: casting `char` as `f32` is invalid
   |
   |
LL |     let _ = 'a' as f32; //~ ERROR is invalid
   |
   = help: cast through an integer first


error[E0606]: casting `bool` as `*const u8` is invalid
   |
   |
LL |     let _ = false as *const u8; //~ ERROR is invalid


error[E0606]: casting `E` as `*const u8` is invalid
   |
   |
LL |     let _ = E::A as *const u8; //~ ERROR is invalid


error[E0606]: casting `char` as `*const u8` is invalid
   |
   |
LL |     let _ = 'a' as *const u8; //~ ERROR is invalid


error[E0606]: casting `usize` as `*const [u8]` is invalid
   |
   |
LL |     let _ = 42usize as *const [u8]; //~ ERROR is invalid


error[E0607]: cannot cast thin pointer `*const u8` to fat pointer `*const [u8]`
   |
   |
LL |     let _ = v as *const [u8]; //~ ERROR cannot cast


error[E0606]: casting `&dyn Foo` as `*const str` is invalid
   |
   |
LL |     let _ = foo as *const str; //~ ERROR is invalid


error[E0606]: casting `&dyn Foo` as `*mut str` is invalid
   |
   |
LL |     let _ = foo as *mut str; //~ ERROR is invalid


error[E0606]: casting `fn() {main}` as `*mut str` is invalid
   |
   |
LL |     let _ = main as *mut str; //~ ERROR is invalid


error[E0606]: casting `&f32` as `*mut f32` is invalid
   |
   |
LL |     let _ = &f as *mut f32; //~ ERROR is invalid


error[E0606]: casting `&f32` as `*const f64` is invalid
   |
   |
LL |     let _ = &f as *const f64; //~ ERROR is invalid


error[E0606]: casting `*const [i8]` as `usize` is invalid
   |
   |
LL |     let _ = fat_sv as usize; //~ ERROR is invalid
   |
   = help: cast through a thin pointer first


error[E0606]: casting `*const dyn Foo` as `*const [u16]` is invalid
   |
   |
LL |     let _ = cf as *const [u16]; //~ ERROR is invalid
   |
   |
   = note: vtable kinds may not match

error[E0606]: casting `*const dyn Foo` as `*const dyn Bar` is invalid
   |
   |
LL |     let _ = cf as *const dyn Bar; //~ ERROR is invalid
   |
   |
   = note: vtable kinds may not match
error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
  --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:53:13
   |
   |
LL |     let _ = fat_v as *const dyn Foo; //~ ERROR the size for values of type
   |
   = help: the trait `Sized` is not implemented for `[u8]`
   = note: required for the cast to the object type `dyn Foo`


error[E0277]: the size for values of type `str` cannot be known at compilation time
  --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:62:13
   |
LL |     let _ = a as *const dyn Foo; //~ ERROR the size for values of type
   |
   = help: the trait `Sized` is not implemented for `str`
   = note: required for the cast to the object type `dyn Foo`


error[E0606]: casting `&{float}` as `f32` is invalid
   |
   |
LL |     vec![0.0].iter().map(|s| s as f32).collect::<Vec<f32>>(); //~ ERROR is invalid
   |                              -^^^^^^^
   |                              |
   |                              cannot cast `&{float}` as `f32`
   |                              help: dereference the expression: `*s`
error: aborting due to 34 previous errors

Some errors have detailed explanations: E0054, E0277, E0604, E0605, E0606, E0607, E0609.
For more information about an error, try `rustc --explain E0054`.
---
test result: FAILED. 12358 passed; 4 failed; 119 ignored; 0 measured; 0 filtered out; finished in 116.02s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:10:53
