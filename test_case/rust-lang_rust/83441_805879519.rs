plain
.................................................................................................... 9300/11704
.................................................................................................... 9400/11704
.................................................................................................... 9500/11704
...............................................i......i............................................. 9600/11704
.............................................................................................iiiiiii 9700/11704
..iiiiii.i.......................................................................................... 9800/11704
.................................................................................................... 10000/11704
.................................................................................................... 10100/11704
.................................................................................................... 10200/11704
.................................................................................................... 10300/11704
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.111 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 11.106 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.40s

 finished in 2.465 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---

---- [ui] ui-fulldeps/session-derive-errors.rs stdout ----
diff of stderr:

1 error: `#[derive(SessionDiagnostic)]` can only be used on structs
2   --> $DIR/session-derive-errors.rs:28:1
3    |
- LL | / #[error = "E0123"]
- LL | |
- LL | | enum SessionDiagnosticOnEnum {
- LL | |     Foo,
- LL | |     Bar,
- LL | | }
-    | |_^
+ LL | #[error = "E0123"]
11 
11 
12 error: `#[label = ...]` is not a valid SessionDiagnostic struct attribute
13   --> $DIR/session-derive-errors.rs:37:1
14    |
14    |
15 LL | #[label = "This is in the wrong place"]
+    | ^
17 
17 
18 error: `#[suggestion = ...]` is not a valid SessionDiagnostic field attribute
19   --> $DIR/session-derive-errors.rs:44:5
20    |
20    |
21 LL |     #[suggestion = "this is the wrong kind of attribute"]
+    |     ^
23 
23 
24 error: `error` specified multiple times
25   --> $DIR/session-derive-errors.rs:52:11
37   --> $DIR/session-derive-errors.rs:67:1
38    |
38    |
39 LL | struct ErrorCodeNotProvided {}
+    | ^^^^^^
41    |
41    |
42    = help: use the [code = "..."] attribute to set this diagnostic's error code 

45   --> $DIR/session-derive-errors.rs:95:5
46    |
46    |
47 LL |     #[message = "this message is applied to a String field"]
+    |     ^
49 
49 
50 error: `name` doesn't refer to a field on this type
51   --> $DIR/session-derive-errors.rs:102:1
52    |
52    |
53 LL | #[message = "This error has a field, and references {name}"]
+    | ^
55 
55 
56 error: invalid format string: expected `'}'` but string was terminated
57   --> $DIR/session-derive-errors.rs:110:1
77   --> $DIR/session-derive-errors.rs:138:5
78    |
78    |
79 LL |     #[label = "See here"]
+    |     ^
81 
81 
82 error: `nonsense` is not a valid key for `#[suggestion(...)]`
83   --> $DIR/session-derive-errors.rs:163:18
84    |
84    |
85 LL |     #[suggestion(nonsense = "This is nonsense")]
+    |                  ^^^^^^^^
87 
87 
88 error: `msg` is not a valid key for `#[suggestion(...)]`
89   --> $DIR/session-derive-errors.rs:171:18
90    |
90    |
91 LL |     #[suggestion(msg = "This is a suggestion")]
+    |                  ^^^
93 
94 error: missing suggestion message
95   --> $DIR/session-derive-errors.rs:179:7
95   --> $DIR/session-derive-errors.rs:179:7

96    |
97 LL |     #[suggestion(code = "This is suggested code")]
+    |       ^^^^^^^^^^
99    |
99    |
100    = help: provide a suggestion message using #[suggestion(message = "...")]

102 error: wrong field type for suggestion
103   --> $DIR/session-derive-errors.rs:194:5
104    |
104    |
- LL | /     #[suggestion(message = "This is a message", code = "This is suggested code")]
- LL | |
- LL | |     suggestion: Applicability,
-    | |_____________________________^
+ LL |     #[suggestion(message = "This is a message", code = "This is suggested code")]
109    |
109    |
110    = help: #[suggestion(...)] should be applied to fields of type Span or (Span, Applicability)


112 error: type of field annotated with `#[suggestion(...)]` contains more than one Span
113   --> $DIR/session-derive-errors.rs:209:5
114    |
- LL | /     #[suggestion(message = "This is a message", code = "This is suggested code")]
- LL | |
- LL | |     suggestion: (Span, Span, Applicability),
-    | |___________________________________________^
+ LL |     #[suggestion(message = "This is a message", code = "This is suggested code")]
119 
119 
120 error: type of field annotated with `#[suggestion(...)]` contains more than one Applicability
121   --> $DIR/session-derive-errors.rs:217:5
122    |
122    |
- LL | /     #[suggestion(message = "This is a message", code = "This is suggested code")]
- LL | |
- LL | |     suggestion: (Applicability, Applicability, Span),
-    | |____________________________________________________^
+ LL |     #[suggestion(message = "This is a message", code = "This is suggested code")]
127 
127 
128 error: invalid annotation list `#[label(...)]`
129   --> $DIR/session-derive-errors.rs:225:7
130    |
130    |
131 LL |     #[label("wrong kind of annotation for label")]
+    |       ^^^^^
133 
134 error: aborting due to 18 previous errors
135 
135 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-derive-errors/session-derive-errors.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args session-derive-errors.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/session-derive-errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-derive-errors" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/session-derive-errors/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `#[derive(SessionDiagnostic)]` can only be used on structs
   |
LL | #[error = "E0123"]
   | ^


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
   = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: invalid format string: unmatched `}` found
   |
   |
LL | #[message = "This is missing an opening brace: name}"]
   | ^ unmatched `}` in format string
   |
   = note: if you intended to print `}`, you can escape it using `}}`
   = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)

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
LL |     #[suggestion(message = "This is a message", code = "This is suggested code")]
   |
   |
   = help: #[suggestion(...)] should be applied to fields of type Span or (Span, Applicability)

error: type of field annotated with `#[suggestion(...)]` contains more than one Span
   |
   |
LL |     #[suggestion(message = "This is a message", code = "This is suggested code")]


error: type of field annotated with `#[suggestion(...)]` contains more than one Applicability
   |
   |
LL |     #[suggestion(message = "This is a message", code = "This is suggested code")]


error: invalid annotation list `#[label(...)]`
   |
   |
LL |     #[label("wrong kind of annotation for label")]

error: aborting due to 18 previous errors


---
test result: FAILED. 66 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 9.23s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui-fulldeps" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:42
