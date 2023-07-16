plain

---- [ui] ui-fulldeps/session-derive-errors.rs stdout ----
diff of stderr:

1 error: `#[derive(SessionDiagnostic)]` can only be used on structs
-   --> $DIR/session-derive-errors.rs:28:1
+   --> $DIR/session-derive-errors.rs:29:1
3    |
4 LL | / #[error = "E0123"]
5 LL | |
10    | |_^
11 
11 
12 error: `#[label = ...]` is not a valid SessionDiagnostic struct attribute
-   --> $DIR/session-derive-errors.rs:37:1
+   --> $DIR/session-derive-errors.rs:38:1
14    |
15 LL | #[label = "This is in the wrong place"]

17 
17 
18 error: `#[suggestion = ...]` is not a valid SessionDiagnostic field attribute
-   --> $DIR/session-derive-errors.rs:44:5
+   --> $DIR/session-derive-errors.rs:45:5
20    |
21 LL |     #[suggestion = "this is the wrong kind of attribute"]

23 
23 
24 error: `error` specified multiple times
-   --> $DIR/session-derive-errors.rs:52:11
+   --> $DIR/session-derive-errors.rs:53:11
27 LL | #[error = "E0456"]
28    |           ^^^^^^^

29 
29 
30 error: `lint` specified when `error` was already specified
-   --> $DIR/session-derive-errors.rs:58:10
+   --> $DIR/session-derive-errors.rs:59:10
32    |
33 LL | #[lint = "some_useful_lint"]

35 
35 
36 error: `code` not specified
-   --> $DIR/session-derive-errors.rs:67:1
+   --> $DIR/session-derive-errors.rs:68:1
38    |
39 LL | struct ErrorCodeNotProvided {}


42    = help: use the [code = "..."] attribute to set this diagnostic's error code 
43 
44 error: the `#[message = "..."]` attribute can only be applied to fields of type Span
-   --> $DIR/session-derive-errors.rs:95:5
+   --> $DIR/session-derive-errors.rs:96:5
46    |
47 LL |     #[message = "this message is applied to a String field"]

49 
49 
50 error: `name` doesn't refer to a field on this type
-   --> $DIR/session-derive-errors.rs:102:1
+   --> $DIR/session-derive-errors.rs:103:1
52    |
53 LL | #[message = "This error has a field, and references {name}"]

55 
55 
56 error: invalid format string: expected `'}'` but string was terminated
-   --> $DIR/session-derive-errors.rs:110:1
+   --> $DIR/session-derive-errors.rs:111:1
59 LL | #[error = "E0123"]
60    |               - because of this opening brace


65    = note: this error originates in the derive macro `SessionDiagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)
66 
67 error: invalid format string: unmatched `}` found
-   --> $DIR/session-derive-errors.rs:119:1
+   --> $DIR/session-derive-errors.rs:120:1
69    |
70 LL | #[message = "This is missing an opening brace: name}"]
71    | ^ unmatched `}` in format string

74    = note: this error originates in the derive macro `SessionDiagnostic` (in Nightly builds, run with -Z macro-backtrace for more info)
75 
76 error: The `#[label = ...]` attribute can only be applied to fields of type Span
-   --> $DIR/session-derive-errors.rs:138:5
+   --> $DIR/session-derive-errors.rs:139:5
78    |
79 LL |     #[label = "See here"]

81 
81 
82 error: `nonsense` is not a valid key for `#[suggestion(...)]`
-   --> $DIR/session-derive-errors.rs:163:18
+   --> $DIR/session-derive-errors.rs:164:18
84    |
85 LL |     #[suggestion(nonsense = "This is nonsense")]

87 
87 
88 error: `msg` is not a valid key for `#[suggestion(...)]`
-   --> $DIR/session-derive-errors.rs:171:18
+   --> $DIR/session-derive-errors.rs:172:18
90    |
91 LL |     #[suggestion(msg = "This is a suggestion")]

93 
94 error: missing suggestion message
-   --> $DIR/session-derive-errors.rs:179:7
-   --> $DIR/session-derive-errors.rs:179:7
+   --> $DIR/session-derive-errors.rs:180:7
96    |
97 LL |     #[suggestion(code = "This is suggested code")]


100    = help: provide a suggestion message using #[suggestion(message = "...")]
102 error: wrong field type for suggestion
-   --> $DIR/session-derive-errors.rs:194:5
+   --> $DIR/session-derive-errors.rs:195:5
104    |
104    |
105 LL | /     #[suggestion(message = "This is a message", code = "This is suggested code")]
106 LL | |

110    = help: #[suggestion(...)] should be applied to fields of type Span or (Span, Applicability)
111 
112 error: type of field annotated with `#[suggestion(...)]` contains more than one Span
-   --> $DIR/session-derive-errors.rs:209:5
+   --> $DIR/session-derive-errors.rs:210:5
114    |
115 LL | /     #[suggestion(message = "This is a message", code = "This is suggested code")]
116 LL | |
118    | |___________________________________________^
119 
119 
120 error: type of field annotated with `#[suggestion(...)]` contains more than one Applicability
-   --> $DIR/session-derive-errors.rs:217:5
+   --> $DIR/session-derive-errors.rs:218:5
122    |
123 LL | /     #[suggestion(message = "This is a message", code = "This is suggested code")]
124 LL | |
126    | |____________________________________________________^
127 
127 
128 error: invalid annotation list `#[label(...)]`
-   --> $DIR/session-derive-errors.rs:225:7
+   --> $DIR/session-derive-errors.rs:226:7
130    |
131 LL |     #[label("wrong kind of annotation for label")]


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-derive-errors/session-derive-errors.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-derive-errors/session-derive-errors.stderr
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
  --> /checkout/src/test/ui-fulldeps/session-derive-errors.rs:180:7
   |
   |
LL |     #[suggestion(code = "This is suggested code")]
   |
   |
   = help: provide a suggestion message using #[suggestion(message = "...")]
error: wrong field type for suggestion
  --> /checkout/src/test/ui-fulldeps/session-derive-errors.rs:195:5
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
test result: FAILED. 66 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 12.07s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui-fulldeps" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:42
