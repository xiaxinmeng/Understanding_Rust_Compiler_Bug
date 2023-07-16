plain
.................................................................................................... 1400/12373
.................................................................................................... 1500/12373
........................................................i........................................... 1600/12373
.................................................................................................... 1700/12373
...........F...F.F............................................i..................................... 1800/12373
.................................................................................................... 2000/12373
.............................i...................................................................... 2100/12373
.................................................................................................... 2200/12373
.................................................................................................... 2300/12373
---
............................................................i.ii.................................... 12300/12373
.........................................................................
failures:

---- [ui] ui/command/argfile/commandline-argfile-badutf8.rs stdout ----


- error: Failed to load argument file: Utf8 error in $DIR/commandline-argfile-badutf8.args
+ error: Failed to load argument file: IO Error: /checkout/src/test/ui/commandline-argfile-badutf8.args: No such file or directory (os error 2)
3 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/command/argfile/commandline-argfile-badutf8/commandline-argfile-badutf8.stderr
To only update this specific test, also pass `--test-args command/argfile/commandline-argfile-badutf8.rs`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error: 1 errors occurred comparing output.
error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/command/argfile/commandline-argfile-badutf8.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/command/argfile/commandline-argfile-badutf8" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--cfg" "cmdline_set" "@/checkout/src/test/ui/commandline-argfile-badutf8.args" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/command/argfile/commandline-argfile-badutf8/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: Failed to load argument file: IO Error: /checkout/src/test/ui/commandline-argfile-badutf8.args: No such file or directory (os error 2)

------------------------------------------



---- [ui] ui/command/argfile/commandline-argfile-missing.rs stdout ----


- error: Failed to load argument file: IO Error: $DIR/commandline-argfile-missing.args: $FILE_MISSING (os error $ERR)
+ error: Failed to load argument file: IO Error: /checkout/src/test/ui/commandline-argfile-missing.args: $FILE_MISSING (os error $ERR)
3 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/command/argfile/commandline-argfile-missing/commandline-argfile-missing.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args command/argfile/commandline-argfile-missing.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/command/argfile/commandline-argfile-missing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/command/argfile/commandline-argfile-missing" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--cfg" "cmdline_set" "@/checkout/src/test/ui/commandline-argfile-missing.args" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/command/argfile/commandline-argfile-missing/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: Failed to load argument file: IO Error: /checkout/src/test/ui/commandline-argfile-missing.args: No such file or directory (os error 2)

------------------------------------------



---- [ui] ui/command/argfile/commandline-argfile.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/command/argfile/commandline-argfile.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/command/argfile/commandline-argfile" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--cfg" "cmdline_set" "@/checkout/src/test/ui/commandline-argfile.args" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/command/argfile/commandline-argfile/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: Failed to load argument file: IO Error: /checkout/src/test/ui/commandline-argfile.args: No such file or directory (os error 2)

------------------------------------------



---- [ui] ui/traits/issue-77982.rs stdout ----
diff of stderr:

49            - impl From<NonZeroU32> for u32;
50            - impl From<bool> for u32;
51            - impl From<char> for u32;
-            and 3 more
+            and 7 more
53 note: required by `from`
54   --> $SRC_DIR/core/src/convert/mod.rs:LL:COL


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982/issue-77982.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982/issue-77982.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/issue-77982.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-77982.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:8:10
   |
LL |     opts.get(opt.as_ref()); //~ ERROR type annotations needed
   |          ^^^ ------------ this method call resolves to `&T`
   |          |
   |          cannot infer type for type parameter `Q` declared on the associated function `get`
   |
   = note: multiple `impl`s satisfying `String: Borrow<_>` found in the following crates: `alloc`, `core`:
           - impl Borrow<str> for String;
           - impl<T> Borrow<T> for T
             where T: ?Sized;
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:8:18
   |
   |
LL |     opts.get(opt.as_ref()); //~ ERROR type annotations needed
   |              |   |
   |              |   |
   |              |   cannot infer type for type parameter `T` declared on the trait `AsRef`
   |              this method call resolves to `&T`
   |
   = note: multiple `impl`s satisfying `String: AsRef<_>` found in the following crates: `alloc`, `std`:
           - impl AsRef<OsStr> for String;
           - impl AsRef<Path> for String;
           - impl AsRef<[u8]> for String;
           - impl AsRef<str> for String;
