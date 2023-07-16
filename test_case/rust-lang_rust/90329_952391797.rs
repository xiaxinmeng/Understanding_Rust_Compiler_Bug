plain
...................................................................i................................ 11600/12347
.................................................................................................... 11700/12347
.................................................................................................... 11800/12347
.................................................................................................... 11900/12347
.................F.F...........................................F.................................... 12000/12347
.................................................................................................... 12200/12347
..................................i.ii.............................................................. 12300/12347
...............................................
failures:
failures:

---- [ui] ui/derives/derive-assoc-type-not-impl.rs stdout ----
diff of stderr:

19    = help: items from traits can only be used if the trait is implemented and in scope
20    = note: the following trait defines an item `clone`, perhaps you need to implement it:
21            candidate #1: `Clone`
- help: consider annotating `NotClone` with `#[derive(Clone)]`
+ help: consider annotating `NotClone` with `#[derive(Clone, Clone)]`
23    |
- LL | #[derive(Clone)]
+ LL | #[derive(Clone, Clone)]
26 
27 error: aborting due to previous error


---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derive-assoc-type-not-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derive-assoc-type-not-impl" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derive-assoc-type-not-impl/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: the method `clone` exists for struct `Bar<NotClone>`, but its trait bounds were not satisfied
  --> /checkout/src/test/ui/derives/derive-assoc-type-not-impl.rs:18:30
   |
LL | struct Bar<T: Foo> {
   | |
   | |
   | method `clone` not found for this
   | doesn't satisfy `Bar<NotClone>: Clone`
...
LL | struct NotClone;
   | ---------------- doesn't satisfy `NotClone: Clone`
...
LL |     Bar::<NotClone> { x: 1 }.clone(); //~ ERROR
   |                              ^^^^^ method cannot be called on `Bar<NotClone>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `NotClone: Clone`
           which is required by `Bar<NotClone>: Clone`
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `clone`, perhaps you need to implement it:
           candidate #1: `Clone`
help: consider annotating `NotClone` with `#[derive(Clone, Clone)]`
   |
LL | #[derive(Clone, Clone)]

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
---
diff of stderr:

9    |
10    = note: the following trait bounds were not satisfied:
11            `Foo: Debug`
- help: consider annotating `Foo` with `#[derive(Debug)]`
+ help: consider annotating `Foo` with `#[derive(Debug, Debug)]`
- LL | #[derive(Debug)]
+ LL | #[derive(Debug, Debug)]
15    |
16 
---
To only update this specific test, also pass `--test-args mismatched_types/method-help-unsatisfied-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/method-help-unsatisfied-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/method-help-unsatisfied-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/method-help-unsatisfied-bound/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: the method `unwrap` exists for enum `Result<(), Foo>`, but its trait bounds were not satisfied
   |
LL | struct Foo;
   | ----------- doesn't satisfy `Foo: Debug`
...
...
LL |     a.unwrap();
   |       ^^^^^^ method cannot be called on `Result<(), Foo>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
           `Foo: Debug`
           `Foo: Debug`
help: consider annotating `Foo` with `#[derive(Debug, Debug)]`
LL | #[derive(Debug, Debug)]
   |

error: aborting due to previous error
---

31 LL | |     fn default() -> Self;
32 LL | | }
33    | |_^
- help: consider annotating `Enum` with `#[derive(Clone)]`
+ help: consider annotating `Enum` with `#[derive(Clone, Clone)]`
35    |
- LL | #[derive(Clone)]
+ LL | #[derive(Clone, Clone)]
38 
38 
39 error[E0599]: the method `test` exists for struct `Foo<Struct, CloneStruct>`, but its trait bounds were not satisfied
58            `Struct: Clone`
59            `Struct: Default`
59            `Struct: Default`
60            `CloneStruct: Default`
- help: consider annotating `CloneStruct` with `#[derive(Default)]`
+ help: consider annotating `CloneStruct` with `#[derive(Default, Default)]`
- LL | #[derive(Default)]
+ LL | #[derive(Default, Default)]
64    |
64    |
- help: consider annotating `Struct` with `#[derive(Clone, Default)]`
+ help: consider annotating `Struct` with `#[derive(Clone, Clone, Default, Default)]`
- LL | #[derive(Clone, Default)]
- LL | #[derive(Clone, Default)]
+ LL | #[derive(Clone, Clone, Default, Default)]
69 
69 
70 error[E0599]: the method `test` exists for struct `Foo<Vec<Enum>, Instant>`, but its trait bounds were not satisfied

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/derive-trait-for-method-call/derive-trait-for-method-call.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/derive-trait-for-method-call.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/derive-trait-for-method-call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/derive-trait-for-method-call" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/derive-trait-for-method-call/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: the method `test` exists for struct `Foo<Enum, CloneEnum>`, but its trait bounds were not satisfied
   |
