plain

---- [ui] ui/macros/macros-nonfatal-errors.rs stdout ----
diff of stderr:

1 error: the `#[default]` attribute may only be used on unit enum variants
-   --> $DIR/macros-nonfatal-errors.rs:14:5
3    |
4 LL |     #[default]
5    |     ^^^^^^^^^^


6 
7 error: the `#[default]` attribute may only be used on unit enum variants
-   --> $DIR/macros-nonfatal-errors.rs:19:36
9    |
9    |
10 LL | struct DefaultInnerAttrTupleStruct(#[default] ());

12 
12 
13 error: the `#[default]` attribute may only be used on unit enum variants
-   --> $DIR/macros-nonfatal-errors.rs:23:1
15    |
16 LL | #[default]
17    | ^^^^^^^^^^


18 
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
19 error: the `#[default]` attribute may only be used on unit enum variants
-   --> $DIR/macros-nonfatal-errors.rs:27:1
21    |
22 LL | #[default]
23    | ^^^^^^^^^^


24 
25 error: the `#[default]` attribute may only be used on unit enum variants
-   --> $DIR/macros-nonfatal-errors.rs:37:11
27    |
27    |
28 LL |     Foo = #[default] 0,

30 
30 
31 error: the `#[default]` attribute may only be used on unit enum variants
-   --> $DIR/macros-nonfatal-errors.rs:38:14
33    |
33    |
34 LL |     Bar([u8; #[default] 1]),

36 
37 error: no default declared
-   --> $DIR/macros-nonfatal-errors.rs:43:10
-   --> $DIR/macros-nonfatal-errors.rs:43:10
+   --> $DIR/macros-nonfatal-errors.rs:45:10
39    |
40 LL | #[derive(Default)]
41    |          ^^^^^^^

44    = note: this error originates in the derive macro `Default` (in Nightly builds, run with -Z macro-backtrace for more info)
46 error: multiple declared defaults
-   --> $DIR/macros-nonfatal-errors.rs:49:10
+   --> $DIR/macros-nonfatal-errors.rs:51:10
48    |
48    |
49 LL | #[derive(Default)]
50    |          ^^^^^^^

62    = note: this error originates in the derive macro `Default` (in Nightly builds, run with -Z macro-backtrace for more info)
63 
64 error: `#[default]` attribute does not accept a value
-   --> $DIR/macros-nonfatal-errors.rs:61:5
66    |
67 LL |     #[default = 1]
68    |     ^^^^^^^^^^^^^^


70    = help: try using `#[default]`
71 
72 error: multiple `#[default]` attributes
-   --> $DIR/macros-nonfatal-errors.rs:69:5
74    |
75 LL |     #[default]
75 LL |     #[default]
76    |     ---------- `#[default]` used here
81    |
81    |
82    = note: only one `#[default]` attribute is needed
-   --> $DIR/macros-nonfatal-errors.rs:68:5
+   --> $DIR/macros-nonfatal-errors.rs:70:5
85    |
86 LL |     #[default]
86 LL |     #[default]
87    |     ^^^^^^^^^^

88 
89 error: multiple `#[default]` attributes
-   --> $DIR/macros-nonfatal-errors.rs:79:5
91    |
92 LL |     #[default]
92 LL |     #[default]
93    |     ---------- `#[default]` used here
99    |
99    |
100    = note: only one `#[default]` attribute is needed
-   --> $DIR/macros-nonfatal-errors.rs:76:5
+   --> $DIR/macros-nonfatal-errors.rs:78:5
103    |
104 LL |     #[default]
104 LL |     #[default]
105    |     ^^^^^^^^^^

109    |     ^^^^^^^^^^
110 
111 error: the `#[default]` attribute may only be used on unit enum variants
-   --> $DIR/macros-nonfatal-errors.rs:86:5
113    |
114 LL |     Foo {},
115    |     ^^^


117    = help: consider a manual implementation of `Default`
118 
119 error: default variant must be exhaustive
-   --> $DIR/macros-nonfatal-errors.rs:94:5
121    |
121    |
122 LL |     #[non_exhaustive]
123    |     ----------------- declared `#[non_exhaustive]` here
126    |
126    |
127    = help: consider a manual implementation of `Default`
+ error: asm template must be a string literal
+   --> $DIR/macros-nonfatal-errors.rs:101:10
+    |
+    |
+ LL |     asm!(invalid);
+ 
129 error: inline assembly must be a string literal
-   --> $DIR/macros-nonfatal-errors.rs:100:15
+   --> $DIR/macros-nonfatal-errors.rs:102:15
+   --> $DIR/macros-nonfatal-errors.rs:102:15
131    |
132 LL |     llvm_asm!(invalid);

134 
134 
135 error: concat_idents! requires ident args
-   --> $DIR/macros-nonfatal-errors.rs:102:5
137    |
137    |
138 LL |     concat_idents!("not", "idents");

140 
141 error: argument must be a string literal
-   --> $DIR/macros-nonfatal-errors.rs:104:17
-   --> $DIR/macros-nonfatal-errors.rs:104:17
+   --> $DIR/macros-nonfatal-errors.rs:106:17
143    |
144 LL |     option_env!(invalid);

146 
147 error: expected string literal
-   --> $DIR/macros-nonfatal-errors.rs:105:10
-   --> $DIR/macros-nonfatal-errors.rs:105:10
+   --> $DIR/macros-nonfatal-errors.rs:107:10
149    |
150 LL |     env!(invalid);

152 
153 error: expected string literal
-   --> $DIR/macros-nonfatal-errors.rs:106:10
-   --> $DIR/macros-nonfatal-errors.rs:106:10
+   --> $DIR/macros-nonfatal-errors.rs:108:10
155    |
156 LL |     env!(foo, abr, baz);

158 
158 
159 error: environment variable `RUST_HOPEFULLY_THIS_DOESNT_EXIST` not defined
-   --> $DIR/macros-nonfatal-errors.rs:107:5
161    |
161    |
162 LL |     env!("RUST_HOPEFULLY_THIS_DOESNT_EXIST");

165    = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)
166 
167 error: format argument must be a string literal
167 error: format argument must be a string literal
-   --> $DIR/macros-nonfatal-errors.rs:109:13
+   --> $DIR/macros-nonfatal-errors.rs:111:13
169    |
170 LL |     format!(invalid);

176    |             +++++
177 
178 error: argument must be a string literal
178 error: argument must be a string literal
-   --> $DIR/macros-nonfatal-errors.rs:111:14
+   --> $DIR/macros-nonfatal-errors.rs:113:14
180    |
181 LL |     include!(invalid);

183 
184 error: argument must be a string literal
-   --> $DIR/macros-nonfatal-errors.rs:113:18
-   --> $DIR/macros-nonfatal-errors.rs:113:18
+   --> $DIR/macros-nonfatal-errors.rs:115:18
186    |
187 LL |     include_str!(invalid);

189 
189 
190 error: couldn't read $DIR/i'd be quite surprised if a file with this name existed: $FILE_NOT_FOUND_MSG (os error 2)
-   --> $DIR/macros-nonfatal-errors.rs:114:5
192    |
192    |
193 LL |     include_str!("i'd be quite surprised if a file with this name existed");


196    = note: this error originates in the macro `include_str` (in Nightly builds, run with -Z macro-backtrace for more info)
198 error: argument must be a string literal
-   --> $DIR/macros-nonfatal-errors.rs:115:20
+   --> $DIR/macros-nonfatal-errors.rs:117:20
200    |
200    |
201 LL |     include_bytes!(invalid);

203 
203 
204 error: couldn't read $DIR/i'd be quite surprised if a file with this name existed: $FILE_NOT_FOUND_MSG (os error 2)
-   --> $DIR/macros-nonfatal-errors.rs:116:5
206    |
206    |
207 LL |     include_bytes!("i'd be quite surprised if a file with this name existed");

210    = note: this error originates in the macro `include_bytes` (in Nightly builds, run with -Z macro-backtrace for more info)
211 
211 
212 error: trace_macros! accepts only `true` or `false`
-   --> $DIR/macros-nonfatal-errors.rs:118:5
214    |
214    |
215 LL |     trace_macros!(invalid);

- 
- 
- error: cannot find macro `asm` in this scope
-   --> $DIR/macros-nonfatal-errors.rs:99:5
-    |
- LL |     asm!(invalid);
-    |
-    = note: consider importing this macro:
-            std::arch::asm
226 
---
To only update this specific test, also pass `--test-args macros/macros-nonfatal-errors.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macros-nonfatal-errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macros-nonfatal-errors" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macros-nonfatal-errors/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: the `#[default]` attribute may only be used on unit enum variants
   |
   |
LL |     #[default] //~ ERROR the `#[default]` attribute may only be used on unit enum variants


error: the `#[default]` attribute may only be used on unit enum variants
   |
   |
LL | struct DefaultInnerAttrTupleStruct(#[default] ());


error: the `#[default]` attribute may only be used on unit enum variants
   |
   |
LL | #[default] //~ ERROR the `#[default]` attribute may only be used on unit enum variants


error: the `#[default]` attribute may only be used on unit enum variants
   |
   |
LL | #[default] //~ ERROR the `#[default]` attribute may only be used on unit enum variants


error: the `#[default]` attribute may only be used on unit enum variants
   |
   |
LL |     Foo = #[default] 0, //~ ERROR the `#[default]` attribute may only be used on unit enum variants


error: the `#[default]` attribute may only be used on unit enum variants
   |
   |
LL |     Bar([u8; #[default] 1]), //~ ERROR the `#[default]` attribute may only be used on unit enum variants

error: no default declared
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:45:10
   |
   |
LL | #[derive(Default)] //~ ERROR no default declared
   |
   |
   = help: make a unit variant default by placing `#[default]` above it
   = note: this error originates in the derive macro `Default` (in Nightly builds, run with -Z macro-backtrace for more info)
error: multiple declared defaults
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:51:10
   |
   |
LL | #[derive(Default)] //~ ERROR multiple declared defaults
...
LL |     Foo,
   |     --- first default
LL |     #[default]
LL |     #[default]
LL |     Bar,
   |     --- additional default
LL |     #[default]
LL |     Baz,
   |     --- additional default
   |
   = note: only one variant can be default
   = note: this error originates in the derive macro `Default` (in Nightly builds, run with -Z macro-backtrace for more info)

error: `#[default]` attribute does not accept a value
   |
   |
LL |     #[default = 1] //~ ERROR `#[default]` attribute does not accept a value
   |
   |
   = help: try using `#[default]`

error: multiple `#[default]` attributes
   |
LL |     #[default]
LL |     #[default]
   |     ---------- `#[default]` used here
LL |     #[default]
   |     ---------- `#[default]` used again here
LL |     Foo, //~ERROR multiple `#[default]` attributes
   |
   |
   = note: only one `#[default]` attribute is needed
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:70:5
   |
LL |     #[default]
   |     ^^^^^^^^^^
   |     ^^^^^^^^^^

error: multiple `#[default]` attributes
   |
LL |     #[default]
LL |     #[default]
   |     ---------- `#[default]` used here
LL |     #[default]
   |     ---------- `#[default]` used again here
...
LL |     Foo, //~ERROR multiple `#[default]` attributes
   |
   |
   = note: only one `#[default]` attribute is needed
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:78:5
   |
LL |     #[default]
   |     ^^^^^^^^^^
   |     ^^^^^^^^^^
LL |     #[default]
   |     ^^^^^^^^^^
LL |     #[default]
   |     ^^^^^^^^^^

error: the `#[default]` attribute may only be used on unit enum variants
   |
   |
LL |     Foo {}, //~ ERROR the `#[default]` attribute may only be used on unit enum variants
   |
   |
   = help: consider a manual implementation of `Default`

error: default variant must be exhaustive
   |
   |
LL |     #[non_exhaustive]
   |     ----------------- declared `#[non_exhaustive]` here
LL |     Foo, //~ ERROR default variant must be exhaustive
   |
   |
   = help: consider a manual implementation of `Default`
error: asm template must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:101:10
   |
   |
LL |     asm!(invalid); //~ ERROR

error: inline assembly must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:102:15
   |
   |
LL |     llvm_asm!(invalid); //~ ERROR


error: concat_idents! requires ident args
   |
   |
LL |     concat_idents!("not", "idents"); //~ ERROR

error: argument must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:106:17
   |
   |
LL |     option_env!(invalid); //~ ERROR

error: expected string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:107:10
   |
   |
LL |     env!(invalid); //~ ERROR

error: expected string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:108:10
   |
   |
LL |     env!(foo, abr, baz); //~ ERROR


error: environment variable `RUST_HOPEFULLY_THIS_DOESNT_EXIST` not defined
   |
   |
LL |     env!("RUST_HOPEFULLY_THIS_DOESNT_EXIST"); //~ ERROR
   |
   = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)

error: format argument must be a string literal
error: format argument must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:111:13
   |
LL |     format!(invalid); //~ ERROR
   |
help: you might be missing a string literal to format with
   |
   |
LL |     format!("{}", invalid); //~ ERROR

error: argument must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:113:14
   |
   |
LL |     include!(invalid); //~ ERROR

error: argument must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:115:18
   |
   |
LL |     include_str!(invalid); //~ ERROR


error: couldn't read /checkout/src/test/ui/macros/i'd be quite surprised if a file with this name existed: No such file or directory (os error 2)
   |
   |
LL |     include_str!("i'd be quite surprised if a file with this name existed"); //~ ERROR
   |
   |
   = note: this error originates in the macro `include_str` (in Nightly builds, run with -Z macro-backtrace for more info)
error: argument must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:117:20
   |
   |
LL |     include_bytes!(invalid); //~ ERROR


error: couldn't read /checkout/src/test/ui/macros/i'd be quite surprised if a file with this name existed: No such file or directory (os error 2)
   |
   |
LL |     include_bytes!("i'd be quite surprised if a file with this name existed"); //~ ERROR
   |
   = note: this error originates in the macro `include_bytes` (in Nightly builds, run with -Z macro-backtrace for more info)


error: trace_macros! accepts only `true` or `false`
   |
   |
LL |     trace_macros!(invalid); //~ ERROR

error: aborting due to 27 previous errors


---
test result: FAILED. 12351 passed; 1 failed; 118 ignored; 0 measured; 0 filtered out; finished in 145.24s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:40