help: use the fully qualified path for the potential candidates
   |
LL |     opts.get(<String as AsRef<OsStr>>::as_ref(opt)); //~ ERROR type annotations needed
   |              ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
LL |     opts.get(<String as AsRef<Path>>::as_ref(opt)); //~ ERROR type annotations needed
   |              ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
LL |     opts.get(<String as AsRef<[u8]>>::as_ref(opt)); //~ ERROR type annotations needed
   |              ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
LL |     opts.get(<String as AsRef<str>>::as_ref(opt)); //~ ERROR type annotations needed

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:13:44
   |
   |
LL |     let ips: Vec<_> = (0..100_000).map(|_| u32::from(0u32.into())).collect();
   |                                            ^^^^^^^^^ ----------- this method call resolves to `T`
   |                                            |
   |                                            cannot infer type for type parameter `T` declared on the trait `From`
   |
   = note: multiple `impl`s satisfying `u32: From<_>` found in the following crates: `core`, `std`:
           - impl From<Ipv4Addr> for u32;
           - impl From<NonZeroU32> for u32;
           - impl From<bool> for u32;
           - impl From<char> for u32;
           and 7 more
note: required by `from`
   |
   |
LL |     fn from(_: T) -> Self;

error[E0283]: type annotations needed for `Box<T>`
  --> /checkout/src/test/ui/traits/issue-77982.rs:36:16
   |
   |
LL |     let _ = ().foo(); //~ ERROR type annotations needed
   |         -      ^^^ cannot infer type for type parameter `T` declared on the trait `Foo`
   |         |
   |         consider giving this pattern the explicit type `Box<T>`, where the type parameter `T` is specified
   |
note: multiple `impl`s satisfying `(): Foo<'_, _>` found
   |
   |
LL | impl Foo<'static, u32> for () {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | impl<'a> Foo<'a, i16> for () {}

error[E0283]: type annotations needed for `Box<T>`
  --> /checkout/src/test/ui/traits/issue-77982.rs:40:19
   |
   |
LL |     let _ = (&()).bar(); //~ ERROR type annotations needed
   |         -         ^^^ cannot infer type for type parameter `T` declared on the trait `Bar`
   |         |
   |         consider giving this pattern the explicit type `Box<T>`, where the type parameter `T` is specified
   |
note: multiple `impl`s satisfying `&(): Bar<'_, _>` found
   |
   |
LL | impl<'a> Bar<'static, u32> for &'a () {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | impl<'a> Bar<'a, i16> for &'a () {}

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0283`.
---
---- [ui] ui/try-trait/bad-interconversion.rs stdout ----
diff of stderr:

8    |
9    = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
10    = help: the following implementations were found:
+              <u8 as From<&bool>>
+              <u8 as From<&u8>>
11              <u8 as From<NonZeroU8>>
12              <u8 as From<bool>>
13    = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, i32>>` for `Result<u64, u8>`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/bad-interconversion/bad-interconversion.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args try-trait/bad-interconversion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-trait/bad-interconversion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/bad-interconversion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/bad-interconversion/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `?` couldn't convert the error to `u8`
   |
   |
