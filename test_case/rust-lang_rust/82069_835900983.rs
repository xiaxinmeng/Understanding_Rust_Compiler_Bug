plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.164 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 3.61s

 finished in 3.683 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 67 tests
Some tests failed in compiletest suite=ui-fulldeps mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...F......F..........................................F.............

---- [ui] ui-fulldeps/hash-stable-is-unstable.rs stdout ----
diff of stderr:


42    |
43    = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
44    = help: add `#![feature(rustc_private)]` to the crate attributes to enable
-    = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the derive macro `HashStable` (in Nightly builds, run with -Z macro-backtrace for more info)
47 error: aborting due to 5 previous errors
48 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/hash-stable-is-unstable.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hash-stable-is-unstable.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/hash-stable-is-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/hash-stable-is-unstable/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
   |
LL | extern crate rustc_data_structures;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(rustc_private)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
   |
LL | extern crate rustc_middle;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(rustc_private)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
   |
LL | extern crate rustc_macros;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(rustc_private)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
   |
LL | use rustc_macros::HashStable;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(rustc_private)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?
   |
   |
LL | #[derive(HashStable)]
   |
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = note: see issue #27812 <https://github.com/rust-lang/rust/issues/27812> for more information
   = help: add `#![feature(rustc_private)]` to the crate attributes to enable
   = note: this error originates in the derive macro `HashStable` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0658`.

---
diff of stderr:

21    | -------------------------- in this macro invocation
22    |
23    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
-    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `custom_lint_pass_macro` (in Nightly builds, run with -Z macro-backtrace for more info)
26 error: aborting due to 2 previous errors
27 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/lint_pass_impl_without_macro.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args internal-lints/lint_pass_impl_without_macro.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: implementing `LintPass` by hand
   |
   |
LL | impl LintPass for Foo { //~ERROR implementing `LintPass` by hand
   |
note: the lint level is defined here
  --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:4:9
   |
   |
LL | #![deny(rustc::lint_pass_impl_without_macro)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead

error: implementing `LintPass` by hand
   |
   |
LL |         impl LintPass for Custom { //~ERROR implementing `LintPass` by hand
...
...
LL | custom_lint_pass_macro!();
   |
   |
   = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
   = note: this error originates in the macro `custom_lint_pass_macro` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui-fulldeps/session-derive-errors.rs stdout ----
diff of stderr:

62    | ^ expected `'}'` in format string
63    |
64    = note: if you intended to print `{`, you can escape it using `{{`
-    = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the derive macro `SessionDiagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)
66 
67 error: invalid format string: unmatched `}` found
68   --> $DIR/session-derive-errors.rs:119:1

71    | ^ unmatched `}` in format string
72    |
73    = note: if you intended to print `}`, you can escape it using `}}`
-    = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the derive macro `SessionDiagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)
75 
76 error: The `#[label = ...]` attribute can only be applied to fields of type Span
77   --> $DIR/session-derive-errors.rs:138:5

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-derive-errors/session-derive-errors.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args session-derive-errors.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/session-derive-errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-derive-errors" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-derive-errors/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `#[derive(SessionDiagnostic)]` can only be used on structs
   |
   |
LL | / #[error = "E0123"]
LL | | //~^ ERROR `#[derive(SessionDiagnostic)]` can only be used on structs
LL | | enum SessionDiagnosticOnEnum {
LL | |     Foo,
LL | |     Bar,
LL | | }


error: `#[label = ...]` is not a valid SessionDiagnostic struct attribute
   |
   |
LL | #[label = "This is in the wrong place"]


error: `#[suggestion = ...]` is not a valid SessionDiagnostic field attribute
   |
   |
LL |     #[suggestion = "this is the wrong kind of attribute"]


error: `error` specified multiple times
   |
   |
LL | #[error = "E0456"] //~ ERROR `error` specified multiple times


error: `lint` specified when `error` was already specified
   |
   |
LL | #[lint = "some_useful_lint"] //~ ERROR `lint` specified when `error` was already specified


error: `code` not specified
   |
   |
LL | struct ErrorCodeNotProvided {} //~ ERROR `code` not specified
   |
   |
   = help: use the [code = "..."] attribute to set this diagnostic's error code 

error: the `#[message = "..."]` attribute can only be applied to fields of type Span
   |
   |
LL |     #[message = "this message is applied to a String field"]


error: `name` doesn't refer to a field on this type
   |
   |
LL | #[message = "This error has a field, and references {name}"]


error: invalid format string: expected `'}'` but string was terminated
   |
LL | #[error = "E0123"]
   |               - because of this opening brace
   |               - because of this opening brace
LL | #[message = "This is missing a closing brace: {name"]
   | ^ expected `'}'` in format string
   |
   = note: if you intended to print `{`, you can escape it using `{{`
   = note: this error originates in the derive macro `SessionDiagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)

error: invalid format string: unmatched `}` found
   |
   |
LL | #[message = "This is missing an opening brace: name}"]
   | ^ unmatched `}` in format string
   |
   = note: if you intended to print `}`, you can escape it using `}}`
   = note: this error originates in the derive macro `SessionDiagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)

error: The `#[label = ...]` attribute can only be applied to fields of type Span
   |
   |
LL |     #[label = "See here"]


error: `nonsense` is not a valid key for `#[suggestion(...)]`
   |
   |
LL |     #[suggestion(nonsense = "This is nonsense")]


error: `msg` is not a valid key for `#[suggestion(...)]`
   |
   |
LL |     #[suggestion(msg = "This is a suggestion")]

error: missing suggestion message
  --> /checkout/src/test/ui-fulldeps/session-derive-errors.rs:179:7
   |
   |
LL |     #[suggestion(code = "This is suggested code")]
   |
   |
   = help: provide a suggestion message using #[suggestion(message = "...")]
error: wrong field type for suggestion
  --> /checkout/src/test/ui-fulldeps/session-derive-errors.rs:194:5
   |
   |
LL | /     #[suggestion(message = "This is a message", code = "This is suggested code")]
LL | |     //~^ ERROR wrong field type for suggestion
LL | |     suggestion: Applicability,
   |
   |
   = help: #[suggestion(...)] should be applied to fields of type Span or (Span, Applicability)

error: type of field annotated with `#[suggestion(...)]` contains more than one Span
   |
   |
LL | /     #[suggestion(message = "This is a message", code = "This is suggested code")]
LL | |     //~^ ERROR type of field annotated with `#[suggestion(...)]` contains more than one Span
LL | |     suggestion: (Span, Span, Applicability),


error: type of field annotated with `#[suggestion(...)]` contains more than one Applicability
   |
   |
LL | /     #[suggestion(message = "This is a message", code = "This is suggested code")]
LL | |     //~^ ERROR type of field annotated with `#[suggestion(...)]` contains more than one
LL | |     suggestion: (Applicability, Applicability, Span),


error: invalid annotation list `#[label(...)]`
   |
   |
LL |     #[label("wrong kind of annotation for label")]

error: aborting due to 18 previous errors


---
test result: FAILED. 64 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 12.41s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui-fulldeps" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:19
