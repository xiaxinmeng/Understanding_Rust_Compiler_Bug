plain
GITHUB_ENV=/home/runner/work/_temp/_runner_file_commands/set_env_e4ebcea5-4718-466f-aa2d-227461018f6c
GITHUB_EVENT_NAME=pull_request
GITHUB_EVENT_PATH=/home/runner/work/_temp/_github_workflow/event.json
GITHUB_GRAPHQL_URL=https://api.github.com/graphql
GITHUB_HEAD_REF=pattORns-2
GITHUB_JOB=pr
GITHUB_PATH=/home/runner/work/_temp/_runner_file_commands/add_path_e4ebcea5-4718-466f-aa2d-227461018f6c
GITHUB_REF=refs/pull/80100/merge
GITHUB_REPOSITORY_OWNER=rust-lang
GITHUB_RETENTION_DAYS=90
GITHUB_RUN_ID=429046233
GITHUB_RUN_NUMBER=21581
---
Building wheels for collected packages: PyYAML
  Running setup.py bdist_wheel for PyYAML: started
  Running setup.py bdist_wheel for PyYAML: finished with status 'error'
  Failed building wheel for PyYAML
  Complete output from command /usr/bin/python3 -u -c "import setuptools, tokenize;__file__='/tmp/pip-build-ojyvanx4/PyYAML/setup.py';f=getattr(tokenize, 'open', open)(__file__);code=f.read().replace('\r\n', '\n');f.close();exec(compile(code, __file__, 'exec'))" bdist_wheel -d /tmp/tmpb422usgnpip-wheel- --python-tag cp36:
     or: -c --help [cmd1 cmd2 ...]
     or: -c --help-commands
     or: -c cmd --help
  
---
.................................................................................................... 7300/11180
.................................................................................................... 7400/11180
....................i..ii...........................................................ii.............. 7500/11180
.................................................................................................... 7600/11180
.....i..F........................................................................................... 7700/11180
.................................................................................................... 7900/11180
.................................................................................................... 8000/11180
............................i....................................................................... 8100/11180
.................................................................................................... 8200/11180
---
.................................................................................................... 9000/11180
.................................................................................................... 9100/11180
........................................................................i......i.................... 9200/11180
.................................................................................................... 9300/11180
...........iiiiii..iiiiii.i......................................................................... 9400/11180
.................................................................................................... 9600/11180
.................................................................................................... 9700/11180
..............................................................................................test [ui] ui/issues/issue-74564-if-expr-stack-overflow.rs has been running for over 60 seconds
...... 9800/11180
---

---- [ui] ui/or-patterns/or-patterns-syntactic-fail-2018.rs stdout ----
diff of stderr:

1 error: no rules expected the token `|`
-   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail-2018.rs:14:15
+   --> $DIR/or-patterns-syntactic-fail-2018.rs:14:15
3    |
4 LL | macro_rules! accept_pat {
5    | ----------------------- when calling this macro
6 ...
6 ...
- LL | accept_pat!(p | q); //~ ERROR no rules expected the token `|`
+ LL | accept_pat!(p | q);
8    |               ^ no rules expected this token in macro call
9 
10 error: no rules expected the token `|`
-   --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail-2018.rs:15:13
+   --> $DIR/or-patterns-syntactic-fail-2018.rs:15:13
12    |
12    |
13 LL | macro_rules! accept_pat {
14    | ----------------------- when calling this macro
15 ...
15 ...
- LL | accept_pat!(|p| q); //~ ERROR no rules expected the token `|`
+ LL | accept_pat!(|p| q);
17    |             ^ no rules expected this token in macro call
18 error: aborting due to 2 previous errors
19 
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-fail-2018/or-patterns-syntactic-fail-2018.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args or-patterns/or-patterns-syntactic-fail-2018.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail-2018.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-fail-2018" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-fail-2018/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: no rules expected the token `|`
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail-2018.rs:14:15
   |
LL | macro_rules! accept_pat {
   | ----------------------- when calling this macro
...
LL | accept_pat!(p | q); //~ ERROR no rules expected the token `|`
   |               ^ no rules expected this token in macro call

error: no rules expected the token `|`
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail-2018.rs:15:13
   |
LL | macro_rules! accept_pat {
   | ----------------------- when calling this macro
...
LL | accept_pat!(|p| q); //~ ERROR no rules expected the token `|`
   |             ^ no rules expected this token in macro call
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/pattern/or-pattern-macro-pat.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/or-pattern-macro-pat.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/or-pattern-macro-pat/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/or-pattern-macro-pat/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: argument for `--edition` must be one of: 2015|2018. (instead was `2021`)

------------------------------------------


---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:17:55