LL | enum Enum {
   | ---------
   | |
   | |
   | doesn't satisfy `Enum: Clone`
   | doesn't satisfy `Enum: Default`
...
LL | enum CloneEnum {
   | -------------- doesn't satisfy `CloneEnum: Default`
...
LL | struct Foo<X, Y> (X, Y);
   | ------------------------ method `test` not found for this
...
LL |     let y = x.test();
   |               ^^^^ method cannot be called on `Foo<Enum, CloneEnum>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
           `Enum: Clone`
           `Enum: Default`
           `Enum: Default`
           `CloneEnum: Default`
  --> /checkout/library/core/src/default.rs:85:1
   |
   |
LL | / pub trait Default: Sized {
LL | |     /// Returns the "default value" for a type.
LL | |     ///
LL | |     /// Default values are often some kind of initial value, identity value, or anything else that
LL | |     fn default() -> Self;
LL | | }
   | |_^
   | |_^
help: consider annotating `Enum` with `#[derive(Clone, Clone)]`
   |
LL | #[derive(Clone, Clone)]


error[E0599]: the method `test` exists for struct `Foo<Struct, CloneStruct>`, but its trait bounds were not satisfied
   |
LL | struct Struct {
   | -------------
   | |
   | |
   | doesn't satisfy `Struct: Clone`
   | doesn't satisfy `Struct: Default`
...
LL | struct CloneStruct {
   | ------------------ doesn't satisfy `CloneStruct: Default`
...
LL | struct Foo<X, Y> (X, Y);
   | ------------------------ method `test` not found for this
...
LL |     let y = x.test();
   |               ^^^^ method cannot be called on `Foo<Struct, CloneStruct>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
           `Struct: Clone`
           `Struct: Default`
           `Struct: Default`
           `CloneStruct: Default`
help: consider annotating `CloneStruct` with `#[derive(Default, Default)]`
LL | #[derive(Default, Default)]
   |
   |
help: consider annotating `Struct` with `#[derive(Clone, Clone, Default, Default)]`
   |
LL | #[derive(Clone, Clone, Default, Default)]


error[E0599]: the method `test` exists for struct `Foo<Vec<Enum>, Instant>`, but its trait bounds were not satisfied
   |
   |
LL | struct Foo<X, Y> (X, Y);
   | ------------------------ method `test` not found for this
...
LL |     let y = x.test();
   |               ^^^^ method cannot be called on `Foo<Vec<Enum>, Instant>` due to unsatisfied trait bounds
  ::: /checkout/library/std/src/time.rs:130:1
   |
LL | pub struct Instant(time::Instant);
LL | pub struct Instant(time::Instant);
   | ---------------------------------- doesn't satisfy `Instant: Default`
  ::: /checkout/library/alloc/src/vec/mod.rs:400:1
   |
   |
LL | pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
   | ------------------------------------------------------------------------------------------------ doesn't satisfy `Vec<Enum>: Clone`
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `Vec<Enum>: Clone`
           `Instant: Default`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0599`.


------------------------------------------


---- [ui] ui/union/union-derive-clone.rs#thirunsafeck stdout ----
diff of stderr:

16    = note: the following trait bounds were not satisfied:
17            `CloneNoCopy: Copy`
18            which is required by `U5<CloneNoCopy>: Clone`
- help: consider annotating `CloneNoCopy` with `#[derive(Copy)]`
+ help: consider annotating `CloneNoCopy` with `#[derive(Copy, Copy)]`
20    |
- LL | #[derive(Copy)]
+ LL | #[derive(Copy, Copy)]
23 
23 
24 error[E0277]: the trait bound `U1: Copy` is not satisfied

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-clone.thirunsafeck/union-derive-clone.thirunsafeck.stderr
To only update this specific test, also pass `--test-args union/union-derive-clone.rs`


error in revision `thirunsafeck`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-derive-clone.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thirunsafeck" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-clone.thirunsafeck" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "thir-unsafeck" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-clone.thirunsafeck/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: the method `clone` exists for union `U5<CloneNoCopy>`, but its trait bounds were not satisfied
   |
   |
LL | union U5<T> {
   | |
   | |
   | method `clone` not found for this
   | doesn't satisfy `U5<CloneNoCopy>: Clone`
...
LL | struct CloneNoCopy;
   | ------------------- doesn't satisfy `CloneNoCopy: Copy`
...
LL |     let w = u.clone(); //~ ERROR the method
   |               ^^^^^ method cannot be called on `U5<CloneNoCopy>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `CloneNoCopy: Copy`
           which is required by `U5<CloneNoCopy>: Clone`
help: consider annotating `CloneNoCopy` with `#[derive(Copy, Copy)]`
   |
LL | #[derive(Copy, Copy)]


error[E0277]: the trait bound `U1: Copy` is not satisfied
   |
   |
LL | #[derive(Clone)] //~ ERROR the trait bound `U1: Copy` is not satisfied
   |          ^^^^^ the trait `Copy` is not implemented for `U1`
   |
note: required by a bound in `AssertParamIsCopy`
   |
   |
LL | pub struct AssertParamIsCopy<T: Copy + ?Sized> {
   |                                 ^^^^ required by this bound in `AssertParamIsCopy`
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0599.
For more information about an error, try `rustc --explain E0277`.
---
---- [ui] ui/union/union-derive-clone.rs#mirunsafeck stdout ----
diff of stderr:

16    = note: the following trait bounds were not satisfied:
17            `CloneNoCopy: Copy`
18            which is required by `U5<CloneNoCopy>: Clone`
- help: consider annotating `CloneNoCopy` with `#[derive(Copy)]`
+ help: consider annotating `CloneNoCopy` with `#[derive(Copy, Copy)]`
20    |
- LL | #[derive(Copy)]
+ LL | #[derive(Copy, Copy)]
23 
23 
24 error[E0277]: the trait bound `U1: Copy` is not satisfied

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-clone.mirunsafeck/union-derive-clone.mirunsafeck.stderr
To only update this specific test, also pass `--test-args union/union-derive-clone.rs`


error in revision `mirunsafeck`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-derive-clone.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mirunsafeck" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-clone.mirunsafeck" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-clone.mirunsafeck/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: the method `clone` exists for union `U5<CloneNoCopy>`, but its trait bounds were not satisfied
   |
   |
LL | union U5<T> {
   | |
   | |
   | method `clone` not found for this
   | doesn't satisfy `U5<CloneNoCopy>: Clone`
...
LL | struct CloneNoCopy;
   | ------------------- doesn't satisfy `CloneNoCopy: Copy`
...
LL |     let w = u.clone(); //~ ERROR the method
   |               ^^^^^ method cannot be called on `U5<CloneNoCopy>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `CloneNoCopy: Copy`
           which is required by `U5<CloneNoCopy>: Clone`
help: consider annotating `CloneNoCopy` with `#[derive(Copy, Copy)]`
   |
LL | #[derive(Copy, Copy)]


error[E0277]: the trait bound `U1: Copy` is not satisfied
   |
   |
LL | #[derive(Clone)] //~ ERROR the trait bound `U1: Copy` is not satisfied
   |          ^^^^^ the trait `Copy` is not implemented for `U1`
   |
note: required by a bound in `AssertParamIsCopy`
   |
   |
LL | pub struct AssertParamIsCopy<T: Copy + ?Sized> {
   |                                 ^^^^ required by this bound in `AssertParamIsCopy`
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0599.
For more information about an error, try `rustc --explain E0277`.
---
---- [ui] ui/unique-pinned-nocopy.rs stdout ----
diff of stderr:

21    = help: items from traits can only be used if the trait is implemented and in scope
22    = note: the following trait defines an item `clone`, perhaps you need to implement it:
23            candidate #1: `Clone`
- help: consider annotating `R` with `#[derive(Clone)]`
+ help: consider annotating `R` with `#[derive(Clone, Clone)]`
25    |
- LL | #[derive(Clone)]
+ LL | #[derive(Clone, Clone)]
28 
29 error: aborting due to previous error



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unique-pinned-nocopy/unique-pinned-nocopy.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unique-pinned-nocopy.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unique-pinned-nocopy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unique-pinned-nocopy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unique-pinned-nocopy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: the method `clone` exists for struct `Box<R>`, but its trait bounds were not satisfied
   |
LL |   struct R {
LL |   struct R {
   |   -------- doesn't satisfy `R: Clone`
...
LL |       let _j = i.clone(); //~ ERROR the method
   |                  ^^^^^ method cannot be called on `Box<R>` due to unsatisfied trait bounds
  ::: /checkout/library/alloc/src/boxed.rs:172:1
   |
LL | / pub struct Box<
LL | / pub struct Box<
LL | |     T: ?Sized,
LL | |     #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
LL | | >(Unique<T>, A);
   | |________________- doesn't satisfy `Box<R>: Clone`
   = note: the following trait bounds were not satisfied:
           `R: Clone`
           `R: Clone`
           which is required by `Box<R>: Clone`
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `clone`, perhaps you need to implement it:
           candidate #1: `Clone`
help: consider annotating `R` with `#[derive(Clone, Clone)]`
   |
LL | #[derive(Clone, Clone)]

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
---
test result: FAILED. 12231 passed; 6 failed; 110 ignored; 0 measured; 0 filtered out; finished in 136.67s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:40