LL | fn result_to_result() -> Result<u64, u8> {
   |                          --------------- expected `u8` because of this
LL |     Ok(Err(123_i32)?)
   |                    ^ the trait `From<i32>` is not implemented for `u8`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: the following implementations were found:
             <u8 as From<&bool>>
             <u8 as From<&u8>>
             <u8 as From<NonZeroU8>>
             <u8 as From<bool>>
   = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, i32>>` for `Result<u64, u8>`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


error[E0277]: the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`
   |
   |
LL | / fn option_to_result() -> Result<u64, String> {
LL | |     Some(3)?;
   | |            ^ use `.ok_or(...)?` to provide an error compatible with `Result<u64, String>`
LL | |     //~^ ERROR the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`
LL | |     Ok(10)
LL | | }
   | |_- this function returns a `Result`
   |
   = help: the trait `FromResidual<Option<Infallible>>` is not implemented for `Result<u64, String>`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


error[E0277]: the `?` operator can only be used on `Result`s in a function that returns `Result`
   |
   |
LL | / fn control_flow_to_result() -> Result<u64, String> {
LL | |     Ok(ControlFlow::Break(123)?)
   | |                               ^ this `?` produces `ControlFlow<{integer}, Infallible>`, which is incompatible with `Result<u64, String>`
LL | |     //~^ ERROR the `?` operator can only be used on `Result`s in a function that returns `Result`
LL | | }
   | |_- this function returns a `Result`
   |
   = help: the trait `FromResidual<ControlFlow<{integer}, Infallible>>` is not implemented for `Result<u64, String>`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


error[E0277]: the `?` operator can only be used on `Option`s, not `Result`s, in a function that returns `Option`
   |
LL | / fn result_to_option() -> Option<u16> {
LL | / fn result_to_option() -> Option<u16> {
LL | |     Some(Err("hello")?)
   | |                      ^ use `.ok()?` if you want to discard the `Result<Infallible, &str>` error information
LL | |     //~^ ERROR the `?` operator can only be used on `Option`s, not `Result`s, in a function that returns `Option`
LL | | }
   | |_- this function returns an `Option`
   |
   = help: the trait `FromResidual<Result<Infallible, &str>>` is not implemented for `Option<u16>`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


error[E0277]: the `?` operator can only be used on `Option`s in a function that returns `Option`
   |
LL | / fn control_flow_to_option() -> Option<u64> {
LL | / fn control_flow_to_option() -> Option<u64> {
LL | |     Some(ControlFlow::Break(123)?)
   | |                                 ^ this `?` produces `ControlFlow<{integer}, Infallible>`, which is incompatible with `Option<u64>`
LL | |     //~^ ERROR the `?` operator can only be used on `Option`s in a function that returns `Option`
LL | | }
   | |_- this function returns an `Option`
   |
   = help: the trait `FromResidual<ControlFlow<{integer}, Infallible>>` is not implemented for `Option<u64>`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


error[E0277]: the `?` operator can only be used on `ControlFlow`s in a function that returns `ControlFlow`
   |
   |
LL | / fn result_to_control_flow() -> ControlFlow<String> {
LL | |     ControlFlow::Continue(Err("hello")?)
   | |                                       ^ this `?` produces `Result<Infallible, &str>`, which is incompatible with `ControlFlow<String>`
LL | |     //~^ ERROR the `?` operator can only be used on `ControlFlow`s in a function that returns `ControlFlow`
LL | | }
   | |_- this function returns a `ControlFlow`
   |
   = help: the trait `FromResidual<Result<Infallible, &str>>` is not implemented for `ControlFlow<String>`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


error[E0277]: the `?` operator can only be used on `ControlFlow`s in a function that returns `ControlFlow`
   |
   |
LL | / fn option_to_control_flow() -> ControlFlow<u64> {
LL | |     Some(3)?;
   | |            ^ this `?` produces `Option<Infallible>`, which is incompatible with `ControlFlow<u64>`
LL | |     //~^ ERROR the `?` operator can only be used on `ControlFlow`s in a function that returns `ControlFlow`
LL | |     ControlFlow::Break(10)
LL | | }
   | |_- this function returns a `ControlFlow`
   |
   = help: the trait `FromResidual<Option<Infallible>>` is not implemented for `ControlFlow<u64>`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;


error[E0277]: the `?` operator in a function that returns `ControlFlow<B, _>` can only be used on other `ControlFlow<B, _>`s (with the same Break type)
   |
   |
LL | / fn control_flow_to_control_flow() -> ControlFlow<i64> {
LL | |     ControlFlow::Break(4_u8)?;
   | |                             ^ this `?` produces `ControlFlow<u8, Infallible>`, which is incompatible with `ControlFlow<i64>`
LL | |     //~^ ERROR the `?` operator in a function that returns `ControlFlow<B, _>` can only be used on other `ControlFlow<B, _>`s
LL | |     ControlFlow::Continue(())
LL | | }
   | |_- this function returns a `ControlFlow`
   |
   = help: the trait `FromResidual<ControlFlow<u8, Infallible>>` is not implemented for `ControlFlow<i64>`
   = note: unlike `Result`, there's no `From`-conversion performed for `ControlFlow`
note: required by `from_residual`
   |
   |
LL |     fn from_residual(residual: R) -> Self;

error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0277`.
---
test result: FAILED. 12258 passed; 5 failed; 110 ignored; 0 measured; 0 filtered out; finished in 131.93s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:34
