plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +4cf8af1623bd60bf128e26cfa380aa0c115c3125:refs/remotes/pull/80100/merge
---
.................................................................................................... 7300/11173
.................................................................................................... 7400/11173
..................i..ii...........................................................ii................ 7500/11173
.................................................................................................... 7600/11173
...F..F............................................................................................. 7700/11173
.................................................................................................... 7900/11173
.................................................................................................... 8000/11173
..........................i......................................................................... 8100/11173
.................................................................................................... 8200/11173
---
.................................................................................................... 9000/11173
.................................................................................................... 9100/11173
..................................................................i......i.......................... 9200/11173
.................................................................................................... 9300/11173
.....iiiiii..iiiiii.i............................................................................... 9400/11173
.................................................................................................... 9600/11173
.................................................................................................... 9700/11173
.................................................................................................... 9800/11173
.................................................................................................... 9900/11173
---
failures:

---- [ui] ui/or-patterns/or-patterns-syntactic-pass-2021.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/or-patterns-syntactic-pass-2021.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-pass-2021/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-pass-2021/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: argument for `--edition` must be one of: 2015|2018. (instead was `2021`)

------------------------------------------



---- [ui] ui/or-patterns/or-patterns-syntactic-fail-2018.rs stdout ----
normalized stderr:
error: no rules expected the token `|`
  --> $DIR/or-patterns-syntactic-fail-2018.rs:14:15
   |
LL | macro_rules! accept_pat {
   | ----------------------- when calling this macro
...
LL | accept_pat!(p | q);
   |               ^ no rules expected this token in macro call

error: no rules expected the token `|`
  --> $DIR/or-patterns-syntactic-fail-2018.rs:15:13
   |
LL | macro_rules! accept_pat {
   | ----------------------- when calling this macro
...
LL | accept_pat!(|p| q);
   |             ^ no rules expected this token in macro call
error: aborting due to 2 previous errors





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
Build completed unsuccessfully in 0:16:47
